use crate::error::Error;
use crate::models::helper::ValueSerializationProxy;
use crate::models::NonFungibleAddress;
use crate::traits::{Request, Validate};

use scrypto::prelude::PublicKey;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

// ==========================
// Request & Response Models
// ==========================

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeriveNonFungibleAddressFromPublicKeyRequest {
    pub public_key: PublicKey,
    pub network_id: u8,
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeriveNonFungibleAddressFromPublicKeyResponse {
    #[serde_as(as = "ValueSerializationProxy")]
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

// =======================
// Request Implementation
// =======================

impl<'r> Request<'r, DeriveNonFungibleAddressFromPublicKeyResponse>
    for DeriveNonFungibleAddressFromPublicKeyRequest
{
    fn handle_request(self) -> Result<DeriveNonFungibleAddressFromPublicKeyResponse, Error> {
        let non_fungible_address =
            NonFungibleAddress::from_public_key(&self.public_key, self.network_id);

        Ok(DeriveNonFungibleAddressFromPublicKeyResponse {
            non_fungible_address,
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
