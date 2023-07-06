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
use radix_engine_toolkit_core::instruction_visitor::visitors::account_proofs_visitor::AccountProofsVisitor;
use scrypto::prelude::*;
use transaction::prelude::*;

#[test]
fn account_proofs_visitor_picks_up_on_calls_to_create_proof() {
    // Arrange
    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_amount(account(), RADIX_TOKEN, dec!("1"))
        .create_proof_from_account_of_amount(account(), ACCOUNT_OWNER_BADGE, dec!("1"))
        .build();

    // Act
    let resource_addresses = account_proofs(&manifest.instructions);

    // Assert
    assert_eq!(
        resource_addresses,
        HashSet::from([RADIX_TOKEN, ACCOUNT_OWNER_BADGE])
    );
}

#[test]
fn account_proofs_visitor_picks_up_on_calls_to_create_proof_of_amount() {
    // Arrange
    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_amount(account(), RADIX_TOKEN, dec!("10"))
        .create_proof_from_account_of_amount(account(), ACCOUNT_OWNER_BADGE, dec!("10"))
        .build();

    // Act
    let resource_addresses = account_proofs(&manifest.instructions);

    // Assert
    assert_eq!(
        resource_addresses,
        HashSet::from([RADIX_TOKEN, ACCOUNT_OWNER_BADGE])
    );
}

#[test]
fn account_proofs_visitor_picks_up_on_calls_to_create_proof_of_non_fungibles() {
    // Arrange
    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_non_fungibles(
            account(),
            RADIX_TOKEN,
            &BTreeSet::from([NonFungibleLocalId::integer(1)]),
        )
        .create_proof_from_account_of_non_fungibles(
            account(),
            ACCOUNT_OWNER_BADGE,
            &BTreeSet::from([NonFungibleLocalId::integer(1)]),
        )
        .build();

    // Act
    let resource_addresses = account_proofs(&manifest.instructions);

    // Assert
    assert_eq!(
        resource_addresses,
        HashSet::from([RADIX_TOKEN, ACCOUNT_OWNER_BADGE])
    );
}

fn account() -> ComponentAddress {
    let private_key = Secp256k1PrivateKey::from_u64(1).unwrap();
    let public_key = private_key.public_key();
    ComponentAddress::virtual_account_from_public_key(&public_key)
}

fn account_proofs(instructions: &[InstructionV1]) -> HashSet<ResourceAddress> {
    let mut account_proofs_visitor = AccountProofsVisitor::default();
    traverse(instructions, &mut [&mut account_proofs_visitor]).unwrap();
    account_proofs_visitor.output()
}
