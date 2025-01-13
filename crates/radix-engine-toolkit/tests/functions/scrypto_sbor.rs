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

use radix_common::prelude::{
    scrypto_encode, AddressBech32Encoder, ScryptoCustomSchema, ScryptoValue,
};
use radix_common::ScryptoSbor;
use sbor::generate_full_schema_from_single_type;
use sbor::representations::SerializationMode;

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
    let encoded =
        radix_engine_toolkit::functions::scrypto_sbor::encode(&value).unwrap();

    // Act
    let decoded =
        radix_engine_toolkit::functions::scrypto_sbor::decode(encoded);

    // Assert
    assert!(decoded.is_ok());
    assert_eq!(decoded, Ok(value));
}

#[test]
fn scrypto_value_can_be_represented_as_a_string() {
    // Arrange
    let value = MyStruct { value: true };
    let encoded_value = scrypto_encode(&value).unwrap();

    let (local_type_id, schema) =
        generate_full_schema_from_single_type::<MyStruct, ScryptoCustomSchema>(
        );

    let serialization_modes_params = [
        SerializationMode::Model,
        SerializationMode::Natural,
        SerializationMode::Programmatic,
    ];
    let schema_params = [None, Some((local_type_id, schema.v1().clone()))];
    let bech32_encoder = AddressBech32Encoder::for_simulator();

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
