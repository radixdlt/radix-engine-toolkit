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

use crate::functions::traits::InvocationHandler;
use crate::model::address::Bech32Coder;
use crate::model::value::manifest_sbor::ManifestSborValueConversionError;
use crate::model::value::scrypto_sbor::{ScryptoSborValue, ScryptoSborValueConversionError};
use crate::utils;
use crate::{model::value::manifest_sbor::ManifestSborValue, utils::debug_string};
use sbor::representations::{SerializationMode, SerializationParameters};
use sbor::{DecodeError, LocalTypeIndex, Schema};
use scrypto::prelude::*;
use scrypto_utils::ContextualSerialize;
use toolkit_derive::serializable;

// =================
// Model Definition
// =================

/// Takes in a byte array of SBOR byte and attempts to decode it to a [`Value`]. Since some of the
/// types in the [`Value`] model are network aware, this request also takes in a network id which
/// is primarily used for the Bech32m encoding of addresses.
#[serializable]
pub struct Input {
    /// A byte array serialized as a hex string of the SBOR buffer to attempt to decode as a
    /// [`Value`]
    #[schemars(with = "String")]
    #[schemars(regex(pattern = "[0-9a-fA-F]+"))]
    #[serde_as(as = "serde_with::hex::Hex")]
    pub encoded_value: Vec<u8>,

    /// An 8 bit unsigned integer serialized as a string which represents the id of the network
    /// that the decoded data will be used on. This is primarily used for the Bech32m encoding of
    /// addresses.
    #[schemars(with = "String")]
    #[schemars(regex(pattern = "[0-9]+"))]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub network_id: u8,

    pub schema: Option<TypeSchema>,
}

/// The response from the [`Input`].
#[serializable]
#[serde(tag = "type")]
pub enum Output {
    ScryptoSbor {
        value: ScryptoSborValue,
    },
    ManifestSbor {
        manifest_string: String,
        value: ManifestSborValue,
    },
}

#[serializable]
pub struct TypeSchema {
    #[schemars(with = "String")]
    #[schemars(regex(pattern = "[0-9a-fA-F]+"))]
    #[serde_as(as = "serde_with::hex::Hex")]
    local_type_index: Vec<u8>,

    #[schemars(with = "String")]
    #[schemars(regex(pattern = "[0-9a-fA-F]+"))]
    #[serde_as(as = "serde_with::hex::Hex")]
    schema: Vec<u8>,
}

impl TypeSchema {
    pub fn local_index_and_schema(
        &self,
    ) -> Result<(LocalTypeIndex, Schema<ScryptoCustomSchema>), Error> {
        let local_type_index =
            scrypto_decode(&self.local_type_index).map_err(|_| Error::FailedToDecodeSchema)?;
        let schema = scrypto_decode(&self.schema).map_err(|_| Error::FailedToDecodeSchema)?;
        Ok((local_type_index, schema))
    }
}

impl From<(LocalTypeIndex, Schema<ScryptoCustomSchema>)> for TypeSchema {
    fn from((local_type_index, schema): (LocalTypeIndex, Schema<ScryptoCustomSchema>)) -> Self {
        Self {
            local_type_index: scrypto_encode(&local_type_index).unwrap(),
            schema: scrypto_encode(&schema).unwrap(),
        }
    }
}

// ===============
// Implementation
// ===============

pub struct Handler;
impl InvocationHandler<Input, Output> for Handler {
    type Error = Error;

    fn pre_process(input: Input) -> Result<Input, Error> {
        Ok(input)
    }

    fn handle(input: &Input) -> Result<Output, Error> {
        // The Bech32 coder used for the decoding.
        let bech32_coder = Bech32Coder::new(input.network_id);

        // If Schema was passed then attempt to decode it as Scrypto Schema
        let schema = if let Some(ref type_schema) = input.schema {
            Some(type_schema.local_index_and_schema()?)
        } else {
            None
        };

        match input.encoded_value.first().copied() {
            Some(SCRYPTO_SBOR_V1_PAYLOAD_PREFIX) => {
                let payload = ScryptoRawPayload::new_from_valid_slice(&input.encoded_value);
                let serialization_context =
                    ScryptoValueDisplayContext::with_optional_bech32(Some(bech32_coder.encoder()));
                let serialized = if let Some((local_type_index, schema)) = schema {
                    let serializable = payload.serializable(SerializationParameters::WithSchema {
                        mode: SerializationMode::Programmatic,
                        custom_context: serialization_context,
                        schema: &schema,
                        type_index: local_type_index,
                    });
                    serde_json::to_value(serializable).unwrap()
                } else {
                    let serializable = payload.serializable(SerializationParameters::Schemaless {
                        mode: SerializationMode::Programmatic,
                        custom_context: serialization_context,
                    });
                    serde_json::to_value(serializable).unwrap()
                };

                Ok(Output::ScryptoSbor {
                    value: serde_json::from_value(serialized).unwrap(),
                })
            }
            Some(MANIFEST_SBOR_V1_PAYLOAD_PREFIX) => {
                let payload = ManifestRawPayload::new_from_valid_slice(&input.encoded_value);
                let serialization_context =
                    ManifestValueDisplayContext::with_optional_bech32(Some(bech32_coder.encoder()));
                let serialized = if let Some((local_type_index, schema)) = schema {
                    let serializable = payload.serializable(SerializationParameters::WithSchema {
                        mode: SerializationMode::Programmatic,
                        custom_context: serialization_context,
                        schema: &schema,
                        type_index: local_type_index,
                    });
                    serde_json::to_value(serializable).unwrap()
                } else {
                    let serializable = payload.serializable(SerializationParameters::Schemaless {
                        mode: SerializationMode::Programmatic,
                        custom_context: serialization_context,
                    });
                    serde_json::to_value(serializable).unwrap()
                };

                let manifest_string = utils::manifest_string_representation(
                    &manifest_decode::<ManifestValue>(&input.encoded_value).unwrap(),
                    &bech32_coder,
                );

                Ok(Output::ManifestSbor {
                    manifest_string,
                    value: serde_json::from_value(serialized).unwrap(),
                })
            }
            Some(p) => Err(Error::InvalidSborVariant {
                expected: vec![
                    SCRYPTO_SBOR_V1_PAYLOAD_PREFIX,
                    MANIFEST_SBOR_V1_PAYLOAD_PREFIX,
                ],
                actual: p,
            }),
            None => Err(Error::EmptyPayloadError),
        }
    }

    fn post_process(_: &Input, output: Output) -> Result<Output, Error> {
        Ok(output)
    }
}

#[serializable]
#[serde(tag = "type", content = "error")]
pub enum Error {
    /// An error emitted by the SBOR upstream functions that perform the decoding.
    Error { message: String },

    /// An error emitted when the passed SBOR payload is of an unknown variant and thus can not be
    /// decoded.
    InvalidSborVariant { expected: Vec<u8>, actual: u8 },

    /// Passed payload is empty; thus can not be decoded.
    EmptyPayloadError,

    /// Emitted if the conversion from the Native manifest SBOR model to the RET manifest SBOR
    /// model fails.
    ManifestSborValueConversionError(ManifestSborValueConversionError),

    /// Emitted if the conversion from the Native scrypto SBOR model to the RET manifest SBOR
    /// model fails.
    ScryptoSborValueConversionError(ScryptoSborValueConversionError),

    /// Emitted when the schema fails to be decoded
    FailedToDecodeSchema,
}

impl From<DecodeError> for Error {
    fn from(value: DecodeError) -> Self {
        Self::Error {
            message: debug_string(value),
        }
    }
}

impl From<ManifestSborValueConversionError> for Error {
    fn from(value: ManifestSborValueConversionError) -> Self {
        Self::ManifestSborValueConversionError(value)
    }
}

impl From<ScryptoSborValueConversionError> for Error {
    fn from(value: ScryptoSborValueConversionError) -> Self {
        Self::ScryptoSborValueConversionError(value)
    }
}
