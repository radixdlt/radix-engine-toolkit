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

use crate::internal_prelude::*;

// I don't want to add this to the internal prelude because I don't want the
// core::hash::Hash and radix_common::prelude::Hash types to have a conflict.
use core::hash::Hash;

/// An abstraction over the [`RuntimeToolkitTransactionReceipt`] type that is
/// made specifically for use in manifest analysis and classification. Accessing
/// data on this receipt is safe since it's been checked to have come from a
/// transaction that was committed successfully.
///
/// # Note
///
/// We decided not to store referenced to data in this type since in in the
/// pipeline that manifests go through for classification there is only one
/// use for the receipt and it's to create an analysis transaction receipt. Thus
/// the receipt will be converted into a manifest transaction receipt without
/// the need for clones or references.
pub struct AnalysisTransactionReceipt {
    /// The state updates summary from the transaction.
    state_updates_summary:
        radix_engine_toolkit_common::receipt::StateUpdatesSummary<
            RuntimeTypeSelector,
        >,
    /// The instruction-by-instruction worktop updates that took place in the
    /// transaction.
    worktop_changes: WorktopChanges,
    /// The summary of the fees paid in the transaction.
    fee_summary:
        radix_engine_toolkit_common::receipt::FeeSummary<RuntimeTypeSelector>,
    /// Information about the fees locked by the transaction.
    locked_fees:
        radix_engine_toolkit_common::receipt::LockedFees<RuntimeTypeSelector>,
}

impl AnalysisTransactionReceipt {
    pub fn new(receipt: RuntimeToolkitTransactionReceipt) -> Option<Self> {
        if let RuntimeToolkitTransactionReceipt::CommitSuccess {
            state_updates_summary,
            worktop_changes,
            fee_summary,
            locked_fees,
        } = receipt
        {
            Some(Self {
                state_updates_summary,
                worktop_changes: worktop_changes.into(),
                fee_summary,
                locked_fees,
            })
        } else {
            None
        }
    }

    pub fn new_global_entities(&self) -> IndexSet<GlobalAddress> {
        self.new_entities_of_type()
    }

    pub fn new_component_entities(&self) -> IndexSet<ComponentAddress> {
        self.new_entities_of_type()
    }

    pub fn new_resource_entities(&self) -> IndexSet<ResourceAddress> {
        self.new_entities_of_type()
    }

    pub fn new_package_entities(&self) -> IndexSet<PackageAddress> {
        self.new_entities_of_type()
    }

    pub fn new_internal_entities(&self) -> IndexSet<InternalAddress> {
        self.new_entities_of_type()
    }

    pub fn new_non_fungibles(&self) -> &IndexSet<NonFungibleGlobalId> {
        &self.state_updates_summary.newly_minted_non_fungibles
    }

    pub fn non_fungible_data(
        &self,
        non_fungible_global_id: &NonFungibleGlobalId,
    ) -> Option<&[u8]> {
        self.state_updates_summary
            .non_fungible_data_updates
            .get(non_fungible_global_id)
            .map(|value| value.as_slice())
    }

    pub fn metadata_of_new_entities(
        &self,
    ) -> IndexMap<GlobalAddress, IndexMap<String, Option<MetadataValue>>> {
        self.new_entities_of_type_iter::<GlobalAddress>().fold(
            IndexMap::new(),
            |mut acc, global_address| {
                // If the global address doesn't have an entry in the metadata
                // updates of the receipt then we just return. Nothing to do
                // here.
                let Some(entity_metadata_updates) = self
                    .state_updates_summary
                    .metadata_updates
                    .get(global_address.as_node_id())
                else {
                    return acc;
                };

                // Construct an iterator of the entity's metadata from what's
                // been obtained from the receipt.
                let entity_metadata = entity_metadata_updates.iter().map(
                    |(metadata_key, metadata_value)| {
                        let metadata_key = metadata_key.clone();
                        let metadata_value = match metadata_value {
                            MetadataUpdate::Set(metadata_value) => {
                                Some(metadata_value.clone())
                            }
                            MetadataUpdate::Delete => None,
                        };
                        (metadata_key, metadata_value)
                    },
                );

                // Add it to the entity's metadata in the accumulator.
                acc.entry(global_address)
                    .or_default()
                    .extend(entity_metadata);

                acc
            },
        )
    }

    pub fn fee_locks(&self) -> FeeLocks {
        FeeLocks {
            lock: self.locked_fees.non_contingent,
            contingent_lock: self.locked_fees.contingent,
        }
    }

    pub fn fee_summary(&self) -> FeeSummary {
        FeeSummary {
            execution_cost: self.fee_summary.execution_fees_in_xrd,
            finalization_cost: self.fee_summary.finalization_fees_in_xrd,
            storage_expansion_cost: self.fee_summary.storage_fees_in_xrd,
            royalty_cost: self.fee_summary.royalty_fees_in_xrd,
        }
    }

    pub fn worktop_changes(&self) -> &WorktopChanges {
        &self.worktop_changes
    }

    pub fn new_entities_summary(&self) -> NewEntitiesOutput {
        NewEntitiesOutput {
            new_global_entities: self.new_entities_of_type(),
            new_internal_entities: self.new_entities_of_type(),
            new_component_entities: self.new_entities_of_type(),
            new_resource_entities: self.new_entities_of_type(),
            new_package_entities: self.new_entities_of_type(),
            new_non_fungibles: self.new_non_fungibles().clone(),
            global_entities_metadata: self.metadata_of_new_entities(),
        }
    }

    fn new_entities_of_type<T: TryFrom<NodeId> + Hash + Eq>(
        &self,
    ) -> IndexSet<T> {
        self.new_entities_of_type_iter().collect()
    }

    fn new_entities_of_type_iter<T: TryFrom<NodeId>>(
        &self,
    ) -> impl Iterator<Item = T> + use<'_, T> {
        self.state_updates_summary
            .new_entities
            .iter()
            .filter_map(|node_id| T::try_from(*node_id).ok())
    }
}
