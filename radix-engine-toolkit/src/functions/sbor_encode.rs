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
use crate::model::value::manifest_sbor::ManifestSborValueConversionError;
use crate::model::value::scrypto_sbor::{ScryptoSborValue, ScryptoSborValueConversionError};
use crate::{model::value::manifest_sbor::ManifestSborValue, utils::debug_string};
use sbor::EncodeError;
use scrypto::prelude::{manifest_encode, scrypto_encode};
use toolkit_derive::serializable;

// =================
// Model Definition
// =================

/// This request takes in a [`Value`] and attempts to SBOR encode it and return back an SBOR byte
/// array.
#[serializable]
#[serde(tag = "type", content = "value")]
pub enum Input {
    ScryptoSbor(ScryptoSborValue),
    ManifestSbor(ManifestSborValue),
}

/// The response from the [`Input`].
#[serializable]
pub struct Output {
    /// A byte array serialized as a hex string of the SBOR encoded value.
    #[schemars(with = "String")]
    #[schemars(regex(pattern = "[0-9a-fA-F]+"))]
    #[serde_as(as = "serde_with::hex::Hex")]
    pub encoded_value: Vec<u8>,
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
        match input {
            Input::ManifestSbor(value) => Ok(Output {
                encoded_value: manifest_encode(&value.to_manifest_sbor_value()?)?,
            }),
            Input::ScryptoSbor(value) => Ok(Output {
                encoded_value: scrypto_encode(&value.to_scrypto_sbor_value()?)?,
            }),
        }
    }

    fn post_process(_: &Input, output: Output) -> Result<Output, Error> {
        Ok(output)
    }
}

#[serializable]
#[serde(tag = "type", content = "error")]
pub enum Error {
    ManifestSborValueConversionError(ManifestSborValueConversionError),
    ScryptoSborValueConversionError(ScryptoSborValueConversionError),
    Error { message: String },
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

impl From<EncodeError> for Error {
    fn from(value: EncodeError) -> Self {
        Self::Error {
            message: debug_string(value),
        }
    }
}
