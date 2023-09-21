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

use radix_engine::system::system_substates::{KeyValueEntrySubstate, KeyValueEntrySubstateV1};
use radix_engine::track::{BatchPartitionStateUpdate, NodeStateUpdates, PartitionStateUpdates};
use radix_engine_common::prelude::NetworkDefinition;
use radix_engine_queries::typed_substate_layout::{
    to_typed_substate_key, to_typed_substate_value, NonFungibleResourceManagerDataEntryPayload,
    NonFungibleResourceManagerDataEntrySubstate, NonFungibleResourceManagerDataKeyPayload,
    NonFungibleResourceManagerTypedSubstateKey, NonFungibleResourceManagerTypedSubstateValue,
    TypedMainModuleSubstateKey, TypedMainModuleSubstateValue, TypedMetadataModuleSubstateKey,
    TypedMetadataModuleSubstateValue, TypedSubstateKey, TypedSubstateValue, VersionedMetadataEntry,
};
use radix_engine_store_interface::interface::DatabaseUpdate;
use regex::Regex;
use sbor::{generate_full_schema_from_single_type, validate_payload_against_schema};
use scrypto::{api::node_modules::metadata::MetadataValue, prelude::*};
use transaction::model::IntentV1;
use transaction::prelude::{DynamicGlobalAddress, TransactionManifestV1};

use crate::functions::execution::ExecutionAnalysisTransactionReceipt;
use crate::models::node_id::{InvalidEntityTypeIdError, TypedNodeId};

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
            if let Some(network_id_string) = numeric_network_specifier.split('_').nth(1) {
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
    let (local_type_id, VersionedSchema::V1(schema)) =
        generate_full_schema_from_single_type::<S, ScryptoCustomSchema>();
    let encoded_payload = manifest_encode(&value).map_err(|_| ())?;
    validate_payload_against_schema::<ManifestCustomExtension, _>(
        &encoded_payload,
        &schema,
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
                        | EntityType::GlobalVirtualSecp256k1Account
                        | EntityType::GlobalVirtualEd25519Account
                )
            )
        }
    }
}

pub fn is_validator<A: Into<DynamicGlobalAddress> + Clone>(node_id: &A) -> bool {
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

pub fn is_access_controller<A: Into<DynamicGlobalAddress> + Clone>(node_id: &A) -> bool {
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
                        | EntityType::GlobalVirtualSecp256k1Identity
                        | EntityType::GlobalVirtualEd25519Identity
                )
            )
        }
    }
}

pub fn metadata_of_newly_created_entities(
    receipt: &ExecutionAnalysisTransactionReceipt,
) -> Result<HashMap<GlobalAddress, HashMap<String, Option<MetadataValue>>>, InvalidEntityTypeIdError>
{
    let addresses = addresses_of_newly_created_entities(receipt)?;
    let mut map = HashMap::<GlobalAddress, HashMap<String, Option<MetadataValue>>>::new();
    for typed_node_id in addresses.into_iter() {
        let Ok(global_address) = GlobalAddress::try_from(typed_node_id) else {
            // Ignore all addresses that are not of global entities.
            continue;
        };

        let entry = map.entry(global_address).or_default();
        if let Some(NodeStateUpdates::Delta { by_partition }) = receipt
            .expect_commit_success()
            .state_updates
            .by_node
            .get(global_address.as_node_id())
        {
            let entries = match by_partition.get(&METADATA_BASE_PARTITION) {
                Some(PartitionStateUpdates::Delta { by_substate }) => by_substate
                    .iter()
                    .filter_map(|(key, value)| match value {
                        DatabaseUpdate::Set(value) => Some((key.clone(), value.clone())),
                        DatabaseUpdate::Delete => None,
                    })
                    .collect::<IndexMap<_, _>>(),
                Some(PartitionStateUpdates::Batch(BatchPartitionStateUpdate::Reset {
                    new_substate_values,
                })) => new_substate_values.clone(),
                None => continue,
            };

            for (substate_key, data) in entries.into_iter() {
                if let Ok((
                    TypedSubstateKey::MetadataModule(key),
                    TypedSubstateValue::MetadataModule(value),
                )) = to_typed_substate_key(
                    global_address.as_node_id().entity_type().unwrap(),
                    METADATA_BASE_PARTITION,
                    &substate_key,
                )
                .and_then(|typed_substate_key| {
                    to_typed_substate_value(&typed_substate_key, &data)
                        .map(|typed_substate_value| (typed_substate_key, typed_substate_value))
                }) {
                    let TypedMetadataModuleSubstateKey::MetadataEntryKey(key) = key;
                    let value = match value {
                        TypedMetadataModuleSubstateValue::MetadataEntry(
                            KeyValueEntrySubstate::V1(KeyValueEntrySubstateV1 { value, .. }),
                        ) => value,
                    };
                    entry.insert(
                        key,
                        value.map(|metadata_entry| {
                            let VersionedMetadataEntry::V1(metadata) = metadata_entry.content;
                            metadata
                        }),
                    );
                }
            }
        }
    }

    Ok(map)
}

pub fn addresses_of_newly_created_entities(
    receipt: &ExecutionAnalysisTransactionReceipt,
) -> Result<HashSet<TypedNodeId>, InvalidEntityTypeIdError> {
    let commit_result = receipt.commit_result();
    commit_result
        .new_component_addresses()
        .iter()
        .map(|address| address.into_node_id())
        .chain(
            commit_result
                .new_package_addresses()
                .iter()
                .map(|address| address.into_node_id()),
        )
        .chain(
            commit_result
                .new_resource_addresses()
                .iter()
                .map(|address| address.into_node_id()),
        )
        .map(TypedNodeId::new)
        .collect::<Result<HashSet<_>, _>>()
}

pub fn data_of_newly_minted_non_fungibles(
    receipt: &ExecutionAnalysisTransactionReceipt,
) -> HashMap<ResourceAddress, HashMap<NonFungibleLocalId, ScryptoValue>> {
    let mut map = HashMap::<ResourceAddress, HashMap<NonFungibleLocalId, ScryptoValue>>::new();
    let commit_result = receipt.expect_commit_success();

    for (node_id, node_state_update) in commit_result.state_updates.by_node.iter() {
        if !node_id.entity_type().map_or(false, |entity_type| {
            entity_type.is_global_non_fungible_resource_manager()
        }) {
            continue;
        }

        let NodeStateUpdates::Delta { by_partition } = node_state_update;

        let entries: IndexMap<(PartitionNumber, SubstateKey), Vec<u8>> = by_partition
            .iter()
            .flat_map(
                |(partition_number, partition_state_update)| match partition_state_update {
                    PartitionStateUpdates::Delta { by_substate } => by_substate
                        .iter()
                        .filter_map(|(key, value)| match value {
                            DatabaseUpdate::Set(value) => {
                                Some(((*partition_number, key.clone()), value.clone()))
                            }
                            DatabaseUpdate::Delete => None,
                        })
                        .collect::<IndexMap<(PartitionNumber, SubstateKey), Vec<u8>>>(),
                    PartitionStateUpdates::Batch(BatchPartitionStateUpdate::Reset {
                        new_substate_values,
                    }) => new_substate_values
                        .iter()
                        .map(|(substate_key, data)| {
                            ((*partition_number, substate_key.clone()), data.clone())
                        })
                        .collect(),
                },
            )
            .collect();

        for ((partition_number, substate_key), data) in entries {
            if let Ok((
                TypedSubstateKey::MainModule(
                    TypedMainModuleSubstateKey::NonFungibleResourceManager(
                        NonFungibleResourceManagerTypedSubstateKey::DataKeyValueEntry(
                            NonFungibleResourceManagerDataKeyPayload {
                                content: non_fungible_local_id,
                            },
                        ),
                    ),
                ),
                TypedSubstateValue::MainModule(
                    TypedMainModuleSubstateValue::NonFungibleResourceManager(
                        NonFungibleResourceManagerTypedSubstateValue::DataKeyValue(
                            NonFungibleResourceManagerDataEntrySubstate::V1(
                                KeyValueEntrySubstateV1 {
                                    value:
                                        Some(NonFungibleResourceManagerDataEntryPayload {
                                            content: non_fungible_data,
                                        }),
                                    ..
                                },
                            ),
                        ),
                    ),
                ),
            )) = to_typed_substate_key(
                node_id.entity_type().unwrap(),
                partition_number,
                &substate_key,
            )
            .and_then(|typed_substate_key| {
                to_typed_substate_value(&typed_substate_key, &data)
                    .map(|typed_substate_value| (typed_substate_key, typed_substate_value))
            }) {
                let resource_address = ResourceAddress::new_or_panic(node_id.0);
                let non_fungible_local_id = non_fungible_local_id;
                let non_fungible_data =
                    scrypto_decode::<ScryptoValue>(&scrypto_encode(&non_fungible_data).unwrap())
                        .unwrap();

                map.entry(resource_address)
                    .or_default()
                    .insert(non_fungible_local_id, non_fungible_data);
            }
        }
    }

    map
}
