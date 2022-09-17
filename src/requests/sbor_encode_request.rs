use crate::error::Error;
use crate::export_handler;
use crate::models::Value;
use crate::traits::Validate;
use serde::{Deserialize, Serialize};

// ==========================
// Request & Response Models
// ==========================

#[derive(Serialize, Deserialize, Clone)]
pub struct SBOREncodeRequest {
    #[serde(flatten)]
    pub value: Value,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SBOREncodeResponse {
    #[serde(with = "hex::serde")]
    pub encoded_value: Vec<u8>,
}

// ===========
// Validation
// ===========

impl Validate for SBOREncodeRequest {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

impl Validate for SBOREncodeResponse {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

// ========
// Handler
// ========

pub fn handle_sbor_encode(request: SBOREncodeRequest) -> Result<SBOREncodeResponse, Error> {
    let response: SBOREncodeResponse = SBOREncodeResponse {
        encoded_value: request.value.encode()?,
    };
    Ok(response)
}

export_handler!(handle_sbor_encode as sbor_encode);

// ======
// Tests
// ======

#[cfg(test)]
mod tests {
    // TODO: Unit tests for this request type
}
