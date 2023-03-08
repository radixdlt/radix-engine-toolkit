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

use scrypto::prelude::{ComponentAddress, PublicKey};
use toolkit_derive::serializable;

use crate::error::Result;
use crate::model::address::EntityAddress;
use crate::model::address::NetworkAwareComponentAddress;
use crate::request::traits::Handler;

// =================
// Model Definition
// =================

/// Derives the virtual identity component address given a public key and a network id.
#[serializable]
pub struct DeriveVirtualIdentityAddressRequest {
    /// An unsigned 8 bit integer serialized as a string which represents the ID of the network
    /// that the address will be used on. The primary use of this is for any Bech32m encoding
    /// or decoding of addresses
    #[schemars(with = "String")]
    #[schemars(regex(pattern = "[0-9]+"))]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub network_id: u8,

    /// The public key to derive the virtual identity address for
    #[schemars(with = "crate::model::crypto::PublicKey")]
    #[serde_as(as = "serde_with::FromInto<crate::model::crypto::PublicKey>")]
    pub public_key: PublicKey,
}

/// The response form [`DeriveVirtualIdentityAddressRequest`] requests
#[serializable]
pub struct DeriveVirtualIdentityAddressResponse {
    /// The virtual identity component address serialized as a `ComponentAddress` from the `Value`
    /// model.
    #[schemars(with = "EntityAddress")]
    #[serde_as(as = "serde_with::TryFromInto<EntityAddress>")]
    pub virtual_identity_address: NetworkAwareComponentAddress,
}

// ===============
// Implementation
// ===============

pub struct DeriveVirtualIdentityAddressHandler;

impl Handler<DeriveVirtualIdentityAddressRequest, DeriveVirtualIdentityAddressResponse>
    for DeriveVirtualIdentityAddressHandler
{
    fn pre_process(
        request: DeriveVirtualIdentityAddressRequest,
    ) -> Result<DeriveVirtualIdentityAddressRequest> {
        Ok(request)
    }

    fn handle(
        request: &DeriveVirtualIdentityAddressRequest,
    ) -> Result<DeriveVirtualIdentityAddressResponse> {
        Ok(DeriveVirtualIdentityAddressResponse {
            virtual_identity_address: NetworkAwareComponentAddress {
                network_id: request.network_id,
                address: ComponentAddress::virtual_identity_from_public_key(&request.public_key),
            },
        })
    }

    fn post_process(
        _: &DeriveVirtualIdentityAddressRequest,
        response: DeriveVirtualIdentityAddressResponse,
    ) -> Result<DeriveVirtualIdentityAddressResponse> {
        Ok(response)
    }
}
