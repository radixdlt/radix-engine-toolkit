// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use radix_engine::blueprints::consensus_manager::ValidatorSubstate;
use radix_engine::system::system_modules::execution_trace::ResourceSpecifier;
use radix_engine::transaction::*;
use radix_engine_interface::blueprints::account::*;
use radix_engine_interface::blueprints::consensus_manager::*;
use radix_engine_toolkit_core::functions::execution::{self, *};
use radix_engine_toolkit_core::instruction_visitor::visitors::transaction_type::general_transaction_visitor::*;
use radix_engine_toolkit_core::instruction_visitor::visitors::transaction_type::transfer_visitor::*;
use scrypto::prelude::*;
use scrypto_unit::*;
use transaction::prelude::*;
use radix_engine_toolkit_core::instruction_visitor::core::traverser::*;
use radix_engine_toolkit_core::instruction_visitor::visitors::transaction_type::stake_visitor::*;

#[test]
fn simple_transfer_is_picked_up_as_a_simple_account_transfer_transaction() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();
    let (public_key1, _, account1) = test_runner.new_account(true);
    let (public_key2, _, account2) = test_runner.new_account(true);

    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account1, XRD, dec!("10"))
        .take_from_worktop(XRD, dec!("10"), "bucket")
        .try_deposit_or_abort(account2, None, "bucket")
        .build();
    let receipt = test_runner.preview_manifest(
        manifest.clone(),
        vec![public_key1.into(), public_key2.into()],
        0,
        PreviewFlags {
            use_free_credit: true,
            assume_all_signature_proofs: true,
            skip_epoch_check: true,
        },
    );
    receipt.expect_commit_success();

    // Act
    let transaction_type = transaction_types(&manifest.instructions, &receipt)
        .into_iter()
        .find(|transaction_type| matches!(transaction_type, TransactionType::SimpleTransfer(..)))
        .unwrap();

    // Assert
    assert_eq!(
        transaction_type,
        TransactionType::SimpleTransfer(Box::new(SimpleTransferTransactionType {
            from: account1,
            to: account2,
            transferred: ResourceSpecifier::Amount(XRD, dec!("10"))
        }))
    )
}

#[test]
fn transfer_is_picked_up_as_an_account_transfer_transaction() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();
    let (public_key1, _, account1) = test_runner.new_account(true);
    let (public_key2, _, account2) = test_runner.new_account(true);
    let (public_key3, _, account3) = test_runner.new_account(true);

    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account1, XRD, dec!("20"))
        .take_from_worktop(XRD, dec!("10"), "bucket")
        .try_deposit_or_abort(account2, None, "bucket")
        .take_from_worktop(XRD, dec!("10"), "bucket2")
        .try_deposit_or_abort(account3, None, "bucket2")
        .build();
    let receipt = test_runner.preview_manifest(
        manifest.clone(),
        vec![public_key1.into(), public_key2.into(), public_key3.into()],
        0,
        PreviewFlags {
            use_free_credit: true,
            assume_all_signature_proofs: true,
            skip_epoch_check: true,
        },
    );
    receipt.expect_commit_success();

    // Act
    let transaction_type = transaction_types(&manifest.instructions, &receipt)
        .into_iter()
        .find(|transaction_type| matches!(transaction_type, TransactionType::Transfer(..)))
        .unwrap();

    // Assert
    assert_eq!(
        transaction_type,
        TransactionType::Transfer(Box::new(TransferTransactionType {
            from: account1,
            transfers: hashmap! {
                account2 => hashmap! {
                    XRD => Resources::Amount(dec!("10")),
                },
                account3 => hashmap! {
                    XRD => Resources::Amount(dec!("10")),
                }
            }
        }))
    )
}

#[test]
fn complex_transfer_is_picked_up_as_an_general_transaction() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();
    let (public_key1, _, account1) = test_runner.new_account(true);
    let (public_key2, _, account2) = test_runner.new_account(true);
    let (public_key3, _, account3) = test_runner.new_account(true);

    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account1, XRD, dec!("10"))
        .withdraw_from_account(account2, XRD, dec!("10"))
        .take_from_worktop(XRD, dec!("10"), "bucket")
        .try_deposit_or_abort(account2, None, "bucket")
        .take_from_worktop(XRD, dec!("10"), "bucket1")
        .try_deposit_or_abort(account3, None, "bucket1")
        .build();
    let receipt = test_runner.preview_manifest(
        manifest.clone(),
        vec![public_key1.into(), public_key2.into(), public_key3.into()],
        0,
        PreviewFlags {
            use_free_credit: true,
            assume_all_signature_proofs: true,
            skip_epoch_check: true,
        },
    );
    receipt.expect_commit_success();

    // Act
    let transaction_type = transaction_types(&manifest.instructions, &receipt)
        .into_iter()
        .find(|transaction_type| {
            matches!(transaction_type, TransactionType::GeneralTransaction(..))
        })
        .unwrap();

    // Assert
    assert!(matches!(
        transaction_type,
        TransactionType::GeneralTransaction(..)
    ))
}

#[test]
fn general_transaction_handles_take_non_fungible_ids_from_worktop_correctly() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().without_trace().build();
    let (public_key1, _, account1) = test_runner.new_account(false);
    let (public_key2, _, account2) = test_runner.new_account(false);
    let manifest = ManifestBuilder::new()
        .create_non_fungible_resource(
            OwnerRole::None,
            NonFungibleIdType::Integer,
            true,
            NonFungibleResourceRoles::default(),
            metadata! {
                init {
                    "name" => true, locked;
                }
            },
            Some(btreemap!(
                NonFungibleLocalId::integer(1) => (),
                NonFungibleLocalId::integer(2) => (),
            )),
        )
        .try_deposit_entire_worktop_or_abort(account1, None)
        .build();
    let resource_address = *test_runner
        .execute_manifest_ignoring_fee(manifest, vec![])
        .expect_commit_success()
        .new_resource_addresses()
        .first()
        .unwrap();

    // Act
    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account1, resource_address, 2)
        .take_from_worktop(resource_address, 2, "bucket")
        .with_bucket("bucket", |builder, bucket| {
            builder.try_deposit_or_abort(account2, None, bucket)
        })
        .drop_all_proofs()
        .build();
    let receipt = test_runner.preview_manifest(
        manifest.clone(),
        vec![public_key1.into(), public_key2.into()],
        0,
        PreviewFlags {
            use_free_credit: true,
            assume_all_signature_proofs: true,
            skip_epoch_check: true,
        },
    );
    let transaction_type = transaction_types(&manifest.instructions, &receipt)
        .into_iter()
        .find_map(|transaction_type| match transaction_type {
            TransactionType::GeneralTransaction(general_transaction) => Some(general_transaction),
            _ => None,
        })
        .unwrap();

    // Assert
    assert_eq!(
        transaction_type.account_withdraws,
        hashmap! {
            account1 => vec![
                ResourceTracker::NonFungible {
                    resource_address,
                    amount: Source::Guaranteed(dec!("2")),
                    ids: Source::Predicted(0, IndexSet::from([
                        NonFungibleLocalId::integer(1),
                        NonFungibleLocalId::integer(2),
                    ]))
                }
            ]
        }
    );
    assert_eq!(
        transaction_type.account_deposits,
        hashmap! {
            account2 => vec![
                ResourceTracker::NonFungible {
                    resource_address,
                    amount: Source::Guaranteed(dec!("2")),
                    ids: Source::Predicted(1, IndexSet::from([
                        NonFungibleLocalId::integer(1),
                        NonFungibleLocalId::integer(2),
                    ]))
                }
            ]
        }
    );
}

#[test]
pub fn deposit_and_deposit_batch_of_nothing_should_not_result_in_an_error() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();
    let (_, _, account) = test_runner.new_account(true);

    let manifest = ManifestBuilder::new().deposit_batch(account).build();
    let receipt = test_runner.preview_manifest(
        manifest.clone(),
        vec![],
        0,
        PreviewFlags {
            use_free_credit: true,
            assume_all_signature_proofs: true,
            skip_epoch_check: true,
        },
    );

    // Act
    let tx_types = transaction_types(&manifest.instructions, &receipt);

    // Assert
    assert!(!tx_types.is_empty());
}

fn test_manifest_with_lock_fee(
    method_name: impl Into<String>,
    arguments: impl ResolvableArguments,
) {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();
    let (pk, _, account) = test_runner.new_account(true);

    let manifest = ManifestBuilder::new()
        .call_method(account, method_name, arguments)
        .withdraw_from_account(account, XRD, 10)
        .take_from_worktop(XRD, 10, "bucket")
        .try_deposit_or_abort(account, None, "bucket")
        .build();
    let receipt = test_runner.preview_manifest(
        manifest.clone(),
        vec![pk.into()],
        0,
        PreviewFlags {
            use_free_credit: true,
            assume_all_signature_proofs: true,
            skip_epoch_check: true,
        },
    );
    receipt.expect_commit_success();

    // Act
    let transaction_types = transaction_types(
        &manifest.instructions,
        &ExecutionAnalysisTransactionReceipt::new(&VersionedTransactionReceipt::V1(receipt))
            .unwrap(),
    );

    // Assert
    assert_eq!(transaction_types.len(), 0)
}

#[test]
fn manifest_with_a_lock_fee_should_not_be_conforming() {
    test_manifest_with_lock_fee(
        ACCOUNT_LOCK_FEE_IDENT,
        AccountLockFeeInput { amount: dec!("1") },
    )
}

#[test]
fn manifest_with_a_lock_contingent_fee_should_not_be_conforming() {
    test_manifest_with_lock_fee(
        ACCOUNT_LOCK_CONTINGENT_FEE_IDENT,
        AccountLockContingentFeeInput { amount: dec!("1") },
    )
}

#[test]
fn simple_stake_transaction_is_detected_by_the_stake_visitor() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();
    let (_, _, account1) = test_runner.new_account(false);
    let (_, _, validator1, stake_unit1, _) = new_registered_validator(&mut test_runner);

    // Act
    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account1, XRD, 100)
        .take_from_worktop(XRD, 100, "XRD")
        .stake_validator(validator1, "XRD")
        .deposit_batch(account1)
        .build();
    let stakes = execute_and_run_stake_visitor(manifest, &mut test_runner);

    // Assert
    let stakes = stakes.expect("Must be valid!");
    let [stake1] = stakes.as_slice() else {
        panic!("Unexpected number of stakes!")
    };
    assert_eq!(
        *stake1,
        StakeInformation {
            from_account: account1,
            validator_address: validator1,
            stake_unit_resource: stake_unit1,
            stake_unit_amount: 100.into(),
            staked_xrd: 100.into(),
        }
    )
}

#[test]
fn simple_stake_transaction_using_take_all_from_worktop_deposit_is_detected_by_the_stake_visitor() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();
    let (_, _, account1) = test_runner.new_account(false);
    let (_, _, validator1, stake_unit1, _) = new_registered_validator(&mut test_runner);

    // Act
    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account1, XRD, 100)
        .take_from_worktop(XRD, 100, "XRD")
        .stake_validator(validator1, "XRD")
        .take_all_from_worktop(stake_unit1, "StakeUnit1")
        .deposit(account1, "StakeUnit1")
        .build();
    let stakes = execute_and_run_stake_visitor(manifest, &mut test_runner);

    // Assert
    let stakes = stakes.expect("Must be valid!");
    let [stake1] = stakes.as_slice() else {
        panic!("Unexpected number of stakes!")
    };
    assert_eq!(
        *stake1,
        StakeInformation {
            from_account: account1,
            validator_address: validator1,
            stake_unit_resource: stake_unit1,
            stake_unit_amount: 100.into(),
            staked_xrd: 100.into(),
        }
    )
}

#[test]
fn stake_with_multi_withdraw_and_multi_deposits_is_detected_as_stake_by_stake_visitor() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();

    let (_, _, account1) = test_runner.new_account(false);
    let (_, _, validator1, stake_unit1, _) = new_registered_validator(&mut test_runner);

    let (_, _, account2) = test_runner.new_account(false);
    let (_, _, validator2, stake_unit2, _) = new_registered_validator(&mut test_runner);

    // Act
    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account1, XRD, 100)
        .take_from_worktop(XRD, 100, "XRD1")
        .stake_validator(validator1, "XRD1")
        .deposit_batch(account1)
        .withdraw_from_account(account2, XRD, 200)
        .take_from_worktop(XRD, 200, "XRD2")
        .stake_validator(validator2, "XRD2")
        .deposit_batch(account2)
        .build();
    let stakes = execute_and_run_stake_visitor(manifest, &mut test_runner);

    // Assert
    let stakes = stakes.expect("Must be valid!");
    let [stake1, stake2] = stakes.as_slice() else {
        panic!("Unexpected number of stakes!")
    };
    assert_eq!(
        *stake1,
        StakeInformation {
            from_account: account1,
            validator_address: validator1,
            stake_unit_resource: stake_unit1,
            stake_unit_amount: 100.into(),
            staked_xrd: 100.into(),
        }
    );
    assert_eq!(
        *stake2,
        StakeInformation {
            from_account: account2,
            validator_address: validator2,
            stake_unit_resource: stake_unit2,
            stake_unit_amount: 200.into(),
            staked_xrd: 200.into(),
        }
    )
}

#[test]
fn staking_from_one_account_to_multiple_validators_is_detected_as_a_stake_transaction() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();
    let (_, _, account1) = test_runner.new_account(false);
    let (_, _, validator1, stake_unit1, _) = new_registered_validator(&mut test_runner);
    let (_, _, validator2, stake_unit2, _) = new_registered_validator(&mut test_runner);

    // Act
    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account1, XRD, 100)
        .take_from_worktop(XRD, 50, "XRD1")
        .stake_validator(validator1, "XRD1")
        .take_from_worktop(XRD, 50, "XRD2")
        .stake_validator(validator2, "XRD2")
        .deposit_batch(account1)
        .build();
    let stakes = execute_and_run_stake_visitor(manifest, &mut test_runner);

    // Assert
    let stakes = stakes.expect("Must be valid!");
    let [stake1, stake2] = stakes.as_slice() else {
        panic!("Unexpected number of stakes!")
    };
    assert_eq!(
        *stake1,
        StakeInformation {
            from_account: account1,
            validator_address: validator1,
            stake_unit_resource: stake_unit1,
            stake_unit_amount: 50.into(),
            staked_xrd: 50.into(),
        }
    );
    assert_eq!(
        *stake2,
        StakeInformation {
            from_account: account1,
            validator_address: validator2,
            stake_unit_resource: stake_unit2,
            stake_unit_amount: 50.into(),
            staked_xrd: 50.into(),
        }
    )
}

#[test]
fn staking_of_zero_xrd_is_considered_valid_by_the_stake_visitor() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();
    let (_, _, account1) = test_runner.new_account(false);
    let (_, _, validator1, stake_unit1, _) = new_registered_validator(&mut test_runner);

    // Act
    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account1, XRD, 0)
        .take_from_worktop(XRD, 0, "XRD")
        .stake_validator(validator1, "XRD")
        .deposit_batch(account1)
        .build();
    let stakes = execute_and_run_stake_visitor(manifest, &mut test_runner);

    // Assert
    let stakes = stakes.expect("Must be valid!");
    let [stake1] = stakes.as_slice() else {
        panic!("Unexpected number of stakes!")
    };
    assert_eq!(
        *stake1,
        StakeInformation {
            from_account: account1,
            validator_address: validator1,
            stake_unit_resource: stake_unit1,
            stake_unit_amount: 0.into(),
            staked_xrd: 0.into(),
        }
    )
}

#[test]
fn staking_transaction_that_used_take_all_from_worktop_is_considered_valid_by_the_stake_visitor() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();
    let (_, _, account1) = test_runner.new_account(false);
    let (_, _, validator1, stake_unit1, _) = new_registered_validator(&mut test_runner);

    // Act
    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account1, XRD, 200)
        .take_all_from_worktop(XRD, "XRD")
        .stake_validator(validator1, "XRD")
        .deposit_batch(account1)
        .build();
    let stakes = execute_and_run_stake_visitor(manifest, &mut test_runner);

    // Assert
    let stakes = stakes.expect("Must be valid!");
    let [stake1] = stakes.as_slice() else {
        panic!("Unexpected number of stakes!")
    };
    assert_eq!(
        *stake1,
        StakeInformation {
            from_account: account1,
            validator_address: validator1,
            stake_unit_resource: stake_unit1,
            stake_unit_amount: 200.into(),
            staked_xrd: 200.into(),
        }
    )
}

#[test]
fn staking_transaction_that_used_take_all_from_worktop_is_considered_valid_by_the_stake_visitor2() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();
    let (_, _, account1) = test_runner.new_account(false);
    let (_, _, validator1, stake_unit1, _) = new_registered_validator(&mut test_runner);
    let (_, _, validator2, stake_unit2, _) = new_registered_validator(&mut test_runner);

    // Act
    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account1, XRD, 100)
        .take_from_worktop(XRD, 50, "XRD1")
        .stake_validator(validator1, "XRD1")
        .take_all_from_worktop(XRD, "XRD2")
        .stake_validator(validator2, "XRD2")
        .deposit_batch(account1)
        .build();
    let stakes = execute_and_run_stake_visitor(manifest, &mut test_runner);

    // Assert
    let stakes = stakes.expect("Must be valid!");
    let [stake1, stake2] = stakes.as_slice() else {
        panic!("Unexpected number of stakes!")
    };
    assert_eq!(
        *stake1,
        StakeInformation {
            from_account: account1,
            validator_address: validator1,
            stake_unit_resource: stake_unit1,
            stake_unit_amount: 50.into(),
            staked_xrd: 50.into(),
        }
    );
    assert_eq!(
        *stake2,
        StakeInformation {
            from_account: account1,
            validator_address: validator2,
            stake_unit_resource: stake_unit2,
            stake_unit_amount: 50.into(),
            staked_xrd: 50.into(),
        }
    )
}

#[test]
fn staking_but_not_using_all_withdrawn_xrd_invalidates_staking_transaction() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();
    let (_, _, account1) = test_runner.new_account(false);
    let (_, _, validator1, _, _) = new_registered_validator(&mut test_runner);

    // Act
    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account1, XRD, 100)
        .take_from_worktop(XRD, 50, "XRD1")
        .stake_validator(validator1, "XRD1")
        .deposit_batch(account1)
        .build();
    let stakes = execute_and_run_stake_visitor(manifest, &mut test_runner);

    // Assert
    assert!(stakes.is_none());
}

#[test]
fn staking_and_withdrawing_from_one_account_and_depositing_into_another_invalidates_stake_transaction(
) {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();
    let (_, _, account1) = test_runner.new_account(false);
    let (_, _, account2) = test_runner.new_account(false);
    let (_, _, validator1, _, _) = new_registered_validator(&mut test_runner);

    // Act
    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account1, XRD, 100)
        .take_from_worktop(XRD, 100, "XRD1")
        .stake_validator(validator1, "XRD1")
        .deposit_batch(account2)
        .build();
    let stakes = execute_and_run_stake_visitor(manifest, &mut test_runner);

    // Assert
    assert!(stakes.is_none());
}

fn new_registered_validator(
    test_runner: &mut DefaultTestRunner,
) -> (
    Secp256k1PublicKey,
    Secp256k1PrivateKey,
    ComponentAddress,
    ResourceAddress,
    ResourceAddress,
) {
    let (public_key, private_key, account) = test_runner.new_account(false);
    let validator = test_runner.new_validator_with_pub_key(public_key, account);
    test_runner
        .execute_manifest(
            ManifestBuilder::new()
                .lock_fee_from_faucet()
                .create_proof_from_account_of_non_fungible(
                    account,
                    NonFungibleGlobalId::new(
                        VALIDATOR_OWNER_BADGE,
                        NonFungibleLocalId::bytes(validator.as_node_id().0).unwrap(),
                    ),
                )
                .register_validator(validator)
                .call_method(
                    validator,
                    VALIDATOR_UPDATE_ACCEPT_DELEGATED_STAKE_IDENT,
                    ValidatorUpdateAcceptDelegatedStakeInput {
                        accept_delegated_stake: true,
                    },
                )
                .build(),
            vec![NonFungibleGlobalId::from_public_key(&public_key)],
        )
        .expect_commit_success();

    let ValidatorSubstate {
        claim_nft,
        stake_unit_resource,
        ..
    } = test_runner.get_validator_info(validator);

    (
        public_key,
        private_key,
        validator,
        stake_unit_resource,
        claim_nft,
    )
}

fn execute_and_run_stake_visitor(
    manifest: TransactionManifestV1,
    test_runner: &mut DefaultTestRunner,
) -> Option<Vec<StakeInformation>> {
    let receipt = test_runner.preview_manifest(
        manifest.clone(),
        vec![],
        0,
        PreviewFlags {
            use_free_credit: true,
            assume_all_signature_proofs: true,
            skip_epoch_check: true,
        },
    );
    let execution_trace = receipt
        .expect_commit_success()
        .execution_trace
        .as_ref()
        .unwrap();
    let mut stake_visitor = StakeVisitor::new(execution_trace);
    traverse(&manifest.instructions, &mut [&mut stake_visitor]).unwrap();
    stake_visitor.output()
}

fn transaction_types(
    manifest_instructions: &[InstructionV1],
    receipt: &TransactionReceipt,
) -> Vec<TransactionType> {
    let analysis = execution::analyze(
        manifest_instructions,
        &ExecutionAnalysisTransactionReceipt::new(&VersionedTransactionReceipt::V1(
            receipt.clone(),
        ))
        .unwrap(),
    )
    .unwrap();
    analysis.transaction_types
}
