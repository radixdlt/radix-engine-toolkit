// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use crate::error::Result;
use crate::model::address::{EntityAddress, EntityType};
use crate::request::Handler;
use crate::utils::network_definition_from_network_id;
use bech32::{self, FromBase32, Variant};
use scrypto::radix_engine_interface::address::AddressError;
use serializable::serializable;

// =================
// Model Definition
// =================

/// This request can be used to decode a Bech32m encoded address string into its equivalent hrp and
/// data. In addition to that, this request provides other useful information on the address such
/// as the network id and name that it is used for, and the entity type of the address.
#[serializable]
pub struct DecodeAddressRequest {
    /// A string of the Bech32m encoded address to decode. Decoding this address will expose its
    /// entity type, network id, network name, underlying data, as well as it's Bech32m HRP.
    pub address: String,
}

#[serializable]
pub struct DecodeAddressResponse {
    /// An 8 bit unsigned integer serialized as a string which represents the id of the network
    /// that this address exists on. This is derived from the HRP of the Bech32m encoded
    /// address.
    #[schemars(with = "String")]
    #[schemars(regex(pattern = "[0-9]+"))]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub network_id: u8,

    /// A string which represents the name of the network that this address exists on. This is
    /// derived from the HRP of the Bech32m encoded address.
    pub network_name: String,

    /// An [`EntityType`] enum representing the type of entity addressed with the passed address.
    /// This is derived from the entity byte on the address data.
    pub entity_type: EntityType,

    /// A byte array of 27 bytes (54 hex characters) serialized as a hex string which represents
    /// the data encoded in the address.
    #[schemars(with = "String")]
    #[schemars(length(equal = 54))]
    #[schemars(regex(pattern = "[0-9a-fA-F]+"))]
    #[serde_as(as = "serde_with::hex::Hex")]
    pub data: Vec<u8>,

    /// A string which represents the Bech32m Human Readable Part (HRP) of the passed address
    /// string
    pub hrp: String,
}

// ===============
// Implementation
// ===============

pub struct DecodeAddressHandler;

impl Handler<DecodeAddressRequest, DecodeAddressResponse> for DecodeAddressHandler {
    fn pre_process(request: DecodeAddressRequest) -> Result<DecodeAddressRequest> {
        Ok(request)
    }

    fn handle(request: &DecodeAddressRequest) -> Result<DecodeAddressResponse> {
        // We need to deduce the network from the HRP of the passed address. Therefore, we need to
        // begin by decoding the address, and getting the HRP.
        let (hrp, data, variant) =
            bech32::decode(&request.address).map_err(AddressError::Bech32mDecodingError)?;
        let data = Vec::<u8>::from_base32(&data).map_err(AddressError::Bech32mDecodingError)?;

        match variant {
            Variant::Bech32m => Ok(()),
            variant => Err(AddressError::InvalidVariant(variant)),
        }?;

        let address = request.address.parse::<EntityAddress>()?;
        let network_definition = network_definition_from_network_id(address.network_id());

        Ok(DecodeAddressResponse {
            network_id: network_definition.id,
            network_name: network_definition.logical_name,
            hrp,
            data,
            entity_type: address.kind().into(),
        })
    }

    fn post_process(
        _: &DecodeAddressRequest,
        response: DecodeAddressResponse,
    ) -> DecodeAddressResponse {
        response
    }
}
