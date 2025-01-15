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

use crate::prelude::*;

/// This test ensures that a transaction intent manifest is classified as
/// uncategorized in the Gateway. The wallet currently uses the classification
/// of the transaction intent manifest as the classification of the transaction.
/// For an MVP, we want the wallet to show transactions with subintents as
/// Complex/Uncategorized, so this test checks that. In future, we may wish to
/// revisit this and change this restriction, but we should make sure that the
/// transactions still display reasonably in the wallet.
#[test]
fn subintent_with_yield_to_child_doesnt_classify_as_any_type() {
    // Arrange
    let manifest = ManifestBuilder::new_subintent_v2()
        .use_child("example", SubintentHash(Hash([0; 32])))
        .yield_to_child("example", ())
        .yield_to_parent(())
        .build();

    // Act
    let StaticAnalysis {
        manifest_classification,
        ..
    } = statically_analyze(&manifest).unwrap();

    // Assert
    assert!(manifest_classification.is_empty());
}

#[test]
fn transfer_subintent_classifies_as_general_subintent() {
    // Arrange
    let mut allocator = TestAddressAllocator::new();
    let account1 = allocator.new_account_address();
    let account2 = allocator.new_account_address();
    let manifest = ManifestBuilder::new_subintent_v2()
        .withdraw_from_account(account1, XRD, 10)
        .take_all_from_worktop(XRD, "xrd")
        .try_deposit_or_abort(account2, None, "xrd")
        .yield_to_parent(())
        .build();

    // Act
    let StaticAnalysis {
        manifest_classification,
        ..
    } = statically_analyze(&manifest).unwrap();

    // Assert
    assert_eq!(manifest_classification.len(), 1);
    assert_eq!(
        manifest_classification.first(),
        Some(&ManifestClassification::GeneralSubintent)
    );
}

#[test]
fn subintent_with_verify_parent_classifies_as_general_subintent() {
    // Arrange
    let mut allocator = TestAddressAllocator::new();
    let account1 = allocator.new_account_address();
    let account2 = allocator.new_account_address();
    let manifest = ManifestBuilder::new_subintent_v2()
        .withdraw_from_account(account1, XRD, 10)
        .take_all_from_worktop(XRD, "xrd")
        .try_deposit_or_abort(account2, None, "xrd")
        .verify_parent(rule!(allow_all))
        .yield_to_parent(())
        .build();

    // Act
    let StaticAnalysis {
        manifest_classification,
        ..
    } = statically_analyze(&manifest).unwrap();

    // Assert
    assert_eq!(manifest_classification.len(), 1);
    assert_eq!(
        manifest_classification.first(),
        Some(&ManifestClassification::GeneralSubintent)
    );
}

#[test]
fn subintent_update_metadata_doesnt_classify_as_general_subintent() {
    // Arrange
    let mut allocator = TestAddressAllocator::new();
    let account1 = allocator.new_account_address();
    let account2 = allocator.new_account_address();
    let manifest = ManifestBuilder::new_subintent_v2()
        .withdraw_from_account(account1, XRD, 10)
        .take_all_from_worktop(XRD, "xrd")
        .try_deposit_or_abort(account2, None, "xrd")
        .set_metadata(account1, "key", "value")
        .yield_to_parent(())
        .build();

    // Act
    let StaticAnalysis {
        manifest_classification,
        ..
    } = statically_analyze(&manifest).unwrap();

    // Assert
    assert_eq!(manifest_classification.len(), 0);
}
