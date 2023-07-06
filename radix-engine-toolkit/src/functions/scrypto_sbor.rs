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

use std::ops::Deref;

use crate::prelude::*;

use radix_engine_common::prelude::*;
use radix_engine_toolkit_core::utils::*;
use sbor::{LocalTypeIndex, Schema};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

//===============================
// Scrypto Sbor Decode to String
//===============================

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct ScryptoSborDecodeToStringInput {
    encoded_payload: SerializableBytes,
    representation: SerializableSerializationMode,
    network_id: SerializableU8,
    schema: Option<(SerializableLocalTypeIndex, SerializableBytes)>,
}
pub type ScryptoSborDecodeToStringOutput = String;

pub struct ScryptoSborDecodeToString;
impl<'f> Function<'f> for ScryptoSborDecodeToString {
    type Input = ScryptoSborDecodeToStringInput;
    type Output = ScryptoSborDecodeToStringOutput;

    fn handle(
        ScryptoSborDecodeToStringInput {
            encoded_payload,
            network_id,
            representation,
            schema,
        }: Self::Input,
    ) -> Result<Self::Output, crate::error::InvocationHandlingError> {
        let encoded_payload = encoded_payload.deref().clone();
        let network_id = *network_id;
        let representation = representation.into();
        let schema = if let Some((local_type_index, schema)) = schema {
            let local_type_index = LocalTypeIndex::from(local_type_index);
            let schema =
                scrypto_decode::<Schema<ScryptoCustomSchema>>(&schema).map_err(|error| {
                    InvocationHandlingError::DecodeError(debug_string(error), debug_string(schema))
                })?;

            Some((local_type_index, schema))
        } else {
            None
        };
        let network_definition = network_definition_from_network_id(network_id);
        let bech32_encoder = AddressBech32Encoder::new(&network_definition);

        let string =
            radix_engine_toolkit_core::functions::scrypto_sbor::decode_to_string_representation(
                encoded_payload,
                representation,
                &bech32_encoder,
                schema,
            )?;

        Ok(string)
    }
}

export_function!(ScryptoSborDecodeToString as scrypto_sbor_decode_to_string);
export_jni_function!(ScryptoSborDecodeToString as scryptoSborDecodeToString);