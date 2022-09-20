use crate::error::Error;
use crate::export_request;
use crate::traits::{Request, Validate};
use scrypto::prelude::{NonFungibleAddress, NonFungibleId, ResourceAddress};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

// ==========================
// Request & Response Models
// ==========================

#[serde_as]
#[derive(Serialize, Deserialize, Clone)]
pub struct DeriveNonFungibleAddressRequest {
    #[serde_as(as = "DisplayFromStr")]
    pub resource_address: ResourceAddress,
    #[serde_as(as = "DisplayFromStr")]
    pub non_fungible_id: NonFungibleId,
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone)]
pub struct DeriveNonFungibleAddressResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub non_fungible_address: NonFungibleAddress,
}

// ===========
// Validation
// ===========

impl Validate for DeriveNonFungibleAddressRequest {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

impl Validate for DeriveNonFungibleAddressResponse {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

// =======================
// Request Implementation
// =======================

impl<'r> Request<'r, DeriveNonFungibleAddressResponse> for DeriveNonFungibleAddressRequest {
    fn handle_request(self) -> Result<DeriveNonFungibleAddressResponse, Error> {
        let non_fungible_address: NonFungibleAddress =
            NonFungibleAddress::new(self.resource_address, self.non_fungible_id);
        let response: DeriveNonFungibleAddressResponse = DeriveNonFungibleAddressResponse {
            non_fungible_address: non_fungible_address,
        };
        Ok(response)
    }
}

export_request!(DeriveNonFungibleAddressRequest as derive_non_fungible_address);

// ======
// Tests
// ======

#[cfg(test)]
mod tests {
    // TODO: Unit tests for this request type
}
