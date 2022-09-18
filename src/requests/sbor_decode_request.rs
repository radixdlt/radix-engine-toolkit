use crate::error::Error;
use crate::export_handler;
use crate::models::Value;
use crate::traits::Validate;
use serde::{Deserialize, Serialize};

// ==========================
// Request & Response Models
// ==========================

#[derive(Serialize, Deserialize, Clone)]
pub struct SBORDecodeRequest {
    #[serde(with = "hex::serde")]
    pub encoded_value: Vec<u8>,
    pub network_id: u8,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SBORDecodeResponse {
    #[serde(flatten)]
    pub value: Value,
}

// ===========
// Validation
// ===========

impl Validate for SBORDecodeRequest {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

impl Validate for SBORDecodeResponse {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

// ========
// Handler
// ========

pub fn handle_sbor_decode(request: SBORDecodeRequest) -> Result<SBORDecodeResponse, Error> {
    let response: SBORDecodeResponse = SBORDecodeResponse {
        value: Value::decode(&request.encoded_value, request.network_id)?,
    };
    Ok(response)
}

export_handler!(handle_sbor_decode(SBORDecodeRequest) as sbor_decode);

// ======
// Tests
// ======

#[cfg(test)]
mod tests {
    // TODO: Unit tests for this request type
}
