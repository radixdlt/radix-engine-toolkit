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

use radix_engine::system::system_modules::execution_trace::ResourceSpecifier;
use radix_engine::transaction::TransactionReceipt;
use radix_engine_toolkit_core::functions::execution::{self, *};
use radix_engine_toolkit_core::instruction_visitor::visitors::transaction_type::general_transaction_visitor::{ResourceTracker, Source};
use radix_engine_toolkit_core::instruction_visitor::visitors::transaction_type::transfer_visitor::Resources;
use scrypto::prelude::*;
use scrypto_unit::*;
use transaction::prelude::*;

#[test]
fn simple_transfer_is_picked_up_as_a_simple_account_transfer_transaction() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();
    let (public_key1, _, account1) = test_runner.new_account(true);
    let (public_key2, _, account2) = test_runner.new_account(true);

    let manifest = ManifestBuilder::new()
        .lock_fee(account1, dec!("10"))
        .withdraw_from_account(account1, XRD, dec!("10"))
        .take_from_worktop(XRD, dec!("10"), "bucket")
        .try_deposit_or_abort(account2, None, "bucket")
        .build();
    let receipt = test_runner.preview_manifest(
        manifest.clone(),
        vec![public_key1.into(), public_key2.into()],
        0,
        PreviewFlags::default(),
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
        .lock_fee(account1, dec!("10"))
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
        PreviewFlags::default(),
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
        .try_deposit_batch_or_abort(account1, None)
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
                    ids: Source::Predicted(0, btreeset!(
                        NonFungibleLocalId::integer(1),
                        NonFungibleLocalId::integer(2),
                    ))
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
                    ids: Source::Predicted(1, btreeset!(
                        NonFungibleLocalId::integer(1),
                        NonFungibleLocalId::integer(2),
                    ))
                }
            ]
        }
    );
}

fn transaction_types(
    manifest_instructions: &[InstructionV1],
    receipt: &TransactionReceipt,
) -> Vec<TransactionType> {
    let analysis = execution::analyze(manifest_instructions, receipt).unwrap();
    analysis.transaction_types
}
