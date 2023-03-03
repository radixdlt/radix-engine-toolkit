// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at

//   http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

#![allow(clippy::enum_variant_names)]

use native_transaction::manifest::{generator::GeneratorError, DecompileError};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    RadixEngineToolkitError(radix_engine_toolkit::error::Error),
    InvalidFileFormat {
        expected: Vec<String>,
        found: String,
    },
    IoError(std::io::Error),
    DeserializationError(serde_json::error::Error),
    HexDecodeError(hex::FromHexError),
    GeneratorError(GeneratorError),
    DecompileError(DecompileError),
    InvalidPublicKey,
    InvalidStringConversion,
}

impl From<radix_engine_toolkit::error::Error> for Error {
    fn from(value: radix_engine_toolkit::error::Error) -> Self {
        Self::RadixEngineToolkitError(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(value: serde_json::error::Error) -> Self {
        Self::DeserializationError(value)
    }
}

impl From<hex::FromHexError> for Error {
    fn from(value: hex::FromHexError) -> Self {
        Self::HexDecodeError(value)
    }
}

impl From<GeneratorError> for Error {
    fn from(value: GeneratorError) -> Self {
        Self::GeneratorError(value)
    }
}

impl From<DecompileError> for Error {
    fn from(value: DecompileError) -> Self {
        Self::DecompileError(value)
    }
}
