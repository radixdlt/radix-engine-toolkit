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

use radix_engine_toolkit_core::functions::manifest::parse_transfer_information;
use radix_engine_toolkit_core::instruction_visitor::visitors::transaction_type::transfer_visitor::{
    Resources, TransferTransactionTypeVisitor,
};
use radix_engine_toolkit_core::traverse_instructions;
use scrypto::prelude::*;
use transaction::prelude::ManifestBuilder;

mod test_data;

#[test]
pub fn transfer_visitor_can_pick_up_fungible_transfer() {
    // Arrange
    let account1 = test_data::account1();
    let account2 = test_data::account2();

    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account1, XRD, dec!("10"))
        .take_from_worktop(XRD, dec!("10"), "bucket")
        .with_bucket("bucket", |builder, bucket| {
            builder.call_method(account2, "deposit", manifest_args!(bucket))
        })
        .build();

    // Act
    let mut transfer_visitor = TransferTransactionTypeVisitor::default();
    let (source_account, deposits) =
        traverse_instructions!(&manifest.instructions, transfer_visitor)
            .unwrap()
            .unwrap();

    // Assert
    assert_eq!(source_account, account1);
    assert_eq!(
        deposits,
        hashmap!(
            account2 => hashmap! {
                XRD => Resources::Amount(dec!("10"))
            }
        )
    );
}

#[test]
pub fn transfer_visitor_can_pick_up_fungible_transfer_with_lock_fee() {
    // Arrange
    let account1 = test_data::account1();
    let account2 = test_data::account2();

    let manifest = ManifestBuilder::new()
        .lock_fee(account1, dec!("500"))
        .withdraw_from_account(account1, XRD, dec!("10"))
        .take_from_worktop(XRD, dec!("10"), "bucket")
        .with_bucket("bucket", |builder, bucket| {
            builder.call_method(account2, "deposit", manifest_args!(bucket))
        })
        .build();

    // Act
    let (source_account, deposits) =
        parse_transfer_information(&manifest, true)
            .unwrap()
            .unwrap();

    // Assert
    assert_eq!(source_account, account1);
    assert_eq!(
        deposits,
        hashmap!(
            account2 => hashmap! {
                XRD => Resources::Amount(dec!("10"))
            }
        )
    );
}

#[test]
pub fn transfer_visitor_can_pick_up_fungible_transfer_with_lock_fee_and_withdraw(
) {
    // Arrange
    let account1 = test_data::account1();
    let account2 = test_data::account2();

    let manifest = ManifestBuilder::new()
        .lock_fee_and_withdraw(account1, dec!("500"), XRD, dec!("10"))
        .take_from_worktop(XRD, dec!("10"), "bucket")
        .with_bucket("bucket", |builder, bucket| {
            builder.call_method(account2, "deposit", manifest_args!(bucket))
        })
        .build();

    // Act
    let (source_account, deposits) =
        parse_transfer_information(&manifest, true)
            .unwrap()
            .unwrap();

    // Assert
    assert_eq!(source_account, account1);
    assert_eq!(
        deposits,
        hashmap!(
            account2 => hashmap! {
                XRD => Resources::Amount(dec!("10"))
            }
        )
    );
}

#[test]
pub fn transfer_visitor_can_pick_up_fungible_transfer_from_a_single_source_to_multiple_recipients(
) {
    // Arrange
    let account1 = test_data::account1();
    let account2 = test_data::account2();
    let account3 = test_data::account3();

    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account1, XRD, dec!("20"))
        .take_from_worktop(XRD, dec!("15"), "bucket")
        .with_bucket("bucket", |builder, bucket| {
            builder.call_method(account2, "deposit", manifest_args!(bucket))
        })
        .take_from_worktop(XRD, dec!("5"), "bucket1")
        .with_bucket("bucket1", |builder, bucket| {
            builder.call_method(account3, "deposit", manifest_args!(bucket))
        })
        .build();

    // Act
    let mut transfer_visitor = TransferTransactionTypeVisitor::default();
    let (source_account, deposits) =
        traverse_instructions!(&manifest.instructions, transfer_visitor)
            .unwrap()
            .unwrap();

    // Assert
    assert_eq!(source_account, account1);
    assert_eq!(
        deposits,
        hashmap!(
            account2 => hashmap! {
                XRD => Resources::Amount(dec!("15"))
            },
            account3 => hashmap! {
                XRD => Resources::Amount(dec!("5"))
            }
        )
    );
}

#[test]
pub fn transfer_visitor_invalidated_transfer_from_multiple_accounts() {
    // Arrange
    let account1 = test_data::account1();
    let account2 = test_data::account2();

    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account1, XRD, dec!("10"))
        .withdraw_from_account(account2, XRD, dec!("10"))
        .take_from_worktop(XRD, dec!("20"), "bucket")
        .with_bucket("bucket", |builder, bucket| {
            builder.call_method(account2, "deposit", manifest_args!(bucket))
        })
        .build();

    // Act
    let mut transfer_visitor = TransferTransactionTypeVisitor::default();
    let result =
        traverse_instructions!(&manifest.instructions, transfer_visitor)
            .unwrap();

    // Assert
    assert!(result.is_none());
}

#[test]
pub fn transfer_visitor_can_pick_up_non_fungible_transfer() {
    // Arrange
    let account1 = test_data::account1();
    let account2 = test_data::account2();

    let manifest = ManifestBuilder::new()
        .withdraw_non_fungibles_from_account(
            account1,
            XRD,
            IndexSet::from([
                NonFungibleLocalId::integer(1),
                NonFungibleLocalId::integer(2),
            ]),
        )
        .take_from_worktop(XRD, dec!("2"), "bucket")
        .with_bucket("bucket", |builder, bucket| {
            builder.call_method(account2, "deposit", manifest_args!(bucket))
        })
        .build();

    // Act
    let mut transfer_visitor = TransferTransactionTypeVisitor::default();
    let (source_account, deposits) =
        traverse_instructions!(&manifest.instructions, transfer_visitor)
            .unwrap()
            .unwrap();

    // Assert
    assert_eq!(source_account, account1);
    assert_eq!(
        deposits,
        hashmap!(
            account2 => hashmap! {
                XRD => Resources::Ids(IndexSet::from([
                    NonFungibleLocalId::integer(1),
                    NonFungibleLocalId::integer(2),
                ]))
            }
        )
    );
}
