use crate::error::Error;
use crate::export_request;
use crate::models::Address;
use crate::traits::{Request, Validate};
use serde::{Deserialize, Serialize};

// ==========================
// Request & Response Models
// ==========================

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EncodeAddressRequest {
    #[serde(with = "hex::serde")]
    pub address: Vec<u8>,

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
        let address: &[u8] = &self.address;
        let address: Address = Address::from_u8_array(address, self.network_id)?;
        let response: EncodeAddressResponse = EncodeAddressResponse { address };
        Ok(response)
    }
}

export_request!(EncodeAddressRequest as encode_address);

// ======
// Tests
// ======

#[cfg(test)]
mod tests {
    // TODO: Unit tests for this request type
}
