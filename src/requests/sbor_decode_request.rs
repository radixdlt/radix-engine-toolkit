use crate::error::Error;
use crate::export_request;
use crate::models::Value;
use crate::traits::{Request, Validate};
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

// =======================
// Request Implementation
// =======================

impl<'r> Request<'r, SBORDecodeResponse> for SBORDecodeRequest {
    fn handle_request(self) -> Result<SBORDecodeResponse, Error> {
        let response: SBORDecodeResponse = SBORDecodeResponse {
            value: Value::decode(&self.encoded_value, self.network_id)?,
        };
        Ok(response)
    }
}

export_request!(SBORDecodeRequest as sbor_decode);

// ======
// Tests
// ======

#[cfg(test)]
mod tests {
    // TODO: Unit tests for this request type
}
