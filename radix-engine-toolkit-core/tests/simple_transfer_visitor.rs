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
use radix_engine_toolkit_core::{
    instruction_visitor::visitors::transaction_type::simple_transfer_visitor::SimpleTransactionTypeVisitor,
    traverse_instructions,
};
use scrypto::prelude::*;
use transaction::prelude::ManifestBuilder;

mod test_data;

#[test]
pub fn simple_transfer_visitor_can_pick_up_fungible_transfer() {
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
    let mut simple_transfer_visitor = SimpleTransactionTypeVisitor::default();
    let (from_account, to_account, resource_specifier) =
        traverse_instructions!(&manifest.instructions, simple_transfer_visitor)
            .unwrap()
            .unwrap();

    // Assert
    assert_eq!(from_account, account1);
    assert_eq!(to_account, account2);
    assert_eq!(
        resource_specifier,
        ResourceSpecifier::Amount(XRD, dec!("10"))
    );
}

#[test]
pub fn simple_transfer_visitor_can_pick_up_non_fungible_transfer() {
    // Arrange
    let account1 = test_data::account1();
    let account2 = test_data::account2();

    let manifest = ManifestBuilder::new()
        .withdraw_non_fungibles_from_account(
            account1,
            XRD,
            BTreeSet::from([
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
    let mut simple_transfer_visitor = SimpleTransactionTypeVisitor::default();
    let (from_account, to_account, resource_specifier) =
        traverse_instructions!(&manifest.instructions, simple_transfer_visitor)
            .unwrap()
            .unwrap();

    // Assert
    assert_eq!(from_account, account1);
    assert_eq!(to_account, account2);
    assert_eq!(
        resource_specifier,
        ResourceSpecifier::Ids(
            XRD,
            BTreeSet::from([
                NonFungibleLocalId::integer(1),
                NonFungibleLocalId::integer(2),
            ])
        )
    );
}

#[test]
pub fn simple_transfer_visitor_invalidated_transfer_with_an_additional_withdraw() {
    // Arrange
    let account1 = test_data::account1();
    let account2 = test_data::account2();

    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account1, XRD, dec!("10"))
        .withdraw_non_fungibles_from_account(
            account1,
            XRD,
            BTreeSet::from([
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
    let mut simple_transfer_visitor = SimpleTransactionTypeVisitor::default();
    let transfer_data =
        traverse_instructions!(&manifest.instructions, simple_transfer_visitor).unwrap();

    // Assert
    assert!(transfer_data.is_none())
}
