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

use super::traits::Handler;
use crate::model::address::utils::network_definition_from_network_id;
use crate::model::address::EntityType;
use crate::model::address::NetworkAwareNodeId;
use crate::utils::debug_string;
use scrypto::prelude::NodeId;
use toolkit_derive::serializable;

use bech32::{self};

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
    #[schemars(with = "String")]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub address: NetworkAwareNodeId,
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

    /// A byte array of 30 bytes (60 hex characters) serialized as a hex string which represents
    /// the data encoded in the address.
    #[schemars(with = "String")]
    #[schemars(length(equal = 60))]
    #[schemars(regex(pattern = "[0-9a-fA-F]+"))]
    #[serde_as(as = "serde_with::hex::Hex")]
    pub data: [u8; NodeId::LENGTH],
}

// ===============
// Implementation
// ===============

pub struct DecodeAddressHandler;

impl Handler<DecodeAddressRequest, DecodeAddressResponse> for DecodeAddressHandler {
    type Error = DecodeAddressError;

    fn pre_process(
        request: DecodeAddressRequest,
    ) -> Result<DecodeAddressRequest, DecodeAddressError> {
        Ok(request)
    }

    fn handle(request: &DecodeAddressRequest) -> Result<DecodeAddressResponse, DecodeAddressError> {
        let network_definition = network_definition_from_network_id(request.address.network_id());

        Ok(DecodeAddressResponse {
            network_id: network_definition.id,
            network_name: network_definition.logical_name,
            data: request.address.0,
            entity_type: request.address.node_id().entity_type().map_or(
                Err(DecodeAddressError::NoCorrespondingEntityType),
                |entity_type| Ok(entity_type.into()),
            )?,
        })
    }

    fn post_process(
        _: &DecodeAddressRequest,
        response: DecodeAddressResponse,
    ) -> Result<DecodeAddressResponse, DecodeAddressError> {
        Ok(response)
    }
}

#[serializable]
pub enum DecodeAddressError {
    /// An error emitted when a Bech32 operation fails.
    Bech32Error { message: String },

    /// An error emitted when an unexpected Bech32 variant is encountered. In this case, it means
    /// that we expected Bech32m but encountered Bech32
    InvalidBech32Variant,

    /// An error emitted when the address does not have a corresponding entity type
    NoCorrespondingEntityType,
}

impl From<bech32::Error> for DecodeAddressError {
    fn from(value: bech32::Error) -> Self {
        Self::Bech32Error {
            message: debug_string(value),
        }
    }
}
