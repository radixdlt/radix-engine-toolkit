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
use native_transaction::manifest::decompile;
use native_transaction::manifest::generator::generate_manifest;
use radix_engine_toolkit::error::VisitorError;
use radix_engine_toolkit::model::address::Bech32Coder;
use radix_engine_toolkit::model::instruction::Instruction;
use radix_engine_toolkit::model::transaction::{InstructionKind, InstructionList};
use radix_engine_toolkit::model::value::ast::ManifestAstValue;
use radix_engine_toolkit::utils::checked_copy_u8_slice;
use radix_engine_toolkit::visitor::{traverse_instruction, ManifestAstValueVisitor};
use scrypto::prelude::Hash;

use crate::error::{Error, Result};
use crate::utils::pretty_print;

#[derive(Parser, Debug)]
/// Converts transaction manifests from one format to another. Currently, this can perform two way
/// conversion from and to the regular string based format of manifests to a JSON based format of
/// the Abstract Syntax Tree (AST) of the manifest.
///
/// The conversion or output format is implicit rather than explicit. If a string manifest is the
/// input then a JSON manifest is the output. Similarly, if a JSON manifest is the input, then a
/// string manifest is the output.
pub struct ConvertManifest {
    /// The path to a manifest file. This can either be a standard `.rtm` file of the manifest in
    /// text form or could be the path to a `.json` file of the JSON based manifest abstract syntax
    /// tree.
    #[clap(short, long)]
    manifest_path: PathBuf,

    /// The id of the network to use when performing the manifest conversion. This will be used in
    /// validation and also in the generation of the converted manifest.
    #[clap(short, long)]
    network_id: u8,
}

impl ConvertManifest {
    pub fn run<O: std::io::Write>(&self, out: &mut O) -> Result<()> {
        // The Bech32 Encoder and Decoder to use for this operation
        let bech32_coder = Bech32Coder::new(self.network_id);

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
        let mut instructions = {
            let file_content = std::fs::read_to_string(&self.manifest_path)?;
            match input_type {
                InstructionKind::String => InstructionList::String(file_content),
                InstructionKind::Parsed => serde_json::from_str(&file_content)?,
            }
        };

        // Attempt to get whatever blobs we need from the instructions that we have.
        let blob_references = match instructions {
            InstructionList::String(..) => {
                // Parse the string manifest into a native abstract syntax tree manifest.
                let instructions = instructions.ast_instructions(&bech32_coder).unwrap();

                // We will aggregate the blob references from the package publishing into a vector.
                // We do not care about other blobs since they're technically unusable in Scrypto
                // and would not make sense to include
                let mut blob_references = Vec::new();
                for instruction in instructions {
                    if let native_transaction::manifest::ast::Instruction::PublishPackage {
                        code: native_transaction::manifest::ast::Value::Blob(code),
                        schema: native_transaction::manifest::ast::Value::Blob(abi),
                        ..
                    } = instruction
                    {
                        if let (
                            native_transaction::manifest::ast::Value::String(code),
                            native_transaction::manifest::ast::Value::String(abi),
                        ) = (*code, *abi)
                        {
                            blob_references
                                .push(checked_copy_u8_slice(&hex::decode(code)?).unwrap());
                            blob_references
                                .push(checked_copy_u8_slice(&hex::decode(abi)?).unwrap());
                        }
                    }
                }
                blob_references
            }
            InstructionList::Parsed(ref mut instructions) => {
                let mut value_visitor = BlobValueVisitor::default();
                for instruction in instructions.iter_mut() {
                    traverse_instruction(instruction, &mut [&mut value_visitor], &mut []).unwrap();
                }
                value_visitor.0
            }
        };

        // Perform the conversion into the other kind of instructions
        let output = match input_type {
            InstructionKind::String => {
                // Parse the string manifest into a native abstract syntax tree manifest.
                let instructions = instructions.ast_instructions(&bech32_coder).unwrap();
                let instructions = instructions
                    .into_iter()
                    .map(|instruction| {
                        Instruction::from_ast_instruction(&instruction, &bech32_coder)
                    })
                    .collect::<std::result::Result<Vec<_>, _>>()
                    .unwrap();
                InstructionList::Parsed(instructions)
            }
            InstructionKind::Parsed => {
                let instructions = instructions.ast_instructions(&bech32_coder).unwrap();
                let manifest = generate_manifest(
                    &instructions,
                    bech32_coder.decoder(),
                    blob_references
                        .into_iter()
                        .map(|blob_hash| (Hash(blob_hash), vec![]))
                        .collect(),
                )?;
                let string = decompile(&manifest.instructions, bech32_coder.network_definition())?;
                InstructionList::String(string)
            }
        };

        pretty_print(&output, out)?;
        Ok(())
    }
}

#[derive(Default)]
struct BlobValueVisitor(Vec<[u8; 32]>);
impl ManifestAstValueVisitor for BlobValueVisitor {
    fn visit_blob(
        &mut self,
        value: &mut radix_engine_toolkit::model::value::ast::ManifestAstValue,
    ) -> std::result::Result<(), VisitorError> {
        if let ManifestAstValue::Blob { hash } = value {
            self.0.push(hash.0);
        }
        Ok(())
    }
}
