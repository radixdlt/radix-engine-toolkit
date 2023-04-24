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

use std::str::FromStr;

use crate::error::{Error, Result};
use crate::utils::pretty_print;
use clap::Parser;
use radix_engine_toolkit::error::{InvocationHandlingError, RETError};
use radix_engine_toolkit::model::transaction::InstructionKind;
use radix_engine_toolkit::request::{
    DecompileUnknownTransactionIntentHandler, DecompileUnknownTransactionIntentRequest, Handler,
};

/// Decompiles a Manifest and Scrypto SBOR encoded payloads.
#[derive(Parser, Debug)]
pub struct Decompile {
    /// The SBOR encoded payload to decode
    #[clap(short, long)]
    payload: String,

    /// The network id to use. This is primarily used for decoding addresses
    #[clap(short, long, default_value = "String")]
    output_instructions_kind: ManifestInstructionKind,
}

impl Decompile {
    pub fn run<O: std::io::Write>(&self, out: &mut O) -> Result<()> {
        let request = DecompileUnknownTransactionIntentRequest {
            compiled_unknown_intent: hex::decode(&self.payload)?,
            instructions_output_kind: self.output_instructions_kind.clone().into(),
        };
        let response =
            DecompileUnknownTransactionIntentHandler::fulfill(request).map_err(|error| {
                RETError::InvocationHandlingError(
                    InvocationHandlingError::DecompileUnknownTransactionIntentError(error),
                )
            })?;
        pretty_print(&response, out)
    }
}

#[derive(Debug, Clone)]
pub enum ManifestInstructionKind {
    String,
    Parsed,
}

impl From<String> for ManifestInstructionKind {
    // Clap no longer works with FromStr.
    fn from(value: String) -> Self {
        Self::from_str(&value).unwrap()
    }
}

impl FromStr for ManifestInstructionKind {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let lowercase_string = s.to_lowercase();
        match lowercase_string.as_str() {
            "string" => Ok(Self::String),
            "parsed" => Ok(Self::Parsed),
            _ => Err(Error::InvalidStringConversion),
        }
    }
}

impl std::fmt::Display for ManifestInstructionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Self::String => "String",
            Self::Parsed => "Parsed",
        };
        write!(f, "{string}")
    }
}

impl From<ManifestInstructionKind> for InstructionKind {
    fn from(value: ManifestInstructionKind) -> Self {
        match value {
            ManifestInstructionKind::Parsed => Self::Parsed,
            ManifestInstructionKind::String => Self::String,
        }
    }
}

impl From<InstructionKind> for ManifestInstructionKind {
    fn from(value: InstructionKind) -> Self {
        match value {
            InstructionKind::Parsed => Self::Parsed,
            InstructionKind::String => Self::String,
        }
    }
}
