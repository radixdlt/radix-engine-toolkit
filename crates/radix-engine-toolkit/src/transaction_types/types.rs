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

use std::ops::*;

use radix_engine::system::system_substates::*;
use radix_engine::track::*;
use radix_engine_queries::typed_substate_layout::*;
use radix_engine_store_interface::interface::*;
use scrypto::prelude::*;

use radix_engine::system::system_modules::execution_trace::*;
use radix_engine::transaction::*;
use radix_engine_interface::blueprints::account::*;

use super::*;

/// A summary of the manifest
#[derive(Clone, Debug)]
pub struct ManifestSummary {
    /// The set of the resource addresses of proofs that were presented in
    /// the manifest.
    pub presented_proofs: IndexSet<ResourceAddress>,
    /// The set of accounts withdrawn from observed in the manifest.
    pub accounts_withdrawn_from: IndexSet<ComponentAddress>,
    /// The set of accounts deposited into observed in the manifest.
    pub accounts_deposited_into: IndexSet<ComponentAddress>,
    /// The set of all the global entities encountered in the manifest. This is
    /// to be primarily used for the "using dApps" section of the wallet's tx
    /// review screen.
    pub encountered_entities: IndexSet<GlobalAddress>,
    /// The set of accounts encountered in the manifest where privileged
    /// methods were called.
    pub accounts_requiring_auth: IndexSet<ComponentAddress>,
    /// The set of identities encountered in the manifest where privileged
    /// methods were called.
    pub identities_requiring_auth: IndexSet<ComponentAddress>,
    /// The set of instructions encountered in the manifest that are reserved
    /// and can only be included in the manifest by the wallet itself.
    pub reserved_instructions: IndexSet<ReservedInstruction>,
    /// The various classifications that this manifest matched against. Note
    /// that an empty set means that the manifest is non-conforming.
    pub classification: IndexSet<ManifestClass>,
}

/// A summary of the execution of the manifest and the information that can
#[derive(Clone, Debug)]
pub struct ExecutionSummary {
    /// The withdraws done in the manifest.
    pub account_withdraws: IndexMap<ComponentAddress, Vec<ResourceIndicator>>,
    /// The deposits done in the manifest.
    pub account_deposits: IndexMap<ComponentAddress, Vec<ResourceIndicator>>,
    /// The set of the resource addresses of proofs that were presented in
    /// the manifest.
    pub presented_proofs: IndexSet<ResourceAddress>,
    /// Information on the global entities created in the transaction.
    pub new_entities: NewEntities,
    /// The set of all the global entities encountered in the manifest. This is
    /// to be primarily used for the "using dApps" section of the wallet's tx
    /// review screen.
    pub encountered_entities: IndexSet<GlobalAddress>,
    /// The set of accounts encountered in the manifest where privileged
    /// methods were called.
    pub accounts_requiring_auth: IndexSet<ComponentAddress>,
    /// The set of identities encountered in the manifest where privileged
    /// methods were called.
    pub identities_requiring_auth: IndexSet<ComponentAddress>,
    /// The set of instructions encountered in the manifest that are reserved
    /// and can only be included in the manifest by the wallet itself.
    pub reserved_instructions: IndexSet<ReservedInstruction>,
    /// Information on how much fees were contingent and how much were not.
    pub fee_locks: FeeLocks,
    /// Detailed information on the amount of cost units consumed.
    pub fee_summary: FeeSummary,
    /// The various classifications that this manifest matched against. Note
    /// that an empty set means that the manifest is non-conforming.
    pub detailed_classification: Vec<DetailedManifestClass>,
    /// List of newly created Non-Fungibles during this transaction.
    pub newly_created_non_fungibles: HashSet<NonFungibleGlobalId>,
    pub worktop_content: Vec<WorktopContent>,
}

/// The classification process classifies manifests into classes. The following
/// are the classes that the Radix Engine Toolkit supports.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum ManifestClass {
    /// A general manifest that involves any amount of arbitrary components
    /// and packages where nothing more concrete can be said about the manifest
    /// and its nature.
    General,
    /// A manifest of a 1-to-1 transfer to a one-to-many transfer of resources.
    Transfer,
    /// A manifest that contributed some amount of resources to a liquidity
    /// pool that can be a one-resource pool, two-resource pool, or a
    /// multi-resource pool.
    PoolContribution,
    /// A manifest that redeemed resources from a liquidity pool. Similar to
    /// contributions, this can be any of the three pool blueprints available
    /// in the pool package.
    PoolRedemption,
    /// A manifest where XRD is staked to one or more validators.
    ValidatorStake,
    /// A manifest where XRD is unstaked from one or more validators.
    ValidatorUnstake,
    /// A manifest where XRD is claimed from one or more validators.
    ValidatorClaim,
    /// A manifest that updated the deposit settings of the account.
    AccountDepositSettingsUpdate,
}

/// The execution summary process not only determines the class of the manifest,
/// but also includes additional information about this class that the wallet
/// requires to display to the user.
///
/// # Note
///
/// This enum must have as many variants as the [`ManifestClass`] and there
/// must always be a valid implementation of [`Into<ManifestClass>`] for this
/// enum.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DetailedManifestClass {
    /// A general manifest that involves any amount of arbitrary components
    /// and packages where nothing more concrete can be said about the manifest
    /// and its nature.
    ///
    /// No additional information is required beyond what the execution summary
    /// will provide.
    General,
    /// A manifest of a 1-to-1 transfer to a one-to-many transfer of resources.
    Transfer {
        /// When `true`, then this is a one-to-one transfer and the wallet can
        /// regard this as a "simple transfer" and communicate this information
        /// to the ledger hardware wallet. Otherwise, if `false`, then this is
        /// not a one-to-one transfer.
        is_one_to_one: bool,
    },
    /// A manifest that contributed some amount of resources to a liquidity
    /// pool that can be a one-resource pool, two-resource pool, or a
    /// multi-resource pool.
    PoolContribution {
        /// The set of pools in the transaction
        pool_addresses: IndexSet<ComponentAddress>,
        /// The contribution observed in the transaction
        pool_contributions: Vec<TrackedPoolContribution>,
    },
    /// A manifest that redeemed resources from a liquidity pool. Similar to
    /// contributions, this can be any of the three pool blueprints available
    /// in the pool package.
    PoolRedemption {
        /// The set of pools in the transaction
        pool_addresses: IndexSet<ComponentAddress>,
        /// The redemptions observed in the transaction
        pool_redemptions: Vec<TrackedPoolRedemption>,
    },
    /// A manifest where XRD is staked to one or more validators.
    ValidatorStake {
        /// The set of validators in the transaction
        validator_addresses: IndexSet<ComponentAddress>,
        /// The stake observed in the transaction
        validator_stakes: Vec<TrackedValidatorStake>,
    },
    /// A manifest where XRD is unstaked from one or more validators.
    ValidatorUnstake {
        /// The set of validators in the transaction
        validator_addresses: IndexSet<ComponentAddress>,
        /// The unstakes observed in the transaction
        validator_unstakes: Vec<TrackedValidatorUnstake>,
    },
    /// A manifest where XRD is claimed from one or more validators.
    ValidatorClaim {
        /// The set of validators in the transaction
        validator_addresses: IndexSet<ComponentAddress>,
        /// The claims observed in the transaction
        validator_claims: Vec<TrackedValidatorClaim>,
    },
    /// A manifest that updated the deposit settings of the account.
    AccountDepositSettingsUpdate {
        /// Updates to the resource preferences of the account deposit settings.
        /// account_address -> (resource_address -> Update<new_preference>)
        resource_preferences_updates: IndexMap<
            ComponentAddress,
            IndexMap<ResourceAddress, Update<ResourcePreference>>,
        >,
        /// Changes to the account's deposit mode.
        /// account_address -> new_default_deposit_mode
        deposit_mode_updates: IndexMap<ComponentAddress, DefaultDepositRule>,
        /// Updates to the authorized depositors specifying which were added
        /// and removed in the transaction.
        authorized_depositors_updates: IndexMap<
            ComponentAddress,
            IndexMap<ResourceOrNonFungible, Operation>,
        >,
    },
}

impl From<DetailedManifestClass> for ManifestClass {
    fn from(value: DetailedManifestClass) -> Self {
        match value {
            DetailedManifestClass::General => ManifestClass::General,
            DetailedManifestClass::Transfer { .. } => ManifestClass::Transfer,
            DetailedManifestClass::PoolContribution { .. } => {
                ManifestClass::PoolContribution
            }
            DetailedManifestClass::PoolRedemption { .. } => {
                ManifestClass::PoolRedemption
            }
            DetailedManifestClass::ValidatorStake { .. } => {
                ManifestClass::ValidatorStake
            }
            DetailedManifestClass::ValidatorUnstake { .. } => {
                ManifestClass::ValidatorUnstake
            }
            DetailedManifestClass::ValidatorClaim { .. } => {
                ManifestClass::ValidatorClaim
            }
            DetailedManifestClass::AccountDepositSettingsUpdate { .. } => {
                ManifestClass::AccountDepositSettingsUpdate
            }
        }
    }
}

/// A receipt used for the calculation of the execution summary. This receipt
/// must belong to a transaction that executed successfully and the execution
/// trace must be present.
#[derive(Clone, Debug)]
pub struct TransactionTypesReceipt<'r> {
    receipt: &'r TransactionReceipt,
    commit_result: &'r CommitResult,
    execution_trace: &'r TransactionExecutionTrace,
}

impl<'r> TransactionTypesReceipt<'r> {
    pub fn new(receipt: &'r TransactionReceipt) -> Option<Self> {
        if let TransactionResult::Commit(
            ref commit_result @ CommitResult {
                execution_trace: Some(ref execution_trace),
                outcome: TransactionOutcome::Success(..),
                ..
            },
        ) = &receipt.result
        {
            Some(Self {
                receipt,
                commit_result,
                execution_trace,
            })
        } else {
            None
        }
    }
}

impl<'r> TransactionTypesReceipt<'r> {
    pub fn new_components(&self) -> &'r IndexSet<ComponentAddress> {
        self.commit_result.new_component_addresses()
    }

    pub fn new_resources(&self) -> &'r IndexSet<ResourceAddress> {
        self.commit_result.new_resource_addresses()
    }

    pub fn new_packages(&self) -> &'r IndexSet<PackageAddress> {
        self.commit_result.new_package_addresses()
    }

    pub fn execution_trace(&self) -> &'r TransactionExecutionTrace {
        self.execution_trace
    }

    pub fn metadata_of_new_entities(
        &self,
    ) -> IndexMap<GlobalAddress, IndexMap<String, Option<MetadataValue>>> {
        let mut map = IndexMap::<
            GlobalAddress,
            IndexMap<String, Option<MetadataValue>>,
        >::new();

        for global_address in self.new_entities() {
            let entry = map.entry(global_address).or_default();
            if let Some(NodeStateUpdates::Delta { by_partition }) = self
                .commit_result
                .state_updates
                .by_node
                .get(global_address.as_node_id())
            {
                let entries = match by_partition.get(&METADATA_BASE_PARTITION) {
                    Some(PartitionStateUpdates::Delta { by_substate }) => {
                        by_substate
                            .iter()
                            .filter_map(|(key, value)| match value {
                                DatabaseUpdate::Set(value) => {
                                    Some((key.clone(), value.clone()))
                                }
                                DatabaseUpdate::Delete => None,
                            })
                            .collect::<IndexMap<_, _>>()
                    }
                    Some(PartitionStateUpdates::Batch(
                        BatchPartitionStateUpdate::Reset {
                            new_substate_values,
                        },
                    )) => new_substate_values.clone(),
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
                        to_typed_substate_value(&typed_substate_key, &data).map(
                            |typed_substate_value| {
                                (typed_substate_key, typed_substate_value)
                            },
                        )
                    }) {
                        let TypedMetadataModuleSubstateKey::MetadataEntryKey(
                            key,
                        ) = key;
                        let value = match value {
                            TypedMetadataModuleSubstateValue::MetadataEntry(
                                KeyValueEntrySubstate::V1(
                                    KeyValueEntrySubstateV1 { value, .. },
                                ),
                            ) => value,
                        };
                        entry.insert(
                            key,
                            value.map(|metadata_entry| {
                                let VersionedMetadataEntry::V1(metadata) =
                                    metadata_entry.content;
                                metadata
                            }),
                        );
                    }
                }
            }
        }

        map
    }

    fn new_entities(&self) -> IndexSet<GlobalAddress> {
        self.new_components()
            .iter()
            .map(|item| GlobalAddress::from(*item))
            .chain(
                self.new_resources()
                    .iter()
                    .map(|item| GlobalAddress::from(*item)),
            )
            .chain(
                self.new_packages()
                    .iter()
                    .map(|item| GlobalAddress::from(*item)),
            )
            .collect()
    }

    pub fn new_non_fungibles(&self) -> HashSet<NonFungibleGlobalId> {
        let mut minted_id_list = HashSet::new();
        let mut burnt_id_list = HashSet::new();
        for (event_type, event_payload) in
            self.commit_result.application_events.iter()
        {
            match event_type.0 {
                Emitter::Method(node_id, ..) => {
                    match ResourceAddress::try_from(node_id.as_bytes()) {
                        Ok(address) if !address.is_fungible() => {
                            if event_type.1
                                == MintNonFungibleResourceEvent::EVENT_NAME
                            {
                                let event: MintNonFungibleResourceEvent =
                                    scrypto_decode(event_payload).unwrap();
                                for local_id in event.ids {
                                    minted_id_list.insert(
                                        NonFungibleGlobalId::new(
                                            address, local_id,
                                        ),
                                    );
                                }
                            } else if event_type.1
                                == BurnNonFungibleResourceEvent::EVENT_NAME
                            {
                                let event: BurnNonFungibleResourceEvent =
                                    scrypto_decode(event_payload).unwrap();
                                for local_id in event.ids {
                                    burnt_id_list.insert(
                                        NonFungibleGlobalId::new(
                                            address, local_id,
                                        ),
                                    );
                                }
                            }
                        }
                        _ => {}
                    }
                }
                _ => (),
            }
        }

        minted_id_list.retain(|item| !burnt_id_list.contains(item));
        minted_id_list
    }
}

impl<'r> Deref for TransactionTypesReceipt<'r> {
    type Target = TransactionReceipt;

    fn deref(&self) -> &Self::Target {
        self.receipt
    }
}

/// Information on the entities created in the transaction.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct NewEntities {
    pub component_addresses: IndexSet<ComponentAddress>,
    pub resource_addresses: IndexSet<ResourceAddress>,
    pub package_addresses: IndexSet<PackageAddress>,
    pub metadata:
        IndexMap<GlobalAddress, IndexMap<String, Option<MetadataValue>>>,
}

/// The set of instructions that is only allowed in manifests created by the
/// wallet itself.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReservedInstruction {
    AccountLockFee,
    AccountSecurify,
    IdentitySecurify,
    AccountUpdateSettings,
    AccessControllerMethod,
}

pub enum FnRule {
    Allowed,
    Disallowed,
}

/// A struct that stores information on the methods that the general transaction
/// visitor allows and does not allow.
pub struct FnRules {
    pub allowed: &'static [&'static str],
    pub disallowed: &'static [&'static str],
    pub default: FnRule,
}

impl FnRules {
    pub fn is_fn_permitted(&self, fn_name: &str) -> bool {
        if self.allowed.contains(&fn_name) {
            true
        } else if self.disallowed.contains(&fn_name) {
            false
        } else {
            match self.default {
                FnRule::Allowed => true,
                FnRule::Disallowed => false,
            }
        }
    }

    pub fn all_allowed() -> Self {
        Self {
            allowed: Default::default(),
            disallowed: Default::default(),
            default: FnRule::Allowed,
        }
    }

    pub fn all_disallowed() -> Self {
        Self {
            allowed: Default::default(),
            disallowed: Default::default(),
            default: FnRule::Disallowed,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResourceIndicator {
    Fungible(ResourceAddress, FungibleResourceIndicator),
    NonFungible(ResourceAddress, NonFungibleResourceIndicator),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FungibleResourceIndicator {
    Guaranteed(Decimal),
    Predicted(Predicted<Decimal>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NonFungibleResourceIndicator {
    ByAll {
        predicted_amount: Predicted<Decimal>,
        predicted_ids: Predicted<IndexSet<NonFungibleLocalId>>,
    },
    ByAmount {
        amount: Decimal,
        predicted_ids: Predicted<IndexSet<NonFungibleLocalId>>,
    },
    ByIds(IndexSet<NonFungibleLocalId>),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Predicted<T> {
    pub value: T,
    pub instruction_index: usize,
}

impl<T> Deref for Predicted<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for Predicted<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl Deref for FungibleResourceIndicator {
    type Target = Decimal;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Guaranteed(value)
            | Self::Predicted(Predicted { value, .. }) => value,
        }
    }
}

impl DerefMut for FungibleResourceIndicator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Guaranteed(value)
            | Self::Predicted(Predicted { value, .. }) => value,
        }
    }
}

impl ResourceIndicator {
    pub fn resource_address(&self) -> ResourceAddress {
        match self {
            Self::Fungible(resource_address, _)
            | Self::NonFungible(resource_address, _) => *resource_address,
        }
    }
}

impl From<ResourceIndicator> for ResourceSpecifier {
    fn from(value: ResourceIndicator) -> Self {
        match value {
            ResourceIndicator::Fungible(
                resource_address,
                FungibleResourceIndicator::Guaranteed(amount),
            )
            | ResourceIndicator::Fungible(
                resource_address,
                FungibleResourceIndicator::Predicted(Predicted {
                    value: amount,
                    ..
                }),
            ) => ResourceSpecifier::Amount(resource_address, amount),
            ResourceIndicator::NonFungible(
                resource_address,
                NonFungibleResourceIndicator::ByAll {
                    predicted_ids: Predicted { value: ids, .. },
                    ..
                }
                | NonFungibleResourceIndicator::ByAmount {
                    predicted_ids: Predicted { value: ids, .. },
                    ..
                }
                | NonFungibleResourceIndicator::ByIds(ids),
            ) => ResourceSpecifier::Ids(resource_address, ids),
        }
    }
}

#[extend::ext]
pub impl ResourceSpecifier {
    fn resource_address(&self) -> ResourceAddress {
        match self {
            Self::Amount(x, ..) | Self::Ids(x, ..) => *x,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Update<T> {
    Set(T),
    Remove,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Operation {
    Added,
    Removed,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct FeeSummary {
    pub execution_cost: Decimal,
    pub finalization_cost: Decimal,
    pub storage_expansion_cost: Decimal,
    pub royalty_cost: Decimal,
}
