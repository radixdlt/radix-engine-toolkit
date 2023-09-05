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

use crate::prelude::*;

#[derive(Clone, Debug, Object)]
pub struct TransactionManifest {
    pub instructions: Arc<Instructions>,
    pub blobs: Vec<Vec<u8>>,
}

#[uniffi::export]
impl TransactionManifest {
    #[uniffi::constructor]
    pub fn new(instructions: Arc<Instructions>, blobs: Vec<Vec<u8>>) -> Arc<Self> {
        Arc::new(Self {
            instructions,
            blobs,
        })
    }

    pub fn instructions(&self) -> Arc<Instructions> {
        self.instructions.clone()
    }

    pub fn blobs(&self) -> Vec<Vec<u8>> {
        self.blobs.clone()
    }

    pub fn statically_validate(&self) -> Result<()> {
        core_instructions_statically_validate(&self.instructions.0)?;
        core_manifest_statically_validate(&self.to_native())?;
        Ok(())
    }

    pub fn extract_addresses(&self) -> HashMap<EntityType, Vec<Arc<Address>>> {
        let network_id = self.instructions.1;
        let (addresses, _) = core_instructions_extract_addresses(&self.instructions.0);

        let mut map = HashMap::<EntityType, Vec<Arc<Address>>>::new();
        for address in addresses {
            let entity_type = EntityType::from(address.entity_type());
            let address = Arc::new(Address::from_typed_node_id(address, network_id));
            map.entry(entity_type).or_default().push(address);
        }
        map
    }

    pub fn identities_requiring_auth(&self) -> Vec<Arc<Address>> {
        core_instructions_identities_requiring_auth(&self.instructions.0)
            .into_iter()
            .map(|address| Arc::new(Address::from_typed_node_id(address, self.instructions.1)))
            .collect()
    }

    pub fn accounts_requiring_auth(&self) -> Vec<Arc<Address>> {
        core_instructions_accounts_requiring_auth(&self.instructions.0)
            .into_iter()
            .map(|address| Arc::new(Address::from_typed_node_id(address, self.instructions.1)))
            .collect()
    }

    pub fn accounts_withdrawn_from(&self) -> Vec<Arc<Address>> {
        core_instructions_accounts_withdrawn_from(&self.instructions.0)
            .into_iter()
            .map(|address| Arc::new(Address::from_typed_node_id(address, self.instructions.1)))
            .collect()
    }

    pub fn accounts_deposited_into(&self) -> Vec<Arc<Address>> {
        core_instructions_accounts_deposited_into(&self.instructions.0)
            .into_iter()
            .map(|address| Arc::new(Address::from_typed_node_id(address, self.instructions.1)))
            .collect()
    }

    pub fn analyze_execution(&self, transaction_receipt: Vec<u8>) -> Result<ExecutionAnalysis> {
        let receipt =
            native_scrypto_decode::<NativeVersionedTransactionReceipt>(&transaction_receipt)?;
        let analysis = core_execution_analyze(
            &self.instructions.0,
            &CoreExecutionAnalysisTransactionReceipt::new(&receipt)?,
        )?;
        Ok(ExecutionAnalysis::from_native(
            &analysis,
            self.instructions.1,
        ))
    }

    pub fn modify(&self, modifications: TransactionManifestModifications) -> Result<Arc<Self>> {
        let modifications = modifications.to_native()?;
        let native_manifest = core_manifest_modify(&self.to_native(), modifications)?;
        let manifest = Self::from_native(&native_manifest, self.instructions.network_id());
        Ok(Arc::new(manifest))
    }
}

impl TransactionManifest {
    pub fn from_native(
        NativeTransactionManifest {
            instructions,
            blobs,
        }: &NativeTransactionManifest,
        network_id: u8,
    ) -> Self {
        let blobs = blobs.iter().map(|(_, v)| v.clone()).collect::<Vec<_>>();
        let instructions = Instructions(instructions.clone(), network_id);
        Self {
            instructions: Arc::new(instructions),
            blobs,
        }
    }

    pub fn to_native(&self) -> NativeTransactionManifest {
        let blobs = self
            .blobs
            .iter()
            .map(|blob| (native_hash(blob), blob.clone()))
            .collect::<BTreeMap<_, _>>();
        let instructions = self.instructions.0.clone();

        NativeTransactionManifest {
            instructions,
            blobs,
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

#[derive(Clone, Debug, Record)]
pub struct FeeLocks {
    pub lock: Arc<Decimal>,
    pub contingent_lock: Arc<Decimal>,
}

#[allow(clippy::large_enum_variant)] // TODO: Consider complying with this
#[derive(Clone, Debug, Enum)]
pub enum TransactionType {
    SimpleTransfer {
        from: Arc<Address>,
        to: Arc<Address>,
        transferred: ResourceSpecifier,
    },
    Transfer {
        from: Arc<Address>,
        transfers: HashMap<String, HashMap<String, Resources>>,
    },
    AccountDepositSettings {
        resource_preference_changes: HashMap<String, HashMap<String, ResourcePreferenceAction>>,
        default_deposit_rule_changes: HashMap<String, AccountDefaultDepositRule>,
        authorized_depositors_changes: HashMap<String, AuthorizedDepositorsChanges>,
    },
    GeneralTransaction {
        account_proofs: Vec<Arc<Address>>,
        account_withdraws: HashMap<String, Vec<ResourceTracker>>,
        account_deposits: HashMap<String, Vec<ResourceTracker>>,
        addresses_in_manifest: HashMap<EntityType, Vec<Arc<Address>>>,
        metadata_of_newly_created_entities: HashMap<String, HashMap<String, Option<MetadataValue>>>,
        data_of_newly_minted_non_fungibles: HashMap<String, HashMap<NonFungibleLocalId, Vec<u8>>>,
        addresses_of_newly_created_entities: Vec<Arc<Address>>,
    },
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

#[derive(Clone, Debug, Enum)]
pub enum Resources {
    Amount { amount: Arc<Decimal> },
    Ids { ids: Vec<NonFungibleLocalId> },
}

#[derive(Clone, Debug, Record)]
pub struct ExecutionAnalysis {
    pub fee_locks: FeeLocks,
    pub fee_summary: FeeSummary,
    pub transaction_types: Vec<TransactionType>,
    pub reserved_instructions: Vec<ReservedInstruction>,
}

impl ExecutionAnalysis {
    pub fn from_native(
        CoreExecutionExecutionAnalysis {
            fee_locks,
            fee_summary,
            transaction_types,
            reserved_instructions,
        }: &CoreExecutionExecutionAnalysis,
        network_id: u8,
    ) -> Self {
        Self {
            transaction_types: transaction_types
                .iter()
                .map(|transaction_type| TransactionType::from_native(transaction_type, network_id))
                .collect(),
            fee_locks: FeeLocks::from_native(fee_locks),
            fee_summary: FeeSummary::from_native(fee_summary),
            reserved_instructions: reserved_instructions
                .iter()
                .map(|value| (*value).into())
                .collect(),
        }
    }
}

impl Resources {
    pub fn from_native(native: &CoreResources) -> Self {
        match native {
            CoreResources::Amount(value) => Self::Amount {
                amount: Arc::new(Decimal(*value)),
            },
            CoreResources::Ids(value) => Self::Ids {
                ids: value.iter().cloned().map(Into::into).collect(),
            },
        }
    }
}

impl ResourceSpecifier {
    pub fn from_native(native: &NativeResourceSpecifier, network_id: u8) -> ResourceSpecifier {
        match native {
            NativeResourceSpecifier::Amount(resource_address, amount) => Self::Amount {
                resource_address: Arc::new(Address::from_typed_node_id(
                    *resource_address,
                    network_id,
                )),
                amount: Arc::new(Decimal(*amount)),
            },
            NativeResourceSpecifier::Ids(resource_address, ids) => Self::Ids {
                resource_address: Arc::new(Address::from_typed_node_id(
                    *resource_address,
                    network_id,
                )),
                ids: ids.iter().cloned().map(Into::into).collect(),
            },
        }
    }
}

impl TransactionType {
    pub fn from_native(native: &CoreExecutionTransactionType, network_id: u8) -> Self {
        match native {
            CoreExecutionTransactionType::SimpleTransfer(value) => {
                let CoreExecutionSimpleTransferTransactionType {
                    from,
                    to,
                    transferred,
                } = value.as_ref();

                Self::SimpleTransfer {
                    from: Arc::new(Address::from_typed_node_id(*from, network_id)),
                    to: Arc::new(Address::from_typed_node_id(*to, network_id)),
                    transferred: ResourceSpecifier::from_native(transferred, network_id),
                }
            }
            CoreExecutionTransactionType::Transfer(value) => {
                let CoreExecutionTransferTransactionType { from, transfers } = value.as_ref();

                Self::Transfer {
                    from: Arc::new(Address::from_typed_node_id(*from, network_id)),
                    transfers: transfers
                        .iter()
                        .map(|(key, value)| {
                            (
                                Address::from_typed_node_id(*key, network_id).as_str(),
                                value
                                    .iter()
                                    .map(|(key, value)| {
                                        (
                                            Address::from_typed_node_id(*key, network_id).as_str(),
                                            Resources::from_native(value),
                                        )
                                    })
                                    .collect(),
                            )
                        })
                        .collect(),
                }
            }
            CoreExecutionTransactionType::AccountDepositSettings(value) => {
                let CoreExecutionAccountDepositSettingsTransactionType {
                    resource_preference_changes,
                    default_deposit_rule_changes,
                    authorized_depositors_changes,
                } = value.as_ref();

                Self::AccountDepositSettings {
                    resource_preference_changes: resource_preference_changes
                        .iter()
                        .map(|(key, value)| {
                            (
                                Arc::new(Address::from_typed_node_id(*key, network_id)).as_str(),
                                value
                                    .iter()
                                    .map(|(key, value)| {
                                        (
                                            Arc::new(Address::from_typed_node_id(*key, network_id))
                                                .as_str(),
                                            <ResourcePreferenceAction as FromNative>::from_native(
                                                *value,
                                            ),
                                        )
                                    })
                                    .collect(),
                            )
                        })
                        .collect(),
                    default_deposit_rule_changes: default_deposit_rule_changes
                        .iter()
                        .map(|(key, value)| {
                            (
                                Arc::new(Address::from_typed_node_id(*key, network_id)).as_str(),
                                <AccountDefaultDepositRule as FromNative>::from_native(
                                    *value,
                                ),
                            )
                        })
                        .collect(),
                        authorized_depositors_changes: authorized_depositors_changes
                        .iter()
                        .map(|(key, value)| {
                            (
                                Arc::new(Address::from_typed_node_id(*key, network_id)).as_str(),
                                <AuthorizedDepositorsChanges as FromNativeWithNetworkContext>::from_native(
                                    value.clone(),
                                    network_id
                                ),
                            )
                        })
                        .collect(),
                }
            }
            CoreExecutionTransactionType::GeneralTransaction(value) => {
                let CoreExecutionGeneralTransactionType {
                    account_proofs,
                    account_withdraws,
                    account_deposits,
                    addresses_in_manifest: (addresses_in_manifest, _),
                    metadata_of_newly_created_entities,
                    data_of_newly_minted_non_fungibles,
                    addresses_of_newly_created_entities,
                } = value.as_ref();

                Self::GeneralTransaction {
                    account_proofs: account_proofs
                        .iter()
                        .map(|value| Arc::new(Address::from_typed_node_id(*value, network_id)))
                        .collect(),
                    account_withdraws: account_withdraws
                        .iter()
                        .map(|(key, value)| {
                            (
                                Address::from_typed_node_id(*key, network_id).as_str(),
                                value
                                    .iter()
                                    .map(|value| ResourceTracker::from_native(value, network_id))
                                    .collect(),
                            )
                        })
                        .collect(),
                    account_deposits: account_deposits
                        .iter()
                        .map(|(key, value)| {
                            (
                                Address::from_typed_node_id(*key, network_id).as_str(),
                                value
                                    .iter()
                                    .map(|value| ResourceTracker::from_native(value, network_id))
                                    .collect(),
                            )
                        })
                        .collect(),
                    addresses_in_manifest: {
                        let mut map = HashMap::<EntityType, Vec<Arc<Address>>>::new();
                        for address in addresses_in_manifest {
                            let entity_type = EntityType::from(address.entity_type());
                            let address =
                                Arc::new(Address::from_typed_node_id(*address, network_id));
                            map.entry(entity_type).or_default().push(address);
                        }
                        map
                    },
                    metadata_of_newly_created_entities: metadata_of_newly_created_entities
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
                                                MetadataValue::from_native(value, network_id)
                                            }),
                                        )
                                    })
                                    .collect(),
                            )
                        })
                        .collect(),
                    data_of_newly_minted_non_fungibles: data_of_newly_minted_non_fungibles
                        .iter()
                        .map(|(key, value)| {
                            (
                                Address::from_typed_node_id(*key, network_id).as_str(),
                                value
                                    .iter()
                                    .map(|(key, value)| {
                                        (key.clone().into(), native_scrypto_encode(value).unwrap())
                                    })
                                    .collect(),
                            )
                        })
                        .collect(),
                    addresses_of_newly_created_entities: addresses_of_newly_created_entities
                        .iter()
                        .map(|node_id| Arc::new(Address::from_typed_node_id(*node_id, network_id)))
                        .collect(),
                }
            }
        }
    }
}

impl FeeLocks {
    pub fn from_native(
        CoreExecutionFeeLocks {
            contingent_lock,
            lock,
        }: &CoreExecutionFeeLocks,
    ) -> Self {
        Self {
            contingent_lock: Arc::new(Decimal(*contingent_lock)),
            lock: Arc::new(Decimal(*lock)),
        }
    }
}

impl FeeSummary {
    pub fn from_native(
        CoreExecutionFeeSummary {
            execution_cost,
            royalty_cost,
            finalization_cost,
            storage_expansion_cost,
        }: &CoreExecutionFeeSummary,
    ) -> Self {
        Self {
            execution_cost: Arc::new(Decimal(*execution_cost)),
            royalty_cost: Arc::new(Decimal(*royalty_cost)),
            finalization_cost: Arc::new(Decimal(*finalization_cost)),
            storage_expansion_cost: Arc::new(Decimal(*storage_expansion_cost)),
        }
    }
}

#[derive(Clone, Debug, Enum)]
pub enum ResourceTracker {
    Fungible {
        resource_address: Arc<Address>,
        amount: DecimalSource,
    },
    NonFungible {
        resource_address: Arc<Address>,
        amount: DecimalSource,
        ids: NonFungibleLocalIdVecSource,
    },
}

impl ResourceTracker {
    pub fn from_native(resource_tracker: &CoreResourceTracker, network_id: u8) -> Self {
        match resource_tracker {
            CoreResourceTracker::Fungible {
                resource_address,
                amount,
            } => Self::Fungible {
                resource_address: Arc::new(Address::from_typed_node_id(
                    *resource_address,
                    network_id,
                )),
                amount: match amount {
                    CoreSource::Guaranteed(value) => DecimalSource::Guaranteed {
                        value: Arc::new(Decimal(*value)),
                    },
                    CoreSource::Predicted(index, value) => DecimalSource::Predicted {
                        instruction_index: *index as u64,
                        value: Arc::new(Decimal(*value)),
                    },
                },
            },
            CoreResourceTracker::NonFungible {
                resource_address,
                amount,
                ids,
            } => Self::NonFungible {
                resource_address: Arc::new(Address::from_typed_node_id(
                    *resource_address,
                    network_id,
                )),
                amount: match amount {
                    CoreSource::Guaranteed(value) => DecimalSource::Guaranteed {
                        value: Arc::new(Decimal(*value)),
                    },
                    CoreSource::Predicted(index, value) => DecimalSource::Predicted {
                        instruction_index: *index as u64,
                        value: Arc::new(Decimal(*value)),
                    },
                },
                ids: match ids {
                    CoreSource::Guaranteed(value) => NonFungibleLocalIdVecSource::Guaranteed {
                        value: value.iter().cloned().map(Into::into).collect(),
                    },
                    CoreSource::Predicted(index, value) => NonFungibleLocalIdVecSource::Predicted {
                        instruction_index: *index as u64,
                        value: value.iter().cloned().map(Into::into).collect(),
                    },
                },
            },
        }
    }
}

#[derive(Clone, Debug, Enum)]
pub enum ResourcePreferenceAction {
    Set { value: ResourcePreference },
    Remove,
}

#[derive(Clone, Debug, Enum)]
pub enum ResourcePreference {
    Allowed,
    Disallowed,
}

#[derive(Clone, Debug, Enum)]
pub enum AccountDefaultDepositRule {
    Accept,
    Reject,
    AllowExisting,
}

#[derive(Clone, Debug, Record)]
pub struct AuthorizedDepositorsChanges {
    pub added: Vec<ResourceOrNonFungible>,
    pub removed: Vec<ResourceOrNonFungible>,
}

impl FromNative for ResourcePreferenceAction {
    type Native = CoreResourcePreferenceAction;

    fn from_native(native: Self::Native) -> Self {
        match native {
            Self::Native::Set(value) => Self::Set {
                value: <ResourcePreference as FromNative>::from_native(value),
            },
            Self::Native::Remove => Self::Remove,
        }
    }
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

impl FromNativeWithNetworkContext for AuthorizedDepositorsChanges {
    type Native = CoreAuthorizedDepositorsChanges;

    fn from_native(native: Self::Native, network_id: u8) -> Self {
        Self {
            added: native
                .added
                .into_iter()
                .map(|value| FromNativeWithNetworkContext::from_native(value, network_id))
                .collect(),
            removed: native
                .removed
                .into_iter()
                .map(|value| FromNativeWithNetworkContext::from_native(value, network_id))
                .collect(),
        }
    }
}

#[derive(Clone, Debug, Enum)]
pub enum ReservedInstruction {
    AccountLockFee,
    AccountSecurify,
    IdentitySecurify,
    AccountUpdateSettings,
    AccessController,
}

impl From<ReservedInstruction> for CoreReservedInstruction {
    fn from(value: ReservedInstruction) -> Self {
        match value {
            ReservedInstruction::AccessController => Self::AccessController,
            ReservedInstruction::AccountLockFee => Self::AccountLockFee,
            ReservedInstruction::AccountSecurify => Self::AccountSecurify,
            ReservedInstruction::IdentitySecurify => Self::IdentitySecurify,
            ReservedInstruction::AccountUpdateSettings => Self::AccountUpdateSettings,
        }
    }
}

impl From<CoreReservedInstruction> for ReservedInstruction {
    fn from(value: CoreReservedInstruction) -> Self {
        match value {
            CoreReservedInstruction::AccessController => Self::AccessController,
            CoreReservedInstruction::AccountLockFee => Self::AccountLockFee,
            CoreReservedInstruction::AccountSecurify => Self::AccountSecurify,
            CoreReservedInstruction::IdentitySecurify => Self::IdentitySecurify,
            CoreReservedInstruction::AccountUpdateSettings => Self::AccountUpdateSettings,
        }
    }
}

macro_rules! define_source_enum {
    ($type: ty) => {
        paste::paste! {
            define_source_enum!($type, [< $type Source >])
        }
    };
    ($type: ty, $type_ident: ident) => {
        #[derive(Clone, Debug, Enum)]
        pub enum $type_ident {
            Guaranteed {
                value: $type,
            },
            Predicted {
                instruction_index: u64,
                value: $type,
            },
        }
    };
}
define_source_enum!(Arc<Decimal>, DecimalSource);
define_source_enum!(Vec<NonFungibleLocalId>, NonFungibleLocalIdVecSource);

#[derive(Clone, Debug, Record)]
pub struct TransactionManifestModifications {
    pub add_access_controller_proofs: Vec<Arc<Address>>,
    pub add_lock_fee: Option<LockFeeModification>,
    pub add_assertions: Vec<IndexedAssertion>,
}

#[derive(Clone, Debug, Enum)]
pub enum Assertion {
    Amount {
        resource_address: Arc<Address>,
        amount: Arc<Decimal>,
    },
    Ids {
        resource_address: Arc<Address>,
        ids: Vec<NonFungibleLocalId>,
    },
}

#[derive(Clone, Debug, Record)]
pub struct IndexedAssertion {
    pub index: u64,
    pub assertion: Assertion,
}

#[derive(Clone, Debug, Record)]
pub struct LockFeeModification {
    pub account_address: Arc<Address>,
    pub amount: Arc<Decimal>,
}

impl ToNative for TransactionManifestModifications {
    type Native = CoreManifestTransactionManifestModifications;

    fn to_native(self) -> Result<Self::Native> {
        Ok(Self::Native {
            add_access_controller_proofs: self
                .add_access_controller_proofs
                .into_iter()
                .map(|value| (*value).try_into())
                .collect::<Result<_>>()?,
            add_assertions: self
                .add_assertions
                .into_iter()
                .map(|IndexedAssertion { index, assertion }| {
                    assertion
                        .to_native()
                        .map(|assertion| (index as usize, assertion))
                })
                .collect::<Result<_>>()?,
            add_lock_fee: if let Some(LockFeeModification {
                account_address,
                amount,
            }) = self.add_lock_fee
            {
                Some(((*account_address).try_into()?, amount.0))
            } else {
                None
            },
        })
    }
}

impl ToNative for Assertion {
    type Native = CoreManifestAssertion;

    fn to_native(self) -> Result<Self::Native> {
        match self {
            Self::Amount {
                resource_address,
                amount,
            } => Ok(Self::Native::Amount {
                resource_address: (*resource_address).try_into()?,
                amount: amount.0,
            }),
            Self::Ids {
                resource_address,
                ids,
            } => Ok(Self::Native::Ids {
                resource_address: (*resource_address).try_into()?,
                ids: ids
                    .into_iter()
                    .map(NativeNonFungibleLocalId::try_from)
                    .collect::<Result<_>>()?,
            }),
        }
    }
}
