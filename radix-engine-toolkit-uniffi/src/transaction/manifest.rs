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
        let addresses = core_instructions_extract_addresses(&self.instructions.0);

        let mut map = HashMap::<EntityType, Vec<Arc<Address>>>::new();
        for address in addresses {
            let entity_type = EntityType::from(address.entity_type().unwrap());
            let address = Arc::new(Address(address, network_id));
            map.entry(entity_type).or_default().push(address);
        }
        map
    }

    pub fn identities_requiring_auth(&self) -> Vec<Arc<Address>> {
        core_instructions_identities_requiring_auth(&self.instructions.0)
            .into_iter()
            .map(|address| Arc::new(Address::from_node_id(address, self.instructions.1)))
            .collect()
    }

    pub fn accounts_requiring_auth(&self) -> Vec<Arc<Address>> {
        core_instructions_accounts_requiring_auth(&self.instructions.0)
            .into_iter()
            .map(|address| Arc::new(Address::from_node_id(address, self.instructions.1)))
            .collect()
    }

    pub fn accounts_withdrawn_from(&self) -> Vec<Arc<Address>> {
        core_instructions_accounts_withdrawn_from(&self.instructions.0)
            .into_iter()
            .map(|address| Arc::new(Address::from_node_id(address, self.instructions.1)))
            .collect()
    }

    pub fn accounts_deposited_into(&self) -> Vec<Arc<Address>> {
        core_instructions_accounts_deposited_into(&self.instructions.0)
            .into_iter()
            .map(|address| Arc::new(Address::from_node_id(address, self.instructions.1)))
            .collect()
    }

    pub fn analyze_execution(&self, transaction_receipt: Vec<u8>) -> Result<ExecutionAnalysis> {
        let receipt = native_scrypto_decode::<NativeTransactionReceipt>(&transaction_receipt)?;
        let analysis = core_execution_analyze(&self.instructions.0, &receipt)?;
        Ok(ExecutionAnalysis::from_native(
            &analysis,
            self.instructions.1,
        ))
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
pub struct ExecutionAnalysis {
    pub fee_locks: FeeLocks,
    pub fee_summary: FeeSummary,
    pub transaction_type: TransactionType,
}

#[derive(Clone, Debug, Record)]
pub struct FeeSummary {
    pub network_fee: Arc<Decimal>,
    pub royalty_fee: Arc<Decimal>,
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
    GeneralTransaction {
        account_proofs: Vec<Arc<Address>>,
        account_withdraws: HashMap<String, Vec<ResourceSpecifier>>,
        account_deposits: HashMap<String, Vec<Source>>,
        addresses_in_manifest: HashMap<EntityType, Vec<Arc<Address>>>,
        metadata_of_newly_created_entities: HashMap<String, HashMap<String, MetadataValue>>,
        data_of_newly_minted_non_fungibles: HashMap<String, HashMap<NonFungibleLocalId, Vec<u8>>>,
    },
    NonConforming,
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
pub enum Source {
    Guaranteed {
        value: ResourceSpecifier,
    },
    Predicted {
        instruction_index: u64,
        value: ResourceSpecifier,
    },
}

#[derive(Clone, Debug, Enum)]
pub enum Resources {
    Amount { amount: Arc<Decimal> },
    Ids { ids: Vec<NonFungibleLocalId> },
}

impl ExecutionAnalysis {
    pub fn from_native(
        CoreExecutionExecutionAnalysis {
            fee_locks,
            fee_summary,
            transaction_type,
        }: &CoreExecutionExecutionAnalysis,
        network_id: u8,
    ) -> Self {
        Self {
            transaction_type: TransactionType::from_native(transaction_type, network_id),
            fee_locks: FeeLocks::from_native(fee_locks),
            fee_summary: FeeSummary::from_native(fee_summary),
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
                resource_address: Arc::new(Address(resource_address.into_node_id(), network_id)),
                amount: Arc::new(Decimal(*amount)),
            },
            NativeResourceSpecifier::Ids(resource_address, ids) => Self::Ids {
                resource_address: Arc::new(Address(resource_address.into_node_id(), network_id)),
                ids: ids.iter().cloned().map(Into::into).collect(),
            },
        }
    }
}

impl Source {
    pub fn from_native(native: &CoreSource<NativeResourceSpecifier>, network_id: u8) -> Self {
        match native {
            CoreSource::Guaranteed(value) => Source::Guaranteed {
                value: ResourceSpecifier::from_native(value, network_id),
            },
            CoreSource::Predicted(instruction_index, value) => Source::Predicted {
                instruction_index: *instruction_index as u64,
                value: ResourceSpecifier::from_native(value, network_id),
            },
        }
    }
}

impl TransactionType {
    pub fn from_native(native: &CoreExecutionTransactionType, network_id: u8) -> Self {
        match native {
            CoreExecutionTransactionType::NonConforming => Self::NonConforming,
            CoreExecutionTransactionType::SimpleTransfer(value) => {
                let CoreExecutionSimpleTransferTransactionType {
                    from,
                    to,
                    transferred,
                } = value.as_ref();

                Self::SimpleTransfer {
                    from: Arc::new(Address::from_node_id(*from, network_id)),
                    to: Arc::new(Address::from_node_id(*to, network_id)),
                    transferred: ResourceSpecifier::from_native(transferred, network_id),
                }
            }
            CoreExecutionTransactionType::Transfer(value) => {
                let CoreExecutionTransferTransactionType { from, transfers } = value.as_ref();

                Self::Transfer {
                    from: Arc::new(Address::from_node_id(*from, network_id)),
                    transfers: transfers
                        .iter()
                        .map(|(key, value)| {
                            (
                                Address::from_node_id(*key, network_id).as_str(),
                                value
                                    .iter()
                                    .map(|(key, value)| {
                                        (
                                            Address::from_node_id(*key, network_id).as_str(),
                                            Resources::from_native(value),
                                        )
                                    })
                                    .collect(),
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
                    addresses_in_manifest,
                    metadata_of_newly_created_entities,
                    data_of_newly_minted_non_fungibles,
                } = value.as_ref();

                Self::GeneralTransaction {
                    account_proofs: account_proofs
                        .iter()
                        .map(|value| Arc::new(Address::from_node_id(*value, network_id)))
                        .collect(),
                    account_withdraws: account_withdraws
                        .iter()
                        .map(|(key, value)| {
                            (
                                Address::from_node_id(*key, network_id).as_str(),
                                value
                                    .iter()
                                    .map(|value| ResourceSpecifier::from_native(value, network_id))
                                    .collect(),
                            )
                        })
                        .collect(),
                    account_deposits: account_deposits
                        .iter()
                        .map(|(key, value)| {
                            (
                                Address::from_node_id(*key, network_id).as_str(),
                                value
                                    .iter()
                                    .map(|value| Source::from_native(value, network_id))
                                    .collect(),
                            )
                        })
                        .collect(),
                    addresses_in_manifest: {
                        let mut map = HashMap::<EntityType, Vec<Arc<Address>>>::new();
                        for address in addresses_in_manifest {
                            let entity_type = EntityType::from(address.entity_type().unwrap());
                            let address = Arc::new(Address(*address, network_id));
                            map.entry(entity_type).or_default().push(address);
                        }
                        map
                    },
                    metadata_of_newly_created_entities: metadata_of_newly_created_entities
                        .iter()
                        .map(|(key, value)| {
                            (
                                Address::from_node_id(*key, network_id).as_str(),
                                value
                                    .iter()
                                    .map(|(key, value)| {
                                        (key.clone(), MetadataValue::from_native(value, network_id))
                                    })
                                    .collect(),
                            )
                        })
                        .collect(),
                    data_of_newly_minted_non_fungibles: data_of_newly_minted_non_fungibles
                        .iter()
                        .map(|(key, value)| {
                            (
                                Address::from_node_id(*key, network_id).as_str(),
                                value
                                    .iter()
                                    .map(|(key, value)| {
                                        (key.clone().into(), native_scrypto_encode(value).unwrap())
                                    })
                                    .collect(),
                            )
                        })
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
            network_fee,
            royalty_fee,
        }: &CoreExecutionFeeSummary,
    ) -> Self {
        Self {
            network_fee: Arc::new(Decimal(*network_fee)),
            royalty_fee: Arc::new(Decimal(*royalty_fee)),
        }
    }
}
