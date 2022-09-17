use crate::error::Error;
use crate::export_handler;
use crate::models::Address;
use crate::traits::Validate;
use serde::{Deserialize, Serialize};

// ==========================
// Request & Response Models
// ==========================

#[derive(Serialize, Deserialize, Clone)]
pub struct EncodeAddressRequest {
    #[serde(with = "hex::serde")]
    pub address: Vec<u8>,

    pub network_id: u8,
}

#[derive(Serialize, Deserialize, Clone)]
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

// ========
// Handler
// ========

pub fn handle_encode_address(
    request: EncodeAddressRequest,
) -> Result<EncodeAddressResponse, Error> {
    let address: &[u8] = &request.address;
    let address: Address = Address::from_u8_array(address, request.network_id)?;
    let response: EncodeAddressResponse = EncodeAddressResponse { address };
    Ok(response)
}

export_handler!(handle_encode_address as encode_address);

// ======
// Tests
// ======

#[cfg(test)]
mod tests {
    // TODO: Unit tests for this request type
}
