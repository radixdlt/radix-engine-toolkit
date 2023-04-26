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

use std::path::PathBuf;

use clap::Parser;
use radix_engine_toolkit::error::{InvocationHandlingError, RETError};
use radix_engine_toolkit::functions::*;
use radix_engine_toolkit::model::transaction::{
    InstructionKind, InstructionList, TransactionManifest,
};

use crate::error::{Error, Result};
use crate::utils::pretty_print;

#[derive(Parser, Debug)]
/// Analyzes the manifest for all of the included addresses in the manifest.
pub struct ExtractAddressesFromManifest {
    /// The path to a manifest file. This can either be a standard `.rtm` file of the manifest in
    /// text form or could be the path to a `.json` file of the JSON based manifest abstract syntax
    /// tree.
    #[clap(short, long)]
    manifest_path: PathBuf,

    /// The id of the network to use for the analysis of the manifest
    #[clap(short, long)]
    network_id: u8,
}

impl ExtractAddressesFromManifest {
    pub fn run<O: std::io::Write>(&self, out: &mut O) -> Result<()> {
        // Determine the type of input to expect from the file extension.
        let input_type = match self
            .manifest_path
            .extension()
            .and_then(|string| string.to_str())
        {
            Some("rtm") => Ok(InstructionKind::String),
            Some("json") => Ok(InstructionKind::Parsed),
            Some(value) => Err(Error::InvalidFileFormat {
                expected: vec!["json".into(), "rtm".into()],
                found: value.to_string(),
            }),
            None => Err(Error::InvalidFileFormat {
                expected: vec!["json".into(), "rtm".into()],
                found: "".into(),
            }),
        }?;

        // Load the instructions from file
        let instructions = {
            let file_content = std::fs::read_to_string(&self.manifest_path)?;
            match input_type {
                InstructionKind::String => InstructionList::String(file_content),
                InstructionKind::Parsed => serde_json::from_str(&file_content)?,
            }
        };

        let request = extract_addresses_from_manifest::Input {
            manifest: TransactionManifest {
                instructions,
                blobs: vec![],
            },
            network_id: self.network_id,
        };
        let response =
            extract_addresses_from_manifest::Handler::fulfill(request).map_err(|error| {
                RETError::InvocationHandlingError(
                    InvocationHandlingError::ExtractAddressesFromManifestError(error),
                )
            })?;
        pretty_print(&response, out)
    }
}
