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

use sbor::prelude::ContextualSerialize;
use sbor::representations::{SerializationMode, SerializationParameters};
use sbor::*;
use scrypto::address::*;
use scrypto::prelude::*;
use transaction::data::{format_manifest_value, ManifestDecompilationDisplayContext};

pub fn encode(value: &ManifestValue) -> Result<Vec<u8>, EncodeError> {
    manifest_encode(value)
}

pub fn decode<T>(value: T) -> Result<ManifestValue, DecodeError>
where
    T: AsRef<[u8]>,
{
    manifest_decode(value.as_ref())
}

pub fn decode_to_string_representation<T>(
    value: T,
    representation: ManifestSborStringRepresentation,
    bech32_encoder: &AddressBech32Encoder,
    schema: Option<(LocalTypeIndex, Schema<ScryptoCustomSchema>)>,
) -> Result<String, ManifestSborError>
where
    T: AsRef<[u8]>,
{
    let value = value.as_ref();

    // Ensure that whatever value was passed either matches the schema if given or is valid Manifest
    // sbor.
    if let Some((ref local_type_index, ref schema)) = schema {
        validate_payload_against_schema::<ManifestCustomExtension, _>(
            value,
            schema,
            *local_type_index,
            &(),
            MANIFEST_SBOR_V1_MAX_DEPTH,
        )
        .map_err(|_| ManifestSborError::SchemaValidationError)?;
    } else {
        decode(value).map_err(ManifestSborError::DecodeError)?;
    };

    let string = match representation {
        ManifestSborStringRepresentation::JSON(representation) => {
            let context = ManifestValueDisplayContext::with_optional_bech32(Some(bech32_encoder));
            let serialization_parameters = if let Some((ref local_type_index, ref schema)) = schema
            {
                SerializationParameters::WithSchema {
                    mode: representation,
                    custom_context: context,
                    schema,
                    type_index: *local_type_index,
                    depth_limit: MANIFEST_SBOR_V1_MAX_DEPTH,
                }
            } else {
                SerializationParameters::Schemaless {
                    mode: representation,
                    custom_context: context,
                    depth_limit: MANIFEST_SBOR_V1_MAX_DEPTH,
                }
            };

            let payload = ManifestRawPayload::new_from_valid_slice(value);
            let serializable = payload.serializable(serialization_parameters);
            serde_json::to_string(&serializable).expect("Impossible Case!")
        }
        ManifestSborStringRepresentation::ManifestString => {
            let context =
                ManifestDecompilationDisplayContext::with_optional_bech32(Some(bech32_encoder));
            let mut string = String::new();
            format_manifest_value(
                &mut string,
                &decode(value).map_err(ManifestSborError::DecodeError)?,
                &context,
                false,
                0,
            )
            .map_err(ManifestSborError::FmtError)?;
            string
        }
    };

    Ok(string)
}

#[derive(Debug, Clone)]
pub enum ManifestSborError {
    SchemaValidationError,
    DecodeError(DecodeError),
    FmtError(std::fmt::Error),
}

#[derive(Clone, Copy)]
pub enum ManifestSborStringRepresentation {
    ManifestString,
    JSON(SerializationMode),
}
