use crate::error::Error;
use crate::export_request;
use crate::traits::{Request, Validate};
use radix_engine::model::extract_abi as engine_extract_abi;
use scrypto::prelude::scrypto_encode;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::HashMap;

// ==========================
// Request & Response Models
// ==========================

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExtractAbiRequest {
    #[serde_as(as = "serde_with::hex::Hex")]
    pub package_wasm: Vec<u8>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExtractAbiResponse {
    #[serde_as(as = "serde_with::hex::Hex")]
    pub code: Vec<u8>,
    #[serde_as(as = "serde_with::hex::Hex")]
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

// =======================
// Request Implementation
// =======================

impl<'r> Request<'r, ExtractAbiResponse> for ExtractAbiRequest {
    fn handle_request(self) -> Result<ExtractAbiResponse, Error> {
        let abi: HashMap<String, radix_engine::types::BlueprintAbi> =
            engine_extract_abi(&self.package_wasm)?;

        Ok(ExtractAbiResponse {
            abi: scrypto_encode(&abi),
            code: self.package_wasm,
        })
    }
}

export_request!(ExtractAbiRequest as extract_abi);

// ======
// Tests
// ======

#[cfg(test)]
mod tests {
    // TODO: Unit tests for this request type
}
