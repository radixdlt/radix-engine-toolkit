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

use native_transaction::builder::{ManifestBuilder, TransactionBuilder};
use native_transaction::ecdsa_secp256k1::EcdsaSecp256k1PrivateKey;
use native_transaction::prelude::{
    NotarizedTransactionV1, TransactionHeaderV1, TransactionPayload,
};
use radix_engine_common::ManifestSbor;
use radix_engine_toolkit::functions::traits::InvocationHandler;
use radix_engine_toolkit::functions::{
    compile_notarized_transaction, decompile_notarized_transaction,
};
use scrypto::prelude::*;

#[test]
fn decompilation_and_compilation_of_simple_transfer_succeeds() {
    // Arrange
    let private_key1 = EcdsaSecp256k1PrivateKey::from_u64(1).unwrap();
    let private_key2 = EcdsaSecp256k1PrivateKey::from_u64(2).unwrap();

    let account1 = ComponentAddress::virtual_account_from_public_key(&private_key1.public_key());
    let account2 = ComponentAddress::virtual_account_from_public_key(&private_key2.public_key());

    let manifest = ManifestBuilder::new()
        .lock_fee(account1, 10.into())
        .withdraw_from_account(account1, RADIX_TOKEN, 1.into())
        .call_method(
            account2,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop),
        )
        .build();

    let transaction = TransactionBuilder::new()
        .header(header(0x0c, private_key1.public_key()))
        .manifest(manifest)
        .notarize(&private_key1)
        .build();

    // Act & Assert
    test_inversion(&transaction);
}

#[test]
fn decompilation_and_compilation_of_creating_a_simple_fungible_resource_succeeds() {
    // Arrange
    let private_key = EcdsaSecp256k1PrivateKey::from_u64(1).unwrap();
    let account = ComponentAddress::virtual_account_from_public_key(&private_key.public_key());

    let manifest = ManifestBuilder::new()
        .lock_fee(account, 10.into())
        .create_fungible_resource(
            18,
            BTreeMap::new(),
            BTreeMap::<_, (_, AccessRule)>::new(),
            None,
        )
        .build();

    let transaction = TransactionBuilder::new()
        .header(header(0x0c, private_key.public_key()))
        .manifest(manifest)
        .notarize(&private_key)
        .build();

    // Act & Assert
    test_inversion(&transaction);
}

#[test]
fn decompilation_and_compilation_of_creating_a_simple_non_fungible_resource_succeeds() {
    // Arrange
    let private_key = EcdsaSecp256k1PrivateKey::from_u64(1).unwrap();
    let account = ComponentAddress::virtual_account_from_public_key(&private_key.public_key());

    let manifest = ManifestBuilder::new()
        .lock_fee(account, 10.into())
        .create_non_fungible_resource(
            NonFungibleIdType::Integer,
            BTreeMap::new(),
            BTreeMap::<_, (_, AccessRule)>::new(),
            None::<BTreeMap<NonFungibleLocalId, EmptyStruct>>,
        )
        .build();

    let transaction = TransactionBuilder::new()
        .header(header(0x0c, private_key.public_key()))
        .manifest(manifest)
        .notarize(&private_key)
        .build();

    // Act & Assert
    test_inversion(&transaction);
}

#[test]
fn decompilation_and_compilation_of_creating_a_simple_non_fungible_resource_with_initial_supply_succeeds(
) {
    // Arrange
    let private_key = EcdsaSecp256k1PrivateKey::from_u64(1).unwrap();
    let account = ComponentAddress::virtual_account_from_public_key(&private_key.public_key());

    let manifest = ManifestBuilder::new()
        .lock_fee(account, 10.into())
        .create_non_fungible_resource(
            NonFungibleIdType::Integer,
            BTreeMap::new(),
            BTreeMap::<_, (_, AccessRule)>::new(),
            Some([(
                NonFungibleLocalId::Integer(IntegerNonFungibleLocalId::new(1)),
                EmptyStruct {},
            )]),
        )
        .build();

    let transaction = TransactionBuilder::new()
        .header(header(0x0c, private_key.public_key()))
        .manifest(manifest)
        .notarize(&private_key)
        .build();

    // Act & Assert
    test_inversion(&transaction);
}

#[test]
fn decompilation_and_compilation_of_minting_non_fungible_tokens_succeeds() {
    // Arrange
    let private_key = EcdsaSecp256k1PrivateKey::from_u64(1).unwrap();
    let account = ComponentAddress::virtual_account_from_public_key(&private_key.public_key());

    let manifest = ManifestBuilder::new()
        .lock_fee(account, 10.into())
        .mint_non_fungible(
            RADIX_TOKEN,
            [(
                NonFungibleLocalId::Integer(IntegerNonFungibleLocalId::new(1)),
                EmptyStruct {},
            )],
        )
        .build();

    let transaction = TransactionBuilder::new()
        .header(header(0x0c, private_key.public_key()))
        .manifest(manifest)
        .notarize(&private_key)
        .build();

    // Act & Assert
    test_inversion(&transaction);
}

fn header<P: Into<PublicKey>>(network_id: u8, notary_public_key: P) -> TransactionHeaderV1 {
    TransactionHeaderV1 {
        network_id,
        start_epoch_inclusive: Epoch::of(10),
        end_epoch_exclusive: Epoch::of(13),
        nonce: 0x02,
        notary_public_key: notary_public_key.into(),
        notary_is_signatory: true,
        tip_percentage: 0,
    }
}

fn test_inversion(transaction: &NotarizedTransactionV1) {
    let native_compiled = transaction.to_payload_bytes().unwrap();
    let compiled_from_decompiled = {
        let input = decompile_notarized_transaction::Input {
            compiled_notarized_intent: native_compiled.clone(),
            instructions_output_kind:
                radix_engine_toolkit::model::transaction::InstructionKind::String,
        };
        let output = decompile_notarized_transaction::Handler::fulfill(input).unwrap();

        let input = compile_notarized_transaction::Input {
            notarized_intent: output.notarized_intent,
        };
        let output = compile_notarized_transaction::Handler::fulfill(input).unwrap();
        output.compiled_intent
    };
    assert_eq!(native_compiled, compiled_from_decompiled)
}

#[derive(ScryptoSbor, NonFungibleData, ManifestSbor)]
struct EmptyStruct {}
