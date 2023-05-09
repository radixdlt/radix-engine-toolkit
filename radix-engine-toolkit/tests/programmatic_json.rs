use native_transaction::ecdsa_secp256k1::EcdsaSecp256k1PrivateKey;
use paste::paste;
use radix_engine_toolkit::model::address::Bech32Coder;
use sbor::representations::SerializationParameters;
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
                from_scrypto_sbor_value,
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
                    custom_context: ScryptoValueDisplayContext::with_optional_bech32(Some(
                        coder.encoder(),
                    )),
                });
                serde_json::to_value(&serializable).unwrap()
            })
        }
    };
}

#[derive(ScryptoSbor)]
enum SimpleEnum {
    Variant1 { field: u8 },
    Variant2(u8),
    Variant3,
}

#[derive(ScryptoSbor)]
struct SimpleStruct1;
#[derive(ScryptoSbor)]
struct SimpleStruct2(u8);
#[derive(ScryptoSbor)]
struct SimpleStruct3 {
    field: u8,
}

type U8FiveElementsArray = [u8; 5];
type U16FiveElementsArray = [u16; 5];
type MapStringU8 = BTreeMap<String, u8>;

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
test_schemaless_serialization!(Scrypto, NonFungibleLocalId, NonFungibleLocalId::Integer(1.into()));
test_schemaless_serialization!(Scrypto, NonFungibleGlobalId, NonFungibleGlobalId::from_public_key(&EcdsaSecp256k1PrivateKey::from_u64(1).unwrap().public_key()));
test_schemaless_serialization!(Scrypto, Reference, Reference(*FAUCET_COMPONENT.as_node_id()));