use radix_transaction::manifest::ast::Instruction as AstInstruction;
use radix_transaction::manifest::decompile;
use radix_transaction::model::Instruction as TransactionInstruction;

use scrypto::prelude::hash;
use serde::{Deserialize, Serialize};

use crate::address::Bech32Manager;
use crate::error::Error;
use crate::model::Instruction;
use crate::traits::Validate;

// ==================
// Model Definitions
// ==================

/// Represents the type of the manifest instructions kind.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ManifestInstructionsKind {
    String,
    JSON,
}

/// A union type of the two states that manifest instructions can be represented in: as a string or
/// as a vector of instructions (called JSON in this case for user friendliness when serialized).
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "value")]
pub enum ManifestInstructions {
    String(String),
    JSON(Vec<Instruction>),
}

// ======================
// Model Implementations
// ======================

impl ManifestInstructions {
    pub fn instructions(&self, bech32_manager: &Bech32Manager) -> Result<Vec<Instruction>, Error> {
        let json_instructions = self.convert_to_json(bech32_manager)?;
        if let ManifestInstructions::JSON(instructions) = json_instructions {
            Ok(instructions)
        } else {
            panic!("Impossible case.")
        }
    }

    pub fn ast_instructions(
        &self,
        bech32_manager: &Bech32Manager,
    ) -> Result<Vec<AstInstruction>, Error> {
        match self {
            Self::String(string) => {
                let tokens = radix_transaction::manifest::lexer::tokenize(string)
                    .map_err(radix_transaction::manifest::CompileError::LexerError)?;

                let instructions = radix_transaction::manifest::parser::Parser::new(tokens)
                    .parse_manifest()
                    .map_err(radix_transaction::manifest::CompileError::ParserError)?;
                Ok(instructions)
            }
            Self::JSON(instructions) => instructions
                .iter()
                .map(|instruction| instruction.to_ast_instruction(bech32_manager))
                .collect::<Result<Vec<_>, _>>(),
        }
    }

    pub fn transaction_instructions(
        &self,
        bech32_manager: &Bech32Manager,
        // TODO: This is a work around for a larger problem. Should definitely be removed in the
        // future. The problem is described in the long comment below.
        blobs: Vec<Vec<u8>>,
    ) -> Result<Vec<TransactionInstruction>, Error> {
        let instructions = self.ast_instructions(bech32_manager)?;
        let instructions = radix_transaction::manifest::generator::generate_manifest(
            &instructions,
            &bech32_manager.decoder,
            blobs.iter().map(|x| (hash(x), x.clone())).collect(),
        )?
        .instructions;
        Ok(instructions)
    }

    pub fn convert_to_string(
        &self,
        bech32_manager: &Bech32Manager,
        // TODO: This is a work around for a larger problem. Should definitely be removed in the
        // future. The problem is described in the long comment below.
        blobs: Vec<Vec<u8>>,
    ) -> Result<Self, Error> {
        match self {
            Self::String(_) => Ok(self.clone()),
            Self::JSON(_) => {
                // The only way to convert a vector of instructions to the string instructions
                // understood by the radix transaction manifest compiler is by going through a
                // series of steps:
                //
                // Vec<Instruction> -> Vec<AstInstruction> -> Vec<TransactionInstruction> -> String
                //
                // This long conversion is because we would like to use the decompiler provided by
                // the Scrypto repo.
                //
                // Q. Why not just implement a Instruction -> TransactionInstruction conversion and
                // skip the AstInstruction phase?
                // A. Because the IdValidator and id validation in general takes place when the
                // instruction is being converted from AstInstruction -> TransactionInstruction.
                // If i implement my own conversion (which is easy) then I lose out on the benefits
                // of running the id validator on transactions and the validation that it performs.
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

                // Vec<Instruction> --> Vec<AstInstruction> --> Vec<TransactionInstruction>
                // Conversion (based on above comment).
                let instructions = self.transaction_instructions(bech32_manager, blobs)?;

                // Vec<TransactionInstruction> --> String Conversion (based on above comment)
                Ok(Self::String(decompile(
                    &instructions,
                    &bech32_manager.network_definition,
                )?))
            }
        }
    }

    pub fn convert_to_json(&self, bech32_manager: &Bech32Manager) -> Result<Self, Error> {
        match self {
            Self::JSON(_) => Ok(self.clone()),
            Self::String(_) => {
                // This function takes advantage of Scrypto's transaction manifest compiler and uses
                // it to parse the transaction manifest instructions into Vec<AstInstruction> and
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
                let ast_instruction = self.ast_instructions(bech32_manager)?;
                let instructions = ast_instruction
                    .iter()
                    .map(|instruction| {
                        Instruction::from_ast_instruction(instruction, bech32_manager)
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Self::JSON(instructions))
            }
        }
    }

    pub fn convert_to_manifest_instructions_kind(
        &self,
        manifest_instructions_kind: ManifestInstructionsKind,
        bech32_manager: &Bech32Manager,
        // TODO: This is a work around for a larger problem. Should definitely be removed in the
        // future. The problem is described in the long comment below.
        blobs: Vec<Vec<u8>>,
    ) -> Result<Self, Error> {
        match manifest_instructions_kind {
            ManifestInstructionsKind::String => self.convert_to_string(bech32_manager, blobs),
            ManifestInstructionsKind::JSON => self.convert_to_json(bech32_manager),
        }
    }
}

// ===========
// Validation
// ===========

impl Validate for ManifestInstructions {
    fn validate(&self) -> Result<(), Error> {
        // TODO: What kind of validation can be done here? This can not use external context so we
        // do not have access to the blobs and can not pass it through the generator to check it.
        Ok(())
    }
}
