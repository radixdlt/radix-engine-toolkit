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

use radix_common::prelude::NetworkDefinition;

use regex::Regex;
use sbor::{
    generate_full_schema_from_single_type, validate_payload_against_schema,
};

use radix_common::prelude::*;
use radix_transactions::model::IntentV1;
use radix_transactions::prelude::*;
use scrypto::prelude::*;

pub fn manifest_from_intent(intent: &IntentV1) -> TransactionManifestV1 {
    let IntentV1 {
        instructions,
        blobs,
        ..
    } = intent;
    TransactionManifestV1 {
        instructions: Vec::clone(&instructions.0),
        blobs: blobs
            .blobs
            .iter()
            .map(|blob| (hash(&blob.0), blob.0.clone()))
            .collect(),
        object_names: Default::default(),
    }
}

pub fn network_definition_from_network_id(network_id: u8) -> NetworkDefinition {
    match network_id {
        // Public facing networks
        0x01 => NetworkDefinition::mainnet(),
        0x02 => NetworkDefinition {
            id: network_id,
            logical_name: "stokenet".into(),
            hrp_suffix: "tdx_2_".into(),
        },

        // Babylon Temporary Testnets
        0x0A => NetworkDefinition::adapanet(),
        0x0B => NetworkDefinition::nebunet(),
        0x0C => NetworkDefinition {
            id: network_id,
            logical_name: "kisharnet".into(),
            hrp_suffix: "tdx_c_".into(),
        },
        0x0D => NetworkDefinition {
            id: network_id,
            logical_name: "ansharnet".into(),
            hrp_suffix: "tdx_d_".into(),
        },
        0x0E => NetworkDefinition {
            id: network_id,
            logical_name: "ansharnet".into(),
            hrp_suffix: "tdx_e_".into(),
        },

        // RDX Works Development
        0x20 => NetworkDefinition {
            id: 0x20,
            logical_name: "gilganet".into(),
            hrp_suffix: "tdx_20_".into(),
        },
        0x21 => NetworkDefinition {
            id: 0x21,
            logical_name: "enkinet".into(),
            hrp_suffix: "tdx_21_".into(),
        },
        0x22 => NetworkDefinition {
            id: 0x22,
            logical_name: "hammunet".into(),
            hrp_suffix: "tdx_22_".into(),
        },
        0x23 => NetworkDefinition {
            id: 0x23,
            logical_name: "nergalnet".into(),
            hrp_suffix: "tdx_23_".into(),
        },
        0x24 => NetworkDefinition {
            id: 0x24,
            logical_name: "mardunet".into(),
            hrp_suffix: "tdx_24_".into(),
        },
        0x25 => NetworkDefinition {
            id: 0x25,
            logical_name: "dumunet".into(),
            hrp_suffix: "tdx_25_".into(),
        },

        // Ephemeral Networks
        0xF0 => NetworkDefinition {
            id: 240,
            logical_name: "localnet".into(),
            hrp_suffix: "loc".into(),
        },
        0xF1 => NetworkDefinition {
            id: 241,
            logical_name: "inttestnet".into(),
            hrp_suffix: "test".into(),
        },
        0xF2 => NetworkDefinition::simulator(),

        // Unnamed
        network_id => NetworkDefinition {
            id: network_id,
            logical_name: "unnamed".into(),
            hrp_suffix: format!("tdx_{:x}_", network_id).into(),
        },
    }
}

pub fn network_id_from_hrp<S: AsRef<str>>(hrp: S) -> Option<u8> {
    let network_specifier = {
        let re = Regex::new("_(sim|loc|rdx|test|tdx_[A-Fa-f0-9]{1,2}_)$")
            .expect("Failed to create Regex. Must panic");
        re.captures(hrp.as_ref())
            .and_then(|captures| captures.get(1))
            .map(|capture| capture.as_str().trim_end_matches('_'))
    };

    match network_specifier {
        Some("rdx") => Some(0x01),
        Some("loc") => Some(0xF0),
        Some("test") => Some(0xF1),
        Some("sim") => Some(0xF2),
        Some(numeric_network_specifier) => {
            if let Some(network_id_string) =
                numeric_network_specifier.split('_').nth(1)
            {
                if let Ok(num) = u8::from_str_radix(network_id_string, 16) {
                    Some(num)
                } else {
                    None
                }
            } else {
                None
            }
        }
        None => None,
    }
}

pub fn network_id_from_address_string<S: AsRef<str>>(address: S) -> Option<u8> {
    if let Ok((hrp, ..)) = bech32::decode(address.as_ref()) {
        network_id_from_hrp(hrp)
    } else {
        None
    }
}

pub fn to_manifest_type<D: ManifestDecode>(value: &ManifestValue) -> Option<D> {
    manifest_encode(value)
        .ok()
        .and_then(|encoded| manifest_decode(&encoded).ok())
}

// TODO: This should return a `bool`.
#[allow(clippy::result_unit_err)]
pub fn validate_manifest_value_against_schema<S: ScryptoDescribe>(
    value: &ManifestValue,
) -> Result<(), ()> {
    let (local_type_id, schema) =
        generate_full_schema_from_single_type::<S, ScryptoCustomSchema>();
    let encoded_payload = manifest_encode(&value).map_err(|_| ())?;
    validate_payload_against_schema::<ManifestCustomExtension, _>(
        &encoded_payload,
        schema.v1(),
        local_type_id,
        &(),
        SCRYPTO_SBOR_V1_MAX_DEPTH,
    )
    .map_err(|_| ())
}

pub fn is_account<A: Into<DynamicGlobalAddress> + Clone>(node_id: &A) -> bool {
    match node_id.clone().into() {
        DynamicGlobalAddress::Named(_) => false,
        DynamicGlobalAddress::Static(address) => {
            matches!(
                address.as_node_id().entity_type(),
                Some(
                    EntityType::GlobalAccount
                        | EntityType::GlobalPreallocatedSecp256k1Account
                        | EntityType::GlobalPreallocatedEd25519Account
                )
            )
        }
    }
}

pub fn is_validator<A: Into<DynamicGlobalAddress> + Clone>(
    node_id: &A,
) -> bool {
    match node_id.clone().into() {
        DynamicGlobalAddress::Named(_) => false,
        DynamicGlobalAddress::Static(address) => {
            matches!(
                address.as_node_id().entity_type(),
                Some(EntityType::GlobalValidator)
            )
        }
    }
}

pub fn is_access_controller<A: Into<DynamicGlobalAddress> + Clone>(
    node_id: &A,
) -> bool {
    match node_id.clone().into() {
        DynamicGlobalAddress::Named(_) => false,
        DynamicGlobalAddress::Static(address) => {
            matches!(
                address.as_node_id().entity_type(),
                Some(EntityType::GlobalAccessController)
            )
        }
    }
}

pub fn is_identity<A: Into<DynamicGlobalAddress> + Clone>(node_id: &A) -> bool {
    match node_id.clone().into() {
        DynamicGlobalAddress::Named(_) => false,
        DynamicGlobalAddress::Static(address) => {
            matches!(
                address.as_node_id().entity_type(),
                Some(
                    EntityType::GlobalIdentity
                        | EntityType::GlobalPreallocatedSecp256k1Identity
                        | EntityType::GlobalPreallocatedEd25519Identity
                )
            )
        }
    }
}

#[macro_export]
macro_rules! contains {
    (
        $item: expr =>
        [
            $($other: expr),* $(,)?
        ] $(,)?
    ) => {
        $(
            $item == $other
        )||*
    };
}
