use crate::error::Error;
use crate::export_request;
use crate::models::Value;
use crate::traits::{Request, Validate};
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

// =======================
// Request Implementation
// =======================

impl<'r> Request<'r, SBOREncodeResponse> for SBOREncodeRequest {
    fn handle_request(self) -> Result<SBOREncodeResponse, Error> {
        let response: SBOREncodeResponse = SBOREncodeResponse {
            encoded_value: self.value.encode()?,
        };
        Ok(response)
    }
}

export_request!(SBOREncodeRequest as sbor_encode);

// ======
// Tests
// ======

#[cfg(test)]
mod tests {
    // TODO: Unit tests for this request type
}
