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

use radix_engine_toolkit_common::receipt::{
    RuntimeToolkitTransactionReceipt, SerializableToolkitTransactionReceipt,
};
use sbor::Versioned;

use crate::prelude::*;

#[derive(Clone, Debug, Object)]
pub struct TransactionManifestV1 {
    pub instructions: Arc<InstructionsV1>,
    pub blobs: Vec<Vec<u8>>,
}

#[uniffi::export]
impl TransactionManifestV1 {
    #[uniffi::constructor]
    pub fn new(
        instructions: Arc<InstructionsV1>,
        blobs: Vec<Vec<u8>>,
    ) -> Arc<Self> {
        Arc::new(Self {
            instructions,
            blobs,
        })
    }

    pub fn instructions(&self) -> Arc<InstructionsV1> {
        self.instructions.clone()
    }

    pub fn blobs(&self) -> Vec<Vec<u8>> {
        self.blobs.clone()
    }

    pub fn to_payload_bytes(&self) -> Result<Vec<u8>> {
        let native = self.clone().to_native();
        Ok(core_transaction_v1_manifest_to_payload_bytes(&native)?)
    }

    #[uniffi::constructor]
    pub fn from_payload_bytes(
        compiled: Vec<u8>,
        network_id: u8,
    ) -> Result<Arc<Self>> {
        let decompiled =
            core_transaction_v1_manifest_from_payload_bytes(compiled)?;
        Ok(Arc::new(Self::from_native(&decompiled, network_id)))
    }

    pub fn statically_validate(&self) -> Result<()> {
        core_transaction_v1_instructions_statically_validate(
            &self.instructions.0,
        )?;
        core_transaction_v1_manifest_statically_validate(&self.to_native())?;
        Ok(())
    }

    pub fn extract_addresses(&self) -> HashMap<EntityType, Vec<Arc<Address>>> {
        let network_id = self.instructions.1;
        let (addresses, _) = core_transaction_v1_instructions_extract_addresses(
            &self.instructions.0,
        );

        let mut map = HashMap::<EntityType, Vec<Arc<Address>>>::new();
        for address in addresses {
            let entity_type = EntityType::from(address.entity_type());
            let address =
                Arc::new(Address::from_typed_node_id(address, network_id));
            map.entry(entity_type).or_default().push(address);
        }
        map
    }

    pub fn static_analysis(&self, network_id: u8) -> Result<StaticAnalysis> {
        let native = self.clone().to_native();
        core_transaction_v1_manifest_statically_analyze(&native)
            .ok_or(RadixEngineToolkitError::StaticAnalysisFailed)
            .map(|static_analysis| {
                StaticAnalysis::from_native(static_analysis, network_id)
            })
    }

    pub fn dynamic_analysis(
        &self,
        network_id: u8,
        toolkit_receipt: String,
    ) -> Result<DynamicAnalysis> {
        let native = self.clone().to_native();
        let network_definition =
            core_network_definition_from_network_id(network_id);
        let receipt = serde_json::from_str::<
            SerializableToolkitTransactionReceipt,
        >(&toolkit_receipt)
        .ok()
        .and_then(|receipt| {
            receipt
                .into_runtime_receipt(&NativeAddressBech32Decoder::new(
                    &network_definition,
                ))
                .ok()
        })
        .ok_or(RadixEngineToolkitError::InvalidReceipt)?;
        core_transaction_v1_manifest_dynamically_analyze(&native, &receipt)
            .map_err(|_| RadixEngineToolkitError::InvalidReceipt)
            .map(|summary| DynamicAnalysis::from_native(summary, network_id))?
    }
}

impl TransactionManifestV1 {
    pub fn from_native(
        NativeTransactionManifestV1 {
            instructions,
            blobs,
            ..
        }: &NativeTransactionManifestV1,
        network_id: u8,
    ) -> Self {
        let blobs = blobs.iter().map(|(_, v)| v.clone()).collect::<Vec<_>>();
        let instructions = InstructionsV1(instructions.clone(), network_id);
        Self {
            instructions: Arc::new(instructions),
            blobs,
        }
    }

    pub fn to_native(&self) -> NativeTransactionManifestV1 {
        let blobs = self
            .blobs
            .iter()
            .map(|blob| (native_hash(blob), blob.clone()))
            .collect::<IndexMap<_, _>>();
        let instructions = self.instructions.0.clone();

        NativeTransactionManifestV1 {
            instructions,
            blobs,
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Record)]
pub struct FeeSummary {
    pub execution_cost: Arc<Decimal>,
    pub finalization_cost: Arc<Decimal>,
    pub storage_expansion_cost: Arc<Decimal>,
    pub royalty_cost: Arc<Decimal>,
}

impl FeeSummary {
    pub fn from_native(
        CoreFeeSummary {
            execution_cost,
            royalty_cost,
            finalization_cost,
            storage_expansion_cost,
        }: &CoreFeeSummary,
    ) -> Self {
        Self {
            execution_cost: Arc::new(Decimal(*execution_cost)),
            royalty_cost: Arc::new(Decimal(*royalty_cost)),
            finalization_cost: Arc::new(Decimal(*finalization_cost)),
            storage_expansion_cost: Arc::new(Decimal(*storage_expansion_cost)),
        }
    }
}

#[derive(Clone, Debug, Record)]
pub struct FeeLocks {
    pub lock: Arc<Decimal>,
    pub contingent_lock: Arc<Decimal>,
}

impl FeeLocks {
    pub fn from_native(
        NativeFeeLocks {
            contingent_lock,
            lock,
        }: &NativeFeeLocks,
    ) -> Self {
        Self {
            contingent_lock: Arc::new(Decimal(*contingent_lock)),
            lock: Arc::new(Decimal(*lock)),
        }
    }
}

#[derive(Clone, Debug, Enum)]
pub enum ResourceSpecifier {
    Amount {
        resource_address: Arc<Address>,
        amount: Arc<Decimal>,
    },
    Ids {
        resource_address: Arc<Address>,
        ids: Vec<NonFungibleLocalId>,
    },
}

impl ResourceSpecifier {
    pub fn from_native(
        native: &NativeResourceSpecifier,
        network_id: u8,
    ) -> ResourceSpecifier {
        match native {
            NativeResourceSpecifier::Amount(resource_address, amount) => {
                Self::Amount {
                    resource_address: Arc::new(Address::from_typed_node_id(
                        *resource_address,
                        network_id,
                    )),
                    amount: Arc::new(Decimal(*amount)),
                }
            }
            NativeResourceSpecifier::Ids(resource_address, ids) => Self::Ids {
                resource_address: Arc::new(Address::from_typed_node_id(
                    *resource_address,
                    network_id,
                )),
                ids: ids.iter().cloned().map(Into::into).collect(),
            },
        }
    }

    pub fn from_native_for_locker_blueprint(
        native: &NativeLockerResourceSpecifier,
        resource_address: &NativeResourceAddress,
        network_id: u8,
    ) -> ResourceSpecifier {
        let address = Arc::new(Address::from_typed_node_id(
            *resource_address,
            network_id,
        ));
        match native {
            NativeLockerResourceSpecifier::Fungible(amount) => {
                ResourceSpecifier::Amount {
                    resource_address: address,
                    amount: Arc::new(Decimal(*amount)),
                }
            }
            NativeLockerResourceSpecifier::NonFungible(native_ids) => {
                ResourceSpecifier::Ids {
                    resource_address: address,
                    ids: native_ids
                        .clone()
                        .into_iter()
                        .map(From::from)
                        .collect(),
                }
            }
        }
    }
}

#[derive(Clone, Debug, Enum)]
pub enum ResourcePreference {
    Allowed,
    Disallowed,
}

impl FromNative for ResourcePreference {
    type Native = NativeResourcePreference;

    fn from_native(native: Self::Native) -> Self {
        match native {
            NativeResourcePreference::Allowed => Self::Allowed,
            NativeResourcePreference::Disallowed => Self::Disallowed,
        }
    }
}

impl ToNative for ResourcePreference {
    type Native = NativeResourcePreference;

    fn to_native(self) -> Result<Self::Native> {
        match self {
            Self::Allowed => Ok(Self::Native::Allowed),
            Self::Disallowed => Ok(Self::Native::Disallowed),
        }
    }
}

#[derive(Clone, Debug, Enum)]
pub enum AccountDefaultDepositRule {
    Accept,
    Reject,
    AllowExisting,
}

impl FromNative for AccountDefaultDepositRule {
    type Native = NativeDefaultDepositRule;

    fn from_native(native: Self::Native) -> Self {
        match native {
            NativeDefaultDepositRule::Accept => Self::Accept,
            NativeDefaultDepositRule::Reject => Self::Reject,
            NativeDefaultDepositRule::AllowExisting => Self::AllowExisting,
        }
    }
}

impl ToNative for AccountDefaultDepositRule {
    type Native = NativeDefaultDepositRule;

    fn to_native(self) -> Result<Self::Native> {
        match self {
            AccountDefaultDepositRule::Accept => Ok(Self::Native::Accept),
            AccountDefaultDepositRule::Reject => Ok(Self::Native::Reject),
            AccountDefaultDepositRule::AllowExisting => {
                Ok(Self::Native::AllowExisting)
            }
        }
    }
}

#[derive(Clone, Debug, Enum)]
pub enum ReservedInstruction {
    AccountLockFee,
    AccountSecurify,
    IdentitySecurify,
    AccountUpdateSettings,
    AccessControllerMethod,
}

impl From<ReservedInstruction> for CoreReservedInstruction {
    fn from(value: ReservedInstruction) -> Self {
        match value {
            ReservedInstruction::AccessControllerMethod => {
                Self::AccessControllerMethod
            }
            ReservedInstruction::AccountLockFee => Self::AccountLockFee,
            ReservedInstruction::AccountSecurify => Self::AccountSecurify,
            ReservedInstruction::IdentitySecurify => Self::IdentitySecurify,
            ReservedInstruction::AccountUpdateSettings => {
                Self::AccountUpdateSettings
            }
        }
    }
}

impl From<CoreReservedInstruction> for ReservedInstruction {
    fn from(value: CoreReservedInstruction) -> Self {
        match value {
            CoreReservedInstruction::AccessControllerMethod => {
                Self::AccessControllerMethod
            }
            CoreReservedInstruction::AccountLockFee => Self::AccountLockFee,
            CoreReservedInstruction::AccountSecurify => Self::AccountSecurify,
            CoreReservedInstruction::IdentitySecurify => Self::IdentitySecurify,
            CoreReservedInstruction::AccountUpdateSettings => {
                Self::AccountUpdateSettings
            }
        }
    }
}

#[derive(Clone, Debug, Enum)]
pub enum ManifestClass {
    General,
    Transfer,
    PoolContribution,
    PoolRedemption,
    ValidatorStake,
    ValidatorUnstake,
    ValidatorClaim,
    AccountDepositSettingsUpdate,
}

impl From<CoreManifestClass> for ManifestClass {
    fn from(value: CoreManifestClass) -> Self {
        match value {
            CoreManifestClass::General => Self::General,
            CoreManifestClass::Transfer => Self::Transfer,
            CoreManifestClass::PoolContribution => Self::PoolContribution,
            CoreManifestClass::PoolRedemption => Self::PoolRedemption,
            CoreManifestClass::ValidatorStake => Self::ValidatorStake,
            CoreManifestClass::ValidatorUnstake => Self::ValidatorUnstake,
            CoreManifestClass::ValidatorClaim => Self::ValidatorClaim,
            CoreManifestClass::AccountDepositSettingsUpdate => {
                Self::AccountDepositSettingsUpdate
            }
        }
    }
}

#[derive(Clone, Debug, Enum)]
pub enum DetailedManifestClass {
    General,
    Transfer {
        is_one_to_one: bool,
    },
    PoolContribution {
        pool_addresses: Vec<Arc<Address>>,
        pool_contributions: Vec<TrackedPoolContribution>,
    },
    PoolRedemption {
        pool_addresses: Vec<Arc<Address>>,
        pool_redemptions: Vec<TrackedPoolRedemption>,
    },
    ValidatorStake {
        validator_addresses: Vec<Arc<Address>>,
        validator_stakes: Vec<TrackedValidatorStake>,
    },
    ValidatorUnstake {
        validator_addresses: Vec<Arc<Address>>,
        validator_unstakes: Vec<TrackedValidatorUnstake>,
        claims_non_fungible_data: Vec<UnstakeDataEntry>,
    },
    ValidatorClaim {
        validator_addresses: Vec<Arc<Address>>,
        validator_claims: Vec<TrackedValidatorClaim>,
    },
    AccountDepositSettingsUpdate {
        resource_preferences_updates:
            HashMap<String, HashMap<String, ResourcePreferenceUpdate>>,
        deposit_mode_updates: HashMap<String, AccountDefaultDepositRule>,
        authorized_depositors_added:
            HashMap<String, Vec<ResourceOrNonFungible>>,
        authorized_depositors_removed:
            HashMap<String, Vec<ResourceOrNonFungible>>,
    },
}

impl DetailedManifestClass {
    pub fn from_native(
        value: CoreDetailedManifestClass,
        network_id: u8,
    ) -> Self {
        match value {
            CoreDetailedManifestClass::General => Self::General,
            CoreDetailedManifestClass::Transfer { is_one_to_one } => {
                Self::Transfer { is_one_to_one }
            }
            CoreDetailedManifestClass::PoolContribution {
                pool_addresses,
                pool_contributions,
            } => Self::PoolContribution {
                pool_addresses: pool_addresses
                    .into_iter()
                    .map(|item| {
                        Arc::new(Address::unsafe_from_raw(
                            item.into_node_id(),
                            network_id,
                        ))
                    })
                    .collect(),
                pool_contributions: pool_contributions
                    .into_iter()
                    .map(|item| TrackedPoolContribution::from_native(item, network_id))
                    .collect(),
            },
            CoreDetailedManifestClass::PoolRedemption {
                pool_addresses,
                pool_redemptions,
            } => Self::PoolRedemption {
                pool_addresses: pool_addresses
                    .into_iter()
                    .map(|item| {
                        Arc::new(Address::unsafe_from_raw(
                            item.into_node_id(),
                            network_id,
                        ))
                    })
                    .collect(),
                pool_redemptions: pool_redemptions
                    .into_iter()
                    .map(|item| TrackedPoolRedemption::from_native(item, network_id))
                    .collect(),
            },
            CoreDetailedManifestClass::ValidatorStake {
                validator_addresses,
                validator_stakes,
            } => Self::ValidatorStake {
                validator_addresses: validator_addresses
                    .into_iter()
                    .map(|item| {
                        Arc::new(Address::unsafe_from_raw(
                            item.into_node_id(),
                            network_id,
                        ))
                    })
                    .collect(),
                validator_stakes: validator_stakes
                    .into_iter()
                    .map(|item| TrackedValidatorStake::from_native(item, network_id))
                    .collect(),
            },
            CoreDetailedManifestClass::ValidatorUnstake {
                validator_addresses,
                validator_unstakes,
                claims_non_fungible_data,
            } => Self::ValidatorUnstake {
                validator_addresses: validator_addresses
                    .into_iter()
                    .map(|item| {
                        Arc::new(Address::unsafe_from_raw(
                            item.into_node_id(),
                            network_id,
                        ))
                    })
                    .collect(),
                validator_unstakes: validator_unstakes
                    .into_iter()
                    .map(|item| TrackedValidatorUnstake::from_native(item, network_id))
                    .collect(),
                claims_non_fungible_data: claims_non_fungible_data
                    .into_iter()
                    .map(|(id, value)| UnstakeDataEntry {
                        non_fungible_global_id: Arc::new(
                            NonFungibleGlobalId(id, network_id)
                        ),
                        data: value.into(),
                    })
                    .collect(),
            },
            CoreDetailedManifestClass::ValidatorClaim {
                validator_addresses,
                validator_claims,
            } => Self::ValidatorClaim {
                validator_addresses: validator_addresses
                    .into_iter()
                    .map(|item| {
                        Arc::new(Address::unsafe_from_raw(
                            item.into_node_id(),
                            network_id,
                        ))
                    })
                    .collect(),
                validator_claims: validator_claims
                    .into_iter()
                    .map(|item| TrackedValidatorClaim::from_native(item, network_id))
                    .collect(),
            },
            CoreDetailedManifestClass::AccountDepositSettingsUpdate {
                resource_preferences_updates,
                deposit_mode_updates,
                authorized_depositors_updates,
            } => Self::AccountDepositSettingsUpdate {
                resource_preferences_updates: resource_preferences_updates
                    .into_iter()
                    .map(|(k, v)| {
                        (
                            Address::unsafe_from_raw(k.into_node_id(), network_id)
                                .address_string(),
                            v.into_iter()
                                .map(|(k, v)| {
                                    (
                                        Address::unsafe_from_raw(
                                            k.into_node_id(),
                                            network_id,
                                        )
                                        .address_string(),
                                        ResourcePreferenceUpdate::from(v),
                                    )
                                })
                                .collect(),
                        )
                    })
                    .collect(),
                deposit_mode_updates: deposit_mode_updates
                    .into_iter()
                    .map(|(k, v)| {
                        (
                            Address::unsafe_from_raw(k.into_node_id(), network_id)
                                .address_string(),
                            <AccountDefaultDepositRule as FromNative>::from_native(v),
                        )
                    })
                    .collect(),
                authorized_depositors_added: authorized_depositors_updates
                    .iter()
                    .map(|(k, v)| {
                        (
                            Address::unsafe_from_raw(k.into_node_id(), network_id)
                                .address_string(),
                            v.into_iter()
                                .filter_map(|(k, v)| {
                                    if let CoreOperation::Added = v {
                                        Some(ResourceOrNonFungible::from_native(
                                            k.clone(),
                                            network_id,
                                        ))
                                    } else {
                                        None
                                    }
                                })
                                .collect(),
                        )
                    })
                    .collect(),
                authorized_depositors_removed: authorized_depositors_updates
                    .iter()
                    .map(|(k, v)| {
                        (
                            Address::unsafe_from_raw(k.into_node_id(), network_id)
                                .address_string(),
                            v.into_iter()
                                .filter_map(|(k, v)| {
                                    if let CoreOperation::Removed = v {
                                        Some(ResourceOrNonFungible::from_native(
                                            k.clone(),
                                            network_id,
                                        ))
                                    } else {
                                        None
                                    }
                                })
                                .collect(),
                        )
                    })
                    .collect(),
            },
        }
    }
}

#[derive(Clone, Debug, Record)]
pub struct DynamicAnalysis {
    pub account_withdraws: HashMap<String, Vec<ResourceIndicator>>,
    pub account_deposits: HashMap<String, Vec<ResourceIndicator>>,
    pub presented_proofs: HashMap<String, Vec<ResourceSpecifier>>,
    pub new_entities: NewEntities,
    pub encountered_entities: Vec<Arc<Address>>,
    pub accounts_requiring_auth: Vec<Arc<Address>>,
    pub identities_requiring_auth: Vec<Arc<Address>>,
    pub reserved_instructions: Vec<ReservedInstruction>,
    pub fee_locks: FeeLocks,
    pub fee_summary: FeeSummary,
    pub detailed_classification: Vec<DetailedManifestClass>,
    pub newly_created_non_fungibles: Vec<Arc<NonFungibleGlobalId>>,
}

impl DynamicAnalysis {
    pub fn from_native(
        native: CoreDynamicAnalysis,
        network_id: u8,
    ) -> Result<Self> {
        Ok(Self {
            account_withdraws: native
                .account_withdraws
                .into_iter()
                .map(|(k, v)| {
                    (
                        Address::unsafe_from_raw(k.into_node_id(), network_id)
                            .address_string(),
                        v.into_iter()
                            .map(|item| {
                                ResourceIndicator::from_native(item, network_id)
                            })
                            .collect(),
                    )
                })
                .collect(),
            account_deposits: native
                .account_deposits
                .into_iter()
                .map(|(k, v)| {
                    (
                        Address::unsafe_from_raw(k.into_node_id(), network_id)
                            .address_string(),
                        v.into_iter()
                            .map(|item| {
                                ResourceIndicator::from_native(item, network_id)
                            })
                            .collect(),
                    )
                })
                .collect(),
            presented_proofs: native
                .presented_proofs
                .into_iter()
                .map(|item| {
                    (
                        Address::unsafe_from_raw(
                            item.0.into_node_id(),
                            network_id,
                        )
                        .address_string(),
                        item.1
                            .iter()
                            .map(|i| {
                                ResourceSpecifier::from_native(i, network_id)
                            })
                            .collect(),
                    )
                })
                .collect(),
            new_entities: NewEntities::from_native(
                native.new_entities,
                network_id,
            ),
            encountered_entities: native
                .encountered_entities
                .into_iter()
                .map(|item| {
                    Arc::new(Address::unsafe_from_raw(
                        item.into_node_id(),
                        network_id,
                    ))
                })
                .collect(),
            accounts_requiring_auth: native
                .accounts_requiring_auth
                .into_iter()
                .map(|item| {
                    Arc::new(Address::unsafe_from_raw(
                        item.into_node_id(),
                        network_id,
                    ))
                })
                .collect(),
            identities_requiring_auth: native
                .identities_requiring_auth
                .into_iter()
                .map(|item| {
                    Arc::new(Address::unsafe_from_raw(
                        item.into_node_id(),
                        network_id,
                    ))
                })
                .collect(),
            reserved_instructions: native
                .reserved_instructions
                .into_iter()
                .map(ReservedInstruction::from)
                .collect(),
            fee_locks: FeeLocks::from_native(&native.fee_locks),
            fee_summary: FeeSummary::from_native(&native.fee_summary),
            detailed_classification: native
                .detailed_classification
                .into_iter()
                .map(|item| {
                    DetailedManifestClass::from_native(item, network_id)
                })
                .collect(),
            newly_created_non_fungibles: native
                .newly_created_non_fungibles
                .into_iter()
                .map(|item| {
                    NonFungibleGlobalId::from_parts(
                        Arc::new(Address::unsafe_from_raw(
                            item.resource_address().into_node_id(),
                            network_id,
                        )),
                        NonFungibleLocalId::from(item.local_id().clone()),
                    )
                })
                .collect::<Result<Vec<_>>>()?,
        })
    }
}

#[derive(Clone, Debug, Record)]
pub struct StaticAnalysis {
    pub presented_proofs: HashMap<String, Vec<ResourceSpecifier>>,
    pub accounts_withdrawn_from: Vec<Arc<Address>>,
    pub accounts_deposited_into: Vec<Arc<Address>>,
    pub encountered_entities: Vec<Arc<Address>>,
    pub accounts_requiring_auth: Vec<Arc<Address>>,
    pub identities_requiring_auth: Vec<Arc<Address>>,
    pub reserved_instructions: Vec<ReservedInstruction>,
    pub classification: Vec<ManifestClass>,
}

impl StaticAnalysis {
    fn from_native(native: CoreStaticAnalysis, network_id: u8) -> Self {
        Self {
            presented_proofs: native
                .presented_proofs
                .into_iter()
                .map(|item| {
                    (
                        Address::unsafe_from_raw(
                            item.0.into_node_id(),
                            network_id,
                        )
                        .address_string(),
                        item.1
                            .iter()
                            .map(|i| {
                                ResourceSpecifier::from_native(i, network_id)
                            })
                            .collect(),
                    )
                })
                .collect(),
            accounts_withdrawn_from: native
                .accounts_withdrawn_from
                .into_iter()
                .map(|item| {
                    Arc::new(Address::unsafe_from_raw(
                        item.into_node_id(),
                        network_id,
                    ))
                })
                .collect(),
            accounts_deposited_into: native
                .accounts_deposited_into
                .into_iter()
                .map(|item| {
                    Arc::new(Address::unsafe_from_raw(
                        item.into_node_id(),
                        network_id,
                    ))
                })
                .collect(),
            encountered_entities: native
                .encountered_entities
                .into_iter()
                .map(|item| {
                    Arc::new(Address::unsafe_from_raw(
                        item.into_node_id(),
                        network_id,
                    ))
                })
                .collect(),
            accounts_requiring_auth: native
                .accounts_requiring_auth
                .into_iter()
                .map(|item| {
                    Arc::new(Address::unsafe_from_raw(
                        item.into_node_id(),
                        network_id,
                    ))
                })
                .collect(),
            identities_requiring_auth: native
                .identities_requiring_auth
                .into_iter()
                .map(|item| {
                    Arc::new(Address::unsafe_from_raw(
                        item.into_node_id(),
                        network_id,
                    ))
                })
                .collect(),
            reserved_instructions: native
                .reserved_instructions
                .into_iter()
                .map(ReservedInstruction::from)
                .collect(),
            classification: native
                .classification
                .into_iter()
                .map(ManifestClass::from)
                .collect(),
        }
    }
}

#[derive(Clone, Debug, Record)]
pub struct TrackedPoolContribution {
    pub pool_address: Arc<Address>,
    /* Input */
    pub contributed_resources: HashMap<String, Arc<Decimal>>,
    /* Output */
    pub pool_units_resource_address: Arc<Address>,
    pub pool_units_amount: Arc<Decimal>,
}

impl TrackedPoolContribution {
    pub fn from_native(
        native: CoreTrackedPoolContribution,
        network_id: u8,
    ) -> Self {
        Self {
            pool_address: Arc::new(Address::unsafe_from_raw(
                native.pool_address.into_node_id(),
                network_id,
            )),
            contributed_resources: native
                .contributed_resources
                .into_iter()
                .map(|(k, v)| {
                    (
                        Address::unsafe_from_raw(k.into_node_id(), network_id)
                            .address_string(),
                        Arc::new(Decimal(v)),
                    )
                })
                .collect(),
            pool_units_resource_address: Arc::new(Address::unsafe_from_raw(
                native.pool_units_resource_address.into_node_id(),
                network_id,
            )),
            pool_units_amount: Arc::new(Decimal(native.pool_units_amount)),
        }
    }
}

#[derive(Clone, Debug, Record)]
pub struct TrackedPoolRedemption {
    pub pool_address: Arc<Address>,
    /* Input */
    pub pool_units_resource_address: Arc<Address>,
    pub pool_units_amount: Arc<Decimal>,
    /* Output */
    pub redeemed_resources: HashMap<String, Arc<Decimal>>,
}

impl TrackedPoolRedemption {
    pub fn from_native(
        native: CoreTrackedPoolRedemption,
        network_id: u8,
    ) -> Self {
        Self {
            pool_address: Arc::new(Address::unsafe_from_raw(
                native.pool_address.into_node_id(),
                network_id,
            )),
            redeemed_resources: native
                .redeemed_resources
                .into_iter()
                .map(|(k, v)| {
                    (
                        Address::unsafe_from_raw(k.into_node_id(), network_id)
                            .address_string(),
                        Arc::new(Decimal(v)),
                    )
                })
                .collect(),
            pool_units_resource_address: Arc::new(Address::unsafe_from_raw(
                native.pool_units_resource_address.into_node_id(),
                network_id,
            )),
            pool_units_amount: Arc::new(Decimal(native.pool_units_amount)),
        }
    }
}

#[derive(Clone, Debug, Record)]
pub struct TrackedValidatorStake {
    pub validator_address: Arc<Address>,
    /* Input */
    pub xrd_amount: Arc<Decimal>,
    /* Output */
    pub liquid_stake_unit_address: Arc<Address>,
    pub liquid_stake_unit_amount: Arc<Decimal>,
}

impl TrackedValidatorStake {
    pub fn from_native(
        native: CoreTrackedValidatorStake,
        network_id: u8,
    ) -> Self {
        Self {
            validator_address: Arc::new(Address::unsafe_from_raw(
                native.validator_address.into_node_id(),
                network_id,
            )),
            xrd_amount: Arc::new(Decimal(native.xrd_amount)),
            liquid_stake_unit_address: Arc::new(Address::unsafe_from_raw(
                native.liquid_stake_unit_address.into_node_id(),
                network_id,
            )),
            liquid_stake_unit_amount: Arc::new(Decimal(
                native.liquid_stake_unit_amount,
            )),
        }
    }
}

#[derive(Clone, Debug, Record)]
pub struct TrackedValidatorUnstake {
    pub validator_address: Arc<Address>,
    /* Input */
    pub liquid_stake_unit_address: Arc<Address>,
    pub liquid_stake_unit_amount: Arc<Decimal>,
    /* Output */
    pub claim_nft_address: Arc<Address>,
    pub claim_nft_ids: Vec<NonFungibleLocalId>,
}

impl TrackedValidatorUnstake {
    pub fn from_native(
        native: CoreTrackedValidatorUnstake,
        network_id: u8,
    ) -> Self {
        Self {
            validator_address: Arc::new(Address::unsafe_from_raw(
                native.validator_address.into_node_id(),
                network_id,
            )),
            liquid_stake_unit_address: Arc::new(Address::unsafe_from_raw(
                native.liquid_stake_unit_address.into_node_id(),
                network_id,
            )),
            liquid_stake_unit_amount: Arc::new(Decimal(
                native.liquid_stake_unit_amount,
            )),
            claim_nft_address: Arc::new(Address::unsafe_from_raw(
                native.claim_nft_address.into_node_id(),
                network_id,
            )),
            claim_nft_ids: native
                .claim_nft_ids
                .into_iter()
                .map(NonFungibleLocalId::from)
                .collect(),
        }
    }
}

#[derive(Clone, Debug, Record)]
pub struct TrackedValidatorClaim {
    pub validator_address: Arc<Address>,
    /* Input */
    pub claim_nft_address: Arc<Address>,
    pub claim_nft_ids: Vec<NonFungibleLocalId>,
    /* Output */
    pub xrd_amount: Arc<Decimal>,
}

impl TrackedValidatorClaim {
    pub fn from_native(
        native: CoreTrackedValidatorClaim,
        network_id: u8,
    ) -> Self {
        Self {
            validator_address: Arc::new(Address::unsafe_from_raw(
                native.validator_address.into_node_id(),
                network_id,
            )),

            xrd_amount: Arc::new(Decimal(native.xrd_amount)),
            claim_nft_address: Arc::new(Address::unsafe_from_raw(
                native.claim_nft_address.into_node_id(),
                network_id,
            )),
            claim_nft_ids: native
                .claim_nft_ids
                .into_iter()
                .map(NonFungibleLocalId::from)
                .collect(),
        }
    }
}

#[derive(Clone, Debug, Enum)]
pub enum Operation {
    Add,
    Remove,
}

impl From<CoreOperation> for Operation {
    fn from(value: CoreOperation) -> Self {
        match value {
            CoreOperation::Added => Self::Add,
            CoreOperation::Removed => Self::Remove,
        }
    }
}

#[derive(Clone, Debug, Enum)]
pub enum ResourcePreferenceUpdate {
    Set { value: ResourcePreference },
    Remove,
}

impl From<CoreUpdate<NativeResourcePreference>> for ResourcePreferenceUpdate {
    fn from(value: CoreUpdate<NativeResourcePreference>) -> Self {
        match value {
            CoreUpdate::Set(value) => Self::Set {
                value: <ResourcePreference as FromNative>::from_native(value),
            },
            CoreUpdate::Remove => Self::Remove,
        }
    }
}

#[derive(Clone, Debug, Record)]
pub struct NewEntities {
    pub component_addresses: Vec<Arc<Address>>,
    pub resource_addresses: Vec<Arc<Address>>,
    pub package_addresses: Vec<Arc<Address>>,
    pub metadata: HashMap<String, HashMap<String, Option<MetadataValue>>>,
}

impl NewEntities {
    pub fn from_native(native: CoreNewEntities, network_id: u8) -> Self {
        Self {
            component_addresses: native
                .component_addresses
                .into_iter()
                .map(|item| {
                    Arc::new(Address::unsafe_from_raw(
                        item.into_node_id(),
                        network_id,
                    ))
                })
                .collect(),
            package_addresses: native
                .package_addresses
                .into_iter()
                .map(|item| {
                    Arc::new(Address::unsafe_from_raw(
                        item.into_node_id(),
                        network_id,
                    ))
                })
                .collect(),
            resource_addresses: native
                .resource_addresses
                .into_iter()
                .map(|item| {
                    Arc::new(Address::unsafe_from_raw(
                        item.into_node_id(),
                        network_id,
                    ))
                })
                .collect(),
            metadata: native
                .metadata
                .iter()
                .map(|(key, value)| {
                    (
                        Address::from_typed_node_id(*key, network_id).as_str(),
                        value
                            .iter()
                            .map(|(key, value)| {
                                (
                                    key.clone(),
                                    value.as_ref().map(|value| {
                                        MetadataValue::from_native(
                                            value, network_id,
                                        )
                                    }),
                                )
                            })
                            .collect(),
                    )
                })
                .collect(),
        }
    }
}

#[derive(Clone, Debug, Enum)]
pub enum ResourceIndicator {
    Fungible {
        resource_address: Arc<Address>,
        indicator: FungibleResourceIndicator,
    },
    NonFungible {
        resource_address: Arc<Address>,
        indicator: NonFungibleResourceIndicator,
    },
}

impl ResourceIndicator {
    pub fn from_native(native: CoreResourceIndicator, network_id: u8) -> Self {
        match native {
            CoreResourceIndicator::Fungible(resource_address, amount) => {
                ResourceIndicator::Fungible {
                    resource_address: Arc::new(Address::unsafe_from_raw(
                        resource_address.into_node_id(),
                        network_id,
                    )),
                    indicator: FungibleResourceIndicator::from(amount),
                }
            }
            CoreResourceIndicator::NonFungible(resource_address, ids) => {
                ResourceIndicator::NonFungible {
                    resource_address: Arc::new(Address::unsafe_from_raw(
                        resource_address.into_node_id(),
                        network_id,
                    )),
                    indicator: NonFungibleResourceIndicator::from(ids),
                }
            }
        }
    }
}

#[derive(Clone, Debug, Enum)]
pub enum FungibleResourceIndicator {
    Guaranteed { amount: Arc<Decimal> },
    Predicted { predicted_amount: PredictedDecimal },
}

impl From<CoreFungibleResourceIndicator> for FungibleResourceIndicator {
    fn from(value: CoreFungibleResourceIndicator) -> Self {
        match value {
            CoreFungibleResourceIndicator::Guaranteed(amount) => {
                FungibleResourceIndicator::Guaranteed {
                    amount: Arc::new(Decimal(amount)),
                }
            }
            CoreFungibleResourceIndicator::Predicted(predicted_amount) => {
                FungibleResourceIndicator::Predicted {
                    predicted_amount: PredictedDecimal {
                        value: Arc::new(Decimal(predicted_amount.value)),
                        instruction_index: predicted_amount.instruction_index
                            as u64,
                    },
                }
            }
        }
    }
}

#[derive(Clone, Debug, Enum)]
pub enum NonFungibleResourceIndicator {
    ByAll {
        predicted_amount: PredictedDecimal,
        predicted_ids: PredictedNonFungibleIds,
    },
    ByAmount {
        amount: Arc<Decimal>,
        predicted_ids: PredictedNonFungibleIds,
    },
    ByIds {
        ids: Vec<NonFungibleLocalId>,
    },
}

impl From<CoreNonFungibleResourceIndicator> for NonFungibleResourceIndicator {
    fn from(value: CoreNonFungibleResourceIndicator) -> Self {
        match value {
            CoreNonFungibleResourceIndicator::ByAll {
                predicted_amount,
                predicted_ids,
            } => NonFungibleResourceIndicator::ByAll {
                predicted_amount: PredictedDecimal {
                    value: Arc::new(Decimal(predicted_amount.value)),
                    instruction_index: predicted_amount.instruction_index
                        as u64,
                },
                predicted_ids: PredictedNonFungibleIds {
                    value: predicted_ids
                        .value
                        .into_iter()
                        .map(Into::into)
                        .collect(),
                    instruction_index: predicted_ids.instruction_index as u64,
                },
            },
            CoreNonFungibleResourceIndicator::ByAmount {
                amount,
                predicted_ids,
            } => NonFungibleResourceIndicator::ByAmount {
                amount: Arc::new(Decimal(amount)),
                predicted_ids: PredictedNonFungibleIds {
                    value: predicted_ids
                        .value
                        .into_iter()
                        .map(Into::into)
                        .collect(),
                    instruction_index: predicted_ids.instruction_index as u64,
                },
            },
            CoreNonFungibleResourceIndicator::ByIds(ids) => {
                NonFungibleResourceIndicator::ByIds {
                    ids: ids.into_iter().map(Into::into).collect(),
                }
            }
        }
    }
}

#[derive(Clone, Debug, Record)]
pub struct PredictedDecimal {
    pub value: Arc<Decimal>,
    pub instruction_index: u64,
}

impl From<CorePredicted<NativeDecimal>> for PredictedDecimal {
    fn from(value: CorePredicted<NativeDecimal>) -> Self {
        Self {
            value: Arc::new(Decimal(value.value)),
            instruction_index: value.instruction_index as u64,
        }
    }
}

#[derive(Clone, Debug, Record)]
pub struct PredictedNonFungibleIds {
    pub value: Vec<NonFungibleLocalId>,
    pub instruction_index: u64,
}

impl From<CorePredicted<IndexSet<NativeNonFungibleLocalId>>>
    for PredictedNonFungibleIds
{
    fn from(value: CorePredicted<IndexSet<NativeNonFungibleLocalId>>) -> Self {
        Self {
            value: value
                .value
                .into_iter()
                .map(NonFungibleLocalId::from)
                .collect(),
            instruction_index: value.instruction_index as u64,
        }
    }
}

#[derive(Clone, Debug, Record)]
pub struct UnstakeDataEntry {
    pub non_fungible_global_id: Arc<NonFungibleGlobalId>,
    pub data: UnstakeData,
}

#[derive(Clone, Debug, Record)]
pub struct UnstakeData {
    pub name: String,
    pub claim_epoch: u64,
    pub claim_amount: Arc<Decimal>,
}

impl From<NativeUnstakeData> for UnstakeData {
    fn from(value: NativeUnstakeData) -> Self {
        Self {
            name: value.name,
            claim_epoch: value.claim_epoch.number(),
            claim_amount: Arc::new(Decimal(value.claim_amount)),
        }
    }
}
