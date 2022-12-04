use crate::error::Error;
use crate::model::helper::ValueSerializationProxy;
use crate::traits::{Request, Validate};

use scrypto::prelude::{FromPublicKey, NonFungibleAddress, PublicKey};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

// ==========================
// Request & Response Models
// ==========================

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeriveNonFungibleAddressFromPublicKeyRequest {
    #[serde(flatten)]
    pub public_key: PublicKey,
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
        let non_fungible_address = NonFungibleAddress::from_public_key(&self.public_key);

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
