use crate::error::Error;
use crate::export_handler;
use crate::traits::Validate;
use scrypto::prelude::{NonFungibleAddress, PublicKey};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

// ==========================
// Request & Response Models
// ==========================

#[derive(Serialize, Deserialize, Clone)]
pub struct DeriveNonFungibleAddressFromPublicKeyRequest {
    pub public_key: PublicKey,
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone)]
pub struct DeriveNonFungibleAddressFromPublicKeyResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub non_fungible_address: NonFungibleAddress,
}

// ===========
// Validation
// ===========

impl Validate for DeriveNonFungibleAddressFromPublicKeyRequest {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

impl Validate for DeriveNonFungibleAddressFromPublicKeyResponse {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

// ========
// Handler
// ========

pub fn handle_derive_non_fungible_address_from_public_key(
    request: DeriveNonFungibleAddressFromPublicKeyRequest,
) -> Result<DeriveNonFungibleAddressFromPublicKeyResponse, Error> {
    let non_fungible_address: NonFungibleAddress =
        NonFungibleAddress::from_public_key(&request.public_key);
    let response: DeriveNonFungibleAddressFromPublicKeyResponse =
        DeriveNonFungibleAddressFromPublicKeyResponse {
            non_fungible_address: non_fungible_address,
        };
    Ok(response)
}

export_handler!(handle_derive_non_fungible_address_from_public_key(
    DeriveNonFungibleAddressFromPublicKeyRequest
) as derive_non_fungible_address_from_public_key);

// ======
// Tests
// ======

#[cfg(test)]
mod tests {
    // TODO: Unit tests for this request type
}
