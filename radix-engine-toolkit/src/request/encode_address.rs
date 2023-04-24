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

use crate::model::address::NetworkAwareNodeId;
use crate::request::traits::Handler;
use crate::utils::checked_copy_u8_slice;

use radix_engine::types::NodeId;
use toolkit_derive::serializable;

// =================
// Model Definition
// =================

/// This request can be used when we have a byte array which we wish to do Bech32m encoding on. In
/// this case, the HRP to use will be determined through the entity byte of the passed address hex
/// string.
#[serializable]
pub struct EncodeAddressRequest {
    /// A byte array of 27 bytes (54 hex characters) serialized as a hex string of the data part of
    /// the address.
    #[schemars(with = "String")]
    #[schemars(length(equal = 54))]
    #[schemars(regex(pattern = "[0-9a-fA-F]+"))]
    #[serde_as(as = "serde_with::hex::Hex")]
    pub address_bytes: Vec<u8>,

    /// An 8 bit unsigned integer serialized as a string which represents the id of the network
    /// that this address exists on.
    #[schemars(with = "String")]
    #[schemars(regex(pattern = "[0-9]+"))]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub network_id: u8,
}

/// The response from [`EncodeAddressRequest`].
#[serializable]
pub struct EncodeAddressResponse {
    /// A discriminated union of entity addresses where addresses are serialized as a Bech32m
    /// encoded string.
    #[schemars(with = "String")]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub address: NetworkAwareNodeId,
}

// ===============
// Implementation
// ===============

pub struct EncodeAddressHandler;

impl Handler<EncodeAddressRequest, EncodeAddressResponse> for EncodeAddressHandler {
    type Error = EncodeAddressError;

    fn pre_process(
        request: EncodeAddressRequest,
    ) -> Result<EncodeAddressRequest, EncodeAddressError> {
        Ok(request)
    }

    fn handle(request: &EncodeAddressRequest) -> Result<EncodeAddressResponse, EncodeAddressError> {
        checked_copy_u8_slice(&request.address_bytes).map_or(
            Err(EncodeAddressError::InvalidLength {
                expected: NodeId::LENGTH,
                actual: request.address_bytes.len(),
            }),
            |address| {
                Ok(EncodeAddressResponse {
                    address: NetworkAwareNodeId(address, request.network_id),
                })
            },
        )
    }

    fn post_process(
        _: &EncodeAddressRequest,
        response: EncodeAddressResponse,
    ) -> Result<EncodeAddressResponse, EncodeAddressError> {
        Ok(response)
    }
}

#[serializable]
#[serde(tag = "type")]
pub enum EncodeAddressError {
    /// An error emitted when the length of the passed data is invalid
    InvalidLength { expected: usize, actual: usize },
}
