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
use sbor::{LocalTypeId, Schema};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

//===============================
// Scrypto Sbor Decode to String
//===============================

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct ScryptoSborDecodeToStringInput {
    encoded_payload: SerializableBytes,
    representation: SerializableSerializationMode,
    network_id: SerializableU8,
    schema: Option<PayloadSchema>,
}
#[typeshare::typeshare]
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
        let schema = if let Some(PayloadSchema {
            local_type_id,
            schema,
        }) = schema
        {
            let local_type_id = LocalTypeId::from(local_type_id);
            let schema =
                scrypto_decode::<Schema<ScryptoCustomSchema>>(&schema).map_err(|error| {
                    InvocationHandlingError::DecodeError(debug_string(error), debug_string(schema))
                })?;

            Some((local_type_id, schema))
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

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
#[serde(tag = "kind", content = "value")]
pub enum SerializableScryptoSborStringRepresentation {
    ProgrammaticJson(String),
}

#[typeshare::typeshare]
pub type ScryptoSborEncodeStringRepresentationInput = SerializableScryptoSborStringRepresentation;

#[typeshare::typeshare]
pub type ScryptoSborEncodeStringRepresentationOutput = SerializableBytes;

pub struct ScryptoSborEncodeStringRepresentation;
impl<'f> Function<'f> for ScryptoSborEncodeStringRepresentation {
    type Input = ScryptoSborEncodeStringRepresentationInput;
    type Output = ScryptoSborEncodeStringRepresentationOutput;

    fn handle(input: Self::Input) -> Result<Self::Output, crate::error::InvocationHandlingError> {
        let input = match input {
            SerializableScryptoSborStringRepresentation::ProgrammaticJson(value) => {
                radix_engine_toolkit_core::functions::scrypto_sbor::StringRepresentation::ProgrammaticJson(value)
            }
        };
        let bytes =
            radix_engine_toolkit_core::functions::scrypto_sbor::encode_string_representation(
                input,
            )?;
        Ok(bytes.into())
    }
}

export_function!(
    ScryptoSborEncodeStringRepresentation as scrypto_sbor_encode_string_representation
);
export_jni_function!(
    ScryptoSborEncodeStringRepresentation as scryptoSborEncodeStringRepresentation
);
