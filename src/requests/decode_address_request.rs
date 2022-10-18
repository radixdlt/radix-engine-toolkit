use std::str::FromStr;

use bech32::{self, u5, FromBase32, Variant};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::error::Error;
use crate::export_request;
use crate::models::{Address, AddressKind};
use crate::traits::{Request, Validate};
use crate::utils::*;

// ==========================
// Request & Response Models
// ==========================

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DecodeAddressRequest {
    pub address: String,
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DecodeAddressResponse {
    pub network_id: u8,
    pub network_name: String,
    pub entity_type: AddressKind,
    #[serde_as(as = "serde_with::hex::Hex")]
    pub data: Vec<u8>,
    pub hrp: String,
    pub address: Address,
}

// ===========
// Validation
// ===========

impl Validate for DecodeAddressRequest {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

impl Validate for DecodeAddressResponse {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

// =======================
// Request Implementation
// =======================

impl<'r> Request<'r, DecodeAddressResponse> for DecodeAddressRequest {
    fn handle_request(self) -> Result<DecodeAddressResponse, Error> {
        // We need to deduce the network from the HRP of the passed address. Therefore, we need to
        // begin by decoding the address, and getting the HRP.
        let (hrp, data, variant): (String, Vec<u5>, Variant) = bech32::decode(&self.address)
            .map_err(scrypto::address::AddressError::Bech32mDecodingError)?;
        let data: Vec<u8> = Vec::<u8>::from_base32(&data)
            .map_err(scrypto::address::AddressError::Bech32mDecodingError)?;

        match variant {
            Variant::Bech32m => Ok(()),
            variant => Err(scrypto::address::AddressError::InvalidVariant(variant)),
        }?;

        let address: Address = Address::from_str(&self.address)?;
        let network_definition: scrypto::core::NetworkDefinition =
            network_definition_from_network_id(address.network_id());

        Ok(DecodeAddressResponse {
            network_id: network_definition.id,
            network_name: network_definition.logical_name,
            hrp,
            data,
            entity_type: address.kind(),
            address,
        })
    }
}

export_request!(DecodeAddressRequest as decode_address);

// ======
// Tests
// ======

#[cfg(test)]
mod tests {
    // TODO: Unit tests for this request type
}
