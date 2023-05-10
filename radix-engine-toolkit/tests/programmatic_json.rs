use native_transaction::ecdsa_secp256k1::EcdsaSecp256k1PrivateKey;
use paste::paste;
use radix_engine_common::ManifestSbor;
use radix_engine_toolkit::model::address::Bech32Coder;
use sbor::{generate_full_schema_from_single_type, representations::SerializationParameters};
use scrypto::prelude::*;
use scrypto_utils::ContextualSerialize;

macro_rules! test_schemaless_serialization {
    (Scrypto, $type: ty, $new: expr, $number: expr) => {
        paste! {
            test_schemaless_serialization! {
                [< test_schemaless_serialization_of_scrypto_ $type:snake _ $number >],
                radix_engine_toolkit::model::value::scrypto_sbor::ScryptoSborValue,
                scrypto::prelude::ScryptoRawPayload,
                scrypto::prelude::scrypto_encode,
                scrypto::prelude::scrypto_decode,
                ScryptoValueDisplayContext,
                from_scrypto_sbor_value,
                $type,
                $new
            }
        }
    };
    (Scrypto, $type: ty, $new: expr) => {
        paste! {
            test_schemaless_serialization! {
                [< test_schemaless_serialization_of_scrypto_ $type:snake >],
                radix_engine_toolkit::model::value::scrypto_sbor::ScryptoSborValue,
                scrypto::prelude::ScryptoRawPayload,
                scrypto::prelude::scrypto_encode,
                scrypto::prelude::scrypto_decode,
                ScryptoValueDisplayContext,
                from_scrypto_sbor_value,
                $type,
                $new
            }
        }
    };
    (Manifest, $type: ty, $new: expr, $number: expr) => {
        paste! {
            test_schemaless_serialization! {
                [< test_schemaless_serialization_of_manifest_ $type:snake _ $number >],
                radix_engine_toolkit::model::value::manifest_sbor::ManifestSborValue,
                scrypto::prelude::ManifestRawPayload,
                scrypto::prelude::manifest_encode,
                scrypto::prelude::manifest_decode,
                ManifestValueDisplayContext,
                from_manifest_sbor_value,
                $type,
                $new
            }
        }
    };
    (Manifest, $type: ty, $new: expr) => {
        paste! {
            test_schemaless_serialization! {
                [< test_schemaless_serialization_of_manifest_ $type:snake >],
                radix_engine_toolkit::model::value::manifest_sbor::ManifestSborValue,
                scrypto::prelude::ManifestRawPayload,
                scrypto::prelude::manifest_encode,
                scrypto::prelude::manifest_decode,
                ManifestValueDisplayContext,
                from_manifest_sbor_value,
                $type,
                $new
            }
        }
    };
    (
        $function_ident: ident,
        $value: ty,
        $raw_payload_type: ty,
        $encode: path,
        $decode: path,
        $context: path,
        $from_fn: ident,
        $type: ty,
        $new: expr
    ) => {
        #[test]
        fn $function_ident() {
            // Arrange
            let value: $type = $new;
            let value: $value =
                <$value>::$from_fn(&$decode(&$encode(&value).unwrap()).unwrap(), 0xf2).unwrap();

            // Act
            let serialized_value = serde_json::to_value(&value).unwrap();

            // Assert
            assert_eq!(serialized_value, {
                let coder = Bech32Coder::new(0xf2);
                let payload = <$raw_payload_type>::new_from_valid_owned(
                    $encode(&{
                        let value: $type = $new;
                        value
                    })
                    .unwrap(),
                );
                let serializable = payload.serializable(SerializationParameters::Schemaless {
                    mode: sbor::representations::SerializationMode::Programmatic,
                    custom_context: <$context>::with_optional_bech32(Some(coder.encoder())),
                });
                serde_json::to_value(&serializable).unwrap()
            })
        }
    };
}

macro_rules! test_schema_serialization {
    (Scrypto, $type: ty, $new: expr, $number: expr) => {
        paste! {
            test_schema_serialization! {
                [< test_schema_serialization_of_scrypto_ $type:snake _ $number >],
                radix_engine_toolkit::model::value::scrypto_sbor::ScryptoSborValue,
                scrypto::prelude::ScryptoRawPayload,
                scrypto::prelude::scrypto_encode,
                scrypto::prelude::scrypto_decode,
                [scrypto::prelude::ScryptoSbor],
                ScryptoValueDisplayContext,
                from_scrypto_sbor_value,
                $type,
                $new
            }
        }
    };
    (Scrypto, $type: ty, $new: expr) => {
        paste! {
            test_schema_serialization! {
                [< test_schema_serialization_of_scrypto_ $type:snake >],
                radix_engine_toolkit::model::value::scrypto_sbor::ScryptoSborValue,
                scrypto::prelude::ScryptoRawPayload,
                scrypto::prelude::scrypto_encode,
                scrypto::prelude::scrypto_decode,
                [scrypto::prelude::ScryptoSbor],
                ScryptoValueDisplayContext,
                from_scrypto_sbor_value,
                $type,
                $new
            }
        }
    };
    (Manifest, $type: ty, $new: expr, $number: expr) => {
        paste! {
            test_schema_serialization! {
                [< test_schema_serialization_of_manifest_ $type:snake _ $number >],
                radix_engine_toolkit::model::value::manifest_sbor::ManifestSborValue,
                scrypto::prelude::ManifestRawPayload,
                scrypto::prelude::manifest_encode,
                scrypto::prelude::manifest_decode,
                [ManifestSbor, radix_engine_derive::ScryptoDescribe],
                ManifestValueDisplayContext,
                from_manifest_sbor_value,
                $type,
                $new
            }
        }
    };
    (Manifest, $type: ty, $new: expr) => {
        paste! {
            test_schema_serialization! {
                [< test_schema_serialization_of_manifest_ $type:snake >],
                radix_engine_toolkit::model::value::manifest_sbor::ManifestSborValue,
                scrypto::prelude::ManifestRawPayload,
                scrypto::prelude::manifest_encode,
                scrypto::prelude::manifest_decode,
                [ManifestSbor, radix_engine_derive::ScryptoDescribe],
                ManifestValueDisplayContext,
                from_manifest_sbor_value,
                $type,
                $new
            }
        }
    };
    (
        $function_ident: ident,
        $value: ty,
        $raw_payload_type: ty,
        $encode: path,
        $decode: path,
        [$($derive: path),*],
        $context: path,
        $from_fn: ident,
        $type: ty,
        $new: expr
    ) => {
        paste! {
            #[test]
            fn [< $function_ident _with_named_fields_struct >] () {
                // Arrange
                use radix_engine_toolkit::functions::traits::*;

                #[derive($($derive,)*)]
                struct Wrapper {
                    item: $type,
                }
                let value = Wrapper { item: $new };
                let (local_type_index, schema) =
                    generate_full_schema_from_single_type::<Wrapper, ScryptoCustomSchema>();

                // Act
                let serialized = {
                    let input = radix_engine_toolkit::functions::sbor_decode::Input {
                        encoded_value: $encode(&value).unwrap(),
                        network_id: 0xf2,
                        schema: Some((local_type_index, schema.clone()).into()),
                    };
                    let output =
                        radix_engine_toolkit::functions::sbor_decode::Handler::fulfill(input).unwrap();
                    output
                };
                let serialized_value = serde_json::to_value(&serialized)
                    .unwrap()
                    .get("value")
                    .unwrap()
                    .clone();

                // Assert
                assert_eq!(serialized_value, {
                    let coder = Bech32Coder::new(0xf2);
                    let payload = <$raw_payload_type>::new_from_valid_owned($encode(&value).unwrap());
                    let serializable = payload.serializable(SerializationParameters::WithSchema {
                        mode: sbor::representations::SerializationMode::Programmatic,
                        custom_context: <$context>::with_optional_bech32(Some(coder.encoder())),
                        schema: &schema,
                        type_index: local_type_index,
                    });
                    serde_json::to_value(&serializable).unwrap()
                })
            }

            #[test]
            fn [< $function_ident _with_unnamed_fields_struct >] () {
                // Arrange
                use radix_engine_toolkit::functions::traits::*;

                #[derive($($derive,)*)]
                struct Wrapper($type);
                let value = Wrapper($new);
                let (local_type_index, schema) =
                    generate_full_schema_from_single_type::<Wrapper, ScryptoCustomSchema>();

                // Act
                let serialized = {
                    let input = radix_engine_toolkit::functions::sbor_decode::Input {
                        encoded_value: $encode(&value).unwrap(),
                        network_id: 0xf2,
                        schema: Some((local_type_index, schema.clone()).into()),
                    };
                    let output =
                        radix_engine_toolkit::functions::sbor_decode::Handler::fulfill(input).unwrap();
                    output
                };
                let serialized_value = serde_json::to_value(&serialized)
                    .unwrap()
                    .get("value")
                    .unwrap()
                    .clone();

                // Assert
                assert_eq!(serialized_value, {
                    let coder = Bech32Coder::new(0xf2);
                    let payload = <$raw_payload_type>::new_from_valid_owned($encode(&value).unwrap());
                    let serializable = payload.serializable(SerializationParameters::WithSchema {
                        mode: sbor::representations::SerializationMode::Programmatic,
                        custom_context: <$context>::with_optional_bech32(Some(coder.encoder())),
                        schema: &schema,
                        type_index: local_type_index,
                    });
                    serde_json::to_value(&serializable).unwrap()
                })
            }
        }
    };
}

#[derive(ScryptoSbor, ManifestSbor)]
enum SimpleEnum {
    Variant1 { field: u8 },
    Variant2(u8),
    Variant3,
}

#[derive(ScryptoSbor, ManifestSbor)]
struct SimpleStruct1;
#[derive(ScryptoSbor, ManifestSbor)]
struct SimpleStruct2(u8);
#[derive(ScryptoSbor, ManifestSbor)]
struct SimpleStruct3 {
    field: u8,
}

#[derive(ScryptoSbor, ManifestSbor)]
#[sbor(transparent)]
struct MySimpleWrapper(u8);

type U8FiveElementsArray = [u8; 5];
type U16FiveElementsArray = [u16; 5];
type MapStringU8 = BTreeMap<String, u8>;

mod scrypto_schemaless {
    use super::*;

    test_schemaless_serialization!(Scrypto, bool, true);
    test_schemaless_serialization!(Scrypto, u8, 1);
    test_schemaless_serialization!(Scrypto, u16, 1);
    test_schemaless_serialization!(Scrypto, u32, 1);
    test_schemaless_serialization!(Scrypto, u64, 1);
    test_schemaless_serialization!(Scrypto, u128, 1);
    test_schemaless_serialization!(Scrypto, i8, 1);
    test_schemaless_serialization!(Scrypto, i16, 1);
    test_schemaless_serialization!(Scrypto, i32, 1);
    test_schemaless_serialization!(Scrypto, i64, 1);
    test_schemaless_serialization!(Scrypto, i128, 1);
    test_schemaless_serialization!(Scrypto, SimpleEnum, SimpleEnum::Variant1 { field: 1 }, 1);
    test_schemaless_serialization!(Scrypto, SimpleEnum, SimpleEnum::Variant2(1), 2);
    test_schemaless_serialization!(Scrypto, SimpleEnum, SimpleEnum::Variant3, 3);
    test_schemaless_serialization!(Scrypto, SimpleStruct1, SimpleStruct1);
    test_schemaless_serialization!(Scrypto, SimpleStruct2, SimpleStruct2(1));
    test_schemaless_serialization!(Scrypto, SimpleStruct3, SimpleStruct3 { field: 1 });
    test_schemaless_serialization!(Scrypto, U8FiveElementsArray, [1, 2, 3, 4, 5]);
    test_schemaless_serialization!(Scrypto, U16FiveElementsArray, [1, 2, 3, 4, 5]);
    test_schemaless_serialization!(Scrypto, MapStringU8, {
        let mut map = BTreeMap::new();
        map.insert("x".to_owned(), 1u8);
        map.insert("y".to_owned(), 2u8);
        map
    });
    test_schemaless_serialization!(Scrypto, Own, Own(*FAUCET_COMPONENT.as_node_id()));
    test_schemaless_serialization!(Scrypto, Decimal, dec!("1"));
    test_schemaless_serialization!(Scrypto, PreciseDecimal, pdec!("1"));
    test_schemaless_serialization!(
        Scrypto,
        NonFungibleLocalId,
        NonFungibleLocalId::Integer(1.into())
    );
    test_schemaless_serialization!(
        Scrypto,
        NonFungibleGlobalId,
        NonFungibleGlobalId::from_public_key(
            &EcdsaSecp256k1PrivateKey::from_u64(1).unwrap().public_key()
        )
    );
    test_schemaless_serialization!(
        Scrypto,
        Reference,
        Reference(*FAUCET_COMPONENT.as_node_id())
    );
}

mod manifest_schemaless {
    use super::*;

    test_schemaless_serialization!(Manifest, bool, true);
    test_schemaless_serialization!(Manifest, u8, 1);
    test_schemaless_serialization!(Manifest, u16, 1);
    test_schemaless_serialization!(Manifest, u32, 1);
    test_schemaless_serialization!(Manifest, u64, 1);
    test_schemaless_serialization!(Manifest, u128, 1);
    test_schemaless_serialization!(Manifest, i8, 1);
    test_schemaless_serialization!(Manifest, i16, 1);
    test_schemaless_serialization!(Manifest, i32, 1);
    test_schemaless_serialization!(Manifest, i64, 1);
    test_schemaless_serialization!(Manifest, i128, 1);
    test_schemaless_serialization!(Manifest, SimpleEnum, SimpleEnum::Variant1 { field: 1 }, 1);
    test_schemaless_serialization!(Manifest, SimpleEnum, SimpleEnum::Variant2(1), 2);
    test_schemaless_serialization!(Manifest, SimpleEnum, SimpleEnum::Variant3, 3);
    test_schemaless_serialization!(Manifest, SimpleStruct1, SimpleStruct1);
    test_schemaless_serialization!(Manifest, SimpleStruct2, SimpleStruct2(1));
    test_schemaless_serialization!(Manifest, SimpleStruct3, SimpleStruct3 { field: 1 });
    test_schemaless_serialization!(Manifest, U8FiveElementsArray, [1, 2, 3, 4, 5]);
    test_schemaless_serialization!(Manifest, U16FiveElementsArray, [1, 2, 3, 4, 5]);
    test_schemaless_serialization!(Manifest, MapStringU8, {
        let mut map = BTreeMap::new();
        map.insert("x".to_owned(), 1u8);
        map.insert("y".to_owned(), 2u8);
        map
    });
    test_schemaless_serialization!(Manifest, ComponentAddress, FAUCET_COMPONENT);
    test_schemaless_serialization!(Manifest, ManifestBucket, ManifestBucket(1));
    test_schemaless_serialization!(Manifest, ManifestProof, ManifestProof(1));
    test_schemaless_serialization!(Manifest, Decimal, dec!("1"));
    test_schemaless_serialization!(Manifest, PreciseDecimal, pdec!("1"));
    test_schemaless_serialization!(
        Manifest,
        NonFungibleLocalId,
        NonFungibleLocalId::Integer(1.into())
    );
    test_schemaless_serialization!(
        Manifest,
        ManifestExpression,
        ManifestExpression::EntireAuthZone
    );
    test_schemaless_serialization!(Manifest, ManifestBlobRef, ManifestBlobRef([0; 32]));
}

mod scrypto_with_schema {
    use super::*;

    test_schema_serialization!(Scrypto, bool, true);
    test_schema_serialization!(Scrypto, u8, 1);
    test_schema_serialization!(Scrypto, u16, 1);
    test_schema_serialization!(Scrypto, u32, 1);
    test_schema_serialization!(Scrypto, u64, 1);
    test_schema_serialization!(Scrypto, u128, 1);
    test_schema_serialization!(Scrypto, i8, 1);
    test_schema_serialization!(Scrypto, i16, 1);
    test_schema_serialization!(Scrypto, i32, 1);
    test_schema_serialization!(Scrypto, i64, 1);
    test_schema_serialization!(Scrypto, i128, 1);
    test_schema_serialization!(Scrypto, SimpleEnum, SimpleEnum::Variant1 { field: 1 }, 1);
    test_schema_serialization!(Scrypto, SimpleEnum, SimpleEnum::Variant2(1), 2);
    test_schema_serialization!(Scrypto, SimpleEnum, SimpleEnum::Variant3, 3);
    test_schema_serialization!(Scrypto, SimpleStruct1, SimpleStruct1);
    test_schema_serialization!(Scrypto, SimpleStruct2, SimpleStruct2(1));
    test_schema_serialization!(Scrypto, SimpleStruct3, SimpleStruct3 { field: 1 });
    // test_schema_serialization!(Scrypto, U8FiveElementsArray, [1, 2, 3, 4, 5]);
    // test_schema_serialization!(Scrypto, U16FiveElementsArray, [1, 2, 3, 4, 5]);
    test_schema_serialization!(Scrypto, MapStringU8, {
        let mut map = BTreeMap::new();
        map.insert("x".to_owned(), 1u8);
        map.insert("y".to_owned(), 2u8);
        map
    });
    test_schema_serialization!(Scrypto, Own, Own(*FAUCET_COMPONENT.as_node_id()));
    test_schema_serialization!(Scrypto, Decimal, dec!("1"));
    test_schema_serialization!(Scrypto, PreciseDecimal, pdec!("1"));
    test_schema_serialization!(
        Scrypto,
        NonFungibleLocalId,
        NonFungibleLocalId::Integer(1.into())
    );
    test_schema_serialization!(
        Scrypto,
        NonFungibleGlobalId,
        NonFungibleGlobalId::from_public_key(
            &EcdsaSecp256k1PrivateKey::from_u64(1).unwrap().public_key()
        )
    );
    test_schema_serialization!(
        Scrypto,
        Reference,
        Reference(*FAUCET_COMPONENT.as_node_id())
    );
    test_schema_serialization!(Scrypto, MySimpleWrapper, MySimpleWrapper(1));
}

// TODO: Something is off about Schemas and Manifest SBOR. Re-enable the tests below when figured
// out.
mod manifest_with_schema {
    use super::*;

    test_schema_serialization!(Manifest, bool, true);
    test_schema_serialization!(Manifest, u8, 1);
    test_schema_serialization!(Manifest, u16, 1);
    test_schema_serialization!(Manifest, u32, 1);
    test_schema_serialization!(Manifest, u64, 1);
    test_schema_serialization!(Manifest, u128, 1);
    test_schema_serialization!(Manifest, i8, 1);
    test_schema_serialization!(Manifest, i16, 1);
    test_schema_serialization!(Manifest, i32, 1);
    test_schema_serialization!(Manifest, i64, 1);
    test_schema_serialization!(Manifest, i128, 1);
    test_schema_serialization!(Manifest, SimpleEnum, SimpleEnum::Variant1 { field: 1 }, 1);
    test_schema_serialization!(Manifest, SimpleEnum, SimpleEnum::Variant2(1), 2);
    test_schema_serialization!(Manifest, SimpleEnum, SimpleEnum::Variant3, 3);
    test_schema_serialization!(Manifest, SimpleStruct1, SimpleStruct1);
    test_schema_serialization!(Manifest, SimpleStruct2, SimpleStruct2(1));
    test_schema_serialization!(Manifest, SimpleStruct3, SimpleStruct3 { field: 1 });
    // test_schema_serialization!(Manifest, U8FiveElementsArray, [1, 2, 3, 4, 5]);
    // test_schema_serialization!(Manifest, U16FiveElementsArray, [1, 2, 3, 4, 5]);
    test_schema_serialization!(Manifest, MapStringU8, {
        let mut map = BTreeMap::new();
        map.insert("x".to_owned(), 1u8);
        map.insert("y".to_owned(), 2u8);
        map
    });
    test_schema_serialization!(Manifest, ComponentAddress, FAUCET_COMPONENT);
    // test_schema_serialization!(Manifest, ManifestBucket, ManifestBucket(1));
    // test_schema_serialization!(Manifest, ManifestProof, ManifestProof(1));
    test_schema_serialization!(Manifest, Decimal, dec!("1"));
    test_schema_serialization!(Manifest, PreciseDecimal, pdec!("1"));
    test_schema_serialization!(
        Manifest,
        NonFungibleLocalId,
        NonFungibleLocalId::Integer(1.into())
    );
    // test_schema_serialization!(
    //     Manifest,
    //     ManifestExpression,
    //     ManifestExpression::EntireAuthZone
    // );
    // test_schema_serialization!(Manifest, ManifestBlobRef, ManifestBlobRef([0; 32]));
}
