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

use radix_engine_common::prelude::NetworkDefinition;
use scrypto::prelude::*;
use transaction::{builder::TransactionManifestV1, model::IntentV1};

pub fn manifest_from_intent(intent: &IntentV1) -> TransactionManifestV1 {
    let IntentV1 {
        instructions,
        blobs,
        ..
    } = intent;
    TransactionManifestV1 {
        instructions: instructions.0.clone(),
        blobs: blobs
            .blobs
            .iter()
            .map(|blob| (hash(&blob.0), blob.0.clone()))
            .collect(),
    }
}

pub fn network_definition_from_network_id(network_id: u8) -> NetworkDefinition {
    match network_id {
        // Public facing networks
        0x01 => NetworkDefinition::mainnet(),
        0x02 => NetworkDefinition {
            id: network_id,
            logical_name: "stokenet".to_string(),
            hrp_suffix: "tdx_2_".to_string(),
        },

        // Babylon Temporary Testnets
        0x0A => NetworkDefinition::adapanet(),
        0x0B => NetworkDefinition::nebunet(),
        0x0C => NetworkDefinition {
            id: network_id,
            logical_name: "kisharnet".to_string(),
            hrp_suffix: "tdx_c_".to_string(),
        },
        0x0D => NetworkDefinition {
            id: network_id,
            logical_name: "ansharnet".to_string(),
            hrp_suffix: "tdx_d_".to_string(),
        },

        // RDX Works Development
        0x20 => NetworkDefinition {
            id: 0x20,
            logical_name: "gilganet".to_string(),
            hrp_suffix: "tdx_20_".to_string(),
        },
        0x21 => NetworkDefinition {
            id: 0x21,
            logical_name: "enkinet".to_string(),
            hrp_suffix: "tdx_21_".to_string(),
        },
        0x22 => NetworkDefinition {
            id: 0x22,
            logical_name: "hammunet".to_string(),
            hrp_suffix: "tdx_22_".to_string(),
        },
        0x23 => NetworkDefinition {
            id: 0x23,
            logical_name: "nergalnet".to_string(),
            hrp_suffix: "tdx_23_".to_string(),
        },
        0x24 => NetworkDefinition {
            id: 0x24,
            logical_name: "mardunet".to_string(),
            hrp_suffix: "tdx_24_".to_string(),
        },
        0x25 => NetworkDefinition {
            id: 0x25,
            logical_name: "dumunet".to_string(),
            hrp_suffix: "tdx_25_".to_string(),
        },

        // Ephemeral Networks
        0xF0 => NetworkDefinition {
            id: 240,
            logical_name: "localnet".to_string(),
            hrp_suffix: "loc".to_string(),
        },
        0xF1 => NetworkDefinition {
            id: 241,
            logical_name: "inttestnet".to_string(),
            hrp_suffix: "test".to_string(),
        },
        0xF2 => NetworkDefinition::simulator(),

        // Unnamed
        network_id => NetworkDefinition {
            id: 0x25,
            logical_name: "unnamed".to_string(),
            hrp_suffix: format!("tdx_{:x}_", network_id),
        },
    }
}

pub fn is_account(node_id: &NodeId) -> bool {
    matches!(
        node_id.entity_type(),
        Some(
            EntityType::GlobalAccount
                | EntityType::InternalAccount
                | EntityType::GlobalVirtualSecp256k1Account
                | EntityType::GlobalVirtualEd25519Account
        )
    )
}

pub fn is_identity(node_id: &NodeId) -> bool {
    matches!(
        node_id.entity_type(),
        Some(
            EntityType::GlobalIdentity
                | EntityType::GlobalVirtualSecp256k1Identity
                | EntityType::GlobalVirtualEd25519Identity
        )
    )
}
