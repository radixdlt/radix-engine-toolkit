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

use std::collections::BTreeSet;

use native_transaction::{builder::ManifestBuilder, ecdsa_secp256k1::EcdsaSecp256k1PrivateKey};
use radix_engine_toolkit::model::address::Bech32Coder;
use radix_engine_toolkit::model::transaction::{
    InstructionKind, InstructionList, TransactionManifest,
};
use radix_engine_toolkit::visitor::{traverse_instruction, AccountProofsInstructionVisitor};
use scrypto::prelude::{
    ComponentAddress, IntegerNonFungibleLocalId, NonFungibleLocalId, RADIX_TOKEN,
};

#[test]
fn account_create_proof_is_captured_by_visitor() {
    // Arrange
    let account = {
        let private_key = EcdsaSecp256k1PrivateKey::from_u64(1).unwrap();
        let public_key = private_key.public_key();
        ComponentAddress::virtual_account_from_public_key(&public_key)
    };

    let mut instructions = {
        let native_manifest = ManifestBuilder::new()
            .create_proof_from_account(account, RADIX_TOKEN)
            .build();
        let manifest = TransactionManifest::from_native_manifest(
            &native_manifest,
            InstructionKind::Parsed,
            &Bech32Coder::new(0x01),
        )
        .unwrap();

        if let InstructionList::Parsed(instructions) = manifest.instructions {
            instructions
        } else {
            panic!("Impossible case!")
        }
    };

    let mut visitor = AccountProofsInstructionVisitor::default();

    // Act
    for instruction in instructions.iter_mut() {
        traverse_instruction(instruction, &mut [], &mut [&mut visitor]).unwrap();
    }

    // Assert
    assert_eq!(
        BTreeSet::from([RADIX_TOKEN]),
        visitor
            .created_proofs
            .into_iter()
            .map(|item| item.address)
            .collect()
    )
}

#[test]
fn account_create_proof_by_amount_is_captured_by_visitor() {
    // Arrange
    let account = {
        let private_key = EcdsaSecp256k1PrivateKey::from_u64(1).unwrap();
        let public_key = private_key.public_key();
        ComponentAddress::virtual_account_from_public_key(&public_key)
    };

    let mut instructions = {
        let native_manifest = ManifestBuilder::new()
            .create_proof_from_account_by_amount(account, RADIX_TOKEN, 1.into())
            .build();
        let manifest = TransactionManifest::from_native_manifest(
            &native_manifest,
            InstructionKind::Parsed,
            &Bech32Coder::new(0x01),
        )
        .unwrap();

        if let InstructionList::Parsed(instructions) = manifest.instructions {
            instructions
        } else {
            panic!("Impossible case!")
        }
    };

    let mut visitor = AccountProofsInstructionVisitor::default();

    // Act
    for instruction in instructions.iter_mut() {
        traverse_instruction(instruction, &mut [], &mut [&mut visitor]).unwrap();
    }

    // Assert
    assert_eq!(
        BTreeSet::from([RADIX_TOKEN]),
        visitor
            .created_proofs
            .into_iter()
            .map(|item| item.address)
            .collect()
    )
}

#[test]
fn account_create_proof_by_ids_is_captured_by_visitor() {
    // Arrange
    let account = {
        let private_key = EcdsaSecp256k1PrivateKey::from_u64(1).unwrap();
        let public_key = private_key.public_key();
        ComponentAddress::virtual_account_from_public_key(&public_key)
    };

    let mut instructions = {
        let native_manifest = ManifestBuilder::new()
            .create_proof_from_account_by_ids(
                account,
                RADIX_TOKEN,
                &[NonFungibleLocalId::Integer(IntegerNonFungibleLocalId::new(
                    1,
                ))]
                .into(),
            )
            .build();
        let manifest = TransactionManifest::from_native_manifest(
            &native_manifest,
            InstructionKind::Parsed,
            &Bech32Coder::new(0x01),
        )
        .unwrap();

        if let InstructionList::Parsed(instructions) = manifest.instructions {
            instructions
        } else {
            panic!("Impossible case!")
        }
    };

    let mut visitor = AccountProofsInstructionVisitor::default();

    // Act
    for instruction in instructions.iter_mut() {
        traverse_instruction(instruction, &mut [], &mut [&mut visitor]).unwrap();
    }

    // Assert
    assert_eq!(
        BTreeSet::from([RADIX_TOKEN]),
        visitor
            .created_proofs
            .into_iter()
            .map(|item| item.address)
            .collect()
    )
}
