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

use native_transaction::builder::TransactionBuilder;
use native_transaction::ecdsa_secp256k1::EcdsaSecp256k1PrivateKey;
use native_transaction::eddsa_ed25519::EddsaEd25519PrivateKey;
use native_transaction::manifest::compile;
use native_transaction::model::{NotarizedTransaction, TransactionHeader};
use radix_engine_toolkit::utils::checked_copy_u8_slice;
use scrypto::network::NetworkDefinition;
use scrypto::prelude::manifest_encode;

use radix_engine_toolkit::model::address::Bech32Coder;
use radix_engine_toolkit::model::transaction::InstructionKind;
use radix_engine_toolkit::model::value::scrypto_sbor::{ScryptoSborValue, ScryptoSborValueKind};

use radix_engine_toolkit::request::traits::Handler;
use scrypto::prelude::*;
use serde::Serialize;

use radix_engine_toolkit::request::*;

pub fn network_definition() -> NetworkDefinition {
    NetworkDefinition::simulator()
}

pub fn notarized_intent() -> NotarizedTransaction {
    TransactionBuilder::new()
        .manifest(
            compile(
                include_str!(
                    "../../radix-engine-toolkit/tests/test_vector/manifest/resources/worktop.rtm"
                ),
                &network_definition(),
                vec![],
            )
            .unwrap(),
        )
        .header(TransactionHeader {
            version: 0x01,
            network_id: network_definition().id,
            cost_unit_limit: 100_000_000,
            start_epoch_inclusive: 0x200,
            end_epoch_exclusive: 0x210,
            nonce: 0x22,
            notary_as_signatory: true,
            notary_public_key: notary_private_key().public_key().into(),
            tip_percentage: 0x00,
        })
        .sign(&EcdsaSecp256k1PrivateKey::from_u64(1).unwrap())
        .sign(&EcdsaSecp256k1PrivateKey::from_u64(2).unwrap())
        .sign(&EcdsaSecp256k1PrivateKey::from_u64(3).unwrap())
        .sign(&EddsaEd25519PrivateKey::from_u64(1).unwrap())
        .sign(&EddsaEd25519PrivateKey::from_u64(2).unwrap())
        .sign(&EddsaEd25519PrivateKey::from_u64(3).unwrap())
        .notarize(&notary_private_key())
        .build()
}

pub fn notary_private_key() -> EcdsaSecp256k1PrivateKey {
    EcdsaSecp256k1PrivateKey::from_u64(1923112).unwrap()
}

pub fn value() -> ScryptoSborValue {
    ScryptoSborValue::Tuple {
        elements: vec![
            ScryptoSborValue::Decimal { value: dec!("10") },
            ScryptoSborValue::PreciseDecimal { value: pdec!("10") },
            ScryptoSborValue::String {
                value: "Hello World!".into(),
            },
            ScryptoSborValue::Tuple {
                elements: vec![
                    ScryptoSborValue::Decimal { value: dec!("10") },
                    ScryptoSborValue::PreciseDecimal { value: pdec!("10") },
                    ScryptoSborValue::String {
                        value: "Hello World!".into(),
                    },
                    ScryptoSborValue::Tuple {
                        elements: vec![
                            ScryptoSborValue::Decimal { value: dec!("10") },
                            ScryptoSborValue::PreciseDecimal { value: pdec!("10") },
                            ScryptoSborValue::String {
                                value: "Hello World!".into(),
                            },
                            ScryptoSborValue::Tuple {
                                elements: vec![
                                    ScryptoSborValue::Decimal { value: dec!("10") },
                                    ScryptoSborValue::PreciseDecimal { value: pdec!("10") },
                                    ScryptoSborValue::String {
                                        value: "Hello World!".into(),
                                    },
                                    ScryptoSborValue::Array {
                                        element_kind: ScryptoSborValueKind::Decimal,
                                        elements: vec![
                                            ScryptoSborValue::Decimal { value: dec!("20") },
                                            ScryptoSborValue::Decimal { value: dec!("100") },
                                            ScryptoSborValue::Decimal {
                                                value: dec!("192.31"),
                                            },
                                        ],
                                    },
                                ],
                            },
                        ],
                    },
                ],
            },
        ],
    }
}

pub struct Example {
    pub request_type_name: String,
    pub response_type_name: String,
    pub request_description: String,
    pub required_features: String,
    pub request: String,
    pub response: String,
}

pub trait ExampleData<I, O>
where
    Self: Handler<I, O>,
    I: Serialize + Clone + Debug,
    O: Serialize + Clone + Debug,
{
    fn description() -> String;

    fn example_request() -> I;

    fn example_response() -> O {
        Self::fulfill(Self::example_request())
            .expect(format!("Failed. Request: {:?}", Self::example_request()).as_str())
    }

    fn request_type_name() -> String {
        std::any::type_name::<I>()
            .split("::")
            .last()
            .unwrap()
            .to_owned()
    }
    fn response_type_name() -> String {
        std::any::type_name::<O>()
            .split("::")
            .last()
            .unwrap()
            .to_owned()
    }

    fn required_features() -> String {
        "default".into()
    }

    fn to_example() -> Example {
        let request = Self::example_request();
        let response = Self::example_response();
        Example {
            request_type_name: Self::request_type_name(),
            response_type_name: Self::response_type_name(),
            request_description: Self::description(),
            required_features: Self::required_features(),
            request: serde_json::to_string_pretty(&request).unwrap(),
            response: serde_json::to_string_pretty(&response).unwrap(),
        }
    }
}

impl ExampleData<InformationRequest, InformationResponse> for InformationHandler {
    fn description() -> String {
        r#"The function provides information information on the currently in-use radix engine toolkit such as the version of the radix engine toolkit. In most cases, this is the first function written when integrating new clients; so, this function is often times seen as the "Hello World" example of the radix engine toolkit."#.to_owned()
    }

    fn example_request() -> InformationRequest {
        InformationRequest {}
    }

    fn example_response() -> InformationResponse {
        let mut response = Self::fulfill(Self::example_request()).unwrap();
        response.last_commit_hash =
            "This is just an example. We don't have a commit hash here".into();
        response
    }
}

impl ExampleData<ConvertManifestRequest, ConvertManifestResponse> for ConvertManifestHandler {
    fn description() -> String {
        r#"Clients have a need to be able to read, parse, understand, and interrogate transaction manifests to get more information on what a transactions might be doing. Transaction manifests have so far existed in one format: as strings. While the string format is very human readable, it is not easily readable by machines as a lexer and parser are needed to make sense of them; thus, it is for clients to programmatically make sense of transactions. As such, there is a need for another transaction manifest format (to supplement, NOT replace) which machines can easily make sense of without the need to implement a lexer and parser.

Therefore, this library introduces a Parsed format for transaction manifests which clients can use when wanting to read and interrogate their transaction manifests in code. The transaction manifest Parsed format has a 1:1 mapping to the string format of transaction manifests, meaning that anything which can be done in the string format of transaction manifests, can be done in the Parsed format as well.

This function allows the client the convert their manifest between the two supported manifest types: string and Parsed."#.to_owned()
    }

    fn example_request() -> ConvertManifestRequest {
        let bec32_coder = Bech32Coder::new(network_definition().id);
        ConvertManifestRequest {
            manifest:
                radix_engine_toolkit::model::transaction::TransactionManifest::from_native_manifest(
                    &notarized_intent().signed_intent.intent.manifest,
                    InstructionKind::Parsed,
                    &bec32_coder,
                )
                .unwrap(),
            instructions_output_kind:
                radix_engine_toolkit::model::transaction::InstructionKind::Parsed,
            network_id: network_definition().id,
        }
    }
}

impl ExampleData<AnalyzeManifestRequest, AnalyzeManifestResponse> for AnalyzeManifestHandler {
    fn description() -> String {
        r#"Analyzes the manifest returning back all of the addresses involved in the manifest alongside some useful information on whether the accounts were withdrawn from, deposited into, or just used in the manifest in general."#.to_owned()
    }

    fn example_request() -> AnalyzeManifestRequest {
        let bec32_coder = Bech32Coder::new(network_definition().id);
        AnalyzeManifestRequest {
            manifest:
                radix_engine_toolkit::model::transaction::TransactionManifest::from_native_manifest(
                    &notarized_intent().signed_intent.intent.manifest,
                    InstructionKind::String,
                    &bec32_coder,
                )
                .unwrap(),
            network_id: network_definition().id,
        }
    }
}

impl ExampleData<CompileTransactionIntentRequest, CompileTransactionIntentResponse>
    for CompileTransactionIntentHandler
{
    fn description() -> String {
        r#"Takes a transaction intent and compiles it by SBOR encoding it and returning it back to the caller. This is mainly useful when creating a transaction."#.to_owned()
    }

    fn example_request() -> CompileTransactionIntentRequest {
        CompileTransactionIntentRequest {
            transaction_intent:
                radix_engine_toolkit::model::transaction::TransactionIntent::from_native_transaction_intent(
                    &notarized_intent().signed_intent.intent,
                    InstructionKind::Parsed,
                )
                .unwrap(),
        }
    }
}

impl
    ExampleData<DeriveOlympiaAddressFromPublicKeyRequest, DeriveOlympiaAddressFromPublicKeyResponse>
    for DeriveOlympiaAddressFromPublicKeyHandler
{
    fn description() -> String {
        r#"Given an ECDSA Secp256k1 Public Key and Olympia network, this function derives the Olympia account address associated with the public key on that network."#
            .to_owned()
    }

    fn example_request() -> DeriveOlympiaAddressFromPublicKeyRequest {
        DeriveOlympiaAddressFromPublicKeyRequest {
            network: OlympiaNetwork::Mainnet,
            public_key: PublicKey::EcdsaSecp256k1(EcdsaSecp256k1PublicKey(
                checked_copy_u8_slice(
                    hex::decode(
                        "026f08db98ef1d0231eb15580da9123db8e25aa1747c8c32e5fd2ec47b8db73d5c",
                    )
                    .unwrap(),
                )
                .unwrap(),
            )),
        }
    }
}

impl ExampleData<DecompileTransactionIntentRequest, DecompileTransactionIntentResponse>
    for DecompileTransactionIntentHandler
{
    fn description() -> String {
        r#"This function does the opposite of the compile_transaction_intent function. It takes in a compiled transaction intent and decompiles it into its human-readable / machine-readable format."#.to_owned()
    }

    fn example_request() -> DecompileTransactionIntentRequest {
        let compiled_transaction_intent =
            manifest_encode(&notarized_intent().signed_intent.intent).unwrap();
        DecompileTransactionIntentRequest {
            compiled_intent: compiled_transaction_intent,
            instructions_output_kind:
                radix_engine_toolkit::model::transaction::InstructionKind::Parsed,
        }
    }
}

impl ExampleData<CompileSignedTransactionIntentRequest, CompileSignedTransactionIntentResponse>
    for CompileSignedTransactionIntentHandler
{
    fn description() -> String {
        r#"This function takes in a raw transaction intent as well as its signatures and compiles it. This is useful when a notary wishes to notarize a signed transaction intent."#.to_owned()
    }

    fn example_request() -> CompileSignedTransactionIntentRequest {
        CompileSignedTransactionIntentRequest {
            signed_intent: radix_engine_toolkit::model::transaction::SignedTransactionIntent::from_native_signed_transaction_intent(&notarized_intent()
            .signed_intent
            , InstructionKind::Parsed)
                .unwrap(),
        }
    }
}

impl ExampleData<DecompileSignedTransactionIntentRequest, DecompileSignedTransactionIntentResponse>
    for DecompileSignedTransactionIntentHandler
{
    fn description() -> String {
        r#"This function does the opposite of the compile_signed_transaction_intent function. This function takes in a compiled signed transaction intent and decompiles it into its transaction intent and signatures."#.to_owned()
    }

    fn example_request() -> DecompileSignedTransactionIntentRequest {
        let compiled_transaction_intent =
            manifest_encode(&notarized_intent().signed_intent).unwrap();
        DecompileSignedTransactionIntentRequest {
            compiled_signed_intent: compiled_transaction_intent,
            instructions_output_kind:
                radix_engine_toolkit::model::transaction::InstructionKind::Parsed,
        }
    }
}

impl ExampleData<CompileNotarizedTransactionRequest, CompileNotarizedTransactionResponse>
    for CompileNotarizedTransactionHandler
{
    fn description() -> String {
        r#"This function takes in a raw signed transaction intent as well as the notary signature and compiles it. This is useful when we wish to submit a transaction to the Gateway API"#.to_owned()
    }

    fn example_request() -> CompileNotarizedTransactionRequest {
        CompileNotarizedTransactionRequest {
            notarized_intent: radix_engine_toolkit::model::transaction::NotarizedTransaction::from_native_notarized_transaction_intent(&notarized_intent()
            , InstructionKind::Parsed)
                .unwrap(),
        }
    }
}

impl ExampleData<DecompileNotarizedTransactionRequest, DecompileNotarizedTransactionResponse>
    for DecompileNotarizedTransactionHandler
{
    fn description() -> String {
        r#"This function does the opposite of the compile_notarized_intent()_intent function. This function takes in a compiled notarized transaction intent and decompiles it into its signed transaction intent and notary signature."#.to_owned()
    }

    fn example_request() -> DecompileNotarizedTransactionRequest {
        let compiled_transaction_intent = manifest_encode(&notarized_intent()).unwrap();
        DecompileNotarizedTransactionRequest {
            compiled_notarized_intent: compiled_transaction_intent,
            instructions_output_kind:
                radix_engine_toolkit::model::transaction::InstructionKind::Parsed,
        }
    }
}

impl
    ExampleData<DecompileUnknownTransactionIntentRequest, DecompileUnknownTransactionIntentResponse>
    for DecompileUnknownTransactionIntentHandler
{
    fn description() -> String {
        r#"There are certain cases where we might have some blob which we suspect is a transaction intent but we have no way of verifying whether that is true or not. Looking at the type id byte of the blob does not help either as it's a generic Struct type which is not too telling. For this specific use case, this library provides this function which attempts to decompile a transaction intent of an unknown type."#.to_owned()
    }

    fn example_request() -> DecompileUnknownTransactionIntentRequest {
        let compiled_transaction_intent = manifest_encode(&notarized_intent()).unwrap();
        DecompileUnknownTransactionIntentRequest {
            compiled_unknown_intent: compiled_transaction_intent,
            instructions_output_kind:
                radix_engine_toolkit::model::transaction::InstructionKind::Parsed,
        }
    }
}

impl ExampleData<EncodeAddressRequest, EncodeAddressResponse> for EncodeAddressHandler {
    fn description() -> String {
        r#"This function can be used when we have a byte array which we wish to do Bech32m encoding on. In this case, the HRP to use will be determined through the entity byte of the passed address hex string."#.to_owned()
    }

    fn example_request() -> EncodeAddressRequest {
        EncodeAddressRequest {
            address_bytes: vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,
            ],
            network_id: 0xf2,
        }
    }
}

impl ExampleData<DecodeAddressRequest, DecodeAddressResponse> for DecodeAddressHandler {
    fn description() -> String {
        r#"This function can be used to decode a Bech32m encoded address string into its equivalent hrp and data. In addition to that, this function provides other useful information on the address such as the network id and name that it is used for, and the entity type of the address."#.to_owned()
    }

    fn example_request() -> DecodeAddressRequest {
        DecodeAddressRequest {
            address: "resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k".into(),
        }
    }
}

impl ExampleData<SborEncodeRequest, SborEncodeResponse> for SborEncodeHandler {
    fn description() -> String {
        r#"This function takes in a ScryptoSborValue and encodes it in SBOR."#.to_owned()
    }

    fn example_request() -> SborEncodeRequest {
        SborEncodeRequest::ScryptoSbor(value())
    }
}

impl ExampleData<SborDecodeRequest, SborDecodeResponse> for SborDecodeHandler {
    fn description() -> String {
        r#"This function takes in a hex string and attempts to decode it into a ScryptoSborValue."#
            .to_owned()
    }

    fn example_request() -> SborDecodeRequest {
        SborDecodeRequest {
            encoded_value: vec![
                77, // prefix
                33, // struct
                10, // field length
                128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, // address
                128, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, // address
                128, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
                2, // address
                129, 4, 0, 0, 0, // bucket
                130, 5, 0, 0, 0, // proof
                131, 1, // expression
                132, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
                6, 6, 6, 6, 6, 6, // blob
                133, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
                7, 7, 7, 7, 7, 7, // decimal
                134, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
                8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
                8, 8, 8, 8, 8, 8, 8, 8, 8, 8, // precise decimal
                135, 0, 3, 97, 98, 99, // non-fungible local id
            ],
            network_id: 0xf2,
        }
    }
}

impl ExampleData<DeriveVirtualAccountAddressRequest, DeriveVirtualAccountAddressResponse>
    for DeriveVirtualAccountAddressHandler
{
    fn description() -> String {
        r#"Derives the virtual account component address given a public key and a network id."#
            .to_owned()
    }

    fn example_request() -> DeriveVirtualAccountAddressRequest {
        DeriveVirtualAccountAddressRequest {
            network_id: 0xf2,
            public_key: notary_private_key().public_key().into(),
        }
    }
}

impl ExampleData<DeriveVirtualIdentityAddressRequest, DeriveVirtualIdentityAddressResponse>
    for DeriveVirtualIdentityAddressHandler
{
    fn description() -> String {
        r#"Derives the virtual identity component address given a public key and a network id."#
            .to_owned()
    }

    fn example_request() -> DeriveVirtualIdentityAddressRequest {
        DeriveVirtualIdentityAddressRequest {
            network_id: 0xf2,
            public_key: notary_private_key().public_key().into(),
        }
    }
}

impl
    ExampleData<
        DeriveNonFungibleGlobalIdFromPublicKeyRequest,
        DeriveNonFungibleGlobalIdFromPublicKeyResponse,
    > for DeriveNonFungibleGlobalIdFromPublicKeyHandler
{
    fn description() -> String {
        r#"Derives the non-fungible global id of the virtual badge associated with a given public key"#
            .to_owned()
    }

    fn example_request() -> DeriveNonFungibleGlobalIdFromPublicKeyRequest {
        DeriveNonFungibleGlobalIdFromPublicKeyRequest {
            network_id: 0xf2,
            public_key: notary_private_key().public_key().into(),
        }
    }
}

impl
    ExampleData<
        DeriveBabylonAddressFromOlympiaAddressRequest,
        DeriveBabylonAddressFromOlympiaAddressResponse,
    > for DeriveBabylonAddressFromOlympiaAddressHandler
{
    fn description() -> String {
        r#"Derives the Babylon account address associated with the given Olympia account address"#
            .to_owned()
    }

    fn example_request() -> DeriveBabylonAddressFromOlympiaAddressRequest {
        DeriveBabylonAddressFromOlympiaAddressRequest {
            network_id: 0x01,
            olympia_account_address:
                "rdx1qspx7zxmnrh36q33av24srdfzg7m3cj65968erpjuh7ja3rm3kmn6hq4j9842".to_owned(),
        }
    }
}

impl ExampleData<KnownEntityAddressesRequest, KnownEntityAddressesResponse>
    for KnownEntityAddressesHandler
{
    fn description() -> String {
        r#"Given a network id, this function derives the Bech32m-encoded addresses of the set of known addresses.
        
        As an example, this function allows users to derive the XRD resource address, faucet component address, or account package address on any network (given that they know its network id)."#
            .to_owned()
    }

    fn example_request() -> KnownEntityAddressesRequest {
        KnownEntityAddressesRequest { network_id: 0x01 }
    }
}

impl ExampleData<StaticallyValidateTransactionRequest, StaticallyValidateTransactionResponse>
    for StaticallyValidateTransactionHandler
{
    fn description() -> String {
        r#"Performs static validation on the given notarized transaction."#.to_owned()
    }

    fn example_request() -> StaticallyValidateTransactionRequest {
        // Making the notarized transaction invalid
        let notarized_transaction = {
            let mut transaction = notarized_intent();
            transaction.notary_signature =
                transaction.signed_intent.intent_signatures[0].signature();
            transaction
        };

        let compiled_transaction_intent = manifest_encode(&notarized_transaction).unwrap();
        let validation_config = native_transaction::validation::ValidationConfig::default(0xf2);
        StaticallyValidateTransactionRequest {
            compiled_notarized_intent: compiled_transaction_intent,
            validation_config,
        }
    }
}

impl ExampleData<HashRequest, HashResponse> for HashHandler {
    fn description() -> String {
        r#"Hashes some payload through the hashing algorithm used in Scrypto and the Radix Engine."#
            .to_owned()
    }

    fn example_request() -> HashRequest {
        // Making the notarized transaction invalid
        let notarized_transaction = {
            let mut transaction = notarized_intent();
            transaction.notary_signature =
                transaction.signed_intent.intent_signatures[0].signature();
            transaction
        };

        let compiled_transaction_intent = manifest_encode(&notarized_transaction).unwrap();
        HashRequest {
            payload: compiled_transaction_intent,
        }
    }
}
