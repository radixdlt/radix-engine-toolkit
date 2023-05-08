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

use super::AddressError;

use bech32;
use regex::Regex;

use radix_engine_common::types::EntityType;
use scrypto::network::NetworkDefinition;
use scrypto::prelude::NodeId;

/// A deterministic function that generates a network definition given a network ID. Implemented
/// with reference to https://github.com/radixdlt/babylon-node/tree/main/common/src/main/java/com/radixdlt/networks/Network.java#L72-L99
pub fn network_definition_from_network_id(network_id: u8) -> NetworkDefinition {
    match network_id {
        0x01 => NetworkDefinition::mainnet(),
        0x02 => NetworkDefinition {
            id: network_id,
            logical_name: "stokenet".to_owned(),
            hrp_suffix: format!("tdx_{:x}_", network_id),
        },

        0x0A => NetworkDefinition {
            id: network_id,
            logical_name: "adapanet".to_owned(),
            hrp_suffix: format!("tdx_{:x}_", network_id),
        },
        0x0B => NetworkDefinition {
            id: network_id,
            logical_name: "nebunet".to_owned(),
            hrp_suffix: format!("tdx_{:x}_", network_id),
        },

        0x20 => NetworkDefinition {
            id: network_id,
            logical_name: "gilganet".to_owned(),
            hrp_suffix: format!("tdx_{:x}_", network_id),
        },
        0x21 => NetworkDefinition {
            id: network_id,
            logical_name: "enkinet".to_owned(),
            hrp_suffix: format!("tdx_{:x}_", network_id),
        },
        0x22 => NetworkDefinition {
            id: network_id,
            logical_name: "hammunet".to_owned(),
            hrp_suffix: format!("tdx_{:x}_", network_id),
        },
        0x23 => NetworkDefinition {
            id: network_id,
            logical_name: "nergalnet".to_owned(),
            hrp_suffix: format!("tdx_{:x}_", network_id),
        },
        0x24 => NetworkDefinition {
            id: network_id,
            logical_name: "mardunet".to_owned(),
            hrp_suffix: format!("tdx_{:x}_", network_id),
        },

        0xF0 => NetworkDefinition {
            id: network_id,
            logical_name: "localnet".to_owned(),
            hrp_suffix: "loc".to_owned(),
        },
        0xF1 => NetworkDefinition {
            id: network_id,
            logical_name: "inttestnet".to_owned(),
            hrp_suffix: "test".to_owned(),
        },
        0xF2 => NetworkDefinition::simulator(),

        _ => NetworkDefinition {
            id: network_id,
            logical_name: "unnamed".to_owned(),
            hrp_suffix: format!("tdx_{:x}_", network_id),
        },
    }
}

pub fn network_id_from_hrp<S: AsRef<str>>(hrp: S) -> Result<u8, AddressError> {
    // Getting the network specifier from the given HRP. Bech32 HRPs used in Babylon are structured
    // as follows:
    // TODO: Better errors and remove unwraps
    let network_specifier = {
        let re = Regex::new("_(sim|loc|rdx|test|tdx_[A-Fa-f0-9]{1,2}_)$")
            .expect("Failed to create Regex. Must panic");
        re.captures(hrp.as_ref())
            .and_then(|captures| captures.get(1))
            .map(|capture| capture.as_str().trim_end_matches('_'))
            .map_or(
                Err(AddressError::NoNetworkSpecifierMatchesFoundInHrp {
                    hrp: hrp.as_ref().to_owned(),
                }),
                Ok,
            )
    }?;

    // Matching the network specifier to obtain the network id from it
    match network_specifier {
        "rdx" => Ok(0x01),
        "loc" => Ok(0xF0),
        "test" => Ok(0xF1),
        "sim" => Ok(0xF2),
        numeric_network_specifier => {
            if let Some(network_id_string) = numeric_network_specifier.split('_').nth(1) {
                u8::from_str_radix(network_id_string, 16)
                    .map_err(|_| AddressError::FailedToExtractNetworkId {
                        hrp: hrp.as_ref().to_owned(),
                        network_specifier: network_specifier.to_owned(),
                        network_id_string: Some(network_id_string.to_owned()),
                    })
                    .map(Ok)
            } else {
                Err(AddressError::FailedToExtractNetworkId {
                    hrp: hrp.as_ref().to_owned(),
                    network_specifier: network_specifier.to_owned(),
                    network_id_string: None,
                })
            }
        }?,
    }
}

pub fn network_id_from_address_string<S: AsRef<str>>(address: S) -> Result<u8, AddressError> {
    // Attempt to Bech32m decode this address to get the hrp and the data type (will not be used).
    // The decoding process also yields a variant. We will not be verifying that this is bech32m
    // since this method is not meant to be a validation method.
    let (hrp, _, _) =
        bech32::decode(address.as_ref()).map_err(|_| AddressError::Bech32DecodeError {
            address: address.as_ref().to_string(),
        })?;
    network_id_from_hrp(hrp)
}

pub fn is_account<A: Into<NodeId>>(node_id: A) -> bool {
    node_id.into().entity_type().map_or(false, |entity_type| {
        matches!(
            entity_type,
            EntityType::GlobalAccount
                | EntityType::GlobalVirtualEcdsaAccount
                | EntityType::GlobalVirtualEddsaAccount
                | EntityType::InternalAccount
        )
    })
}

pub fn is_identity<A: Into<NodeId>>(node_id: A) -> bool {
    node_id.into().entity_type().map_or(false, |entity_type| {
        matches!(
            entity_type,
            EntityType::GlobalIdentity
                | EntityType::GlobalVirtualEcdsaIdentity
                | EntityType::GlobalVirtualEddsaIdentity
        )
    })
}
