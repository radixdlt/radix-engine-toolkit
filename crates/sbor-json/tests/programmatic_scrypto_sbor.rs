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

use radix_common::prelude::*;
use radix_engine_interface::macros::{dec, pdec};
use sbor::generate_full_schema_from_single_type;
use sbor::representations::*;
use sbor_json::common::address::SerializableNodeId;
use sbor_json::scrypto::programmatic::utils::value_contains_network_mismatch;
use sbor_json::scrypto::programmatic::value::{
    ProgrammaticScryptoValue, ProgrammaticScryptoValueKind,
};
use std::fmt::Debug;

serialization_tests! {
    programmatic_json_serialization_of_bool_false: false,
    programmatic_json_serialization_of_bool_true: true,

    programmatic_json_serialization_of_u8: 0u8,
    programmatic_json_serialization_of_u16: 0u16,
    programmatic_json_serialization_of_u32: 0u32,
    programmatic_json_serialization_of_u64: 0u64,
    programmatic_json_serialization_of_u128: 0u128,

    programmatic_json_serialization_of_i8: 0i8,
    programmatic_json_serialization_of_i16: 0i16,
    programmatic_json_serialization_of_i32: 0i32,
    programmatic_json_serialization_of_i64: 0i64,
    programmatic_json_serialization_of_i128: 0i128,

    programmatic_json_serialization_of_string: "Hello World!",

    programmatic_json_serialization_of_enum_unit_variant: MyEnum::UnitVariant,
    programmatic_json_serialization_of_enum_unit_variant_with_no_field_names: MyEnum::VariantWithNoFieldNames(0),
    programmatic_json_serialization_of_enum_unit_variant_with_field_names: MyEnum::VariantWithFieldNames { field: 0 },

    programmatic_json_serialization_of_unit_tuple: (),
    programmatic_json_serialization_of_1_element_tuple: (1u8),
    programmatic_json_serialization_of_2_element_tuple: (1u8, "String"),
    programmatic_json_serialization_of_struct_unit: UnitStruct,
    programmatic_json_serialization_of_struct_no_fields: NoFieldsStruct(),
    programmatic_json_serialization_of_struct_no_field_names: NoFieldNamesStruct(0),
    programmatic_json_serialization_of_struct_with_field_names: NamedFieldsStruct { field: 0 },

    programmatic_json_serialization_of_bytes_vec: vec![0u8, 0u8, 0u8, 0u8],
    programmatic_json_serialization_of_bytes_array: [0u8, 0u8, 0u8, 0u8],
    programmatic_json_serialization_of_vec: vec![0u16, 0u16, 0u16, 0u16],
    programmatic_json_serialization_of_array: [0u16, 0u16, 0u16, 0u16],

    programmatic_json_serialization_of_map: hashmap!(
        "A" => 0u8,
        "B" => 1u8,
    ),
    programmatic_json_serialization_of_reference: Reference(XRD.into_node_id()),
    programmatic_json_serialization_of_own: Own(XRD.into_node_id()),
    programmatic_json_serialization_of_decimal: dec!(100),
    programmatic_json_serialization_of_precise_decimal: pdec!(100),

    programmatic_json_serialization_of_precise_non_fungible_local_id_integer: NonFungibleLocalId::integer(1),
    programmatic_json_serialization_of_precise_non_fungible_local_id_string: NonFungibleLocalId::string("HelloWorld").unwrap(),
    programmatic_json_serialization_of_precise_non_fungible_local_id_bytes: NonFungibleLocalId::bytes([0, 0]).unwrap(),
}

#[test]
pub fn payload_serialized_with_schema_can_be_deserialized_as_no_schema_programmatic_json_model(
) {
    // Arrange
    let value = MyEnum::VariantWithFieldNames { field: 1 };
    let payload = scrypto_encode(&value).unwrap();
    let (local_type_id, schema) =
        generate_full_schema_from_single_type::<MyEnum, ScryptoCustomSchema>();

    let programmatic_json = {
        let encoder = AddressBech32Encoder::for_simulator();
        let serialization_parameters = SerializationParameters::WithSchema {
            mode: SerializationMode::Programmatic,
            custom_context: ScryptoValueDisplayContext::with_optional_bech32(
                Some(&encoder),
            ),
            schema: &schema.v1(),
            type_id: local_type_id,
            depth_limit: SCRYPTO_SBOR_V1_MAX_DEPTH,
        };

        let payload = ScryptoRawPayload::new_from_valid_slice(&payload);
        let serializable = payload.serializable(serialization_parameters);
        serde_json::to_string(&serializable).expect("Impossible Case!")
    };

    // Act
    let deserialized =
        serde_json::from_str::<ProgrammaticScryptoValue>(&programmatic_json)
            .unwrap();

    // Assert
    assert_eq!(
        deserialized,
        ProgrammaticScryptoValue::Enum {
            discriminator: 2,
            fields: vec![ProgrammaticScryptoValue::U8 { value: 1 }]
        }
    )
}

#[test]
pub fn value_with_no_address_has_no_network_mismatch() {
    // Arrange
    let value = ProgrammaticScryptoValue::Array {
        element_value_kind: ProgrammaticScryptoValueKind::Reference,
        elements: vec![],
    };

    // Act
    let contains_network_mismatch = value_contains_network_mismatch(&value);

    // Assert
    assert!(!contains_network_mismatch)
}

#[test]
pub fn value_with_one_address_has_no_network_mismatch() {
    // Arrange
    let value = ProgrammaticScryptoValue::Array {
        element_value_kind: ProgrammaticScryptoValueKind::Reference,
        elements: vec![ProgrammaticScryptoValue::Reference {
            value: SerializableNodeId(XRD.into_node_id(), 1),
        }],
    };

    // Act
    let contains_network_mismatch = value_contains_network_mismatch(&value);

    // Assert
    assert!(!contains_network_mismatch)
}

#[test]
pub fn value_with_two_address_of_the_same_network_has_no_network_mismatch() {
    // Arrange
    let value = ProgrammaticScryptoValue::Array {
        element_value_kind: ProgrammaticScryptoValueKind::Reference,
        elements: vec![
            ProgrammaticScryptoValue::Reference {
                value: SerializableNodeId(XRD.into_node_id(), 1),
            },
            ProgrammaticScryptoValue::Reference {
                value: SerializableNodeId(
                    ACCESS_CONTROLLER_PACKAGE.into_node_id(),
                    1,
                ),
            },
        ],
    };

    // Act
    let contains_network_mismatch = value_contains_network_mismatch(&value);

    // Assert
    assert!(!contains_network_mismatch)
}

#[test]
pub fn value_with_two_address_of_the_differing_networks_has_a_network_mismatch()
{
    // Arrange
    let value = ProgrammaticScryptoValue::Array {
        element_value_kind: ProgrammaticScryptoValueKind::Reference,
        elements: vec![
            ProgrammaticScryptoValue::Reference {
                value: SerializableNodeId(XRD.into_node_id(), 1),
            },
            ProgrammaticScryptoValue::Reference {
                value: SerializableNodeId(
                    ACCESS_CONTROLLER_PACKAGE.into_node_id(),
                    2,
                ),
            },
        ],
    };

    // Act
    let contains_network_mismatch = value_contains_network_mismatch(&value);

    // Assert
    assert!(contains_network_mismatch)
}

/// Tests that the programmatic JSON representation from the
/// radixdlt/radixdlt-scrypto repo and the one in this repo match.
pub fn programmatic_json_representations_match<T>(object: &T)
where
    T: ScryptoEncode + Debug,
{
    // Arrange
    let payload = scrypto_encode(&object).unwrap();
    let expected = {
        let encoder = AddressBech32Encoder::for_simulator();
        let serialization_parameters = SerializationParameters::Schemaless {
            mode: SerializationMode::Programmatic,
            custom_context: ScryptoValueDisplayContext::with_optional_bech32(
                Some(&encoder),
            ),
            depth_limit: SCRYPTO_SBOR_V1_MAX_DEPTH,
        };

        let payload = ScryptoRawPayload::new_from_valid_slice(&payload);
        let serializable = payload.serializable(serialization_parameters);
        serde_json::to_value(serializable).expect("Impossible Case!")
    };

    // Act
    let actual = {
        let scrypto_value = scrypto_decode::<ScryptoValue>(&payload).unwrap();
        let value =
            ProgrammaticScryptoValue::from_scrypto_value(&scrypto_value, 0xF2);
        serde_json::to_value(value).unwrap()
    };

    // Assert
    assert_eq!(
        actual,
        expected,
        "The serialization of \"{object:?}\" produced different programmatic JSON representations:\nExpected: \"{expected:?}\"\nActual:   \"{actual:?}\""
    )
}

macro_rules! serialization_tests {
    (
        $(
            $fn_ident: ident: $value: expr
        ),* $(,)?
    ) => {
        $(
            #[test]
            fn $fn_ident() {
                programmatic_json_representations_match(&$value)
            }
        )*
    };
}
use serialization_tests;

#[derive(ScryptoSbor, Debug, Clone)]
pub enum MyEnum {
    UnitVariant,
    VariantWithNoFieldNames(u8),
    VariantWithFieldNames { field: u8 },
}

#[derive(ScryptoSbor, Debug, Clone)]
pub struct UnitStruct;
#[derive(ScryptoSbor, Debug, Clone)]
pub struct NoFieldsStruct();
#[derive(ScryptoSbor, Debug, Clone)]
pub struct NoFieldNamesStruct(u8);
#[derive(ScryptoSbor, Debug, Clone)]
pub struct NamedFieldsStruct {
    field: u8,
}
