use serde::{Deserialize, Serialize};
use transaction::manifest::ast::Instruction as AstInstruction;

use super::{ast_instruction_from_instruction, instruction_from_ast_instruction, Instruction};
use crate::error::Error;
use crate::utils::network_definition_from_network_id;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ManifestKind {
    String,
    JSON,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "value")]
pub enum Manifest {
    String(String),
    JSON(Vec<Instruction>),
}

impl Manifest {
    pub fn kind(&self) -> ManifestKind {
        match self {
            Self::JSON(_) => ManifestKind::JSON,
            Self::String(_) => ManifestKind::String,
        }
    }

    pub fn to_instructions(&self, network_id: u8) -> Result<Vec<Instruction>, Error> {
        match self {
            Self::JSON(instructions) => Ok(instructions.clone()),
            Self::String(_) => {
                // Converting the manifest string into a Vec<AstInstruction> first.
                let ast_instructions: Vec<AstInstruction> = self.to_ast_instructions(network_id)?;

                // Converting the AstInstructions to Instructions
                let instructions: Vec<Instruction> = ast_instructions
                    .iter()
                    .map(|instruction| instruction_from_ast_instruction(instruction, network_id))
                    .collect::<Result<Vec<Instruction>, _>>()?;
                Ok(instructions)
            }
        }
    }

    pub fn to_ast_instructions(&self, network_id: u8) -> Result<Vec<AstInstruction>, Error> {
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

    pub fn to(&self, manifest_kind: ManifestKind, network_id: u8) -> Result<Self, Error> {
        match manifest_kind {
            ManifestKind::JSON => Ok(self.to_json_manifest(network_id)?),
            ManifestKind::String => Ok(self.to_string_manifest(network_id)?),
        }
    }

    pub fn to_json_manifest(&self, network_id: u8) -> Result<Self, Error> {
        match self {
            Self::JSON(_) => Ok(self.clone()),
            Self::String(_) => Ok(Self::JSON(self.to_instructions(network_id)?)),
        }
    }

    pub fn to_string_manifest(&self, network_id: u8) -> Result<Self, Error> {
        match self {
            Self::String(_) => Ok(self.clone()),
            Self::JSON(_) => {
                // Create a network definition from the passed network id.
                let network_definition: scrypto::core::NetworkDefinition =
                    network_definition_from_network_id(network_id);
                let bech32_encoder: scrypto::address::Bech32Encoder =
                    scrypto::address::Bech32Encoder::new(&network_definition);

                // Converting to a transaction manifest then decompiling the transaction manifest
                // to get a manifest string back
                let manifest_string: String = transaction::manifest::decompile(
                    &self.to_scrypto_transaction_manifest(network_id)?,
                    &bech32_encoder,
                )?;
                Ok(Self::String(manifest_string))
            }
        }
    }

    pub fn to_scrypto_transaction_manifest(
        &self,
        network_id: u8,
    ) -> Result<transaction::model::TransactionManifest, Error> {
        // Create a network definition from the passed network id.
        let network_definition: scrypto::core::NetworkDefinition =
            network_definition_from_network_id(network_id);
        let bech32_decoder: scrypto::address::Bech32Decoder =
            scrypto::address::Bech32Decoder::new(&network_definition);

        let manifest: transaction::model::TransactionManifest =
            transaction::manifest::generator::generate_manifest(
                &self.to_ast_instructions(network_id)?,
                &bech32_decoder,
            )
            .map_err(transaction::manifest::CompileError::GeneratorError)?;
        Ok(manifest)
    }

    pub fn from_scrypto_transaction_manifest(
        transaction_manifest: transaction::model::TransactionManifest,
        network_id: u8,
        manifest_kind: ManifestKind,
    ) -> Result<Self, Error> {
        // Converting the transaction manifest to a string and then converting it to the desired
        // type.
        let network_definition: scrypto::core::NetworkDefinition =
            network_definition_from_network_id(network_id);
        let bech32_encoder: scrypto::address::Bech32Encoder =
            scrypto::address::Bech32Encoder::new(&network_definition);

        let manifest_string: String =
            transaction::manifest::decompile(&transaction_manifest, &bech32_encoder)?;

        let manifest: Self = Self::String(manifest_string);
        manifest.to(manifest_kind, network_id)
    }
}
