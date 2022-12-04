use crate::error::Error;
use crate::models::Address;
use crate::traits::{Request, Validate};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

// ==========================
// Request & Response Models
// ==========================

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EncodeAddressRequest {
    #[serde_as(as = "serde_with::hex::Hex")]
    pub address_bytes: Vec<u8>,

    pub network_id: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EncodeAddressResponse {
    #[serde(flatten)]
    address: Address,
}

// ===========
// Validation
// ===========

impl Validate for EncodeAddressRequest {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

impl Validate for EncodeAddressResponse {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

// =======================
// Request Implementation
// =======================

impl<'r> Request<'r, EncodeAddressResponse> for EncodeAddressRequest {
    fn handle_request(self) -> Result<EncodeAddressResponse, Error> {
        let address = &self.address_bytes;
        let address = Address::from_u8_array(address, self.network_id)?;

        Ok(EncodeAddressResponse { address })
    }
}

// ======
// Tests
// ======

#[cfg(test)]
mod tests {
    // TODO: Unit tests for this request type
}
