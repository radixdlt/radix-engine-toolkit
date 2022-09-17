use crate::error::Error;
use crate::export_handler;
use crate::traits::Validate;
use radix_engine::model::extract_abi as engine_extract_abi;
use scrypto::prelude::scrypto_encode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ==========================
// Request & Response Models
// ==========================

#[derive(Serialize, Deserialize, Clone)]
pub struct ExtractAbiRequest {
    #[serde(with = "hex::serde")]
    pub package_wasm: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ExtractAbiResponse {
    #[serde(with = "hex::serde")]
    pub code: Vec<u8>,
    #[serde(with = "hex::serde")]
    pub abi: Vec<u8>,
}

// ===========
// Validation
// ===========

impl Validate for ExtractAbiRequest {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

impl Validate for ExtractAbiResponse {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

// ========
// Handler
// ========

pub fn handle_extract_abi(request: ExtractAbiRequest) -> Result<ExtractAbiResponse, Error> {
    let abi: HashMap<String, radix_engine::types::BlueprintAbi> =
        engine_extract_abi(&request.package_wasm)?;
    let response: ExtractAbiResponse = ExtractAbiResponse {
        abi: scrypto_encode(&abi),
        code: request.package_wasm,
    };
    Ok(response)
}

export_handler!(handle_extract_abi as extract_abi);

// ======
// Tests
// ======

#[cfg(test)]
mod tests {
    // TODO: Unit tests for this request type
}
