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
use native_transaction::manifest::{compile, decompile};
use native_transaction::model::{NotarizedTransaction, TransactionHeader};
use native_transaction::validation::ValidationConfig;
use radix_engine_common::ManifestSbor;
use radix_engine_constants::DEFAULT_COST_UNIT_LIMIT;
use radix_engine_toolkit::functions::statically_validate_transaction;
use radix_engine_toolkit::functions::traits::InvocationHandler;
use scrypto::prelude::*;

#[test]
fn static_validation_of_simple_transfer_succeeds() {
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

    // Act
    test_inversion(&transaction);
    let validation_result = statically_validate(&transaction);

    // Assert
    assert_eq!(
        validation_result,
        statically_validate_transaction::Output::Valid
    );
}

#[test]
fn static_validation_of_creating_a_simple_fungible_resource_succeeds() {
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

    // Act
    test_inversion(&transaction);
    let validation_result = statically_validate(&transaction);

    // Assert
    assert_eq!(
        validation_result,
        statically_validate_transaction::Output::Valid
    );
}

#[test]
fn static_validation_of_creating_a_simple_non_fungible_resource_succeeds() {
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

    // Act
    test_inversion(&transaction);
    let validation_result = statically_validate(&transaction);

    // Assert
    assert_eq!(
        validation_result,
        statically_validate_transaction::Output::Valid
    );
}

#[test]
fn static_validation_of_creating_a_simple_non_fungible_resource_with_initial_supply_succeeds() {
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

    // Act
    test_inversion(&transaction);
    let validation_result = statically_validate(&transaction);

    // Assert
    assert_eq!(
        validation_result,
        statically_validate_transaction::Output::Valid
    );
}

#[test]
fn static_validation_of_minting_non_fungible_tokens_succeeds() {
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

    // Act
    test_inversion(&transaction);
    let validation_result = statically_validate(&transaction);

    // Assert
    assert_eq!(
        validation_result,
        statically_validate_transaction::Output::Valid
    );
}

fn header<P: Into<PublicKey>>(network_id: u8, notary_public_key: P) -> TransactionHeader {
    TransactionHeader {
        version: 0x01,
        network_id,
        start_epoch_inclusive: 10,
        end_epoch_exclusive: 13,
        nonce: 0x02,
        notary_public_key: notary_public_key.into(),
        notary_as_signatory: true,
        cost_unit_limit: DEFAULT_COST_UNIT_LIMIT,
        tip_percentage: 0,
    }
}

fn test_inversion(transaction: &NotarizedTransaction) {
    let passed_manifest = transaction.signed_intent.intent.manifest.clone();
    let inverted_manifest = {
        let network =
            radix_engine_toolkit::model::address::utils::network_definition_from_network_id(
                transaction.signed_intent.intent.header.network_id,
            );
        let decompiled = decompile(&passed_manifest.instructions, &network).unwrap();
        compile(&decompiled, &network, vec![]).unwrap()
    };
    assert_eq!(passed_manifest, inverted_manifest);
}

fn statically_validate(
    transaction: &NotarizedTransaction,
) -> statically_validate_transaction::Output {
    let encoded_transaction = manifest_encode(&transaction).unwrap();
    let request = statically_validate_transaction::Input {
        compiled_notarized_intent: encoded_transaction,
        validation_config: ValidationConfig::default(
            transaction.signed_intent.intent.header.network_id,
        ),
    };
    statically_validate_transaction::Handler::fulfill(request).unwrap()
}

#[derive(ScryptoSbor, NonFungibleData, ManifestSbor)]
struct EmptyStruct {}
