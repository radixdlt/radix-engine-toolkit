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

use radix_engine_toolkit_core::instruction_visitor::core::traverser::traverse;
use radix_engine_toolkit_core::instruction_visitor::visitors::account_interactions_visitor::AccountInteractionsVisitor;
use radix_engine_toolkit_core::instruction_visitor::visitors::identity_interactions_visitor::IdentityInteractionsVisitor;
use scrypto::{api::node_modules::metadata::MetadataValue, prelude::*};
use transaction::prelude::*;

#[test]
fn account_withdraw_fungibles_interactions_count_as_withdraws_and_auth_requiring_interactions() {
    // Arrange
    let account = account();
    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account, XRD, dec!("1"))
        .build();

    // Act
    let (accounts_requiring_auth, accounts_withdrawn_from, accounts_deposited_into) =
        account_interactions(&manifest.instructions);

    // Assert
    assert_eq!(accounts_requiring_auth.len(), 1);
    assert_eq!(accounts_withdrawn_from.len(), 1);
    assert_eq!(accounts_deposited_into.len(), 0);

    assert!(accounts_requiring_auth.contains(&account));
    assert!(accounts_withdrawn_from.contains(&account));
}

#[test]
fn account_withdraw_non_fungibles_interactions_count_as_withdraws_and_auth_requiring_interactions()
{
    // Arrange
    let account = account();
    let ids = non_fungible_ids();
    let manifest = ManifestBuilder::new()
        .withdraw_non_fungibles_from_account(account, XRD, &ids)
        .build();

    // Act
    let (accounts_requiring_auth, accounts_withdrawn_from, accounts_deposited_into) =
        account_interactions(&manifest.instructions);

    // Assert
    assert_eq!(accounts_requiring_auth.len(), 1);
    assert_eq!(accounts_withdrawn_from.len(), 1);
    assert_eq!(accounts_deposited_into.len(), 0);

    assert!(accounts_requiring_auth.contains(&account));
    assert!(accounts_withdrawn_from.contains(&account));
}

#[test]
fn account_lock_fee_and_withdraw_fungibles_interactions_count_as_withdraws_and_auth_requiring_interactions(
) {
    // Arrange
    let account = account();
    let manifest = ManifestBuilder::new()
        .lock_fee_and_withdraw(account, dec!("1"), XRD, dec!("1"))
        .build();

    // Act
    let (accounts_requiring_auth, accounts_withdrawn_from, accounts_deposited_into) =
        account_interactions(&manifest.instructions);

    // Assert
    assert_eq!(accounts_requiring_auth.len(), 1);
    assert_eq!(accounts_withdrawn_from.len(), 1);
    assert_eq!(accounts_deposited_into.len(), 0);

    assert!(accounts_requiring_auth.contains(&account));
    assert!(accounts_withdrawn_from.contains(&account));
}

#[test]
fn account_lock_fee_and_withdraw_non_fungibles_interactions_count_as_withdraws_and_auth_requiring_interactions(
) {
    // Arrange
    let account = account();
    let ids = non_fungible_ids();
    let manifest = ManifestBuilder::new()
        .lock_fee_and_withdraw_non_fungibles(account, dec!("1"), XRD, &ids)
        .build();

    // Act
    let (accounts_requiring_auth, accounts_withdrawn_from, accounts_deposited_into) =
        account_interactions(&manifest.instructions);

    // Assert
    assert_eq!(accounts_requiring_auth.len(), 1);
    assert_eq!(accounts_withdrawn_from.len(), 1);
    assert_eq!(accounts_deposited_into.len(), 0);

    assert!(accounts_requiring_auth.contains(&account));
    assert!(accounts_withdrawn_from.contains(&account));
}

#[test]
fn account_lock_fee_interactions_count_as_auth_requiring_interactions() {
    // Arrange
    let account = account();
    let manifest = ManifestBuilder::new().lock_fee(account, dec!("1")).build();

    // Act
    let (accounts_requiring_auth, accounts_withdrawn_from, accounts_deposited_into) =
        account_interactions(&manifest.instructions);

    // Assert
    assert_eq!(accounts_requiring_auth.len(), 1);
    assert_eq!(accounts_withdrawn_from.len(), 0);
    assert_eq!(accounts_deposited_into.len(), 0);

    assert!(accounts_requiring_auth.contains(&account));
}

#[test]
fn account_lock_contingent_fee_interactions_count_as_auth_requiring_interactions() {
    // Arrange
    let account = account();
    let manifest = ManifestBuilder::new()
        .lock_contingent_fee(account, dec!("1"))
        .build();

    // Act
    let (accounts_requiring_auth, accounts_withdrawn_from, accounts_deposited_into) =
        account_interactions(&manifest.instructions);

    // Assert
    assert_eq!(accounts_requiring_auth.len(), 1);
    assert_eq!(accounts_withdrawn_from.len(), 0);
    assert_eq!(accounts_deposited_into.len(), 0);

    assert!(accounts_requiring_auth.contains(&account));
}

#[test]
fn account_create_proof_interactions_count_as_auth_requiring_interactions() {
    // Arrange
    let account = account();
    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_amount(account, XRD, dec!("10"))
        .build();

    // Act
    let (accounts_requiring_auth, accounts_withdrawn_from, accounts_deposited_into) =
        account_interactions(&manifest.instructions);

    // Assert
    assert_eq!(accounts_requiring_auth.len(), 1);
    assert_eq!(accounts_withdrawn_from.len(), 0);
    assert_eq!(accounts_deposited_into.len(), 0);

    assert!(accounts_requiring_auth.contains(&account));
}

#[test]
fn account_create_proof_by_amount_interactions_count_as_auth_requiring_interactions() {
    // Arrange
    let account = account();
    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_amount(account, XRD, dec!("1"))
        .build();

    // Act
    let (accounts_requiring_auth, accounts_withdrawn_from, accounts_deposited_into) =
        account_interactions(&manifest.instructions);

    // Assert
    assert_eq!(accounts_requiring_auth.len(), 1);
    assert_eq!(accounts_withdrawn_from.len(), 0);
    assert_eq!(accounts_deposited_into.len(), 0);

    assert!(accounts_requiring_auth.contains(&account));
}

#[test]
fn account_create_proof_by_ids_interactions_count_as_auth_requiring_interactions() {
    // Arrange
    let account = account();
    let ids = non_fungible_ids();
    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_non_fungibles(account, XRD, &ids)
        .build();

    // Act
    let (accounts_requiring_auth, accounts_withdrawn_from, accounts_deposited_into) =
        account_interactions(&manifest.instructions);

    // Assert
    assert_eq!(accounts_requiring_auth.len(), 1);
    assert_eq!(accounts_withdrawn_from.len(), 0);
    assert_eq!(accounts_deposited_into.len(), 0);

    assert!(accounts_requiring_auth.contains(&account));
}

#[test]
fn account_deposit_interactions_count_as_deposit_interactions() {
    // Arrange
    let account = account();
    let manifest = ManifestBuilder::new()
        .take_all_from_worktop(XRD, "bucket")
        .with_bucket("bucket", |builder, bucket| {
            builder.call_method(account, "deposit", manifest_args!(bucket))
        })
        .build();

    // Act
    let (accounts_requiring_auth, accounts_withdrawn_from, accounts_deposited_into) =
        account_interactions(&manifest.instructions);

    // Assert
    assert_eq!(accounts_requiring_auth.len(), 1);
    assert_eq!(accounts_withdrawn_from.len(), 0);
    assert_eq!(accounts_deposited_into.len(), 1);

    assert!(accounts_deposited_into.contains(&account));
}

#[test]
fn account_deposit_batch_interactions_count_as_deposit_interactions() {
    // Arrange
    let account = account();
    let manifest = ManifestBuilder::new()
        .call_method(
            account,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop),
        )
        .build();

    // Act
    let (accounts_requiring_auth, accounts_withdrawn_from, accounts_deposited_into) =
        account_interactions(&manifest.instructions);

    // Assert
    assert_eq!(accounts_requiring_auth.len(), 1);
    assert_eq!(accounts_withdrawn_from.len(), 0);
    assert_eq!(accounts_deposited_into.len(), 1);

    assert!(accounts_deposited_into.contains(&account));
}

#[test]
fn account_set_metadata_interactions_count_as_auth_requiring_interactions() {
    // Arrange
    let account = account();
    let manifest = ManifestBuilder::new()
        .set_metadata(account, "x", MetadataValue::Bool(true))
        .build();

    // Act
    let (accounts_requiring_auth, accounts_withdrawn_from, accounts_deposited_into) =
        account_interactions(&manifest.instructions);

    // Assert
    assert_eq!(accounts_requiring_auth.len(), 1);
    assert_eq!(accounts_withdrawn_from.len(), 0);
    assert_eq!(accounts_deposited_into.len(), 0);

    assert!(accounts_requiring_auth.contains(&account));
}

#[test]
fn identity_set_component_royalty_interactions_count_as_auth_requiring_interactions() {
    // Arrange
    let identity = identity();
    let manifest = ManifestBuilder::new()
        .set_component_royalty(identity, "method", RoyaltyAmount::Free)
        .build();

    // Act
    let identities_requiring_auth = identity_interactions(&manifest.instructions);

    // Assert
    assert_eq!(identities_requiring_auth.len(), 1);
    assert!(identities_requiring_auth.contains(&identity));
}

#[test]
fn identity_set_metadata_interactions_count_as_auth_requiring_interactions() {
    // Arrange
    let identity = identity();
    let manifest = ManifestBuilder::new()
        .set_metadata(identity, "x", MetadataValue::Bool(true))
        .build();

    // Act
    let identities_requiring_auth = identity_interactions(&manifest.instructions);

    // Assert
    assert_eq!(identities_requiring_auth.len(), 1);
    assert!(identities_requiring_auth.contains(&identity));
}

#[test]
fn identity_claim_component_royalty_counts_as_auth_requiring_interactions() {
    // Arrange
    let identity = identity();
    let manifest = ManifestBuilder::new()
        .claim_component_royalties(identity)
        .build();

    // Act
    let identities_requiring_auth = identity_interactions(&manifest.instructions);

    // Assert
    assert_eq!(identities_requiring_auth.len(), 1);
    assert!(identities_requiring_auth.contains(&identity));
}

fn non_fungible_ids() -> BTreeSet<NonFungibleLocalId> {
    BTreeSet::from([NonFungibleLocalId::integer(1)])
}

fn account() -> ComponentAddress {
    let private_key = Secp256k1PrivateKey::from_u64(1).unwrap();
    let public_key = private_key.public_key();
    ComponentAddress::virtual_account_from_public_key(&public_key)
}

fn identity() -> ComponentAddress {
    let private_key = Secp256k1PrivateKey::from_u64(1).unwrap();
    let public_key = private_key.public_key();
    ComponentAddress::virtual_identity_from_public_key(&public_key)
}

fn account_interactions(
    instructions: &[InstructionV1],
) -> (
    HashSet<ComponentAddress>,
    HashSet<ComponentAddress>,
    HashSet<ComponentAddress>,
) {
    let mut account_interactions_visitor = AccountInteractionsVisitor::default();
    traverse(instructions, &mut [&mut account_interactions_visitor]).unwrap();
    account_interactions_visitor.output()
}

fn identity_interactions(instructions: &[InstructionV1]) -> HashSet<ComponentAddress> {
    let mut identity_interactions_visitor = IdentityInteractionsVisitor::default();
    traverse(instructions, &mut [&mut identity_interactions_visitor]).unwrap();
    identity_interactions_visitor.output()
}
