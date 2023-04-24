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

use crate::model::value::manifest_sbor::ManifestSborValueConversionError;
use crate::model::value::scrypto_sbor::ScryptoSborValue;
use crate::request::traits::Handler;
use crate::{model::value::manifest_sbor::ManifestSborValue, utils::debug_string};
use sbor::DecodeError;
use scrypto::prelude::{
    manifest_decode, scrypto_decode, ManifestValue, ScryptoValue, MANIFEST_SBOR_V1_PAYLOAD_PREFIX,
    SCRYPTO_SBOR_V1_PAYLOAD_PREFIX,
};
use toolkit_derive::serializable;

// =================
// Model Definition
// =================

/// Takes in a byte array of SBOR byte and attempts to decode it to a [`Value`]. Since some of the
/// types in the [`Value`] model are network aware, this request also takes in a network id which
/// is primarily used for the Bech32m encoding of addresses.
#[serializable]
pub struct SborDecodeRequest {
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
}

/// The response from the [`SborDecodeRequest`].
#[serializable]
#[serde(tag = "type", content = "value")]
pub enum SborDecodeResponse {
    ScryptoSbor(ScryptoSborValue),
    ManifestSbor(ManifestSborValue),
}

// ===============
// Implementation
// ===============

pub struct SborDecodeHandler;

impl Handler<SborDecodeRequest, SborDecodeResponse> for SborDecodeHandler {
    type Error = SborDecodeError;

    fn pre_process(request: SborDecodeRequest) -> Result<SborDecodeRequest, SborDecodeError> {
        Ok(request)
    }

    fn handle(request: &SborDecodeRequest) -> Result<SborDecodeResponse, SborDecodeError> {
        match request.encoded_value.first().copied() {
            Some(SCRYPTO_SBOR_V1_PAYLOAD_PREFIX) => {
                scrypto_decode::<ScryptoValue>(&request.encoded_value)
                    .map(|scrypto_value| {
                        ScryptoSborValue::from_scrypto_sbor_value(
                            &scrypto_value,
                            request.network_id,
                        )
                    })
                    .map(SborDecodeResponse::ScryptoSbor)
                    .map_err(SborDecodeError::from)
            }
            Some(MANIFEST_SBOR_V1_PAYLOAD_PREFIX) => {
                manifest_decode::<ManifestValue>(&request.encoded_value)
                    .map_err(SborDecodeError::from)
                    .and_then(|manifest_value| {
                        ManifestSborValue::from_manifest_sbor_value(
                            &manifest_value,
                            request.network_id,
                        )
                        .map_err(SborDecodeError::from)
                    })
                    .map(SborDecodeResponse::ManifestSbor)
                    .map_err(SborDecodeError::from)
            }
            Some(p) => Err(SborDecodeError::InvalidSborVariant {
                expected: vec![
                    SCRYPTO_SBOR_V1_PAYLOAD_PREFIX,
                    MANIFEST_SBOR_V1_PAYLOAD_PREFIX,
                ],
                actual: p,
            }),
            None => Err(SborDecodeError::EmptyPayloadError),
        }
    }

    fn post_process(
        _: &SborDecodeRequest,
        response: SborDecodeResponse,
    ) -> Result<SborDecodeResponse, SborDecodeError> {
        Ok(response)
    }
}

#[serializable]
pub enum SborDecodeError {
    /// An error emitted by the SBOR upstream functions that perform the decoding.
    DecodeError { message: String },

    /// An error emitted when the passed SBOR payload is of an unknown variant and thus can not be
    /// decoded.
    InvalidSborVariant { expected: Vec<u8>, actual: u8 },

    /// Passed payload is empty; thus can not be decoded.
    EmptyPayloadError,

    /// Emitted if the conversion from the Native manifest SBOR model to the RET manifest SBOR
    /// model fails.
    ManifestSborValueConversionError(ManifestSborValueConversionError),
}

impl From<DecodeError> for SborDecodeError {
    fn from(value: DecodeError) -> Self {
        Self::DecodeError {
            message: debug_string(value),
        }
    }
}

impl From<ManifestSborValueConversionError> for SborDecodeError {
    fn from(value: ManifestSborValueConversionError) -> Self {
        Self::ManifestSborValueConversionError(value)
    }
}
