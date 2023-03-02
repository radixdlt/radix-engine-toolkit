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

use crate::error::Result;
use crate::model::address::Bech32Coder;
use crate::model::instruction::Instruction;
use native_transaction::manifest::{ast, decompile};
use native_transaction::model as transaction;
use scrypto::prelude::hash;
use toolkit_derive::serializable;

// =================
// Model Definition
// =================

/// A discriminated union of possible representations of manifest instructions. Currently, two
/// representations are supported: a string representation which is the same as that seen in the
/// local simulator, resim, and pretty much everywhere, as well as a parsed format which is a vector
/// of instructions where each instruction is represented through the `Instruction` model.
#[serializable]
#[serde(tag = "type", content = "value")]
pub enum InstructionList {
    #[schemars(example = "crate::example::transaction::instruction_list::string")]
    String(String),
    #[schemars(example = "crate::example::transaction::instruction_list::parsed")]
    Parsed(Vec<Instruction>),
}

/// An enum which describes the kind of manifest instructions.
#[serializable]
#[derive(Copy)]
pub enum InstructionKind {
    String,
    Parsed,
}

// ===============
// Implementation
// ===============

impl InstructionList {
    pub fn kind(&self) -> InstructionKind {
        match self {
            Self::String(..) => InstructionKind::String,
            Self::Parsed(..) => InstructionKind::Parsed,
        }
    }

    pub fn ast_instructions(&self, bech32_coder: &Bech32Coder) -> Result<Vec<ast::Instruction>> {
        match self {
            Self::String(string) => {
                let tokens = native_transaction::manifest::lexer::tokenize(string)
                    .map_err(native_transaction::manifest::CompileError::LexerError)?;

                let instructions = native_transaction::manifest::parser::Parser::new(tokens)
                    .parse_manifest()
                    .map_err(native_transaction::manifest::CompileError::ParserError)?;
                Ok(instructions)
            }
            Self::Parsed(instructions) => instructions
                .iter()
                .map(|instruction| instruction.to_ast_instruction(bech32_coder))
                .collect::<Result<Vec<_>>>(),
        }
    }

    pub fn basic_instructions(
        &self,
        bech32_coder: &Bech32Coder,
        // TODO: This is a work around for a larger issue. Should definitely be removed in the
        // future. The problem is described in the long comment below.
        blobs: Vec<Vec<u8>>,
    ) -> Result<Vec<transaction::Instruction>> {
        let instructions = self.ast_instructions(bech32_coder)?;
        let instructions = native_transaction::manifest::generator::generate_manifest(
            &instructions,
            bech32_coder.decoder(),
            blobs.iter().map(|x| (hash(x), x.clone())).collect(),
        )?
        .instructions;
        Ok(instructions)
    }

    pub fn convert_to_string(
        &self,
        bech32_coder: &Bech32Coder,
        // TODO: This is a work around for a larger issue. Should definitely be removed in the
        // future. The problem is described in the long comment below.
        blobs: Vec<Vec<u8>>,
    ) -> Result<Self> {
        match self {
            Self::String(_) => Ok(self.clone()),
            Self::Parsed(_) => {
                // The only way to convert a vector of instructions to the string instructions
                // understood by the radix transaction manifest compiler is by going through a
                // series of steps:
                //
                // Vec<Instruction> -> Vec<ast::Instruction> -> Vec<transaction::Instruction>
                // -> String
                //
                // This long conversion is because we would like to use the decompiler provided by
                // the Scrypto repo.
                //
                // Q. Why not just implement a Instruction -> transaction::Instruction
                // conversion and skip the ast::Instruction phase?
                // A. Because the IdValidator and id validation in general takes place when the
                // instruction is being converted from ast::Instruction ->
                // transaction::Instruction. If i implement my own conversion
                // (which is easy) then I lose out on the benefits of running the id
                // validator on transactions and the validation that it performs.
                //
                // Q. Why not re-implement the id-validator validation on this side and skip the
                // process of converting between these different types?
                // A. The format is changing quite often and these two implementations are bound to
                // become out of sync in no time at all.
                //
                // Q. Why not just implement Vec<Instruction> -> String conversion directly and skip
                // all of these steps?
                // A. Might be the easiest way to solve this problem, but it means that we lose out
                // on all of the validations and everything provided in the Scrypto repo for
                // manifests. In addition to that, these two implementations can become out of sync
                // in different aspects which is very undesirable.
                //
                // The above is the cause of some of the quirks that I am not too happy about. Like
                // the need to provide the blobs to be able to convert a manifest from one format to
                // another. This is a limitation caused by us needing to take this long path to
                // converting manifests. In the future, something like this should definitely not
                // be required.
                //
                // So, while in the long term, a better solution is for sure needed and required,
                // we should not immediately do something about this.

                // Vec<Instruction> --> Vec<ast::Instruction> --> Vec<transaction::Instruction>
                // Conversion (based on above comment).
                let instructions = self.basic_instructions(bech32_coder, blobs)?;

                // Vec<transaction::Instruction> --> String Conversion (based on above comment)
                Ok(Self::String(decompile(
                    &instructions,
                    bech32_coder.network_definition(),
                )?))
            }
        }
    }

    pub fn convert_to_parsed(
        &self,
        bech32_coder: &Bech32Coder,
        blobs: Vec<Vec<u8>>,
    ) -> Result<Self> {
        match self {
            Self::Parsed(_) => Ok(self.clone()),
            Self::String(_) => {
                // This function takes advantage of Scrypto's transaction manifest compiler and uses
                // it to parse the transaction manifest instructions into Vec<ast::Instruction> and
                // then convert that into the native Instruction type used in this code.
                //
                // The main problem with using Scrypto's manifest compiler is that we need to rely
                // on blobs always being present for the conversion, which makes sense for Scrypto
                // but does not make sense for us since we are simply converting the format from one
                // kind to another.
                //
                // Similar to the previous point and previous comment on this, we will need to look
                // into long term solutions for this to break away from the limitations of relying
                // on the Scrypto toolchain for operations like this.
                let ast_instruction = self.ast_instructions(bech32_coder)?;
                let instructions = ast_instruction
                    .iter()
                    .map(|instruction| Instruction::from_ast_instruction(instruction, bech32_coder))
                    .collect::<Result<Vec<_>>>()
                    .map(Self::Parsed);

                // TODO: Remove this validation step in favor of something better.
                // This step validates that the instruction format is correct by attempting to
                // compile the instructions
                match instructions
                    .clone()
                    .map(|instructions| instructions.convert_to_string(bech32_coder, blobs))
                {
                    Ok(..) => instructions,
                    Err(error) => Err(error),
                }
            }
        }
    }

    pub fn convert_to_manifest_instructions_kind(
        &self,
        manifest_instructions_kind: InstructionKind,
        bech32_coder: &Bech32Coder,
        // TODO: This is a work around for a larger problem. Should definitely be removed in the
        // future. The problem is described in the long comment below.
        blobs: Vec<Vec<u8>>,
    ) -> Result<Self> {
        match manifest_instructions_kind {
            InstructionKind::String => self.convert_to_string(bech32_coder, blobs),
            InstructionKind::Parsed => self.convert_to_parsed(bech32_coder, blobs),
        }
    }
}
