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

use radix_engine::system::system::DynSubstate;
use radix_engine::transaction::TransactionReceipt;
use radix_engine_common::prelude::NetworkDefinition;
use radix_engine_queries::typed_substate_layout::{
    to_typed_substate_key, to_typed_substate_value, TypedMainModuleSubstateKey,
    TypedMainModuleSubstateValue, TypedSubstateKey, TypedSubstateValue,
};
use radix_engine_store_interface::interface::DatabaseUpdate;
use sbor::{generate_full_schema_from_single_type, validate_payload_against_schema};
use scrypto::{api::node_modules::metadata::MetadataValue, prelude::*};
use transaction::{builder::TransactionManifestV1, model::IntentV1, prelude::DynamicGlobalAddress};

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

pub fn to_manifest_type<D: ManifestDecode>(value: &ManifestValue) -> Option<D> {
    manifest_encode(value)
        .ok()
        .and_then(|encoded| manifest_decode(&encoded).ok())
}

#[allow(clippy::result_unit_err)]
pub fn validate_manifest_value_against_schema<S: ScryptoDescribe>(
    value: &ManifestValue,
) -> Result<(), ()> {
    let (local_type_index, schema) =
        generate_full_schema_from_single_type::<S, ScryptoCustomSchema>();
    let encoded_payload = manifest_encode(&value).unwrap();
    validate_payload_against_schema::<ManifestCustomExtension, _>(
        &encoded_payload,
        &schema,
        local_type_index,
        &(),
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
                        | EntityType::InternalAccount
                        | EntityType::GlobalVirtualSecp256k1Account
                        | EntityType::GlobalVirtualEd25519Account
                )
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
                        | EntityType::GlobalVirtualSecp256k1Identity
                        | EntityType::GlobalVirtualEd25519Identity
                )
            )
        }
    }
}

pub fn metadata_of_newly_created_entities(
    receipt: &TransactionReceipt,
) -> Option<HashMap<GlobalAddress, HashMap<String, MetadataValue>>> {
    if !receipt.is_commit_success() {
        return None;
    }

    let mut map = HashMap::<GlobalAddress, HashMap<String, MetadataValue>>::new();
    let commit_success = receipt.expect_commit_success();
    for global_address in commit_success
        .new_component_addresses()
        .iter()
        .map(|address| address.into_node_id())
        .chain(
            commit_success
                .new_package_addresses()
                .iter()
                .map(|address| address.into_node_id()),
        )
        .chain(
            commit_success
                .new_resource_addresses()
                .iter()
                .map(|address| address.into_node_id()),
        )
        .map(|node_id| GlobalAddress::new_or_panic(node_id.0))
    {
        if let Some(key_update_map) = commit_success
            .state_updates
            .system_updates
            .get(&(*global_address.as_node_id(), METADATA_KV_STORE_PARTITION))
        {
            for (substate_key, database_update) in key_update_map.iter() {
                if let DatabaseUpdate::Set(data) = database_update {
                    if let Ok((
                        TypedSubstateKey::MetadataModuleEntryKey(key),
                        TypedSubstateValue::MetadataModuleEntryValue(DynSubstate { value, .. }),
                    )) = to_typed_substate_key(
                        global_address.as_node_id().entity_type().unwrap(),
                        METADATA_KV_STORE_PARTITION,
                        substate_key,
                    )
                    .and_then(|typed_substate_key| {
                        to_typed_substate_value(&typed_substate_key, data)
                            .map(|typed_substate_value| (typed_substate_key, typed_substate_value))
                    }) {
                        map.entry(global_address)
                            .or_default()
                            .insert(key, value.unwrap());
                    }
                } else {
                    continue;
                }
            }
        }
    }

    Some(map)
}

pub fn data_of_newly_minted_non_fungibles(
    receipt: &TransactionReceipt,
) -> Option<HashMap<ResourceAddress, HashMap<NonFungibleLocalId, ScryptoValue>>> {
    if !receipt.is_commit_success() {
        return None;
    }

    let mut map = HashMap::<ResourceAddress, HashMap<NonFungibleLocalId, ScryptoValue>>::new();
    let commit_success = receipt.expect_commit_success();

    for ((node_id, partition_number), database_update_map) in
        commit_success.state_updates.system_updates.iter()
    {
        // Only care about non-fungible resource manager nodes, ignore everything else
        if !node_id.entity_type().map_or(false, |entity_type| {
            entity_type.is_global_non_fungible_resource_manager()
        }) {
            continue;
        }

        for (substate_key, database_update) in database_update_map {
            if let DatabaseUpdate::Set(data) = database_update {
                if let Ok((
                    TypedSubstateKey::MainModule(
                        TypedMainModuleSubstateKey::NonFungibleResourceData(non_fungible_local_id),
                    ),
                    TypedSubstateValue::MainModule(
                        TypedMainModuleSubstateValue::NonFungibleResourceData(DynSubstate {
                            value: non_fungible_data,
                            ..
                        }),
                    ),
                )) = to_typed_substate_key(
                    node_id.entity_type().unwrap(),
                    *partition_number,
                    substate_key,
                )
                .and_then(|typed_substate_key| {
                    to_typed_substate_value(&typed_substate_key, data)
                        .map(|typed_substate_value| (typed_substate_key, typed_substate_value))
                }) {
                    let resource_address = ResourceAddress::new_or_panic(node_id.0);
                    let non_fungible_local_id = non_fungible_local_id;
                    let non_fungible_data = scrypto_decode::<ScryptoValue>(
                        &scrypto_encode(&non_fungible_data.unwrap()).unwrap(),
                    )
                    .unwrap();

                    map.entry(resource_address)
                        .or_default()
                        .insert(non_fungible_local_id, non_fungible_data);
                }
            } else {
                continue;
            }
        }
    }

    Some(map)
}
