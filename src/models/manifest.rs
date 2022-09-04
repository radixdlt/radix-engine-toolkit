use serde::{Deserialize, Serialize};
use transaction::manifest::ast::Instruction as AstInstruction;

use crate::error::Error;
use super::{Instruction, ast_instruction_from_instruction, instruction_from_ast_instruction};

#[derive(Serialize, Deserialize, Debug)]
pub enum ManifestKind {
    String,
    JSON,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "value")]
pub enum Manifest {
    String(String),
    JSON(Vec<Instruction>),
}

impl Manifest {
    pub fn instructions(&self, network_id: u8) -> Result<Vec<Instruction>, Error> {
        match self {
            Self::JSON(instructions) => Ok(instructions.clone()),
            Self::String(_) => {
                // Converting the manifest string into a Vec<AstInstruction> first.
                let ast_instructions: Vec<AstInstruction> = self.ast_instructions(network_id)?;

                // Converting the AstInstructions to Instructions
                let instructions: Vec<Instruction> = ast_instructions
                    .iter()
                    .map(|instruction| instruction_from_ast_instruction(instruction, network_id))
                    .collect::<Result<Vec<Instruction>, _>>()?;
                Ok(instructions)
            }
        }
    }

    pub fn ast_instructions(&self, network_id: u8) -> Result<Vec<AstInstruction>, Error> {
        match self {
            Self::JSON(instructions) => {
                let instructions: Vec<AstInstruction> = instructions
                    .iter()
                    .map(|instruction| ast_instruction_from_instruction(instruction, network_id))
                    .collect::<Result<Vec<AstInstruction>, _>>()?;
                Ok(instructions)
            }
            Self::String(manifest_string) => {
                let tokens = transaction::manifest::lexer::tokenize(manifest_string)
                    .map_err(transaction::manifest::CompileError::LexerError)?;
                let instructions: Vec<AstInstruction> =
                    transaction::manifest::parser::Parser::new(tokens)
                        .parse_manifest()
                        .map_err(transaction::manifest::CompileError::ParserError)?;
                Ok(instructions)
            }
        }
    }
}
