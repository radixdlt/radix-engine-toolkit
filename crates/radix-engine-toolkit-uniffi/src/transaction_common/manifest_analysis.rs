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
use sbor::prelude::{HashMap, Vec};

#[derive(Clone, Debug, Record)]
pub struct StaticAnalysis {
    pub account_interactions_summary: AccountInteractionsOutput,
    pub account_static_resource_movements_summary:
        AccountStaticResourceMovementsOutput,
    pub proofs_created_summary: PresentedProofsOutput,
    pub entities_encountered_summary: EncounteredEntitiesOutput,
    pub entities_requiring_auth_summary: EntitiesRequiringAuthOutput,
    pub reserved_instructions_summary: ReservedInstructionsOutput,
    pub manifest_classification: Vec<ManifestClassification>,
}

impl FromNativeWithNetworkContext for StaticAnalysis {
    type Native = toolkit::StaticAnalysis;

    fn from_native(
        Self::Native {
            account_interactions_summary,
            account_static_resource_movements_summary,
            proofs_created_summary,
            entities_encountered_summary,
            entities_requiring_auth_summary,
            reserved_instructions_summary,
            manifest_classification,
        }: Self::Native,
        network_id: u8,
    ) -> Self {
        Self {
            account_interactions_summary:
                FromNativeWithNetworkContext::from_native(
                    account_interactions_summary,
                    network_id,
                ),
            account_static_resource_movements_summary:
                FromNativeWithNetworkContext::from_native(
                    account_static_resource_movements_summary,
                    network_id,
                ),
            proofs_created_summary: FromNativeWithNetworkContext::from_native(
                proofs_created_summary,
                network_id,
            ),
            entities_encountered_summary:
                FromNativeWithNetworkContext::from_native(
                    entities_encountered_summary,
                    network_id,
                ),
            entities_requiring_auth_summary:
                FromNativeWithNetworkContext::from_native(
                    entities_requiring_auth_summary,
                    network_id,
                ),
            reserved_instructions_summary:
                FromNativeWithNetworkContext::from_native(
                    reserved_instructions_summary,
                    network_id,
                ),
            manifest_classification: manifest_classification
                .into_iter()
                .map(|value| {
                    FromNativeWithNetworkContext::from_native(value, network_id)
                })
                .collect(),
        }
    }
}

#[derive(Clone, Debug, Record)]
pub struct DynamicAnalysis {
    pub account_interactions_summary: AccountInteractionsOutput,
    pub account_static_resource_movements_summary:
        AccountStaticResourceMovementsOutput,
    pub account_dynamic_resource_movements_summary:
        AccountDynamicResourceMovementsOutput,
    pub proofs_created_summary: PresentedProofsOutput,
    pub entities_newly_created_summary: NewEntitiesOutput,
    pub entities_encountered_summary: EncounteredEntitiesOutput,
    pub entities_requiring_auth_summary: EntitiesRequiringAuthOutput,
    pub reserved_instructions_summary: ReservedInstructionsOutput,
    pub fee_locks_summary: FeeLocks,
    pub fee_consumption_summary: FeeSummary,
    pub detailed_manifest_classification: Vec<DetailedManifestClassification>,
}

impl FromNativeWithNetworkContext for DynamicAnalysis {
    type Native = toolkit::DynamicAnalysis;

    fn from_native(
        Self::Native {
            account_interactions_summary,
            account_static_resource_movements_summary,
            account_dynamic_resource_movements_summary,
            proofs_created_summary,
            entities_newly_created_summary,
            entities_encountered_summary,
            entities_requiring_auth_summary,
            reserved_instructions_summary,
            fee_locks_summary,
            fee_consumption_summary,
            detailed_manifest_classification,
        }: Self::Native,
        network_id: u8,
    ) -> Self {
        Self {
            account_interactions_summary:
                FromNativeWithNetworkContext::from_native(
                    account_interactions_summary,
                    network_id,
                ),
            account_static_resource_movements_summary:
                FromNativeWithNetworkContext::from_native(
                    account_static_resource_movements_summary,
                    network_id,
                ),
            account_dynamic_resource_movements_summary:
                FromNativeWithNetworkContext::from_native(
                    account_dynamic_resource_movements_summary,
                    network_id,
                ),
            proofs_created_summary: FromNativeWithNetworkContext::from_native(
                proofs_created_summary,
                network_id,
            ),
            entities_newly_created_summary:
                FromNativeWithNetworkContext::from_native(
                    entities_newly_created_summary,
                    network_id,
                ),
            entities_encountered_summary:
                FromNativeWithNetworkContext::from_native(
                    entities_encountered_summary,
                    network_id,
                ),
            entities_requiring_auth_summary:
                FromNativeWithNetworkContext::from_native(
                    entities_requiring_auth_summary,
                    network_id,
                ),
            reserved_instructions_summary:
                FromNativeWithNetworkContext::from_native(
                    reserved_instructions_summary,
                    network_id,
                ),
            fee_locks_summary: FromNativeWithNetworkContext::from_native(
                fee_locks_summary,
                network_id,
            ),
            fee_consumption_summary: FromNativeWithNetworkContext::from_native(
                fee_consumption_summary,
                network_id,
            ),
            detailed_manifest_classification: detailed_manifest_classification
                .into_iter()
                .map(|value| {
                    FromNativeWithNetworkContext::from_native(value, network_id)
                })
                .collect(),
        }
    }
}

#[derive(Clone, Debug, Record)]
pub struct AccountInteractionsOutput {
    pub accounts_securified: Vec<Arc<Address>>,
    pub accounts_deposited_into: Vec<Arc<Address>>,
    pub accounts_withdrawn_from: Vec<Arc<Address>>,
    pub accounts_locked_fees_from: Vec<Arc<Address>>,
    pub accounts_created_proofs_from: Vec<Arc<Address>>,
    pub accounts_burned_from: Vec<Arc<Address>>,
    pub accounts_set_default_deposit_rule_of: Vec<Arc<Address>>,
    pub accounts_set_resource_preference_into: Vec<Arc<Address>>,
    pub accounts_remove_resource_preference_from: Vec<Arc<Address>>,
    pub accounts_add_authorized_depositor_into: Vec<Arc<Address>>,
    pub accounts_remove_authorized_depositor_from: Vec<Arc<Address>>,
}

impl FromNativeWithNetworkContext for AccountInteractionsOutput {
    type Native = toolkit::AccountInteractionsOutput;

    fn from_native(
        Self::Native {
            accounts_securified,
            accounts_deposited_into,
            accounts_withdrawn_from,
            accounts_locked_fees_from,
            accounts_created_proofs_from,
            accounts_burned_from,
            accounts_set_default_deposit_rule_of,
            accounts_set_resource_preference_into,
            accounts_remove_resource_preference_from,
            accounts_add_authorized_depositor_into,
            accounts_remove_authorized_depositor_from,
        }: Self::Native,
        network_id: u8,
    ) -> Self {
        Self {
            accounts_securified: accounts_securified
                .into_iter()
                .filter_map(|value| value.into_static())
                .map(|value| Arc::new(Address::from_node_id(value, network_id)))
                .collect(),
            accounts_deposited_into: accounts_deposited_into
                .into_iter()
                .filter_map(|value| value.into_static())
                .map(|value| Arc::new(Address::from_node_id(value, network_id)))
                .collect(),
            accounts_withdrawn_from: accounts_withdrawn_from
                .into_iter()
                .filter_map(|value| value.into_static())
                .map(|value| Arc::new(Address::from_node_id(value, network_id)))
                .collect(),
            accounts_locked_fees_from: accounts_locked_fees_from
                .into_iter()
                .filter_map(|value| value.into_static())
                .map(|value| Arc::new(Address::from_node_id(value, network_id)))
                .collect(),
            accounts_created_proofs_from: accounts_created_proofs_from
                .into_iter()
                .filter_map(|value| value.into_static())
                .map(|value| Arc::new(Address::from_node_id(value, network_id)))
                .collect(),
            accounts_burned_from: accounts_burned_from
                .into_iter()
                .filter_map(|value| value.into_static())
                .map(|value| Arc::new(Address::from_node_id(value, network_id)))
                .collect(),
            accounts_set_default_deposit_rule_of:
                accounts_set_default_deposit_rule_of
                    .into_iter()
                    .filter_map(|value| value.into_static())
                    .map(|value| {
                        Arc::new(Address::from_node_id(value, network_id))
                    })
                    .collect(),
            accounts_set_resource_preference_into:
                accounts_set_resource_preference_into
                    .into_iter()
                    .filter_map(|value| value.into_static())
                    .map(|value| {
                        Arc::new(Address::from_node_id(value, network_id))
                    })
                    .collect(),
            accounts_remove_resource_preference_from:
                accounts_remove_resource_preference_from
                    .into_iter()
                    .filter_map(|value| value.into_static())
                    .map(|value| {
                        Arc::new(Address::from_node_id(value, network_id))
                    })
                    .collect(),
            accounts_add_authorized_depositor_into:
                accounts_add_authorized_depositor_into
                    .into_iter()
                    .filter_map(|value| value.into_static())
                    .map(|value| {
                        Arc::new(Address::from_node_id(value, network_id))
                    })
                    .collect(),
            accounts_remove_authorized_depositor_from:
                accounts_remove_authorized_depositor_from
                    .into_iter()
                    .filter_map(|value| value.into_static())
                    .map(|value| {
                        Arc::new(Address::from_node_id(value, network_id))
                    })
                    .collect(),
        }
    }
}

#[derive(Clone, Debug, Record)]
pub struct AccountStaticResourceMovementsOutput {
    pub account_withdraws: HashMap<String, Vec<AccountWithdraw>>,
    pub account_deposits: HashMap<String, Vec<AccountDeposit>>,
}

impl FromNativeWithNetworkContext for AccountStaticResourceMovementsOutput {
    type Native = toolkit::AccountStaticResourceMovementsOutput;

    fn from_native(
        Self::Native {
            account_withdraws,
            account_deposits,
        }: Self::Native,
        network_id: u8,
    ) -> Self {
        Self {
            account_withdraws: account_withdraws
                .into_iter()
                .map(|(account, withdraws)| {
                    let account = Address::from_node_id(account, network_id);
                    let withdraws = withdraws
                        .into_iter()
                        .map(|value| {
                            FromNativeWithNetworkContext::from_native(
                                value, network_id,
                            )
                        })
                        .collect();

                    (account.as_str(), withdraws)
                })
                .collect(),
            account_deposits: {
                let mut map = HashMap::<String, Vec<AccountDeposit>>::new();

                for (account_address, account_deposits) in
                    account_deposits.into_iter()
                {
                    let account_address =
                        Address::from_node_id(account_address, network_id)
                            .as_str();
                    for account_deposit in account_deposits.into_iter() {
                        for (resource_address, resource_bounds) in
                            account_deposit.specified_resources()
                        {
                            let account_deposit = match resource_bounds {
                                engine::SimpleResourceBounds::Fungible(
                                    simple_bounds,
                                ) => AccountDeposit::KnownFungible {
                                    resource_address: Arc::new(
                                        Address::from_node_id(
                                            *resource_address,
                                            network_id,
                                        ),
                                    ),
                                    bounds: FromNative::from_native(
                                        simple_bounds.clone(),
                                    ),
                                },
                                engine::SimpleResourceBounds::NonFungible(
                                    simple_bounds,
                                ) => AccountDeposit::KnownNonFungible {
                                    resource_address: Arc::new(
                                        Address::from_node_id(
                                            *resource_address,
                                            network_id,
                                        ),
                                    ),
                                    bounds: FromNative::from_native(
                                        simple_bounds.clone(),
                                    ),
                                },
                            };
                            map.entry(account_address.clone())
                                .or_default()
                                .push(account_deposit)
                        }
                        if let engine::UnspecifiedResources::MayBePresent(_) =
                            account_deposit.unspecified_resources()
                        {
                            map.entry(account_address.clone())
                                .or_default()
                                .push(AccountDeposit::Unknown)
                        }
                    }
                }

                map
            },
        }
    }
}

#[derive(Clone, Debug, Record)]
pub struct AccountDynamicResourceMovementsOutput {
    pub account_withdraws: HashMap<String, Vec<InvocationIoItem>>,
    pub account_deposits: HashMap<String, Vec<InvocationIoItem>>,
}

impl FromNativeWithNetworkContext for AccountDynamicResourceMovementsOutput {
    type Native = toolkit::AccountDynamicResourceMovementsOutput;

    fn from_native(
        Self::Native {
            account_withdraws,
            account_deposits,
        }: Self::Native,
        network_id: u8,
    ) -> Self {
        Self {
            account_withdraws: account_withdraws
                .into_iter()
                .map(|(account, invocation_ios)| {
                    let account = Address::from_node_id(account, network_id);
                    let invocation_ios = invocation_ios
                        .into_iter()
                        .map(|value| {
                            FromNativeWithNetworkContext::from_native(
                                value, network_id,
                            )
                        })
                        .collect();

                    (account.as_str(), invocation_ios)
                })
                .collect(),
            account_deposits: account_deposits
                .into_iter()
                .map(|(account, invocation_ios)| {
                    let account = Address::from_node_id(account, network_id);
                    let invocation_ios = invocation_ios
                        .into_iter()
                        .map(|value| {
                            FromNativeWithNetworkContext::from_native(
                                value, network_id,
                            )
                        })
                        .collect();

                    (account.as_str(), invocation_ios)
                })
                .collect(),
        }
    }
}

#[derive(Clone, Debug, Record)]
pub struct PresentedProofsOutput {
    pub created_proofs: HashMap<String, Vec<ResourceSpecifier>>,
}

impl FromNativeWithNetworkContext for PresentedProofsOutput {
    type Native = toolkit::PresentedProofsOutput;

    fn from_native(
        Self::Native { created_proofs }: Self::Native,
        network_id: u8,
    ) -> Self {
        Self {
            created_proofs: created_proofs.into_iter().fold(
                Default::default(),
                |mut map, (account, proofs)| {
                    let Some(account) = account.into_static() else {
                        return map;
                    };
                    let account = Address::from_node_id(account, network_id);

                    for proof in proofs {
                        let Ok(proof) = engine::ResourceSpecifier::try_from(
                            proof,
                        )
                        .map(|value| {
                            FromNativeWithNetworkContext::from_native(
                                value, network_id,
                            )
                        }) else {
                            return map;
                        };

                        map.entry(account.as_str()).or_default().push(proof);
                    }

                    map
                },
            ),
        }
    }
}

#[derive(Clone, Debug, Record)]
pub struct EncounteredEntitiesOutput {
    pub entities: Vec<Arc<Address>>,
}

impl FromNativeWithNetworkContext for EncounteredEntitiesOutput {
    type Native = toolkit::EncounteredEntitiesOutput;

    fn from_native(
        Self::Native { entities }: Self::Native,
        network_id: u8,
    ) -> Self {
        Self {
            entities: entities
                .into_iter()
                .filter_map(|value| value.into_static())
                .map(|value| Arc::new(Address::from_node_id(value, network_id)))
                .collect(),
        }
    }
}

#[derive(Clone, Debug, Record)]
pub struct EntitiesRequiringAuthOutput {
    pub accounts: Vec<Arc<Address>>,
    pub identities: Vec<Arc<Address>>,
}

impl FromNativeWithNetworkContext for EntitiesRequiringAuthOutput {
    type Native = toolkit::EntitiesRequiringAuthOutput;

    fn from_native(
        Self::Native {
            accounts,
            identities,
        }: Self::Native,
        network_id: u8,
    ) -> Self {
        Self {
            accounts: accounts
                .into_iter()
                .filter_map(|value| value.into_static())
                .map(|value| Arc::new(Address::from_node_id(value, network_id)))
                .collect(),
            identities: identities
                .into_iter()
                .filter_map(|value| value.into_static())
                .map(|value| Arc::new(Address::from_node_id(value, network_id)))
                .collect(),
        }
    }
}

#[derive(Clone, Debug, Record)]
pub struct NewEntitiesOutput {
    pub new_global_entities: Vec<Arc<Address>>,
    pub new_internal_entities: Vec<Arc<Address>>,
    pub new_component_entities: Vec<Arc<Address>>,
    pub new_resource_entities: Vec<Arc<Address>>,
    pub new_package_entities: Vec<Arc<Address>>,
    pub new_non_fungibles: Vec<Arc<NonFungibleGlobalId>>,
    pub global_entities_metadata:
        HashMap<String, HashMap<String, Option<MetadataValue>>>,
}

impl FromNativeWithNetworkContext for NewEntitiesOutput {
    type Native = toolkit::NewEntitiesOutput;

    fn from_native(
        Self::Native {
            new_global_entities,
            new_internal_entities,
            new_component_entities,
            new_resource_entities,
            new_package_entities,
            new_non_fungibles,
            global_entities_metadata,
        }: Self::Native,
        network_id: u8,
    ) -> Self {
        Self {
            new_global_entities: new_global_entities
                .into_iter()
                .map(|value| Arc::new(Address::from_node_id(value, network_id)))
                .collect(),
            new_internal_entities: new_internal_entities
                .into_iter()
                .map(|value| Arc::new(Address::from_node_id(value, network_id)))
                .collect(),
            new_component_entities: new_component_entities
                .into_iter()
                .map(|value| Arc::new(Address::from_node_id(value, network_id)))
                .collect(),
            new_resource_entities: new_resource_entities
                .into_iter()
                .map(|value| Arc::new(Address::from_node_id(value, network_id)))
                .collect(),
            new_package_entities: new_package_entities
                .into_iter()
                .map(|value| Arc::new(Address::from_node_id(value, network_id)))
                .collect(),
            new_non_fungibles: new_non_fungibles
                .into_iter()
                .map(|value| {
                    Arc::new(FromNativeWithNetworkContext::from_native(value, network_id))
                })
                .collect(),
            global_entities_metadata: global_entities_metadata
                .into_iter()
                .map(|(address, metadata_updates)| {
                    (
                        Arc::new(Address::from_node_id(address, network_id)).as_str(),
                        metadata_updates
                            .into_iter()
                            .map(|(key, value)| {
                                (
                                    key,
                                    value.map(|value| {
                                        FromNativeWithNetworkContext::from_native(
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

#[derive(Clone, Debug, Record)]
pub struct ReservedInstructionsOutput {
    pub account_lock_fee_invocations: Vec<Arc<Address>>,
    pub account_securify_invocations: Vec<Arc<Address>>,
    pub account_lock_owner_keys_metadata_field_invocations: Vec<Arc<Address>>,
    pub account_update_owner_keys_metadata_field_invocations: Vec<Arc<Address>>,
    pub identity_securify_invocations: Vec<Arc<Address>>,
    pub identity_lock_owner_keys_metadata_field_invocations: Vec<Arc<Address>>,
    pub identity_update_owner_keys_metadata_field_invocations:
        Vec<Arc<Address>>,
    pub access_controller_invocations: Vec<Arc<Address>>,
}

impl FromNativeWithNetworkContext for ReservedInstructionsOutput {
    type Native = toolkit::ReservedInstructionsOutput;

    fn from_native(
        Self::Native {
            account_lock_fee_invocations,
            account_securify_invocations,
            account_lock_owner_keys_metadata_field_invocations,
            account_update_owner_keys_metadata_field_invocations,
            identity_securify_invocations,
            identity_lock_owner_keys_metadata_field_invocations,
            identity_update_owner_keys_metadata_field_invocations,
            access_controller_invocations,
        }: Self::Native,
        network_id: u8,
    ) -> Self {
        Self {
            account_lock_fee_invocations: account_lock_fee_invocations
                .into_iter()
                .flat_map(|value| value.into_static())
                .map(|address| {
                    Arc::new(Address::from_node_id(address, network_id))
                })
                .collect(),
            account_securify_invocations: account_securify_invocations
                .into_iter()
                .flat_map(|value| value.into_static())
                .map(|address| {
                    Arc::new(Address::from_node_id(address, network_id))
                })
                .collect(),
            account_lock_owner_keys_metadata_field_invocations:
                account_lock_owner_keys_metadata_field_invocations
                    .into_iter()
                    .flat_map(|value| value.into_static())
                    .map(|address| {
                        Arc::new(Address::from_node_id(address, network_id))
                    })
                    .collect(),
            account_update_owner_keys_metadata_field_invocations:
                account_update_owner_keys_metadata_field_invocations
                    .into_iter()
                    .flat_map(|value| value.into_static())
                    .map(|address| {
                        Arc::new(Address::from_node_id(address, network_id))
                    })
                    .collect(),
            identity_securify_invocations: identity_securify_invocations
                .into_iter()
                .flat_map(|value| value.into_static())
                .map(|address| {
                    Arc::new(Address::from_node_id(address, network_id))
                })
                .collect(),
            identity_lock_owner_keys_metadata_field_invocations:
                identity_lock_owner_keys_metadata_field_invocations
                    .into_iter()
                    .flat_map(|value| value.into_static())
                    .map(|address| {
                        Arc::new(Address::from_node_id(address, network_id))
                    })
                    .collect(),
            identity_update_owner_keys_metadata_field_invocations:
                identity_update_owner_keys_metadata_field_invocations
                    .into_iter()
                    .flat_map(|value| value.into_static())
                    .map(|address| {
                        Arc::new(Address::from_node_id(address, network_id))
                    })
                    .collect(),
            access_controller_invocations: access_controller_invocations
                .into_iter()
                .flat_map(|value| value.into_static())
                .map(|address| {
                    Arc::new(Address::from_node_id(address, network_id))
                })
                .collect(),
        }
    }
}

#[derive(Clone, Debug, Enum)]
pub enum ManifestClassification {
    General,
    GeneralSubintent,
    Transfer,
    ValidatorStake,
    ValidatorUnstake,
    ValidatorClaimXrd,
    PoolContribution,
    PoolRedemption,
    AccountDepositSettingsUpdate,
    EntitySecurify,
    AccessControllerRecovery,
    AccessControllerStopTimedRecovery,
    AccessControllerConfirmTimedRecovery,
}

impl FromNative for ManifestClassification {
    type Native = toolkit::ManifestClassification;

    fn from_native(native: Self::Native) -> Self {
        match native {
            Self::Native::General => Self::General,
            Self::Native::GeneralSubintent => Self::GeneralSubintent,
            Self::Native::Transfer => Self::Transfer,
            Self::Native::ValidatorStake => Self::ValidatorStake,
            Self::Native::ValidatorUnstake => Self::ValidatorUnstake,
            Self::Native::ValidatorClaimXrd => Self::ValidatorClaimXrd,
            Self::Native::PoolContribution => Self::PoolContribution,
            Self::Native::PoolRedemption => Self::PoolRedemption,
            Self::Native::AccountDepositSettingsUpdate => {
                Self::AccountDepositSettingsUpdate
            }
            Self::Native::EntitySecurify => Self::EntitySecurify,
            Self::Native::AccessControllerRecovery => {
                Self::AccessControllerRecovery
            }
            Self::Native::AccessControllerStopTimedRecovery => {
                Self::AccessControllerStopTimedRecovery
            }
            Self::Native::AccessControllerConfirmTimedRecovery => {
                Self::AccessControllerConfirmTimedRecovery
            }
        }
    }
}

#[derive(Clone, Debug, Enum)]
pub enum DetailedManifestClassification {
    General,
    GeneralSubintent,
    Transfer {
        is_one_to_one_transfer: bool,
    },
    ValidatorStake {
        value: ValidatorStakingOutput,
    },
    ValidatorUnstake {
        value: ValidatorUnstakingOutput,
    },
    ValidatorClaimXrd {
        value: ValidatorClaimingXrdOutput,
    },
    PoolContribution {
        value: PoolContributionOutput,
    },
    PoolRedemption {
        value: PoolRedemptionOutput,
    },
    AccountDepositSettingsUpdate {
        value: AccountSettingsUpdateOutput,
    },
    EntitySecurify {
        value: EntitySecurifyOutput,
    },
    AccessControllerRecovery {
        value: AccessControllerRecoveryOutput,
    },
    AccessControllerStopTimedRecovery {
        value: AccessControllerStopTimedRecoveryOutput,
    },
    AccessControllerConfirmTimedRecovery {
        value: AccessControllerConfirmTimedRecoveryOutput,
    },
}

impl FromNativeWithNetworkContext for DetailedManifestClassification {
    type Native = toolkit::DetailedManifestClassification;

    fn from_native(native: Self::Native, network_id: u8) -> Self {
        match native {
            Self::Native::General => Self::General {},
            Self::Native::GeneralSubintent => Self::GeneralSubintent {},
            Self::Native::Transfer {
                is_one_to_one_transfer,
            } => Self::Transfer {
                is_one_to_one_transfer,
            },
            Self::Native::ValidatorStake(output) => Self::ValidatorStake {
                value: FromNativeWithNetworkContext::from_native(
                    output, network_id,
                ),
            },
            Self::Native::ValidatorUnstake(output) => Self::ValidatorUnstake {
                value: FromNativeWithNetworkContext::from_native(
                    output, network_id,
                ),
            },
            Self::Native::ValidatorClaimXrd(output) => {
                Self::ValidatorClaimXrd {
                    value: FromNativeWithNetworkContext::from_native(
                        output, network_id,
                    ),
                }
            }
            Self::Native::PoolContribution(output) => Self::PoolContribution {
                value: FromNativeWithNetworkContext::from_native(
                    output, network_id,
                ),
            },
            Self::Native::PoolRedemption(output) => Self::PoolRedemption {
                value: FromNativeWithNetworkContext::from_native(
                    output, network_id,
                ),
            },
            Self::Native::AccountDepositSettingsUpdate(output) => {
                Self::AccountDepositSettingsUpdate {
                    value: FromNativeWithNetworkContext::from_native(
                        output, network_id,
                    ),
                }
            }
            Self::Native::EntitySecurify(output) => Self::EntitySecurify {
                value: FromNativeWithNetworkContext::from_native(
                    output, network_id,
                ),
            },
            Self::Native::AccessControllerRecovery(output) => {
                Self::AccessControllerRecovery {
                    value: FromNativeWithNetworkContext::from_native(
                        output, network_id,
                    ),
                }
            }
            Self::Native::AccessControllerStopTimedRecovery(output) => {
                Self::AccessControllerStopTimedRecovery {
                    value: FromNativeWithNetworkContext::from_native(
                        output, network_id,
                    ),
                }
            }
            Self::Native::AccessControllerConfirmTimedRecovery(output) => {
                Self::AccessControllerConfirmTimedRecovery {
                    value: FromNativeWithNetworkContext::from_native(
                        output, network_id,
                    ),
                }
            }
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

impl FromNativeWithNetworkContext for ResourceSpecifier {
    type Native = engine::ResourceSpecifier;

    fn from_native(native: Self::Native, network_id: u8) -> Self {
        match native {
            engine::ResourceSpecifier::Amount(resource_address, decimal) => {
                Self::Amount {
                    resource_address: Arc::new(Address::from_node_id(
                        resource_address,
                        network_id,
                    )),
                    amount: Arc::new(Decimal(decimal)),
                }
            }
            engine::ResourceSpecifier::Ids(resource_address, ids) => {
                Self::Ids {
                    resource_address: Arc::new(Address::from_node_id(
                        resource_address,
                        network_id,
                    )),
                    ids: ids.into_iter().map(FromNative::from_native).collect(),
                }
            }
        }
    }
}

impl ResourceSpecifier {
    pub fn from_native_for_locker_blueprint(
        native: &radix_engine_interface::blueprints::locker::ResourceSpecifier,
        resource_address: &engine::ResourceAddress,
        network_id: u8,
    ) -> ResourceSpecifier {
        let address =
            Arc::new(Address::from_node_id(*resource_address, network_id));
        match native {
            radix_engine_interface::blueprints::locker::ResourceSpecifier::Fungible(amount) => {
                ResourceSpecifier::Amount {
                    resource_address: address,
                    amount: Arc::new(Decimal(*amount)),
                }
            }
            radix_engine_interface::blueprints::locker::ResourceSpecifier::NonFungible(ids) => {
                ResourceSpecifier::Ids {
                    resource_address: address,
                    ids: ids.clone().into_iter().map(From::from).collect(),
                }
            }
        }
    }
}

#[derive(Clone, Debug, Enum)]
pub enum AccountWithdraw {
    Amount {
        resource_address: Arc<Address>,
        amount: Arc<Decimal>,
    },
    Ids {
        resource_address: Arc<Address>,
        ids: Vec<NonFungibleLocalId>,
    },
}

impl FromNativeWithNetworkContext for AccountWithdraw {
    type Native = engine::AccountWithdraw;

    fn from_native(native: Self::Native, network_id: u8) -> Self {
        match native {
            engine::AccountWithdraw::Amount(resource_address, decimal) => {
                Self::Amount {
                    resource_address: Arc::new(Address::from_node_id(
                        resource_address,
                        network_id,
                    )),
                    amount: Arc::new(Decimal(decimal)),
                }
            }
            engine::AccountWithdraw::Ids(resource_address, ids) => Self::Ids {
                resource_address: Arc::new(Address::from_node_id(
                    resource_address,
                    network_id,
                )),
                ids: ids.into_iter().map(FromNative::from_native).collect(),
            },
        }
    }
}

#[derive(Clone, Debug, Enum)]
pub enum AccountDeposit {
    KnownFungible {
        resource_address: Arc<Address>,
        bounds: SimpleFungibleResourceBounds,
    },
    KnownNonFungible {
        resource_address: Arc<Address>,
        bounds: SimpleNonFungibleResourceBounds,
    },
    Unknown,
}

#[derive(Clone, Debug, Enum)]
pub enum SimpleFungibleResourceBounds {
    Exact {
        value: Arc<Decimal>,
    },
    AtMost {
        value: Arc<Decimal>,
    },
    AtLeast {
        value: Arc<Decimal>,
    },
    Between {
        lower_bound_inclusive: Arc<Decimal>,
        upper_bound_inclusive: Arc<Decimal>,
    },
    UnknownAmount,
}

impl FromNative for SimpleFungibleResourceBounds {
    type Native = engine::SimpleFungibleResourceBounds;

    fn from_native(native: Self::Native) -> Self {
        match native {
            engine::SimpleFungibleResourceBounds::Exact(decimal) => {
                Self::Exact {
                    value: Arc::new(Decimal(decimal)),
                }
            }
            engine::SimpleFungibleResourceBounds::AtMost(decimal) => {
                Self::AtMost {
                    value: Arc::new(Decimal(decimal)),
                }
            }
            engine::SimpleFungibleResourceBounds::AtLeast(decimal) => {
                Self::AtLeast {
                    value: Arc::new(Decimal(decimal)),
                }
            }
            engine::SimpleFungibleResourceBounds::Between(
                lower_bound,
                upper_bound,
            ) => Self::Between {
                lower_bound_inclusive: Arc::new(Decimal(lower_bound)),
                upper_bound_inclusive: Arc::new(Decimal(upper_bound)),
            },
            engine::SimpleFungibleResourceBounds::UnknownAmount => {
                Self::UnknownAmount
            }
        }
    }
}

#[derive(Clone, Debug, Enum)]
pub enum SimpleNonFungibleResourceBounds {
    Exact {
        amount: Arc<Decimal>,
        certain_ids: Vec<NonFungibleLocalId>,
    },
    NotExact {
        certain_ids: Vec<NonFungibleLocalId>,
        lower_bound: LowerBound,
        upper_bound: UpperBound,
        allowed_ids: AllowedIds,
    },
}

impl FromNative for SimpleNonFungibleResourceBounds {
    type Native = engine::SimpleNonFungibleResourceBounds;

    fn from_native(native: Self::Native) -> Self {
        match native {
            engine::SimpleNonFungibleResourceBounds::Exact {
                amount,
                certain_ids,
            } => Self::Exact {
                amount: Arc::new(Decimal(amount)),
                certain_ids: certain_ids
                    .into_iter()
                    .map(FromNative::from_native)
                    .collect(),
            },
            engine::SimpleNonFungibleResourceBounds::NotExact {
                certain_ids,
                lower_bound,
                upper_bound,
                allowed_ids,
            } => Self::NotExact {
                certain_ids: certain_ids
                    .into_iter()
                    .map(FromNative::from_native)
                    .collect(),
                lower_bound: FromNative::from_native(lower_bound),
                upper_bound: FromNative::from_native(upper_bound),
                allowed_ids: FromNative::from_native(allowed_ids),
            },
        }
    }
}

#[derive(Clone, Debug, Enum)]
pub enum LowerBound {
    NonZero,
    Inclusive { value: Arc<Decimal> },
}

impl FromNative for LowerBound {
    type Native = engine::LowerBound;

    fn from_native(native: Self::Native) -> Self {
        match native {
            engine::LowerBound::NonZero => Self::NonZero,
            engine::LowerBound::Inclusive(decimal) => Self::Inclusive {
                value: Arc::new(Decimal(decimal)),
            },
        }
    }
}

impl From<engine::LowerBound> for LowerBound {
    fn from(value: engine::LowerBound) -> Self {
        match value {
            engine::LowerBound::NonZero => Self::NonZero,
            engine::LowerBound::Inclusive(value) => Self::Inclusive {
                value: Arc::new(Decimal(value)),
            },
        }
    }
}

impl From<LowerBound> for engine::LowerBound {
    fn from(value: LowerBound) -> Self {
        match value {
            LowerBound::NonZero => Self::NonZero,
            LowerBound::Inclusive { value } => Self::Inclusive(value.0),
        }
    }
}

#[derive(Clone, Debug, Enum)]
pub enum UpperBound {
    Inclusive { value: Arc<Decimal> },
    Unbounded,
}

impl FromNative for UpperBound {
    type Native = engine::UpperBound;

    fn from_native(native: Self::Native) -> Self {
        match native {
            engine::UpperBound::Inclusive(decimal) => Self::Inclusive {
                value: Arc::new(Decimal(decimal)),
            },
            engine::UpperBound::Unbounded => Self::Unbounded,
        }
    }
}

impl From<engine::UpperBound> for UpperBound {
    fn from(value: engine::UpperBound) -> Self {
        match value {
            engine::UpperBound::Inclusive(value) => Self::Inclusive {
                value: Arc::new(Decimal(value)),
            },
            engine::UpperBound::Unbounded => Self::Unbounded,
        }
    }
}

impl From<UpperBound> for engine::UpperBound {
    fn from(value: UpperBound) -> Self {
        match value {
            UpperBound::Inclusive { value } => Self::Inclusive(value.0),
            UpperBound::Unbounded => Self::Unbounded,
        }
    }
}

#[derive(Clone, Debug, Enum)]
pub enum AllowedIds {
    Allowlist { ids: Vec<NonFungibleLocalId> },
    Any,
}

impl FromNative for AllowedIds {
    type Native = engine::AllowedIds;

    fn from_native(native: Self::Native) -> Self {
        match native {
            engine::AllowedIds::Allowlist(ids) => Self::Allowlist {
                ids: ids.into_iter().map(FromNative::from_native).collect(),
            },
            engine::AllowedIds::Any => Self::Any,
        }
    }
}

impl ToNative for AllowedIds {
    type Native = engine::AllowedIds;

    fn to_native(self) -> Result<Self::Native> {
        match self {
            Self::Allowlist { ids } => ids
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_>>()
                .map(Self::Native::Allowlist),
            Self::Any => Ok(Self::Native::Any),
        }
    }
}

impl From<engine::AllowedIds> for AllowedIds {
    fn from(value: engine::AllowedIds) -> Self {
        <Self as FromNative>::from_native(value)
    }
}

impl TryFrom<AllowedIds> for engine::AllowedIds {
    type Error = RadixEngineToolkitError;

    fn try_from(value: AllowedIds) -> std::result::Result<Self, Self::Error> {
        value.to_native()
    }
}

#[derive(Clone, Debug, Record)]
pub struct InstructionIndex {
    index: u64,
}

impl FromNative for InstructionIndex {
    type Native = toolkit::InstructionIndex;

    fn from_native(native: Self::Native) -> Self {
        Self {
            index: *native.value() as _,
        }
    }
}

#[derive(Clone, Debug, Enum)]
pub enum InvocationIoItem {
    Fungible {
        address: Arc<Address>,
        amount: EitherGuaranteedOrPredictedDecimal,
    },
    NonFungible {
        address: Arc<Address>,
        ids: EitherGuaranteedOrPredictedNonFungibleIds,
    },
}

impl FromNativeWithNetworkContext for InvocationIoItem {
    type Native = toolkit::InvocationIoItem;

    fn from_native(native: Self::Native, network_id: u8) -> Self {
        match native {
            toolkit::InvocationIoItem::Fungible(resource_address, amount) => {
                Self::Fungible {
                    address: Arc::new(Address::from_node_id(
                        resource_address,
                        network_id,
                    )),
                    amount: FromNative::from_native(amount),
                }
            }
            toolkit::InvocationIoItem::NonFungible(resource_address, ids) => {
                Self::NonFungible {
                    address: Arc::new(Address::from_node_id(
                        resource_address,
                        network_id,
                    )),
                    ids: FromNative::from_native(ids),
                }
            }
        }
    }
}

#[derive(Clone, Debug, Enum)]
pub enum EitherGuaranteedOrPredictedDecimal {
    Guaranteed {
        value: Arc<Decimal>,
    },
    Predicted {
        value: Arc<Decimal>,
        created_at: InstructionIndex,
    },
}

impl FromNative for EitherGuaranteedOrPredictedDecimal {
    type Native = toolkit::EitherGuaranteedOrPredicted<engine::Decimal>;

    fn from_native(native: Self::Native) -> Self {
        match native {
            toolkit::EitherGuaranteedOrPredicted::Guaranteed(value) => {
                Self::Guaranteed {
                    value: Arc::new(Decimal(value)),
                }
            }
            toolkit::EitherGuaranteedOrPredicted::Predicted(tracked) => {
                Self::Predicted {
                    value: Arc::new(Decimal(tracked.value)),
                    created_at: FromNative::from_native(tracked.created_at),
                }
            }
        }
    }
}

#[derive(Clone, Debug, Enum)]
pub enum EitherGuaranteedOrPredictedNonFungibleIds {
    Guaranteed {
        value: Vec<NonFungibleLocalId>,
    },
    Predicted {
        value: Vec<NonFungibleLocalId>,
        created_at: InstructionIndex,
    },
}

impl FromNative for EitherGuaranteedOrPredictedNonFungibleIds {
    type Native = toolkit::EitherGuaranteedOrPredicted<
        engine::IndexSet<engine::NonFungibleLocalId>,
    >;

    fn from_native(native: Self::Native) -> Self {
        match native {
            toolkit::EitherGuaranteedOrPredicted::Guaranteed(value) => {
                Self::Guaranteed {
                    value: value
                        .into_iter()
                        .map(FromNative::from_native)
                        .collect(),
                }
            }
            toolkit::EitherGuaranteedOrPredicted::Predicted(tracked) => {
                Self::Predicted {
                    value: tracked
                        .value
                        .into_iter()
                        .map(FromNative::from_native)
                        .collect(),
                    created_at: FromNative::from_native(tracked.created_at),
                }
            }
        }
    }
}

#[derive(Clone, Debug, Record)]
pub struct FeeLocks {
    pub lock: Arc<Decimal>,
    pub contingent_lock: Arc<Decimal>,
}

impl FromNative for FeeLocks {
    type Native = engine::FeeLocks;

    fn from_native(
        Self::Native {
            lock,
            contingent_lock,
        }: Self::Native,
    ) -> Self {
        Self {
            lock: Arc::new(Decimal(lock)),
            contingent_lock: Arc::new(Decimal(contingent_lock)),
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

impl FromNative for FeeSummary {
    type Native = toolkit::FeeSummary;

    fn from_native(
        Self::Native {
            execution_cost,
            finalization_cost,
            storage_expansion_cost,
            royalty_cost,
        }: Self::Native,
    ) -> Self {
        Self {
            execution_cost: Arc::new(Decimal(execution_cost)),
            finalization_cost: Arc::new(Decimal(finalization_cost)),
            storage_expansion_cost: Arc::new(Decimal(storage_expansion_cost)),
            royalty_cost: Arc::new(Decimal(royalty_cost)),
        }
    }
}

#[derive(Clone, Debug, Record)]
pub struct ValidatorStakingOutput {
    pub stake_operations: Vec<ValidatorStakeOperation>,
}

impl FromNativeWithNetworkContext for ValidatorStakingOutput {
    type Native = toolkit::ValidatorStakingOutput;

    fn from_native(
        Self::Native { stake_operations }: Self::Native,
        network_id: u8,
    ) -> Self {
        Self {
            stake_operations: stake_operations
                .into_iter()
                .map(|native| {
                    FromNativeWithNetworkContext::from_native(
                        native, network_id,
                    )
                })
                .collect(),
        }
    }
}

#[derive(Clone, Debug, Record)]
pub struct ValidatorStakeOperation {
    pub validator_address: Arc<Address>,
    pub staked_xrd_amount: Arc<Decimal>,
    pub liquid_stake_unit_resource_address: Arc<Address>,
    pub liquid_stake_unit_amount: Arc<Decimal>,
}

impl FromNativeWithNetworkContext for ValidatorStakeOperation {
    type Native = toolkit::ValidatorStakeOperation;

    fn from_native(
        Self::Native {
            validator_address,
            staked_xrd_amount,
            liquid_stake_unit_resource_address,
            liquid_stake_unit_amount,
        }: Self::Native,
        network_id: u8,
    ) -> Self {
        Self {
            validator_address: Arc::new(Address::from_node_id(
                validator_address,
                network_id,
            )),
            staked_xrd_amount: Arc::new(Decimal(staked_xrd_amount)),
            liquid_stake_unit_resource_address: Arc::new(
                Address::from_node_id(
                    liquid_stake_unit_resource_address,
                    network_id,
                ),
            ),
            liquid_stake_unit_amount: Arc::new(Decimal(
                liquid_stake_unit_amount,
            )),
        }
    }
}

#[derive(Clone, Debug, Record)]
pub struct ValidatorUnstakingOutput {
    pub unstake_operations: Vec<ValidatorUnstakeOperation>,
}

impl FromNativeWithNetworkContext for ValidatorUnstakingOutput {
    type Native = toolkit::ValidatorUnstakingOutput;

    fn from_native(
        Self::Native { unstake_operations }: Self::Native,
        network_id: u8,
    ) -> Self {
        Self {
            unstake_operations: unstake_operations
                .into_iter()
                .map(|native| {
                    FromNativeWithNetworkContext::from_native(
                        native, network_id,
                    )
                })
                .collect(),
        }
    }
}

#[derive(Clone, Debug, Record)]
pub struct ValidatorUnstakeOperation {
    pub validator_address: Arc<Address>,
    pub liquid_stake_unit_address: Arc<Address>,
    pub liquid_stake_unit_amount: Arc<Decimal>,
    pub claim_nft_address: Arc<Address>,
    pub claim_nft_ids: Vec<NonFungibleLocalId>,
}

impl FromNativeWithNetworkContext for ValidatorUnstakeOperation {
    type Native = toolkit::ValidatorUnstakeOperation;

    fn from_native(
        Self::Native {
            validator_address,
            liquid_stake_unit_address,
            liquid_stake_unit_amount,
            claim_nft_address,
            claim_nfts: claim_nft_ids,
        }: Self::Native,
        network_id: u8,
    ) -> Self {
        Self {
            validator_address: Arc::new(Address::from_node_id(
                validator_address,
                network_id,
            )),
            liquid_stake_unit_address: Arc::new(Address::from_node_id(
                liquid_stake_unit_address,
                network_id,
            )),
            liquid_stake_unit_amount: Arc::new(Decimal(
                liquid_stake_unit_amount,
            )),
            claim_nft_address: Arc::new(Address::from_node_id(
                claim_nft_address,
                network_id,
            )),
            claim_nft_ids: claim_nft_ids
                .into_iter()
                .map(|(k, _)| k)
                .map(FromNative::from_native)
                .collect(),
        }
    }
}

#[derive(Clone, Debug, Record)]
pub struct ValidatorClaimingXrdOutput {
    pub claim_operations: Vec<ValidatorClaimOperation>,
}

impl FromNativeWithNetworkContext for ValidatorClaimingXrdOutput {
    type Native = toolkit::ValidatorClaimingXrdOutput;

    fn from_native(
        Self::Native { claim_operations }: Self::Native,
        network_id: u8,
    ) -> Self {
        Self {
            claim_operations: claim_operations
                .into_iter()
                .map(|native| {
                    FromNativeWithNetworkContext::from_native(
                        native, network_id,
                    )
                })
                .collect(),
        }
    }
}

#[derive(Clone, Debug, Record)]
pub struct ValidatorClaimOperation {
    pub validator_address: Arc<Address>,
    pub claim_nft_address: Arc<Address>,
    pub claim_nft_ids: Vec<NonFungibleLocalId>,
    pub xrd_amount: Arc<Decimal>,
}

impl FromNativeWithNetworkContext for ValidatorClaimOperation {
    type Native = toolkit::ValidatorClaimOperation;

    fn from_native(
        Self::Native {
            validator_address,
            claim_nft_address,
            claim_nft_ids,
            xrd_amount,
        }: Self::Native,
        network_id: u8,
    ) -> Self {
        Self {
            validator_address: Arc::new(Address::from_node_id(
                validator_address,
                network_id,
            )),
            claim_nft_address: Arc::new(Address::from_node_id(
                claim_nft_address,
                network_id,
            )),
            claim_nft_ids: claim_nft_ids
                .into_iter()
                .map(FromNative::from_native)
                .collect(),
            xrd_amount: Arc::new(Decimal(xrd_amount)),
        }
    }
}

#[derive(Clone, Debug, Record)]
pub struct PoolContributionOutput {
    pub contribution_operations: Vec<PoolContributionOperation>,
}

impl FromNativeWithNetworkContext for PoolContributionOutput {
    type Native = toolkit::PoolContributionOutput;

    fn from_native(
        Self::Native {
            contribution_operations,
        }: Self::Native,
        network_id: u8,
    ) -> Self {
        Self {
            contribution_operations: contribution_operations
                .into_iter()
                .map(|native| {
                    FromNativeWithNetworkContext::from_native(
                        native, network_id,
                    )
                })
                .collect(),
        }
    }
}

#[derive(Clone, Debug, Record)]
pub struct PoolContributionOperation {
    pub pool_address: Arc<Address>,
    pub contributed_resources: HashMap<String, Arc<Decimal>>,
    pub pool_units_resource_address: Arc<Address>,
    pub pool_units_amount: Arc<Decimal>,
}

impl FromNativeWithNetworkContext for PoolContributionOperation {
    type Native = toolkit::PoolContributionOperation;

    fn from_native(
        Self::Native {
            pool_address,
            contributed_resources,
            pool_units_resource_address,
            pool_units_amount,
        }: Self::Native,
        network_id: u8,
    ) -> Self {
        Self {
            pool_address: Arc::new(Address::from_node_id(
                pool_address,
                network_id,
            )),
            contributed_resources: contributed_resources
                .into_iter()
                .map(|(resource_address, amount)| {
                    let address =
                        Address::from_node_id(resource_address, network_id);
                    (address.as_str(), Arc::new(Decimal(amount)))
                })
                .collect(),
            pool_units_resource_address: Arc::new(Address::from_node_id(
                pool_units_resource_address,
                network_id,
            )),
            pool_units_amount: Arc::new(Decimal(pool_units_amount)),
        }
    }
}

#[derive(Clone, Debug, Record)]
pub struct PoolRedemptionOutput {
    pub redemption_operations: Vec<PoolRedemptionOperation>,
}

impl FromNativeWithNetworkContext for PoolRedemptionOutput {
    type Native = toolkit::PoolRedemptionOutput;

    fn from_native(
        Self::Native {
            redemption_operations,
        }: Self::Native,
        network_id: u8,
    ) -> Self {
        Self {
            redemption_operations: redemption_operations
                .into_iter()
                .map(|native| {
                    FromNativeWithNetworkContext::from_native(
                        native, network_id,
                    )
                })
                .collect(),
        }
    }
}

#[derive(Clone, Debug, Record)]
pub struct PoolRedemptionOperation {
    pub pool_address: Arc<Address>,
    pub pool_units_resource_address: Arc<Address>,
    pub pool_units_amount: Arc<Decimal>,
    pub redeemed_resources: HashMap<String, Arc<Decimal>>,
}

impl FromNativeWithNetworkContext for PoolRedemptionOperation {
    type Native = toolkit::PoolRedemptionOperation;

    fn from_native(
        Self::Native {
            pool_address,
            pool_units_resource_address,
            pool_units_amount,
            redeemed_resources,
        }: Self::Native,
        network_id: u8,
    ) -> Self {
        Self {
            pool_address: Arc::new(Address::from_node_id(
                pool_address,
                network_id,
            )),
            pool_units_resource_address: Arc::new(Address::from_node_id(
                pool_units_resource_address,
                network_id,
            )),
            pool_units_amount: Arc::new(Decimal(pool_units_amount)),
            redeemed_resources: redeemed_resources
                .into_iter()
                .map(|(resource_address, amount)| {
                    let address =
                        Address::from_node_id(resource_address, network_id);
                    (address.as_str(), Arc::new(Decimal(amount)))
                })
                .collect(),
        }
    }
}

#[derive(Clone, Debug, Record)]
pub struct AccountSettingsUpdateOutput {
    pub resource_preferences_updates:
        HashMap<String, HashMap<String, ResourcePreferenceUpdate>>,
    pub default_deposit_rule_updates:
        HashMap<String, AccountDefaultDepositRule>,
    pub authorized_depositors_updates:
        HashMap<String, HashMap<Operation, Vec<ResourceOrNonFungible>>>,
}

impl FromNativeWithNetworkContext for AccountSettingsUpdateOutput {
    type Native = toolkit::AccountSettingsUpdateOutput;

    fn from_native(
        Self::Native {
            resource_preference_updates,
            default_deposit_rule_updates,
            authorized_depositor_updates,
        }: Self::Native,
        network_id: u8,
    ) -> Self {
        Self {
            resource_preferences_updates: resource_preference_updates
                .into_iter()
                .filter_map(
                    |((account, resource), resource_preference_update)| {
                        if let (
                            engine::ManifestGlobalAddress::Static(account),
                            engine::ManifestResourceAddress::Static(resource),
                        ) = (account, resource)
                        {
                            Some((
                                account,
                                resource,
                                resource_preference_update,
                            ))
                        } else {
                            None
                        }
                    },
                )
                .fold(
                    Default::default(),
                    |mut map, (account, resource, update)| {
                        let account = Address::from_node_id(
                            account.into_node_id(),
                            network_id,
                        );
                        let resource = Address::from_node_id(
                            resource.into_node_id(),
                            network_id,
                        );
                        map.entry(account.as_str()).or_default().insert(
                            resource.as_str(),
                            FromNative::from_native(update),
                        );
                        map
                    },
                ),
            default_deposit_rule_updates: default_deposit_rule_updates
                .into_iter()
                .filter_map(|(account, default_deposit_rule)| {
                    if let engine::ManifestGlobalAddress::Static(account) =
                        account
                    {
                        let account =
                            Address::from_node_id(account, network_id);
                        Some((
                            account.as_str(),
                            FromNative::from_native(default_deposit_rule),
                        ))
                    } else {
                        None
                    }
                })
                .collect(),
            authorized_depositors_updates: authorized_depositor_updates
                .into_iter()
                .filter_map(|((account, badge), operation)| {
                    match (account, badge) {
                        (
                            engine::ManifestGlobalAddress::Static(account),
                            engine::ManifestResourceOrNonFungible::Resource(
                                engine::ManifestResourceAddress::Static(
                                    resource_address,
                                ),
                            ),
                        ) => Some((
                            account,
                            engine::ResourceOrNonFungible::Resource(
                                resource_address,
                            ),
                            operation,
                        )),
                        (
                            engine::ManifestGlobalAddress::Static(account),
                            engine::ManifestResourceOrNonFungible::NonFungible(
                                non_fungible_global_id,
                            ),
                        ) => Some((
                            account,
                            engine::ResourceOrNonFungible::NonFungible(
                                non_fungible_global_id,
                            ),
                            operation,
                        )),
                        _ => None,
                    }
                })
                .fold(
                    Default::default(),
                    |mut map, (account, badge, operation)| {
                        let account =
                            Address::from_node_id(account, network_id);
                        map.entry(account.as_str())
                            .or_default()
                            .entry(FromNative::from_native(operation))
                            .or_default()
                            .push(FromNativeWithNetworkContext::from_native(
                                badge, network_id,
                            ));
                        map
                    },
                ),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Enum)]
pub enum Operation {
    Added,
    Removed,
}

impl FromNative for Operation {
    type Native = toolkit::Operation;

    fn from_native(native: Self::Native) -> Self {
        match native {
            toolkit::Operation::Added => Self::Added,
            toolkit::Operation::Removed => Self::Removed,
        }
    }
}

#[derive(Clone, Debug, Enum)]
pub enum ResourcePreferenceUpdate {
    Set { value: ResourcePreference },
    Remove,
}

impl FromNative for ResourcePreferenceUpdate {
    type Native = toolkit::Update<engine::ResourcePreference>;

    fn from_native(native: Self::Native) -> Self {
        match native {
            toolkit::Update::Set(resource_preference) => Self::Set {
                value: FromNative::from_native(resource_preference),
            },
            toolkit::Update::Remove => Self::Remove,
        }
    }
}

#[derive(Clone, Debug, Enum)]
pub enum ResourcePreference {
    Allowed,
    Disallowed,
}

impl FromNative for ResourcePreference {
    type Native = engine::ResourcePreference;

    fn from_native(native: Self::Native) -> Self {
        match native {
            engine::ResourcePreference::Allowed => Self::Allowed,
            engine::ResourcePreference::Disallowed => Self::Disallowed,
        }
    }
}

impl ToNative for ResourcePreference {
    type Native = engine::ResourcePreference;

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
    type Native = engine::DefaultDepositRule;

    fn from_native(native: Self::Native) -> Self {
        match native {
            engine::DefaultDepositRule::Accept => Self::Accept,
            engine::DefaultDepositRule::Reject => Self::Reject,
            engine::DefaultDepositRule::AllowExisting => Self::AllowExisting,
        }
    }
}

impl ToNative for AccountDefaultDepositRule {
    type Native = engine::DefaultDepositRule;

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

#[derive(Clone, Debug, Record)]
pub struct EntitySecurifyOutput {
    pub securified_accounts: Vec<Arc<Address>>,
    pub securified_identities: Vec<Arc<Address>>,
}

impl FromNativeWithNetworkContext for EntitySecurifyOutput {
    type Native = toolkit::EntitySecurifyOutput;

    fn from_native(
        Self::Native {
            securified_accounts,
            securified_identities,
        }: Self::Native,
        network_id: u8,
    ) -> Self {
        Self {
            securified_accounts: securified_accounts
                .into_iter()
                .filter_map(|account| account.into_static())
                .map(|account| {
                    Arc::new(Address::from_node_id(account, network_id))
                })
                .collect(),

            securified_identities: securified_identities
                .into_iter()
                .filter_map(|identity| identity.into_static())
                .map(|identity| {
                    Arc::new(Address::from_node_id(identity, network_id))
                })
                .collect(),
        }
    }
}

#[derive(Clone, Debug, Record)]
pub struct AccessControllerRecoveryOutput {
    pub access_controllers: Vec<Arc<Address>>,
}

impl FromNativeWithNetworkContext for AccessControllerRecoveryOutput {
    type Native = toolkit::AccessControllerRecoveryOutput;

    fn from_native(
        Self::Native { access_controllers }: Self::Native,
        network_id: u8,
    ) -> Self {
        Self {
            access_controllers: access_controllers
                .into_iter()
                .map(|ac_address| {
                    Arc::new(Address::from_node_id(ac_address, network_id))
                })
                .collect(),
        }
    }
}

#[derive(Clone, Debug, Record)]
pub struct AccessControllerStopTimedRecoveryOutput {
    pub access_controllers: Vec<Arc<Address>>,
}

impl FromNativeWithNetworkContext for AccessControllerStopTimedRecoveryOutput {
    type Native = toolkit::AccessControllerStopTimedRecoveryAnalyzerOutput;

    fn from_native(
        Self::Native { access_controllers }: Self::Native,
        network_id: u8,
    ) -> Self {
        Self {
            access_controllers: access_controllers
                .into_iter()
                .map(|ac_address| {
                    Arc::new(Address::from_node_id(ac_address, network_id))
                })
                .collect(),
        }
    }
}

#[derive(Clone, Debug, Record)]
pub struct AccessControllerConfirmTimedRecoveryOutput {
    pub access_controllers: Vec<Arc<Address>>,
}

impl FromNativeWithNetworkContext
    for AccessControllerConfirmTimedRecoveryOutput
{
    type Native = toolkit::AccessControllerConfirmTimedRecoveryOutput;

    fn from_native(
        Self::Native { access_controllers }: Self::Native,
        network_id: u8,
    ) -> Self {
        Self {
            access_controllers: access_controllers
                .into_iter()
                .map(|ac_address| {
                    Arc::new(Address::from_node_id(ac_address, network_id))
                })
                .collect(),
        }
    }
}
