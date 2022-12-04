use crate::error::Error;
use crate::model::serde::ValueSerializationProxy;
use crate::model::NetworkAwareComponentAddress;
use crate::traits::{Request, Validate};

use scrypto::prelude::{ComponentAddress, PublicKey};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

// ==========================
// Request & Response Models
// ==========================

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeriveVirtualAccountAddressRequest {
    pub network_id: u8,

    pub public_key: PublicKey,
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeriveVirtualAccountAddressResponse {
    #[serde_as(as = "ValueSerializationProxy")]
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

// ======
// Tests
// ======

#[cfg(test)]
mod tests {
    // TODO: Unit tests for this request type
}
