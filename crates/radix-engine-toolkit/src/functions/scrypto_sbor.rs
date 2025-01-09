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

use crate::internal_prelude::*;

pub fn encode(value: &ScryptoValue) -> Result<Vec<u8>, EncodeError> {
    scrypto_encode(value)
}

pub fn decode<T>(value: T) -> Result<ScryptoValue, DecodeError>
where
    T: AsRef<[u8]>,
{
    scrypto_decode(value.as_ref())
}

pub fn decode_to_string_representation<T>(
    value: T,
    representation: SerializationMode,
    bech32_encoder: &AddressBech32Encoder,
    schema: Option<(LocalTypeId, Schema<ScryptoCustomSchema>)>,
) -> Result<String, ScryptoSborError>
where
    T: AsRef<[u8]>,
{
    let value = value.as_ref();

    // Ensure that whatever value was passed either matches the schema if given
    // or is valid Scrypto sbor.
    if let Some((ref local_type_id, ref schema)) = schema {
        validate_payload_against_schema::<ScryptoCustomExtension, _>(
            value,
            schema,
            *local_type_id,
            &(),
            SCRYPTO_SBOR_V1_MAX_DEPTH,
        )
        .map_err(|_| ScryptoSborError::SchemaValidationError)?;
    } else {
        decode(value).map_err(ScryptoSborError::DecodeError)?;
    };

    let context =
        ScryptoValueDisplayContext::with_optional_bech32(Some(bech32_encoder));
    let serialization_parameters =
        if let Some((ref local_type_id, ref schema)) = schema {
            SerializationParameters::WithSchema {
                mode: representation,
                custom_context: context,
                schema,
                type_id: *local_type_id,
                depth_limit: SCRYPTO_SBOR_V1_MAX_DEPTH,
            }
        } else {
            SerializationParameters::Schemaless {
                mode: representation,
                custom_context: context,
                depth_limit: SCRYPTO_SBOR_V1_MAX_DEPTH,
            }
        };

    let payload = ScryptoRawPayload::new_from_valid_slice(value);
    let serializable = payload.serializable(serialization_parameters);
    let serialized =
        serde_json::to_string(&serializable).expect("Impossible Case!");

    Ok(serialized)
}

pub fn encode_string_representation(
    representation: StringRepresentation,
) -> Result<Vec<u8>, ScryptoSborError> {
    match representation {
        StringRepresentation::ProgrammaticJson(value) => {
            let value =
                serde_json::from_str::<ProgrammaticScryptoValue>(&value)
                    .map_err(ScryptoSborError::SerdeDeserializationFailed)?;
            if value_contains_network_mismatch(&value) {
                return Err(ScryptoSborError::ValueContainsNetworkMismatch);
            }

            let value = value.to_scrypto_value();
            scrypto_encode(&value).map_err(ScryptoSborError::EncodeError)
        }
    }
}

#[derive(Debug, Clone)]
pub enum StringRepresentation {
    ProgrammaticJson(String),
}

#[derive(Debug)]
pub enum ScryptoSborError {
    SchemaValidationError,
    DecodeError(DecodeError),
    EncodeError(EncodeError),
    SerdeDeserializationFailed(serde_json::Error),
    ValueContainsNetworkMismatch,
}
