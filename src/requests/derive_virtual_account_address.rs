use crate::error::Error;
use crate::export_request;
use crate::models::NetworkAwareComponentAddress;
use crate::traits::{Request, Validate};
use scrypto::prelude::{ComponentAddress, PublicKey};
use serde::{Deserialize, Serialize};
// ==========================
// Request & Response Models
// ==========================

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeriveVirtualAccountAddressRequest {
    pub network_id: u8,

    pub public_key: PublicKey,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeriveVirtualAccountAddressResponse {
    pub virtual_account_address: NetworkAwareComponentAddress,
}

// ===========
// Validation
// ===========

impl Validate for DeriveVirtualAccountAddressRequest {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

impl Validate for DeriveVirtualAccountAddressResponse {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

// =======================
// Request Implementation
// =======================

impl<'r> Request<'r, DeriveVirtualAccountAddressResponse> for DeriveVirtualAccountAddressRequest {
    fn handle_request(self) -> Result<DeriveVirtualAccountAddressResponse, Error> {
        Ok(DeriveVirtualAccountAddressResponse {
            virtual_account_address: NetworkAwareComponentAddress {
                network_id: self.network_id,
                address: ComponentAddress::virtual_account_from_public_key(&self.public_key),
            },
        })
    }
}

export_request!(DeriveVirtualAccountAddressRequest as derive_non_fungible_address_from_public_key);

// ======
// Tests
// ======

#[cfg(test)]
mod tests {
    // TODO: Unit tests for this request type
}
