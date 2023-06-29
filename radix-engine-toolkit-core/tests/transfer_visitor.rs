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
        .withdraw_from_account(account1, RADIX_TOKEN, 10.into())
        .take_from_worktop(RADIX_TOKEN, 10.into(), |builder, bucket| {
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
                RADIX_TOKEN => Resources::Amount(10.into())
            }
        )
    );
}

#[test]
pub fn transfer_visitor_can_pick_up_fungible_transfer_from_a_single_source_to_multiple_recipients()
{
    // Arrange
    let account1 = test_data::account1();
    let account2 = test_data::account2();
    let account3 = test_data::account3();

    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account1, RADIX_TOKEN, 20.into())
        .take_from_worktop(RADIX_TOKEN, 15.into(), |builder, bucket| {
            builder.call_method(account2, "deposit", manifest_args!(bucket))
        })
        .take_from_worktop(RADIX_TOKEN, 5.into(), |builder, bucket| {
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
                RADIX_TOKEN => Resources::Amount(15.into())
            },
            account3 => hashmap! {
                RADIX_TOKEN => Resources::Amount(5.into())
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
        .withdraw_from_account(account1, RADIX_TOKEN, 10.into())
        .withdraw_from_account(account2, RADIX_TOKEN, 10.into())
        .take_from_worktop(RADIX_TOKEN, 20.into(), |builder, bucket| {
            builder.call_method(account2, "deposit", manifest_args!(bucket))
        })
        .build();

    // Act
    let mut transfer_visitor = TransferTransactionTypeVisitor::default();
    let result = traverse_instructions!(&manifest.instructions, transfer_visitor).unwrap();

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
            RADIX_TOKEN,
            &BTreeSet::from([
                NonFungibleLocalId::integer(1),
                NonFungibleLocalId::integer(2),
            ]),
        )
        .take_from_worktop(RADIX_TOKEN, 2.into(), |builder, bucket| {
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
                RADIX_TOKEN => Resources::Ids(BTreeSet::from([
                    NonFungibleLocalId::integer(1),
                    NonFungibleLocalId::integer(2),
                ]))
            }
        )
    );
}