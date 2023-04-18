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

use bech32::FromBase32;
use radix_engine_common::crypto::EcdsaSecp256k1PublicKey;
use scrypto::prelude::{ComponentAddress, PublicKey};
use toolkit_derive::serializable;

use crate::error::{Error, Result};
use crate::model::address::NetworkAwareNodeId;
use crate::utils::checked_copy_u8_slice;

use super::traits::Handler;

// =================
// Model Definition
// =================

/// Given an Olympia account address, this converts it from an Olympia account address to a Babylon
/// ECDSA Secp256k1 virtual account address and reveals the underlying public key of the Olympia
/// account.
#[serializable]
pub struct DeriveBabylonAddressFromOlympiaAddressRequest {
    /// An unsigned 8 bit integer serialized as a string which represents the ID of the network
    /// that the address will be used on. The primary use of this is for any Bech32m encoding
    /// or decoding of addresses
    #[schemars(with = "String")]
    #[schemars(regex(pattern = "[0-9]+"))]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub network_id: u8,

    /// A string of the address on the Olympia network
    pub olympia_account_address: String,
}

/// The response form [`DeriveBabylonAddressFromOlympiaAddressRequest`] requests
#[serializable]
pub struct DeriveBabylonAddressFromOlympiaAddressResponse {
    /// The Babylon account address associated with the Olympia address.
    #[schemars(with = "String")]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub babylon_account_address: NetworkAwareNodeId,

    /// The public key associated with the Olympia account address.
    #[schemars(with = "crate::model::crypto::PublicKey")]
    #[serde_as(as = "serde_with::FromInto<crate::model::crypto::PublicKey>")]
    pub public_key: PublicKey,
}

// ===============
// Implementation
// ===============

pub struct DeriveBabylonAddressFromOlympiaAddressHandler;

impl
    Handler<
        DeriveBabylonAddressFromOlympiaAddressRequest,
        DeriveBabylonAddressFromOlympiaAddressResponse,
    > for DeriveBabylonAddressFromOlympiaAddressHandler
{
    fn pre_process(
        request: DeriveBabylonAddressFromOlympiaAddressRequest,
    ) -> Result<DeriveBabylonAddressFromOlympiaAddressRequest> {
        Ok(request)
    }

    fn handle(
        request: &DeriveBabylonAddressFromOlympiaAddressRequest,
    ) -> Result<DeriveBabylonAddressFromOlympiaAddressResponse> {
        // All Olympia addresses begin with a letter and then `d` `x`. Verify that the passed string
        // is of an Olympia account address
        if let (Some('d'), Some('x')) = (
            request.olympia_account_address.chars().nth(1),
            request.olympia_account_address.chars().nth(2),
        ) {
            Ok(())
        } else {
            Err(Error::NotAnOlympiaAddress {
                address: request.olympia_account_address.clone(),
            })
        }?;

        // Bech32 decode the passed address. If the Bech32 variant is not Bech32, then this is not
        // an Olympia address
        let (_, data, variant) =
            bech32::decode(&request.olympia_account_address).map_err(|error| {
                Error::AddressError {
                    message: format!("{:?}", error),
                }
            })?;
        if let bech32::Variant::Bech32 = variant {
            Ok(())
        } else {
            Err(Error::NotAnOlympiaAddress {
                address: request.olympia_account_address.clone(),
            })
        }?;

        // Convert from 5 bits to 8 bits.
        let mut data = Vec::<u8>::from_base32(&data).map_err(|error| Error::AddressError {
            message: format!("{:?}", error),
        })?;

        // Check the length of the data to ensure that it's a public key. Length should be 1 + 33
        // where the added 1 byte is because of the 0x04 prefix that public keys have.
        if data.len() != 34 || data.remove(0) != 4 {
            Err(Error::NotAnOlympiaAddress {
                address: request.olympia_account_address.clone(),
            })?;
        };

        // At this point, the data is of a valid Ecdsa Secp256k1 public key. We can now derive the
        // virtual account address associated with this public key.
        let public_key = EcdsaSecp256k1PublicKey(checked_copy_u8_slice(data)?);

        Ok(DeriveBabylonAddressFromOlympiaAddressResponse {
            babylon_account_address: NetworkAwareNodeId(
                ComponentAddress::virtual_account_from_public_key(&public_key)
                    .as_node_id()
                    .0,
                request.network_id,
            ),
            public_key: public_key.into(),
        })
    }

    fn post_process(
        _: &DeriveBabylonAddressFromOlympiaAddressRequest,
        response: DeriveBabylonAddressFromOlympiaAddressResponse,
    ) -> Result<DeriveBabylonAddressFromOlympiaAddressResponse> {
        Ok(response)
    }
}
