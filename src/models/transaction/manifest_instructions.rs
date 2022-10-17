// use radix_transaction::manifest::ast::Instruction as AstInstruction;
// use serde::{Deserialize, Serialize};

// use crate::address::Bech32Manager;
// use crate::error::Error;
// use crate::models::{
//     ast_instruction_from_instruction, instruction_from_ast_instruction, Instruction,
// };

// #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
// pub enum ManifestInstructionsKind {
//     String,
//     JSON,
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// #[serde(tag = "type", content = "value")]
// pub enum ManifestInstructions {
//     String(String),
//     JSON(Vec<Instruction>),
// }

// impl ManifestInstructions {
//     pub fn kind(&self) -> ManifestInstructionsKind {
//         match self {
//             Self::JSON(_) => ManifestInstructionsKind::JSON,
//             Self::String(_) => ManifestInstructionsKind::String,
//         }
//     }

//     pub fn to_instructions(
//         &self,
//         bech32_manager: &Bech32Manager,
//     ) -> Result<Vec<Instruction>, Error> {
//         match self {
//             Self::JSON(instructions) => Ok(instructions.clone()),
//             Self::String(_) => {
//                 // Converting the manifest string into a Vec<AstInstruction> first.
//                 let ast_instructions: Vec<AstInstruction> =
//                     self.to_ast_instructions(bech32_manager)?;

//                 // Converting the AstInstructions to Instructions
//                 let instructions: Vec<Instruction> = ast_instructions
//                     .iter()
//                     .map(|instruction| {
//                         instruction_from_ast_instruction(instruction, bech32_manager)
//                     })
//                     .collect::<Result<Vec<Instruction>, _>>()?;
//                 Ok(instructions)
//             }
//         }
//     }

//     pub fn to_ast_instructions(
//         &self,
//         bech32_manager: &Bech32Manager,
//     ) -> Result<Vec<AstInstruction>, Error> {
//         match self {
//             Self::JSON(instructions) => {
//                 let instructions: Vec<AstInstruction> = instructions
//                     .iter()
//                     .map(|instruction| {
//                         ast_instruction_from_instruction(instruction, bech32_manager)
//                     })
//                     .collect::<Result<Vec<AstInstruction>, _>>()?;
//                 Ok(instructions)
//             }
//             Self::String(manifest_string) => {
//                 let tokens = radix_transaction::manifest::lexer::tokenize(manifest_string)
//                     .map_err(radix_transaction::manifest::CompileError::LexerError)?;
//                 let instructions: Vec<AstInstruction> =
//                     radix_transaction::manifest::parser::Parser::new(tokens)
//                         .parse_manifest()
//                         .map_err(radix_transaction::manifest::CompileError::ParserError)?;
//                 Ok(instructions)
//             }
//         }
//     }

//     pub fn to(
//         &self,
//         manifest_kind: ManifestInstructionsKind,
//         bech32_manager: &Bech32Manager,
//         blobs: Vec<Vec<u8>>,
//     ) -> Result<Self, Error> {
//         match manifest_kind {
//             ManifestInstructionsKind::JSON => Ok(self.to_json_manifest(bech32_manager)?),
//             ManifestInstructionsKind::String => Ok(self.to_string_manifest(bech32_manager, blobs)?),
//         }
//     }

//     pub fn to_json_manifest(&self, bech32_manager: &Bech32Manager) -> Result<Self, Error> {
//         match self {
//             Self::JSON(_) => Ok(self.clone()),
//             Self::String(_) => Ok(Self::JSON(self.to_instructions(bech32_manager)?)),
//         }
//     }

//     pub fn to_string_manifest(
//         &self,
//         bech32_manager: &Bech32Manager,
//         blobs: Vec<Vec<u8>>,
//     ) -> Result<Self, Error> {
//         match self {
//             Self::String(_) => Ok(self.clone()),
//             Self::JSON(_) => {
//                 // Converting to a transaction manifest then decompiling the transaction manifest
//                 // to get a manifest string back
//                 let instructions: &[radix_transaction::model::Instruction] = &self
//                     .to_scrypto_transaction_manifest(bech32_manager, blobs)?
//                     .instructions;
//                 let manifest_string: String = radix_transaction::manifest::decompile(
//                     instructions,
//                     &bech32_manager.network_definition,
//                 )?;
//                 Ok(Self::String(manifest_string))
//             }
//         }
//     }

//     pub fn to_scrypto_transaction_manifest(
//         &self,
//         bech32_manager: &Bech32Manager,
//         blobs: Vec<Vec<u8>>,
//     ) -> Result<radix_transaction::model::TransactionManifest, Error> {
//         let mut manifest: radix_transaction::model::TransactionManifest =
//             radix_transaction::manifest::generator::generate_manifest(
//                 &self.to_ast_instructions(bech32_manager)?,
//                 &bech32_manager.decoder,
//                 blobs
//                     .iter()
//                     .map(|x| (radix_engine::types::hash(x), x.clone()))
//                     .collect(),
//             )
//             .map_err(radix_transaction::manifest::CompileError::GeneratorError)?;
//         manifest.blobs = blobs;
//         Ok(manifest)
//     }

//     pub fn from_scrypto_transaction_manifest(
//         transaction_manifest: &radix_transaction::model::TransactionManifest,
//         bech32_manager: &Bech32Manager,
//         output_manifest_kind: ManifestInstructionsKind,
//     ) -> Result<Self, Error> {
//         let manifest_string: String = radix_transaction::manifest::decompile(
//             &transaction_manifest.instructions,
//             &bech32_manager.network_definition,
//         )?;

//         let manifest: Self = Self::String(manifest_string);
//         manifest.to(
//             output_manifest_kind,
//             bech32_manager,
//             transaction_manifest.blobs.clone(),
//         )
//     }
// }

use radix_transaction::manifest::ast::Instruction as AstInstruction;
use radix_transaction::manifest::decompile;
use radix_transaction::model::Instruction as TransactionInstruction;

use serde::{Deserialize, Serialize};

use crate::address::Bech32Manager;
use crate::error::Error;
use crate::models::{
    ast_instruction_from_instruction, instruction_from_ast_instruction, Instruction,
};
use crate::traits::{TryIntoWithContext, Validate};

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
    pub fn ast_instructions(
        &self,
        bech32_manager: &Bech32Manager,
    ) -> Result<Vec<AstInstruction>, Error> {
        match self {
            Self::String(string) => {
                let tokens = radix_transaction::manifest::lexer::tokenize(&string)
                    .map_err(radix_transaction::manifest::CompileError::LexerError)?;

                let instructions: Vec<AstInstruction> =
                    radix_transaction::manifest::parser::Parser::new(tokens)
                        .parse_manifest()
                        .map_err(radix_transaction::manifest::CompileError::ParserError)?;
                Ok(instructions)
            }
            Self::JSON(instructions) => instructions
                .iter()
                .map(|instruction| ast_instruction_from_instruction(instruction, bech32_manager))
                .collect::<Result<Vec<_>, _>>(),
        }
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
            Self::JSON(instructions) => {
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

                // Vec<Instruction> --> Vec<AstInstruction> Conversion (based on above comment).
                let instructions: Vec<AstInstruction> = instructions
                    .iter()
                    .map(|instruction| {
                        ast_instruction_from_instruction(instruction, bech32_manager)
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                // Vec<AstInstruction> --> Vec<TransactionInstruction> Conversion (based on above
                // comment)
                let instructions: Vec<TransactionInstruction> =
                    radix_transaction::manifest::generator::generate_manifest(
                        &instructions,
                        &bech32_manager.decoder,
                        blobs
                            .iter()
                            .map(|x| (radix_engine::types::hash(x), x.clone()))
                            .collect(),
                    )?
                    .instructions;

                // Vec<TransactionInstruction> --> String Conversion (based on above comment)
                Ok(Self::String(decompile(
                    &instructions,
                    &bech32_manager.network_definition,
                )?))
            }
        }
    }

    pub fn convert_to_json(
        &self,
        bech32_manager: &Bech32Manager,
        // TODO: This is a work around for a larger problem. Should definitely be removed in the
        // future. The problem is described in the long comment below.
        blobs: Vec<Vec<u8>>,
    ) -> Result<Self, Error> {
        match self {
            Self::JSON(_) => Ok(self.clone()),
            Self::String(string) => {
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
                let ast_instruction: Vec<AstInstruction> = self.ast_instructions(bech32_manager)?;
                let instructions: Vec<Instruction> = ast_instruction
                    .iter()
                    .map(|instruction| {
                        instruction_from_ast_instruction(instruction, bech32_manager)
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
            ManifestInstructionsKind::JSON => self.convert_to_json(bech32_manager, blobs)
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