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
use crate::model::address::{
    NetworkAwareComponentAddress, NetworkAwarePackageAddress, NetworkAwareResourceAddress,
};
use crate::request::traits::Handler;
use scrypto::prelude::{
    ACCOUNT_PACKAGE, CLOCK, ECDSA_SECP256K1_TOKEN, EDDSA_ED25519_TOKEN, EPOCH_MANAGER,
    FAUCET_COMPONENT, FAUCET_PACKAGE, PACKAGE_TOKEN, RADIX_TOKEN, SYSTEM_TOKEN,
};
use toolkit_derive::serializable;

// =================
// Model Definition
// =================

/// Given a network id, this function derives the Bech32m-encoded addresses of the set of known
/// addresses.     
/// As an example, this function allows users to derive the XRD resource address, faucet component
/// address, or account package address on any network (given that they know its network id).
#[serializable]
pub struct KnownEntityAddressesRequest {
    /// An unsigned 8 bit integer serialized as a string which represents the ID of the network
    /// that the addresses will be used on. The primary use of this is for any Bech32m encoding
    /// or decoding of addresses
    #[schemars(with = "String")]
    #[schemars(regex(pattern = "[0-9]+"))]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub network_id: u8,
}

/// The response from [`KnownEntityAddressesRequest`] requests
#[serializable]
pub struct KnownEntityAddressesResponse {
    /// A component address serialized as a `ComponentAddress` from the `Value` model which
    /// represents the address of the faucet component on the requested network.
    #[schemars(with = "crate::model::value::ast::ManifestAstValue")]
    #[serde_as(as = "serde_with::TryFromInto<crate::model::value::ast::ManifestAstValue>")]
    pub faucet_component_address: NetworkAwareComponentAddress,

    /// A package address serialized as a `PackageAddress` from the `Value` model which represents
    /// the address of the faucet package on the requested network.
    #[schemars(with = "crate::model::value::ast::ManifestAstValue")]
    #[serde_as(as = "serde_with::TryFromInto<crate::model::value::ast::ManifestAstValue>")]
    pub faucet_package_address: NetworkAwarePackageAddress,

    /// A package address serialized as a `PackageAddress` from the `Value` model which represents
    /// the address of the account package on the requested network.
    #[schemars(with = "crate::model::value::ast::ManifestAstValue")]
    #[serde_as(as = "serde_with::TryFromInto<crate::model::value::ast::ManifestAstValue>")]
    pub account_package_address: NetworkAwarePackageAddress,

    /// A resource address serialized as a `ResourceAddress` from the `Value` model which
    /// represents the address of the XRD resource on the requested network.
    #[schemars(with = "crate::model::value::ast::ManifestAstValue")]
    #[serde_as(as = "serde_with::TryFromInto<crate::model::value::ast::ManifestAstValue>")]
    pub xrd_resource_address: NetworkAwareResourceAddress,

    /// A resource address serialized as a `ResourceAddress` from the `Value` model which
    /// represents the address of the system resource on the requested network.
    #[schemars(with = "crate::model::value::ast::ManifestAstValue")]
    #[serde_as(as = "serde_with::TryFromInto<crate::model::value::ast::ManifestAstValue>")]
    pub system_token_resource_address: NetworkAwareResourceAddress,

    /// A resource address serialized as a `ResourceAddress` from the `Value` model which
    /// represents the address of the Ecdsa Secp256k1 resource on the requested network.
    #[schemars(with = "crate::model::value::ast::ManifestAstValue")]
    #[serde_as(as = "serde_with::TryFromInto<crate::model::value::ast::ManifestAstValue>")]
    pub ecdsa_secp256k1_token_resource_address: NetworkAwareResourceAddress,

    /// A resource address serialized as a `ResourceAddress` from the `Value` model which
    /// represents the address of the EdDSA Ed25519 resource on the requested network.
    #[schemars(with = "crate::model::value::ast::ManifestAstValue")]
    #[serde_as(as = "serde_with::TryFromInto<crate::model::value::ast::ManifestAstValue>")]
    pub eddsa_ed25519_token_resource_address: NetworkAwareResourceAddress,

    /// A resource address serialized as a `ResourceAddress` from the `Value` model which
    /// represents the address of the package token resource on the requested network.
    #[schemars(with = "crate::model::value::ast::ManifestAstValue")]
    #[serde_as(as = "serde_with::TryFromInto<crate::model::value::ast::ManifestAstValue>")]
    pub package_token_resource_address: NetworkAwareResourceAddress,

    /// A system address serialized as a `ComponentAddress` from the `Value` model which represents
    /// the address of the epoch manager on the requested network.
    #[schemars(with = "crate::model::value::ast::ManifestAstValue")]
    #[serde_as(as = "serde_with::TryFromInto<crate::model::value::ast::ManifestAstValue>")]
    pub epoch_manager_system_address: NetworkAwareComponentAddress,

    /// A system address serialized as a `ComponentAddress` from the `Value` model which represents
    /// the address of the clock on the requested network.
    #[schemars(with = "crate::model::value::ast::ManifestAstValue")]
    #[serde_as(as = "serde_with::TryFromInto<crate::model::value::ast::ManifestAstValue>")]
    pub clock_system_address: NetworkAwareComponentAddress,
}

// ===============
// Implementation
// ===============

pub struct KnownEntityAddressesHandler;

impl Handler<KnownEntityAddressesRequest, KnownEntityAddressesResponse>
    for KnownEntityAddressesHandler
{
    fn pre_process(request: KnownEntityAddressesRequest) -> Result<KnownEntityAddressesRequest> {
        Ok(request)
    }

    fn handle(request: &KnownEntityAddressesRequest) -> Result<KnownEntityAddressesResponse> {
        let network_id = request.network_id;
        Ok(KnownEntityAddressesResponse {
            faucet_component_address: NetworkAwareComponentAddress {
                address: FAUCET_COMPONENT,
                network_id,
            },
            faucet_package_address: NetworkAwarePackageAddress {
                address: FAUCET_PACKAGE,
                network_id,
            },
            account_package_address: NetworkAwarePackageAddress {
                address: ACCOUNT_PACKAGE,
                network_id,
            },
            xrd_resource_address: NetworkAwareResourceAddress {
                address: RADIX_TOKEN,
                network_id,
            },
            system_token_resource_address: NetworkAwareResourceAddress {
                address: SYSTEM_TOKEN,
                network_id,
            },
            ecdsa_secp256k1_token_resource_address: NetworkAwareResourceAddress {
                address: ECDSA_SECP256K1_TOKEN,
                network_id,
            },
            eddsa_ed25519_token_resource_address: NetworkAwareResourceAddress {
                address: EDDSA_ED25519_TOKEN,
                network_id,
            },
            package_token_resource_address: NetworkAwareResourceAddress {
                address: PACKAGE_TOKEN,
                network_id,
            },
            epoch_manager_system_address: NetworkAwareComponentAddress {
                address: EPOCH_MANAGER,
                network_id,
            },
            clock_system_address: NetworkAwareComponentAddress {
                address: CLOCK,
                network_id,
            },
        })
    }

    fn post_process(
        _: &KnownEntityAddressesRequest,
        response: KnownEntityAddressesResponse,
    ) -> Result<KnownEntityAddressesResponse> {
        Ok(response)
    }
}
