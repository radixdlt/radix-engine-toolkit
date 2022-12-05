// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at

//   http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use bech32::{self, FromBase32, Variant};

use scrypto::radix_engine_interface::address::AddressError;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::error::Error;
use crate::model::{AddressKind, EntityAddress};
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
    pub address: EntityAddress,
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
        let (hrp, data, variant) =
            bech32::decode(&self.address).map_err(AddressError::Bech32mDecodingError)?;
        let data = Vec::<u8>::from_base32(&data).map_err(AddressError::Bech32mDecodingError)?;

        match variant {
            Variant::Bech32m => Ok(()),
            variant => Err(AddressError::InvalidVariant(variant)),
        }?;

        let address = self.address.parse::<EntityAddress>()?;
        let network_definition = network_definition_from_network_id(address.network_id());

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

// ======
// Tests
// ======

#[cfg(test)]
mod tests {
    // TODO: Unit tests for this request type
}
