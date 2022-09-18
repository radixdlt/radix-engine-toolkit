use crate::error::Error;
use crate::export_handler;
use crate::models::{Address, AddressKind};
use crate::traits::Validate;
use crate::utils::*;
use bech32::{self, u5, FromBase32, Variant};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

// ==========================
// Request & Response Models
// ==========================

#[derive(Serialize, Deserialize, Clone)]
pub struct DecodeAddressRequest {
    pub address: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DecodeAddressResponse {
    pub network_id: u8,
    pub network_name: String,
    pub entity_type: AddressKind,
    #[serde(with = "hex::serde")]
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

// ========
// Handler
// ========

pub fn handle_decode_address(
    request: DecodeAddressRequest,
) -> Result<DecodeAddressResponse, Error> {
    // We need to deduce the network from the HRP of the passed address. Therefore, we need to begin
    // by decoding the address, and getting the HRP.
    let (hrp, data, variant): (String, Vec<u5>, Variant) =
        bech32::decode(&request.address).map_err(scrypto::address::AddressError::DecodingError)?;
    let data: Vec<u8> =
        Vec::<u8>::from_base32(&data).map_err(scrypto::address::AddressError::DecodingError)?;

    match variant {
        Variant::Bech32m => Ok(()),
        variant => Err(scrypto::address::AddressError::InvalidVariant(variant)),
    }?;

    let address: Address = Address::from_str(&request.address)?;
    let network_definition: scrypto::core::NetworkDefinition =
        network_definition_from_network_id(address.network_id());

    let response: DecodeAddressResponse = DecodeAddressResponse {
        network_id: network_definition.id,
        network_name: network_definition.logical_name,
        hrp,
        data,
        entity_type: address.kind(),
        address,
    };

    Ok(response)
}

export_handler!(handle_decode_address(DecodeAddressRequest) as decode_address);

// ======
// Tests
// ======

#[cfg(test)]
mod tests {
    // TODO: Unit tests for this request type
}
