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

use radix_engine::blueprints::consensus_manager::*;
use radix_engine::system::system_modules::execution_trace::ResourceSpecifier;
use radix_engine::transaction::*;
use radix_engine_interface::blueprints::account::*;
use radix_engine_interface::blueprints::consensus_manager::*;
use radix_engine_toolkit_core::functions::execution::{self, *};
use radix_engine_toolkit_core::instruction_visitor::visitors::transaction_type::claim_stake_visitor::*;
use radix_engine_toolkit_core::instruction_visitor::visitors::transaction_type::general_transaction_visitor::*;
use radix_engine_toolkit_core::instruction_visitor::visitors::transaction_type::transfer_visitor::*;
use scrypto::prelude::*;
use scrypto_unit::*;
use transaction::prelude::*;
use radix_engine_toolkit_core::instruction_visitor::core::traverser::*;
use radix_engine_toolkit_core::instruction_visitor::visitors::transaction_type::stake_visitor::*;
use radix_engine_toolkit_core::instruction_visitor::visitors::transaction_type::unstake_visitor::*;

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
        .find(|transaction_type| {
            matches!(transaction_type, TransactionType::SimpleTransfer(..))
        })
        .unwrap();

    // Assert
    assert_eq!(
        transaction_type,
        TransactionType::SimpleTransfer(Box::new(
            SimpleTransferTransactionType {
                from: account1,
                to: account2,
                transferred: ResourceSpecifier::Amount(XRD, dec!("10"))
            }
        ))
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
        .find(|transaction_type| {
            matches!(transaction_type, TransactionType::Transfer(..))
        })
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
            TransactionType::GeneralTransaction(general_transaction) => {
                Some(general_transaction)
            }
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
        &ExecutionAnalysisTransactionReceipt::new(
            &VersionedTransactionReceipt::V1(receipt),
        )
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
    let (_, _, validator1, stake_unit1, _) =
        new_registered_validator(&mut test_runner);

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
fn simple_stake_transaction_using_take_all_from_worktop_deposit_is_detected_by_the_stake_visitor(
) {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();
    let (_, _, account1) = test_runner.new_account(false);
    let (_, _, validator1, stake_unit1, _) =
        new_registered_validator(&mut test_runner);

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
fn stake_with_multi_withdraw_and_multi_deposits_is_detected_as_stake_by_stake_visitor(
) {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();

    let (_, _, account1) = test_runner.new_account(false);
    let (_, _, validator1, stake_unit1, _) =
        new_registered_validator(&mut test_runner);

    let (_, _, account2) = test_runner.new_account(false);
    let (_, _, validator2, stake_unit2, _) =
        new_registered_validator(&mut test_runner);

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
fn staking_from_one_account_to_multiple_validators_is_detected_as_a_stake_transaction(
) {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();
    let (_, _, account1) = test_runner.new_account(false);
    let (_, _, validator1, stake_unit1, _) =
        new_registered_validator(&mut test_runner);
    let (_, _, validator2, stake_unit2, _) =
        new_registered_validator(&mut test_runner);

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
    let (_, _, validator1, stake_unit1, _) =
        new_registered_validator(&mut test_runner);

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
fn staking_transaction_that_used_take_all_from_worktop_is_considered_valid_by_the_stake_visitor(
) {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();
    let (_, _, account1) = test_runner.new_account(false);
    let (_, _, validator1, stake_unit1, _) =
        new_registered_validator(&mut test_runner);

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
fn staking_transaction_that_used_take_all_from_worktop_is_considered_valid_by_the_stake_visitor2(
) {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();
    let (_, _, account1) = test_runner.new_account(false);
    let (_, _, validator1, stake_unit1, _) =
        new_registered_validator(&mut test_runner);
    let (_, _, validator2, stake_unit2, _) =
        new_registered_validator(&mut test_runner);

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

#[test]
fn simple_unstaking_is_recognized_by_unstaking_visitor() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();
    let (_, _, account1) = test_runner.new_account(false);
    let (_, _, validator1, stake_unit_resource, claim_nft_resource) =
        new_registered_validator(&mut test_runner);
    stake(&mut test_runner, account1, validator1);

    // Act
    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account1, stake_unit_resource, "200")
        .take_all_from_worktop(stake_unit_resource, "StakeUnit1")
        .unstake_validator(validator1, "StakeUnit1")
        .deposit_batch(account1)
        .build();
    let unstakes = execute_and_run_unstake_visitor(manifest, &mut test_runner);

    // Assert
    let unstakes = unstakes.expect("Must be valid!");
    let [unstake1] = unstakes.as_slice() else {
        panic!("Unexpected number of stakes!")
    };
    assert_eq!(unstake1.from_account, account1);
    assert_eq!(unstake1.stake_unit_address, stake_unit_resource);
    assert_eq!(unstake1.stake_unit_amount, dec!("200"));
    assert_eq!(unstake1.validator_address, validator1);
    assert_eq!(unstake1.claim_nft_resource, claim_nft_resource);
    assert_eq!(
        unstake1.claim_nft_data,
        UnstakeData {
            name: "Stake Claim".to_owned(),
            claim_epoch: Epoch::of(3),
            claim_amount: 200.into()
        }
    );
}

#[test]
fn unstaking_with_take_from_worktop_by_amount_is_recognized_by_unstaking_visitor(
) {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();
    let (_, _, account1) = test_runner.new_account(false);
    let (_, _, validator1, stake_unit_resource, claim_nft_resource) =
        new_registered_validator(&mut test_runner);
    stake(&mut test_runner, account1, validator1);

    // Act
    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account1, stake_unit_resource, 200)
        .take_from_worktop(stake_unit_resource, 200, "StakeUnit1")
        .unstake_validator(validator1, "StakeUnit1")
        .deposit_batch(account1)
        .build();
    let unstakes = execute_and_run_unstake_visitor(manifest, &mut test_runner);

    // Assert
    let unstakes = unstakes.expect("Must be valid!");
    let [unstake1] = unstakes.as_slice() else {
        panic!("Unexpected number of stakes!")
    };
    assert_eq!(unstake1.from_account, account1);
    assert_eq!(unstake1.stake_unit_address, stake_unit_resource);
    assert_eq!(unstake1.stake_unit_amount, dec!("200"));
    assert_eq!(unstake1.validator_address, validator1);
    assert_eq!(unstake1.claim_nft_resource, claim_nft_resource);
    assert_eq!(
        unstake1.claim_nft_data,
        UnstakeData {
            name: "Stake Claim".to_owned(),
            claim_epoch: Epoch::of(3),
            claim_amount: 200.into()
        }
    );
}

#[test]
fn unstaking_with_take_from_worktop_by_amount_of_claim_nft_is_recognized_by_unstaking_visitor(
) {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();
    let (_, _, account1) = test_runner.new_account(false);
    let (_, _, validator1, stake_unit_resource, claim_nft_resource) =
        new_registered_validator(&mut test_runner);
    stake(&mut test_runner, account1, validator1);

    // Act
    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account1, stake_unit_resource, 200)
        .take_from_worktop(stake_unit_resource, 200, "StakeUnit1")
        .unstake_validator(validator1, "StakeUnit1")
        .take_from_worktop(claim_nft_resource, 1, "ClaimNft")
        .deposit(account1, "ClaimNft")
        .build();
    let unstakes = execute_and_run_unstake_visitor(manifest, &mut test_runner);

    // Assert
    let unstakes = unstakes.expect("Must be valid!");
    let [unstake1] = unstakes.as_slice() else {
        panic!("Unexpected number of stakes!")
    };
    assert_eq!(unstake1.from_account, account1);
    assert_eq!(unstake1.stake_unit_address, stake_unit_resource);
    assert_eq!(unstake1.stake_unit_amount, dec!("200"));
    assert_eq!(unstake1.validator_address, validator1);
    assert_eq!(unstake1.claim_nft_resource, claim_nft_resource);
    assert_eq!(
        unstake1.claim_nft_data,
        UnstakeData {
            name: "Stake Claim".to_owned(),
            claim_epoch: Epoch::of(3),
            claim_amount: 200.into()
        }
    );
}

#[test]
fn unstaking_with_take_all_from_worktop_of_claim_nft_is_recognized_by_unstaking_visitor(
) {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();
    let (_, _, account1) = test_runner.new_account(false);
    let (_, _, validator1, stake_unit_resource, claim_nft_resource) =
        new_registered_validator(&mut test_runner);
    stake(&mut test_runner, account1, validator1);

    // Act
    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account1, stake_unit_resource, 200)
        .take_from_worktop(stake_unit_resource, 200, "StakeUnit1")
        .unstake_validator(validator1, "StakeUnit1")
        .take_all_from_worktop(claim_nft_resource, "ClaimNft")
        .deposit(account1, "ClaimNft")
        .build();
    let unstakes = execute_and_run_unstake_visitor(manifest, &mut test_runner);

    // Assert
    let unstakes = unstakes.expect("Must be valid!");
    let [unstake1] = unstakes.as_slice() else {
        panic!("Unexpected number of stakes!")
    };
    assert_eq!(unstake1.from_account, account1);
    assert_eq!(unstake1.stake_unit_address, stake_unit_resource);
    assert_eq!(unstake1.stake_unit_amount, dec!("200"));
    assert_eq!(unstake1.validator_address, validator1);
    assert_eq!(unstake1.claim_nft_resource, claim_nft_resource);
    assert_eq!(
        unstake1.claim_nft_data,
        UnstakeData {
            name: "Stake Claim".to_owned(),
            claim_epoch: Epoch::of(3),
            claim_amount: 200.into()
        }
    );
}

#[test]
fn unstaking_and_depositing_claim_nft_into_another_account_is_not_allowed_by_unstake_visitor(
) {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();
    let (_, _, account1) = test_runner.new_account(false);
    let (_, _, account2) = test_runner.new_account(false);
    let (_, _, validator1, stake_unit_resource, claim_nft_resource) =
        new_registered_validator(&mut test_runner);
    stake(&mut test_runner, account1, validator1);

    // Act
    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account1, stake_unit_resource, 200)
        .take_from_worktop(stake_unit_resource, 200, "StakeUnit1")
        .unstake_validator(validator1, "StakeUnit1")
        .take_all_from_worktop(claim_nft_resource, "ClaimNft")
        .deposit(account2, "ClaimNft")
        .build();
    let unstakes = execute_and_run_unstake_visitor(manifest, &mut test_runner);

    // Assert
    assert!(unstakes.is_none());
}

#[test]
fn unstaking_and_depositing_claim_nft_into_another_account_is_not_allowed_by_unstake_visitor2(
) {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();
    let (_, _, account1) = test_runner.new_account(false);
    let (_, _, account2) = test_runner.new_account(false);
    let (_, _, validator1, stake_unit_resource, ..) =
        new_registered_validator(&mut test_runner);
    stake(&mut test_runner, account1, validator1);

    // Act
    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account1, stake_unit_resource, 200)
        .take_from_worktop(stake_unit_resource, 200, "StakeUnit1")
        .unstake_validator(validator1, "StakeUnit1")
        .deposit_batch(account2)
        .build();
    let unstakes = execute_and_run_unstake_visitor(manifest, &mut test_runner);

    // Assert
    assert!(unstakes.is_none());
}

#[test]
fn multiple_unstakes_is_recognized_by_unstaking_visitor() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();
    let (_, _, account1) = test_runner.new_account(false);
    let (_, _, validator1, stake_unit_resource, claim_nft_resource) =
        new_registered_validator(&mut test_runner);
    stake(&mut test_runner, account1, validator1);

    // Act
    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account1, stake_unit_resource, 200)
        .take_from_worktop(stake_unit_resource, 200, "StakeUnit1")
        .unstake_validator(validator1, "StakeUnit1")
        .take_all_from_worktop(claim_nft_resource, "ClaimNft1")
        .deposit(account1, "ClaimNft1")
        .withdraw_from_account(account1, stake_unit_resource, 200)
        .take_from_worktop(stake_unit_resource, 200, "StakeUnit2")
        .unstake_validator(validator1, "StakeUnit2")
        .take_all_from_worktop(claim_nft_resource, "ClaimNft2")
        .deposit(account1, "ClaimNft2")
        .build();
    let unstakes = execute_and_run_unstake_visitor(manifest, &mut test_runner);

    // Assert
    let unstakes = unstakes.expect("Must be valid!");
    let [unstake1, unstake2] = unstakes.as_slice() else {
        panic!("Unexpected number of stakes!")
    };

    assert_eq!(unstake1.from_account, account1);
    assert_eq!(unstake1.stake_unit_address, stake_unit_resource);
    assert_eq!(unstake1.stake_unit_amount, dec!("200"));
    assert_eq!(unstake1.validator_address, validator1);
    assert_eq!(unstake1.claim_nft_resource, claim_nft_resource);
    assert_eq!(
        unstake1.claim_nft_data,
        UnstakeData {
            name: "Stake Claim".to_owned(),
            claim_epoch: Epoch::of(3),
            claim_amount: 200.into()
        }
    );

    assert_eq!(unstake2.from_account, account1);
    assert_eq!(unstake2.stake_unit_address, stake_unit_resource);
    assert_eq!(unstake2.stake_unit_amount, dec!("200"));
    assert_eq!(unstake2.validator_address, validator1);
    assert_eq!(unstake2.claim_nft_resource, claim_nft_resource);
    assert_eq!(
        unstake2.claim_nft_data,
        UnstakeData {
            name: "Stake Claim".to_owned(),
            claim_epoch: Epoch::of(3),
            claim_amount: 200.into()
        }
    );
}

#[test]
fn simple_claim_transaction_can_be_caught_by_claim_visitor() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();
    let (_, _, account1) = test_runner.new_account(false);
    let (_, _, validator1, _, claim_nft_resource) =
        new_registered_validator(&mut test_runner);
    let claim_nft_local_id =
        stake_and_unstake(&mut test_runner, account1, validator1);

    // Act
    let manifest = ManifestBuilder::new()
        .withdraw_non_fungible_from_account(
            account1,
            NonFungibleGlobalId::new(
                claim_nft_resource,
                claim_nft_local_id.clone(),
            ),
        )
        .take_all_from_worktop(claim_nft_resource, "StakeUnit1")
        .with_bucket("StakeUnit1", |builder, bucket| {
            builder.call_method(
                validator1,
                VALIDATOR_CLAIM_XRD_IDENT,
                manifest_args!(bucket),
            )
        })
        .deposit_batch(account1)
        .build();
    let claims = execute_and_run_claim_visitor(manifest, &mut test_runner);

    // Assert
    let claims = claims.expect("Must be valid!");
    let [claim1] = claims.as_slice() else {
        panic!("Unexpected number of stakes!")
    };
    assert_eq!(claim1.from_account, account1);
    assert_eq!(claim1.validator_address, validator1);
    assert_eq!(claim1.claim_nft_resource, claim_nft_resource);
    assert_eq!(
        claim1.claim_nft_local_ids.clone(),
        btreeset!(claim_nft_local_id)
    );
    assert_eq!(claim1.claimed_xrd, dec!("10000"));
}

#[test]
fn stake_claim_with_multiple_nfts_can_be_caught_by_claim_visitor() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();
    let (_, _, account1) = test_runner.new_account(false);
    let (_, _, validator1, _, claim_nft_resource) =
        new_registered_validator(&mut test_runner);

    stake_and_unstake(&mut test_runner, account1, validator1);
    stake_and_unstake(&mut test_runner, account1, validator1);
    stake_and_unstake(&mut test_runner, account1, validator1);

    let (claim_nft_local_id1, claim_nft_local_id2, claim_nft_local_id3) = {
        let vault_id = *test_runner
            .get_component_vaults(account1, claim_nft_resource)
            .first()
            .unwrap();
        let mut iter =
            test_runner.inspect_non_fungible_vault(vault_id).unwrap().1;
        (
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        )
    };

    // Act
    let manifest = ManifestBuilder::new()
        .withdraw_non_fungibles_from_account(
            account1,
            claim_nft_resource,
            [
                claim_nft_local_id1.clone(),
                claim_nft_local_id2.clone(),
                claim_nft_local_id3.clone(),
            ],
        )
        .take_all_from_worktop(claim_nft_resource, "StakeUnit1")
        .with_bucket("StakeUnit1", |builder, bucket| {
            builder.call_method(
                validator1,
                VALIDATOR_CLAIM_XRD_IDENT,
                manifest_args!(bucket),
            )
        })
        .deposit_batch(account1)
        .build();
    let claims = execute_and_run_claim_visitor(manifest, &mut test_runner);

    // Assert
    let claims = claims.expect("Must be valid!");
    let [claim1] = claims.as_slice() else {
        panic!("Unexpected number of stakes!")
    };
    assert_eq!(dbg!(claim1).from_account, account1);
    assert_eq!(claim1.validator_address, validator1);
    assert_eq!(claim1.claim_nft_resource, claim_nft_resource);
    assert_eq!(
        claim1.claim_nft_local_ids.clone(),
        btreeset!(
            claim_nft_local_id1,
            claim_nft_local_id2,
            claim_nft_local_id3
        ),
    );
    assert_eq!(claim1.claimed_xrd, dec!("30000"));
}

#[test]
fn simple_claim_transaction_can_be_caught_by_claim_visitor1() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();
    let (_, _, account1) = test_runner.new_account(false);
    let (_, _, validator1, _, claim_nft_resource) =
        new_registered_validator(&mut test_runner);
    let claim_nft_local_id =
        stake_and_unstake(&mut test_runner, account1, validator1);

    // Act
    let manifest = ManifestBuilder::new()
        .withdraw_non_fungible_from_account(
            account1,
            NonFungibleGlobalId::new(
                claim_nft_resource,
                claim_nft_local_id.clone(),
            ),
        )
        .take_from_worktop(claim_nft_resource, 1, "StakeUnit1")
        .with_bucket("StakeUnit1", |builder, bucket| {
            builder.call_method(
                validator1,
                VALIDATOR_CLAIM_XRD_IDENT,
                manifest_args!(bucket),
            )
        })
        .deposit_batch(account1)
        .build();
    let claims = execute_and_run_claim_visitor(manifest, &mut test_runner);

    // Assert
    let claims = claims.expect("Must be valid!");
    let [claim1] = claims.as_slice() else {
        panic!("Unexpected number of stakes!")
    };
    assert_eq!(claim1.from_account, account1);
    assert_eq!(claim1.validator_address, validator1);
    assert_eq!(claim1.claim_nft_resource, claim_nft_resource);
    assert_eq!(
        claim1.claim_nft_local_ids.clone(),
        btreeset!(claim_nft_local_id)
    );
    assert_eq!(claim1.claimed_xrd, dec!("10000"));
}

#[test]
fn simple_claim_transaction_can_be_caught_by_claim_visitor2() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();
    let (_, _, account1) = test_runner.new_account(false);
    let (_, _, validator1, _, claim_nft_resource) =
        new_registered_validator(&mut test_runner);
    let claim_nft_local_id =
        stake_and_unstake(&mut test_runner, account1, validator1);

    // Act
    let manifest = ManifestBuilder::new()
        .withdraw_non_fungible_from_account(
            account1,
            NonFungibleGlobalId::new(
                claim_nft_resource,
                claim_nft_local_id.clone(),
            ),
        )
        .take_all_from_worktop(claim_nft_resource, "StakeUnit1")
        .with_bucket("StakeUnit1", |builder, bucket| {
            builder.call_method(
                validator1,
                VALIDATOR_CLAIM_XRD_IDENT,
                manifest_args!(bucket),
            )
        })
        .deposit_batch(account1)
        .build();
    let claims = execute_and_run_claim_visitor(manifest, &mut test_runner);

    // Assert
    let claims = claims.expect("Must be valid!");
    let [claim1] = claims.as_slice() else {
        panic!("Unexpected number of stakes!")
    };
    assert_eq!(claim1.from_account, account1);
    assert_eq!(claim1.validator_address, validator1);
    assert_eq!(claim1.claim_nft_resource, claim_nft_resource);
    assert_eq!(
        claim1.claim_nft_local_ids.clone(),
        btreeset!(claim_nft_local_id)
    );
    assert_eq!(claim1.claimed_xrd, dec!("10000"));
}

#[test]
fn simple_claim_transaction_can_be_caught_by_claim_visitor3() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();
    let (_, _, account1) = test_runner.new_account(false);
    let (_, _, validator1, _, claim_nft_resource) =
        new_registered_validator(&mut test_runner);
    let claim_nft_local_id =
        stake_and_unstake(&mut test_runner, account1, validator1);

    // Act
    let manifest = ManifestBuilder::new()
        .withdraw_non_fungible_from_account(
            account1,
            NonFungibleGlobalId::new(
                claim_nft_resource,
                claim_nft_local_id.clone(),
            ),
        )
        .take_all_from_worktop(claim_nft_resource, "StakeUnit1")
        .with_bucket("StakeUnit1", |builder, bucket| {
            builder.call_method(
                validator1,
                VALIDATOR_CLAIM_XRD_IDENT,
                manifest_args!(bucket),
            )
        })
        .take_all_from_worktop(XRD, "XRD1")
        .deposit(account1, "XRD1")
        .build();
    let claims = execute_and_run_claim_visitor(manifest, &mut test_runner);

    // Assert
    let claims = claims.expect("Must be valid!");
    let [claim1] = claims.as_slice() else {
        panic!("Unexpected number of stakes!")
    };
    assert_eq!(claim1.from_account, account1);
    assert_eq!(claim1.validator_address, validator1);
    assert_eq!(claim1.claim_nft_resource, claim_nft_resource);
    assert_eq!(
        claim1.claim_nft_local_ids.clone(),
        btreeset!(claim_nft_local_id)
    );
    assert_eq!(claim1.claimed_xrd, dec!("10000"));
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
                        NonFungibleLocalId::bytes(validator.as_node_id().0)
                            .unwrap(),
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

fn execute_and_run_unstake_visitor(
    manifest: TransactionManifestV1,
    test_runner: &mut DefaultTestRunner,
) -> Option<Vec<UnstakeInformation<UnstakeData>>> {
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
    let receipt = VersionedTransactionReceipt::V1(receipt);
    let receipt = ExecutionAnalysisTransactionReceipt::new(&receipt).unwrap();
    let mut unstake_visitor = UnstakeVisitor::new(&receipt);
    traverse(&manifest.instructions, &mut [&mut unstake_visitor]).unwrap();
    unstake_visitor.output()
}

fn execute_and_run_claim_visitor(
    manifest: TransactionManifestV1,
    test_runner: &mut DefaultTestRunner,
) -> Option<Vec<ClaimStakeInformation>> {
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
    let mut claim_visitor = ClaimStakeVisitor::new(execution_trace);
    traverse(&manifest.instructions, &mut [&mut claim_visitor]).unwrap();
    claim_visitor.output()
}

fn stake(
    test_runner: &mut DefaultTestRunner,
    account: ComponentAddress,
    validator: ComponentAddress,
) {
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .get_free_xrd_from_faucet()
        .take_from_worktop(XRD, 10_000, "XRD")
        .stake_validator(validator, "XRD")
        .try_deposit_entire_worktop_or_refund(account, None)
        .build();
    test_runner
        .execute_manifest(manifest, vec![])
        .expect_commit_success();
}

fn stake_and_unstake(
    test_runner: &mut DefaultTestRunner,
    account: ComponentAddress,
    validator: ComponentAddress,
) -> NonFungibleLocalId {
    let ValidatorSubstate {
        stake_unit_resource,
        claim_nft,
        ..
    } = test_runner.get_validator_info(validator);
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .get_free_xrd_from_faucet()
        .take_from_worktop(XRD, 10_000, "XRD")
        .stake_validator(validator, "XRD")
        .take_all_from_worktop(stake_unit_resource, "StakeUnit")
        .unstake_validator(validator, "StakeUnit")
        .try_deposit_entire_worktop_or_refund(account, None)
        .build();
    test_runner
        .execute_manifest(manifest, vec![])
        .expect_commit_success();
    let current_epoch = test_runner.get_current_epoch();
    test_runner.set_current_epoch(current_epoch.after(200).unwrap());

    let vault_id = *test_runner
        .get_component_vaults(account, claim_nft)
        .first()
        .unwrap();
    test_runner
        .inspect_non_fungible_vault(vault_id)
        .unwrap()
        .1
        .next()
        .unwrap()
}

fn transaction_types(
    manifest_instructions: &[InstructionV1],
    receipt: &TransactionReceipt,
) -> Vec<TransactionType> {
    let analysis = execution::analyze(
        manifest_instructions,
        &ExecutionAnalysisTransactionReceipt::new(
            &VersionedTransactionReceipt::V1(receipt.clone()),
        )
        .unwrap(),
    )
    .unwrap();
    analysis.transaction_types
}
