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
use crate::request::Handler;
use crate::{
    NetworkAwareComponentAddress, NetworkAwarePackageAddress, NetworkAwareResourceAddress,
    NetworkAwareSystemAddress,
};
use scrypto::prelude::{
    ACCOUNT_PACKAGE, CLOCK, ECDSA_SECP256K1_TOKEN, EDDSA_ED25519_TOKEN, EPOCH_MANAGER,
    FAUCET_COMPONENT, FAUCET_PACKAGE, RADIX_TOKEN, SYSTEM_TOKEN,
};
use serializable::serializable;

// =================
// Model Definition
// =================

#[serializable]
pub struct KnownEntityAddressesRequest {
    #[schemars(with = "String")]
    #[schemars(regex(pattern = "[0-9]+"))]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub network_id: u8,
}

#[serializable]
pub struct KnownEntityAddressesResponse {
    #[schemars(with = "crate::model::value::Value")]
    #[serde_as(as = "serde_with::TryFromInto<crate::model::value::Value>")]
    faucet_component_address: NetworkAwareComponentAddress,

    #[schemars(with = "crate::model::value::Value")]
    #[serde_as(as = "serde_with::TryFromInto<crate::model::value::Value>")]
    faucet_package_address: NetworkAwarePackageAddress,

    #[schemars(with = "crate::model::value::Value")]
    #[serde_as(as = "serde_with::TryFromInto<crate::model::value::Value>")]
    account_package_address: NetworkAwarePackageAddress,

    #[schemars(with = "crate::model::value::Value")]
    #[serde_as(as = "serde_with::TryFromInto<crate::model::value::Value>")]
    xrd_resource_address: NetworkAwareResourceAddress,

    #[schemars(with = "crate::model::value::Value")]
    #[serde_as(as = "serde_with::TryFromInto<crate::model::value::Value>")]
    system_token_resource_address: NetworkAwareResourceAddress,

    #[schemars(with = "crate::model::value::Value")]
    #[serde_as(as = "serde_with::TryFromInto<crate::model::value::Value>")]
    ecdsa_secp256k1_token_resource_address: NetworkAwareResourceAddress,

    #[schemars(with = "crate::model::value::Value")]
    #[serde_as(as = "serde_with::TryFromInto<crate::model::value::Value>")]
    eddsa_ed25519_token_resource_address: NetworkAwareResourceAddress,

    #[schemars(with = "crate::model::value::Value")]
    #[serde_as(as = "serde_with::TryFromInto<crate::model::value::Value>")]
    epoch_manager_system_address: NetworkAwareSystemAddress,

    #[schemars(with = "crate::model::value::Value")]
    #[serde_as(as = "serde_with::TryFromInto<crate::model::value::Value>")]
    clock_system_address: NetworkAwareSystemAddress,
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
            epoch_manager_system_address: NetworkAwareSystemAddress {
                address: EPOCH_MANAGER,
                network_id,
            },
            clock_system_address: NetworkAwareSystemAddress {
                address: CLOCK,
                network_id,
            },
        })
    }

    fn post_process(
        _: &KnownEntityAddressesRequest,
        response: KnownEntityAddressesResponse,
    ) -> KnownEntityAddressesResponse {
        response
    }
}
