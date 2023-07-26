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
use crate::error::{Error, Result};
use crate::model::address::{EntityAddress, NetworkAwareResourceAddress};
use bech32::FromBase32;
use radix_engine_common::address::AddressError;
use toolkit_derive::serializable;

// =================
// Model Definition
// =================

/// Given an Olympia account address, this converts it from an Olympia account address to a Babylon
/// ECDSA Secp256k1 virtual account address and reveals the underlying public key of the Olympia
/// account.
#[serializable]
pub struct DeriveBabylonResourceAddressFromOlympiaResourceAddressRequest {
    /// An unsigned 8 bit integer serialized as a string which represents the ID of the network
    /// that the address will be used on. The primary use of this is for any Bech32m encoding
    /// or decoding of addresses
    #[schemars(with = "String")]
    #[schemars(regex(pattern = "[0-9]+"))]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub network_id: u8,

    /// A string of the address on the Olympia network
    pub olympia_resource_address: String,
}

/// The response form [`DeriveBabylonResourceAddressFromOlympiaResourceAddressRequest`] requests
#[serializable]
pub struct DeriveBabylonResourceAddressFromOlympiaResourceAddressResponse {
    /// The Babylon account address associated with the Olympia address.
    #[schemars(with = "EntityAddress")]
    #[serde_as(as = "serde_with::TryFromInto<EntityAddress>")]
    pub babylon_resource_address: NetworkAwareResourceAddress,
}

// ===============
// Implementation
// ===============

pub struct DeriveBabylonResourceAddressFromOlympiaResourceAddressHandler;

impl
    Handler<
        DeriveBabylonResourceAddressFromOlympiaResourceAddressRequest,
        DeriveBabylonResourceAddressFromOlympiaResourceAddressResponse,
    > for DeriveBabylonResourceAddressFromOlympiaResourceAddressHandler
{
    fn pre_process(
        request: DeriveBabylonResourceAddressFromOlympiaResourceAddressRequest,
    ) -> Result<DeriveBabylonResourceAddressFromOlympiaResourceAddressRequest> {
        Ok(request)
    }

    fn handle(
        request: &DeriveBabylonResourceAddressFromOlympiaResourceAddressRequest,
    ) -> Result<DeriveBabylonResourceAddressFromOlympiaResourceAddressResponse> {
        // Bech32 decode the passed address. If the Bech32 variant is not Bech32, then this is not
        // an Olympia address
        let (_, data, variant) = bech32::decode(&request.olympia_resource_address)
            .map_err(AddressError::Bech32mDecodingError)?;
        if let bech32::Variant::Bech32 = variant {
            Ok(())
        } else {
            Err(Error::NotAnOlympiaAddress {
                address: request.olympia_resource_address.clone(),
            })
        }?;

        // Convert from 5 bits to 8 bits.
        let mut data = Vec::<u8>::from_base32(&data).map_err(AddressError::Bech32mDecodingError)?;

        // Check the length of the data to ensure that it's valid.
        if data.is_empty() {
            Err(Error::NotAnOlympiaAddress {
                address: request.olympia_resource_address.clone(),
            })?;
        }

        let prefix = data.remove(0);
        let length = data.len();

        let resource_address = match (prefix, length) {
            (0x01, 0) => Ok(scrypto::prelude::XRD),
            (0x03, 26) => Ok(scrypto::prelude::ResourceAddress::Fungible(
                data.try_into().unwrap(),
            )),
            _ => Err(Error::NotAnOlympiaAddress {
                address: request.olympia_resource_address.clone(),
            }),
        }?;

        Ok(
            DeriveBabylonResourceAddressFromOlympiaResourceAddressResponse {
                babylon_resource_address: NetworkAwareResourceAddress {
                    address: resource_address,
                    network_id: request.network_id,
                },
            },
        )
    }

    fn post_process(
        _: &DeriveBabylonResourceAddressFromOlympiaResourceAddressRequest,
        response: DeriveBabylonResourceAddressFromOlympiaResourceAddressResponse,
    ) -> Result<DeriveBabylonResourceAddressFromOlympiaResourceAddressResponse> {
        Ok(response)
    }
}
