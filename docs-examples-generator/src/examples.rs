use radix_engine_toolkit_core::{
    model::{Bech32Coder, Value, ValueKind, NetworkAwareResourceAddress},
    requests::*,
    traits::{Request, TryIntoWithContext, Validate},
};
use transaction::builder::TransactionBuilder;
use transaction::manifest::compile;
use transaction::model::{NotarizedTransaction, TransactionHeader};
use transaction::signing::{EcdsaSecp256k1PrivateKey, EddsaEd25519PrivateKey};

use scrypto::{prelude::*, radix_engine_interface::core::NetworkDefinition};
use serde::Serialize;

lazy_static::lazy_static! {
    pub static ref NETWORK_DEFINITION: NetworkDefinition = NetworkDefinition::simulator();

    pub static ref NOTARY_PRIVATE_KEY: EcdsaSecp256k1PrivateKey = EcdsaSecp256k1PrivateKey::from_u64(1923112).unwrap();

    pub static ref NOTARIZED_TRANSACTION: NotarizedTransaction = TransactionBuilder::new()
        .manifest(
            compile(
                include_str!("../../radix-engine-toolkit-core/tests/test_vector/manifest/complex.rtm"),
                &NETWORK_DEFINITION,
                vec![
                    include_bytes!("../../radix-engine-toolkit-core/tests/test_vector/manifest/complex.code").to_vec(),
                    include_bytes!("../../radix-engine-toolkit-core/tests/test_vector/manifest/complex.abi").to_vec(),
                ]
            ).unwrap()
        )
        .header(TransactionHeader {
            version: 0x01,
            network_id: NETWORK_DEFINITION.id,
            cost_unit_limit: 100_000_000,
            start_epoch_inclusive: 0x200,
            end_epoch_exclusive: 0x210,
            nonce: 0x22,
            notary_as_signatory: true,
            notary_public_key: NOTARY_PRIVATE_KEY.public_key().into(),
            tip_percentage: 0x00
        })
        .sign(&EcdsaSecp256k1PrivateKey::from_u64(1).unwrap())
        .sign(&EcdsaSecp256k1PrivateKey::from_u64(2).unwrap())
        .sign(&EcdsaSecp256k1PrivateKey::from_u64(3).unwrap())
        .sign(&EddsaEd25519PrivateKey::from_u64(1).unwrap())
        .sign(&EddsaEd25519PrivateKey::from_u64(2).unwrap())
        .sign(&EddsaEd25519PrivateKey::from_u64(3).unwrap())
        .notarize(&*NOTARY_PRIVATE_KEY)
        .build();

    pub static ref VALUE: Value = Value::Tuple {
        elements: vec![
            Value::Decimal { value: dec!("10") },
            Value::PreciseDecimal { value: pdec!("10") },
            Value::String {
                value: "Hello World!".into(),
            },
            Value::Tuple {
                elements: vec![
                    Value::Decimal { value: dec!("10") },
                    Value::PreciseDecimal { value: pdec!("10") },
                    Value::String {
                        value: "Hello World!".into(),
                    },
                    Value::Tuple {
                        elements: vec![
                            Value::Decimal { value: dec!("10") },
                            Value::PreciseDecimal { value: pdec!("10") },
                            Value::String {
                                value: "Hello World!".into(),
                            },
                            Value::Tuple {
                                elements: vec![
                                    Value::Decimal { value: dec!("10") },
                                    Value::PreciseDecimal { value: pdec!("10") },
                                    Value::String {
                                        value: "Hello World!".into(),
                                    },
                                    Value::Array {
                                        element_type: ValueKind::Decimal,
                                        elements: vec![
                                            Value::Decimal { value: dec!("20") },
                                            Value::Decimal { value: dec!("100") },
                                            Value::Decimal {
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
    };
}

pub struct Example {
    pub request_type_name: String,
    pub response_type_name: String,
    pub request_description: String,
    pub request: String,
    pub response: String,
}

pub trait RequestExample<'a, Response>
where
    Self: Sized + Request<'a, Response> + Serialize,
    Response: Serialize + Validate,
{
    fn description() -> String;
    fn example_request() -> Self;

    fn request_type_name() -> String {
        std::any::type_name::<Self>()
            .split("::")
            .last()
            .unwrap()
            .to_owned()
    }
    fn response_type_name() -> String {
        std::any::type_name::<Response>()
            .split("::")
            .last()
            .unwrap()
            .to_owned()
    }

    fn to_example() -> Example {
        let request = Self::example_request();
        let response = Self::example_request().fulfill_request().unwrap();
        Example {
            request_type_name: Self::request_type_name(),
            response_type_name: Self::response_type_name(),
            request_description: Self::description(),
            request: serde_json::to_string_pretty(&request).unwrap(),
            response: serde_json::to_string_pretty(&response).unwrap(),
        }
    }
}

impl<'a, R> RequestExample<'a, R> for InformationRequest
where
    InformationRequest: Request<'a, R>,
    R: Serialize + Validate,
{
    fn description() -> String {
        r#"The function provides information information on the currently in-use radix engine toolkit such as the version of the radix engine toolkit. In most cases, this is the first function written when integrating new clients; so, this function is often times seen as the "Hello World" example of the radix engine toolkit."#.to_owned()
    }

    fn example_request() -> Self {
        Self {}
    }
}

impl<'a, R> RequestExample<'a, R> for ConvertManifestRequest
where
    ConvertManifestRequest: Request<'a, R>,
    R: Serialize + Validate,
{
    fn description() -> String {
        r#"Clients have a need to be able to read, parse, understand, and interrogate transaction manifests to get more information on what a transactions might be doing. Transaction manifests have so far existed in one format: as strings. While the string format is very human readable, it is not easily readable by machines as a lexer and parser are needed to make sense of them; thus, it is for clients to programmatically make sense of transactions. As such, there is a need for another transaction manifest format (to supplement, NOT replace) which machines can easily make sense of without the need to implement a lexer and parser.

Therefore, this library introduces a JSON format for transaction manifests which clients can use when wanting to read and interrogate their transaction manifests in code. The transaction manifest JSON format has a 1:1 mapping to the string format of transaction manifests, meaning that anything which can be done in the string format of transaction manifests, can be done in the JSON format as well.

This function allows the client the convert their manifest between the two supported manifest types: string and JSON."#.to_owned()
    }

    fn example_request() -> Self {
        let bec32_coder = Bech32Coder::new(NETWORK_DEFINITION.id);
        Self {
            manifest: NOTARIZED_TRANSACTION
                .signed_intent
                .intent
                .manifest
                .clone()
                .try_into_with_context((
                    radix_engine_toolkit_core::model::ManifestInstructionsKind::String,
                    bec32_coder,
                ))
                .unwrap(),
            manifest_instructions_output_format:
                radix_engine_toolkit_core::model::ManifestInstructionsKind::JSON,
            network_id: NETWORK_DEFINITION.id,
            transaction_version: 0x01,
        }
    }
}

impl<'a, R> RequestExample<'a, R> for CompileTransactionIntentRequest
where
    CompileTransactionIntentRequest: Request<'a, R>,
    R: Serialize + Validate,
{
    fn description() -> String {
        r#"Takes a transaction intent and compiles it by SBOR encoding it and returning it back to the caller. This is mainly useful when creating a transaction."#.to_owned()
    }

    fn example_request() -> Self {
        Self { transaction_intent: NOTARIZED_TRANSACTION.signed_intent.intent.clone().try_into_with_context(radix_engine_toolkit_core::model::ManifestInstructionsKind::String).unwrap() }
    }
}

impl<'a, R> RequestExample<'a, R> for DecompileTransactionIntentRequest
where
    DecompileTransactionIntentRequest: Request<'a, R>,
    R: Serialize + Validate,
{
    fn description() -> String {
        r#"This function does the opposite of the compile_transaction_intent function. It takes in a compiled transaction intent and decompiles it into its human-readable / machine-readable format."#.to_owned()
    }

    fn example_request() -> Self {
        let compiled_transaction_intent = scrypto_encode(&NOTARIZED_TRANSACTION.signed_intent.intent).unwrap();
        Self {
            compiled_intent: compiled_transaction_intent,
            manifest_instructions_output_format: radix_engine_toolkit_core::model::ManifestInstructionsKind::JSON
        }
    }
}

impl<'a, R> RequestExample<'a, R> for CompileSignedTransactionIntentRequest
where
    CompileSignedTransactionIntentRequest: Request<'a, R>,
    R: Serialize + Validate,
{
    fn description() -> String {
        r#"This function takes in a raw transaction intent as well as its signatures and compiles it. This is useful when a notary wishes to notarize a signed transaction intent."#.to_owned()
    }

    fn example_request() -> Self {
        Self { signed_intent: NOTARIZED_TRANSACTION.signed_intent.clone().try_into_with_context(radix_engine_toolkit_core::model::ManifestInstructionsKind::String).unwrap() }
    }
}

impl<'a, R> RequestExample<'a, R> for DecompileSignedTransactionIntentRequest
where
    DecompileSignedTransactionIntentRequest: Request<'a, R>,
    R: Serialize + Validate,
{
    fn description() -> String {
        r#"This function does the opposite of the compile_signed_transaction_intent function. This function takes in a compiled signed transaction intent and decompiles it into its transaction intent and signatures."#.to_owned()
    }

    fn example_request() -> Self {
        let compiled_transaction_intent = scrypto_encode(&NOTARIZED_TRANSACTION.signed_intent).unwrap();
        Self {
            compiled_signed_intent: compiled_transaction_intent,
            manifest_instructions_output_format: radix_engine_toolkit_core::model::ManifestInstructionsKind::JSON
        }
    }
}

impl<'a, R> RequestExample<'a, R> for CompileNotarizedTransactionIntentRequest
where
    CompileNotarizedTransactionIntentRequest: Request<'a, R>,
    R: Serialize + Validate,
{
    fn description() -> String {
        r#"This function takes in a raw signed transaction intent as well as the notary signature and compiles it. This is useful when we wish to submit a transaction to the Gateway API"#.to_owned()
    }

    fn example_request() -> Self {
        Self { notarized_transaction: NOTARIZED_TRANSACTION.clone().try_into_with_context(radix_engine_toolkit_core::model::ManifestInstructionsKind::String).unwrap() }
    }
}

impl<'a, R> RequestExample<'a, R> for DecompileNotarizedTransactionIntentRequest
where
    DecompileNotarizedTransactionIntentRequest: Request<'a, R>,
    R: Serialize + Validate,
{
    fn description() -> String {
        r#"This function does the opposite of the compile_notarized_transaction_intent function. This function takes in a compiled notarized transaction intent and decompiles it into its signed transaction intent and notary signature."#.to_owned()
    }

    fn example_request() -> Self {
        let compiled_transaction_intent = scrypto_encode(&*NOTARIZED_TRANSACTION).unwrap();
        Self {
            compiled_notarized_intent: compiled_transaction_intent,
            manifest_instructions_output_format: radix_engine_toolkit_core::model::ManifestInstructionsKind::JSON
        }
    }
}

impl<'a, R> RequestExample<'a, R> for DecompileUnknownTransactionIntentRequest
where
    DecompileUnknownTransactionIntentRequest: Request<'a, R>,
    R: Serialize + Validate,
{
    fn description() -> String {
        r#"There are certain cases where we might have some blob which we suspect is a transaction intent but we have no way of verifying whether that is true or not. Looking at the type id byte of the blob does not help either as it's a generic Struct type which is not too telling. For this specific use case, this library provides this function which attempts to decompile a transaction intent of an unknown type."#.to_owned()
    }

    fn example_request() -> Self {
        let compiled_transaction_intent = scrypto_encode(&*NOTARIZED_TRANSACTION).unwrap();
        Self {
            compiled_unknown_intent: compiled_transaction_intent,
            manifest_instructions_output_format: radix_engine_toolkit_core::model::ManifestInstructionsKind::JSON
        }
    }
}

impl<'a, R> RequestExample<'a, R> for EncodeAddressRequest
where
    EncodeAddressRequest: Request<'a, R>,
    R: Serialize + Validate,
{
    fn description() -> String {
        r#"This function can be used when we have a byte array which we wish to do Bech32m encoding on. In this case, the HRP to use will be determined through the entity byte of the passed address hex string."#.to_owned()
    }

    fn example_request() -> Self {
        Self { 
            address_bytes: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2],
            network_id: 0xf2
        }
    }
}

impl<'a, R> RequestExample<'a, R> for DecodeAddressRequest
where
    DecodeAddressRequest: Request<'a, R>,
    R: Serialize + Validate,
{
    fn description() -> String {
        r#"This function can be used to decode a Bech32m encoded address string into its equivalent hrp and data. In addition to that, this function provides other useful information on the address such as the network id and name that it is used for, and the entity type of the address."#.to_owned()
    }

    fn example_request() -> Self {
        Self { 
            address: "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqpqd60rqz".into()
        }
    }
}

impl<'a, R> RequestExample<'a, R> for SBOREncodeRequest
where
    SBOREncodeRequest: Request<'a, R>,
    R: Serialize + Validate,
{
    fn description() -> String {
        r#"This function takes in a Value and encodes it in SBOR."#.to_owned()
    }

    fn example_request() -> Self {
        Self { 
            value: (*VALUE).clone()
        }
    }
}

impl<'a, R> RequestExample<'a, R> for SBORDecodeRequest
where
    SBORDecodeRequest: Request<'a, R>,
    R: Serialize + Validate,
{
    fn description() -> String {
        r#"This function takes in a hex string and attemps to decode it into a Value."#.to_owned()
    }

    fn example_request() -> Self {
        Self {
            encoded_value: (*VALUE).encode().unwrap(),
            network_id: 0xf2
        }
    }
}

impl<'a, R> RequestExample<'a, R> for DeriveNonFungibleAddressFromPublicKeyRequest
where
    DeriveNonFungibleAddressFromPublicKeyRequest: Request<'a, R>,
    R: Serialize + Validate,
{
    fn description() -> String {
        r#"This function derives the `NonFungibleAddress` of the virtual badge associated with a given public key."#.to_owned()
    }

    fn example_request() -> Self {
        Self { 
            public_key: NOTARY_PRIVATE_KEY.public_key().into(), 
            network_id: 0xf2 
        }
    }
}

impl<'a, R> RequestExample<'a, R> for DeriveNonFungibleAddressRequest
where
    DeriveNonFungibleAddressRequest: Request<'a, R>,
    R: Serialize + Validate,
{
    fn description() -> String {
        r#"Given a ResourceAddress and a NonFungibleId, this function is able to derive the NonFungibleAddress corresponding to a specific NonFungible."#.to_owned()
    }

    fn example_request() -> Self {
        Self { 
            resource_address: NetworkAwareResourceAddress {
                address: RADIX_TOKEN,
                network_id: 0xf2
            }, 
            non_fungible_id: NonFungibleId::U64(192) 
        }
    }
}

impl<'a, R> RequestExample<'a, R> for DeriveVirtualAccountAddressRequest
where
    DeriveVirtualAccountAddressRequest: Request<'a, R>,
    R: Serialize + Validate,
{
    fn description() -> String {
        r#"Derives the virtual account component address given a public key and a network id."#.to_owned()
    }

    fn example_request() -> Self {
        Self { network_id: 0xf2, public_key: NOTARY_PRIVATE_KEY.public_key().into() }
    }
}
