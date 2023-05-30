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

use crate::functions::traits::InvocationHandler;
use crate::model::address::NetworkAwareNodeId;
use scrypto::prelude::FAUCET;
use scrypto::prelude::{
    ACCOUNT_PACKAGE, CONSENSUS_MANAGER, ECDSA_SECP256K1_SIGNATURE_VIRTUAL_BADGE,
    EDDSA_ED25519_SIGNATURE_VIRTUAL_BADGE, FAUCET_PACKAGE, PACKAGE_OF_DIRECT_CALLER_VIRTUAL_BADGE,
    RADIX_TOKEN, SYSTEM_TRANSACTION_BADGE,
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
pub struct Input {
    /// An unsigned 8 bit integer serialized as a string which represents the ID of the network
    /// that the addresses will be used on. The primary use of this is for any Bech32m encoding
    /// or decoding of addresses
    #[schemars(with = "String")]
    #[schemars(regex(pattern = "[0-9]+"))]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub network_id: u8,
}

/// The response from [`Input`] requests
#[serializable]
pub struct Output {
    /// A component address serialized as an `Address` from the `Value` model which represents
    /// the address of the faucet component on the requested network.
    #[schemars(with = "Option<String>")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faucet_component_address: Option<NetworkAwareNodeId>,

    /// A package address serialized as an `Address` from the `Value` model which represents
    /// the address of the faucet package on the requested network.
    #[schemars(with = "Option<String>")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faucet_package_address: Option<NetworkAwareNodeId>,

    /// A package address serialized as an `Address` from the `Value` model which represents
    /// the address of the account package on the requested network.
    #[schemars(with = "String")]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub account_package_address: NetworkAwareNodeId,

    /// A resource address serialized as an `Address` from the `Value` model which
    /// represents the address of the XRD resource on the requested network.
    #[schemars(with = "String")]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub xrd_resource_address: NetworkAwareNodeId,

    /// A resource address serialized as an `Address` from the `Value` model which
    /// represents the address of the system resource on the requested network.
    #[schemars(with = "String")]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub system_token_resource_address: NetworkAwareNodeId,

    /// A resource address serialized as an `Address` from the `Value` model which
    /// represents the address of the Ecdsa Secp256k1 resource on the requested network.
    #[schemars(with = "String")]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub ecdsa_secp256k1_token_resource_address: NetworkAwareNodeId,

    /// A resource address serialized as an `Address` from the `Value` model which
    /// represents the address of the EdDSA Ed25519 resource on the requested network.
    #[schemars(with = "String")]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub eddsa_ed25519_token_resource_address: NetworkAwareNodeId,

    /// A resource address serialized as an `Address` from the `Value` model which
    /// represents the address of the package token resource on the requested network.
    #[schemars(with = "String")]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub package_token_resource_address: NetworkAwareNodeId,

    /// A system address serialized as an `Address` from the `Value` model which represents
    /// the address of the consensus manager on the requested network.
    #[schemars(with = "String")]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub consensus_manager_component_address: NetworkAwareNodeId,
}

// ===============
// Implementation
// ===============

pub struct Handler;
impl InvocationHandler<Input, Output> for Handler {
    type Error = Error;

    fn pre_process(input: Input) -> Result<Input, Error> {
        Ok(input)
    }

    fn handle(input: &Input) -> Result<Output, Error> {
        let network_id = input.network_id;
        Ok(Output {
            faucet_component_address: if network_id == 1 {
                None
            } else {
                Some(NetworkAwareNodeId(FAUCET.as_node_id().0, network_id))
            },
            faucet_package_address: if network_id == 1 {
                None
            } else {
                Some(NetworkAwareNodeId(
                    FAUCET_PACKAGE.as_node_id().0,
                    network_id,
                ))
            },
            account_package_address: NetworkAwareNodeId(ACCOUNT_PACKAGE.as_node_id().0, network_id),
            xrd_resource_address: NetworkAwareNodeId(RADIX_TOKEN.as_node_id().0, network_id),
            system_token_resource_address: NetworkAwareNodeId(
                SYSTEM_TRANSACTION_BADGE.as_node_id().0,
                network_id,
            ),
            ecdsa_secp256k1_token_resource_address: NetworkAwareNodeId(
                ECDSA_SECP256K1_SIGNATURE_VIRTUAL_BADGE.as_node_id().0,
                network_id,
            ),
            eddsa_ed25519_token_resource_address: NetworkAwareNodeId(
                EDDSA_ED25519_SIGNATURE_VIRTUAL_BADGE.as_node_id().0,
                network_id,
            ),
            package_token_resource_address: NetworkAwareNodeId(
                PACKAGE_OF_DIRECT_CALLER_VIRTUAL_BADGE.as_node_id().0,
                network_id,
            ),
            consensus_manager_component_address: NetworkAwareNodeId(
                CONSENSUS_MANAGER.as_node_id().0,
                network_id,
            ),
        })
    }

    fn post_process(_: &Input, output: Output) -> Result<Output, Error> {
        Ok(output)
    }
}

#[serializable]
pub struct Error;
