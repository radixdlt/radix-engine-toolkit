use radix_engine_common::{
    prelude::{scrypto_encode, Bech32Encoder, ScryptoCustomSchema, ScryptoValue},
    ScryptoSbor,
};
use sbor::{generate_full_schema_from_single_type, representations::SerializationMode};

#[test]
fn scrypto_value_can_be_encoded() {
    // Arrange
    let value = ScryptoValue::Bool { value: false };

    // Act
    let result = radix_engine_toolkit::functions::scrypto_sbor::encode(&value);

    // Assert
    assert!(result.is_ok())
}

#[test]
fn scrypto_value_can_be_encoded_and_decoded_later() {
    // Arrange
    let value = ScryptoValue::Bool { value: false };
    let encoded = radix_engine_toolkit::functions::scrypto_sbor::encode(&value).unwrap();

    // Act
    let decoded = radix_engine_toolkit::functions::scrypto_sbor::decode(encoded);

    // Assert
    assert!(decoded.is_ok());
    assert_eq!(decoded, Ok(value));
}

#[test]
fn scrypto_value_can_be_represented_as_a_string() {
    // Arrange
    let value = MyStruct { value: true };
    let encoded_value = scrypto_encode(&value).unwrap();

    let (local_type_index, schema) =
        generate_full_schema_from_single_type::<MyStruct, ScryptoCustomSchema>();

    let serialization_modes_params = [
        SerializationMode::Model,
        SerializationMode::Natural,
        SerializationMode::Programmatic,
    ];
    let schema_params = [None, Some((local_type_index, schema))];
    let bech32_encoder = Bech32Encoder::for_simulator();

    for representation in serialization_modes_params {
        for schema in schema_params.clone() {
            // Act
            let result =
                radix_engine_toolkit::functions::scrypto_sbor::decode_to_string_representation(
                    encoded_value.clone(),
                    representation,
                    &bech32_encoder,
                    schema,
                );

            // Assert
            assert!(result.is_ok())
        }
    }
}

#[derive(ScryptoSbor)]
struct MyStruct {
    value: bool,
}
