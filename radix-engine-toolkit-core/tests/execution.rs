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
use radix_engine_toolkit_core::instruction_visitor::visitors::transaction_type::transfer_visitor::Resources;
use scrypto::blueprints::account::*;
use scrypto::prelude::*;
use scrypto_unit::*;
use transaction::prelude::*;

#[test]
fn simple_transfer_is_picked_up_as_a_simple_account_transfer_transaction() {
    // Arrange
    let mut test_runner = TestRunner::builder().build();
    let (public_key1, _, account1) = test_runner.new_account(true);
    let (public_key2, _, account2) = test_runner.new_account(true);

    let manifest = ManifestBuilder::new()
        .lock_fee(account1, dec!("10"))
        .withdraw_from_account(account1, RADIX_TOKEN, dec!("10"))
        .take_from_worktop(RADIX_TOKEN, dec!("10"), "bucket")
        .with_bucket("bucket", |builder, bucket| {
            builder.call_method(
                account2,
                ACCOUNT_TRY_DEPOSIT_OR_ABORT_IDENT,
                manifest_args!(bucket),
            )
        })
        .build();
    let receipt = test_runner.preview_manifest(
        manifest.clone(),
        vec![public_key1.into(), public_key2.into()],
        0,
        PreviewFlags::default(),
    );
    receipt.expect_commit_success();

    // Act
    let transaction_type = transaction_type(&manifest.instructions, &receipt);

    // Assert
    assert_eq!(
        transaction_type,
        TransactionType::SimpleTransfer(Box::new(SimpleTransferTransactionType {
            from: account1,
            to: account2,
            transferred: ResourceSpecifier::Amount(RADIX_TOKEN, dec!("10"))
        }))
    )
}

#[test]
fn transfer_is_picked_up_as_an_account_transfer_transaction() {
    // Arrange
    let mut test_runner = TestRunner::builder().build();
    let (public_key1, _, account1) = test_runner.new_account(true);
    let (public_key2, _, account2) = test_runner.new_account(true);
    let (public_key3, _, account3) = test_runner.new_account(true);

    let manifest = ManifestBuilder::new()
        .lock_fee(account1, dec!("10"))
        .withdraw_from_account(account1, RADIX_TOKEN, dec!("20"))
        .take_from_worktop(RADIX_TOKEN, dec!("10"), "bucket")
        .with_bucket("bucket", |builder, bucket| {
            builder.call_method(
                account2,
                ACCOUNT_TRY_DEPOSIT_OR_ABORT_IDENT,
                manifest_args!(bucket),
            )
        })
        .take_from_worktop(RADIX_TOKEN, dec!("10"), "bucket2")
        .with_bucket("bucket2", |builder, bucket| {
            builder.call_method(
                account3,
                ACCOUNT_TRY_DEPOSIT_OR_ABORT_IDENT,
                manifest_args!(bucket),
            )
        })
        .build();
    let receipt = test_runner.preview_manifest(
        manifest.clone(),
        vec![public_key1.into(), public_key2.into(), public_key3.into()],
        0,
        PreviewFlags::default(),
    );
    receipt.expect_commit_success();

    // Act
    let transaction_type = transaction_type(&manifest.instructions, &receipt);

    // Assert
    assert_eq!(
        transaction_type,
        TransactionType::Transfer(Box::new(TransferTransactionType {
            from: account1,
            transfers: hashmap! {
                account2 => hashmap! {
                    RADIX_TOKEN => Resources::Amount(dec!("10")),
                },
                account3 => hashmap! {
                    RADIX_TOKEN => Resources::Amount(dec!("10")),
                }
            }
        }))
    )
}

#[test]
fn complex_transfer_is_picked_up_as_an_general_transaction() {
    // Arrange
    let mut test_runner = TestRunner::builder().build();
    let (public_key1, _, account1) = test_runner.new_account(true);
    let (public_key2, _, account2) = test_runner.new_account(true);
    let (public_key3, _, account3) = test_runner.new_account(true);

    let manifest = ManifestBuilder::new()
        .lock_fee(account1, dec!("10"))
        .withdraw_from_account(account1, RADIX_TOKEN, dec!("10"))
        .withdraw_from_account(account2, RADIX_TOKEN, dec!("10"))
        .take_from_worktop(RADIX_TOKEN, dec!("10"), "bucket")
        .with_bucket("bucket", |builder, bucket| {
            builder.call_method(
                account2,
                ACCOUNT_TRY_DEPOSIT_OR_ABORT_IDENT,
                manifest_args!(bucket),
            )
        })
        .take_from_worktop(RADIX_TOKEN, dec!("10"), "bucket1")
        .with_bucket("bucket1", |builder, bucket| {
            builder.call_method(
                account3,
                ACCOUNT_TRY_DEPOSIT_OR_ABORT_IDENT,
                manifest_args!(bucket),
            )
        })
        .build();
    let receipt = test_runner.preview_manifest(
        manifest.clone(),
        vec![public_key1.into(), public_key2.into(), public_key3.into()],
        0,
        PreviewFlags::default(),
    );
    receipt.expect_commit_success();

    // Act
    let transaction_type = transaction_type(&manifest.instructions, &receipt);

    // Assert
    assert!(matches!(
        transaction_type,
        TransactionType::GeneralTransaction(..)
    ))
}

fn transaction_type(
    manifest_instructions: &[InstructionV1],
    receipt: &TransactionReceipt,
) -> TransactionType {
    let analysis = execution::analyze(manifest_instructions, receipt).unwrap();
    analysis.transaction_type
}
