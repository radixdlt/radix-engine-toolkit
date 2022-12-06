use crate::error::Error;
use crate::model::Value;
use crate::traits::{Request, Validate};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

// ==========================
// Request & Response Models
// ==========================

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SBOREncodeRequest {
    #[serde(flatten)]
    pub value: Value,
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SBOREncodeResponse {
    #[serde_as(as = "serde_with::hex::Hex")]
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
        Ok(SBOREncodeResponse {
            encoded_value: self.value.encode()?,
        })
    }
}

// ======
// Tests
// ======

#[cfg(test)]
mod tests {
    // TODO: Unit tests for this request type
}
