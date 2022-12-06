use crate::error::Error;
use crate::model::Value;
use crate::traits::{Request, Validate};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

// ==========================
// Request & Response Models
// ==========================

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SBORDecodeRequest {
    #[serde_as(as = "serde_with::hex::Hex")]
    pub encoded_value: Vec<u8>,
    pub network_id: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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
        Ok(SBORDecodeResponse {
            value: Value::decode(&self.encoded_value, self.network_id)?,
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
