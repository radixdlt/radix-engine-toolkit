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

use crate::error::Error;
use crate::model::helper::ValueSerializationProxy;
use crate::model::{
    NetworkAwareComponentAddress, NetworkAwarePackageAddress, NetworkAwareResourceAddress,
    NetworkAwareSystemAddress,
};
use crate::traits::{Request, Validate};
use scrypto::prelude::{
    ACCOUNT_PACKAGE, CLOCK, ECDSA_SECP256K1_TOKEN, EDDSA_ED25519_TOKEN, EPOCH_MANAGER,
    FAUCET_COMPONENT, FAUCET_PACKAGE, RADIX_TOKEN, SYSTEM_TOKEN,
};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

// ==========================
// Request & Response Models
// ==========================

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct KnownEntityAddressesRequest {
    pub network_id: u8,
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct KnownEntityAddressesResponse {
    #[serde_as(as = "ValueSerializationProxy")]
    faucet_component_address: NetworkAwareComponentAddress,

    #[serde_as(as = "ValueSerializationProxy")]
    faucet_package_address: NetworkAwarePackageAddress,

    #[serde_as(as = "ValueSerializationProxy")]
    account_package_address: NetworkAwarePackageAddress,

    #[serde_as(as = "ValueSerializationProxy")]
    xrd_resource_address: NetworkAwareResourceAddress,

    #[serde_as(as = "ValueSerializationProxy")]
    system_token_resource_address: NetworkAwareResourceAddress,

    #[serde_as(as = "ValueSerializationProxy")]
    ecdsa_secp256k1_token_resource_address: NetworkAwareResourceAddress,

    #[serde_as(as = "ValueSerializationProxy")]
    eddsa_ed25519_token_resource_address: NetworkAwareResourceAddress,

    #[serde_as(as = "ValueSerializationProxy")]
    epoch_manager_system_address: NetworkAwareSystemAddress,

    #[serde_as(as = "ValueSerializationProxy")]
    clock_system_address: NetworkAwareSystemAddress,
}

// ===========
// Validation
// ===========

impl Validate for KnownEntityAddressesRequest {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

impl Validate for KnownEntityAddressesResponse {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

// =======================
// Request Implementation
// =======================

impl<'r> Request<'r, KnownEntityAddressesResponse> for KnownEntityAddressesRequest {
    fn handle_request(self) -> Result<KnownEntityAddressesResponse, Error> {
        let network_id = self.network_id;
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
}
