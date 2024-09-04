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

use radix_transactions::prelude::manifest_instruction::*;
use radix_transactions::prelude::*;
use scrypto::prelude::*;

use radix_engine_interface::blueprints::account::*;

use crate::transaction_types::*;
use crate::utils;

pub struct AccountSettingsUpdateDetector {
    is_valid: bool,
    /// Determines if one of the account setting update instructions were met.
    account_settings_instruction_encountered: bool,
    /// Updated resource preferences
    resource_preferences: IndexMap<
        ComponentAddress,
        IndexMap<ResourceAddress, Update<ResourcePreference>>,
    >,
    /// Updated default deposit rules
    default_deposit_rules: IndexMap<ComponentAddress, DefaultDepositRule>,
    /// Updates to the authorized depositors
    authorized_depositors:
        IndexMap<ComponentAddress, IndexMap<ResourceOrNonFungible, Update<()>>>,
}

impl AccountSettingsUpdateDetector {
    pub fn output(
        self,
    ) -> Option<(
        IndexMap<
            ComponentAddress,
            IndexMap<ResourceAddress, Update<ResourcePreference>>,
        >,
        IndexMap<ComponentAddress, DefaultDepositRule>,
        IndexMap<ComponentAddress, IndexMap<ResourceOrNonFungible, Update<()>>>,
    )> {
        if self.is_valid() {
            Some((
                self.resource_preferences,
                self.default_deposit_rules,
                self.authorized_depositors,
            ))
        } else {
            None
        }
    }
}

impl ManifestSummaryCallback for AccountSettingsUpdateDetector {
    fn on_finish(&mut self, instructions_count: usize) {
        if instructions_count == 0 {
            self.is_valid = false
        }
    }

    fn on_instruction(&mut self, instruction: &InstructionV2, _: usize) {
        // Determine the validity based on the instructions
        self.is_valid &= match instruction {
            /* Maybe Permitted - Need more info */
            InstructionV2::CallMethod(CallMethod {
                address,
                method_name,
                ..
            }) => {
                Self::construct_fn_rules(address).is_fn_permitted(method_name)
            }
            /* Not Permitted */
            InstructionV2::BurnResource(..)
            | InstructionV2::CallRoyaltyMethod(..)
            | InstructionV2::CallMetadataMethod(..)
            | InstructionV2::CallRoleAssignmentMethod(..)
            | InstructionV2::CallDirectVaultMethod(..)
            | InstructionV2::AllocateGlobalAddress(..)
            | InstructionV2::ReturnToWorktop(..)
            | InstructionV2::PopFromAuthZone(..)
            | InstructionV2::PushToAuthZone(..)
            | InstructionV2::CreateProofFromAuthZoneOfAmount(..)
            | InstructionV2::CreateProofFromAuthZoneOfNonFungibles(..)
            | InstructionV2::CreateProofFromAuthZoneOfAll(..)
            | InstructionV2::DropAuthZoneProofs(..)
            | InstructionV2::DropAuthZoneRegularProofs(..)
            | InstructionV2::DropAuthZoneSignatureProofs(..)
            | InstructionV2::CreateProofFromBucketOfAmount(..)
            | InstructionV2::CreateProofFromBucketOfNonFungibles(..)
            | InstructionV2::CreateProofFromBucketOfAll(..)
            | InstructionV2::CloneProof(..)
            | InstructionV2::DropProof(..)
            | InstructionV2::DropNamedProofs(..)
            | InstructionV2::DropAllProofs(..)
            | InstructionV2::CallFunction(..)
            | InstructionV2::TakeFromWorktop(..)
            | InstructionV2::TakeNonFungiblesFromWorktop(..)
            | InstructionV2::TakeAllFromWorktop(..)
            | InstructionV2::AssertWorktopContainsAny(..)
            | InstructionV2::AssertWorktopContains(..)
            | InstructionV2::AssertWorktopContainsNonFungibles(..)
            | InstructionV2::YieldToParent(_)
            | InstructionV2::YieldToChild(_)
            | InstructionV2::AuthenticateParent(_) => false,
        };

        // Determine if the instruction is an account settings instruction.
        self.account_settings_instruction_encountered |=
            if let InstructionV2::CallMethod(CallMethod {
                address,
                method_name,
                ..
            }) = instruction
            {
                Self::construct_specific_fn_rules(address)
                    .is_fn_permitted(method_name)
            } else {
                false
            };

        // Process the instructions
        let InstructionV2::CallMethod(CallMethod {
            address: dynamic_address @ DynamicGlobalAddress::Static(address),
            method_name,
            args,
        }) = instruction
        else {
            // Case already accounted for in the above validity check - no need
            // to invalidate here as it's impossible for us to get here in the
            // first place.
            return;
        };
        if !utils::is_account(dynamic_address) {
            return;
        }

        let address =
            ComponentAddress::try_from(*address).expect("Must succeed");
        let encoded_args = manifest_encode(args).expect("Must succeed!");

        if method_name == ACCOUNT_SET_RESOURCE_PREFERENCE_IDENT {
            if let Ok(AccountSetResourcePreferenceInput {
                resource_address,
                resource_preference,
            }) = manifest_decode(&encoded_args)
            {
                self.resource_preferences
                    .entry(address)
                    .or_default()
                    .insert(resource_address, Update::Set(resource_preference));
            }
        } else if method_name == ACCOUNT_REMOVE_RESOURCE_PREFERENCE_IDENT {
            if let Ok(AccountRemoveResourcePreferenceInput {
                resource_address,
            }) = manifest_decode(&encoded_args)
            {
                self.resource_preferences
                    .entry(address)
                    .or_default()
                    .insert(resource_address, Update::Remove);
            }
        } else if method_name == ACCOUNT_ADD_AUTHORIZED_DEPOSITOR {
            if let Ok(AccountAddAuthorizedDepositorInput { badge }) =
                manifest_decode(&encoded_args)
            {
                self.authorized_depositors
                    .entry(address)
                    .or_default()
                    .insert(badge, Update::Set(()));
            }
        } else if method_name == ACCOUNT_REMOVE_AUTHORIZED_DEPOSITOR {
            if let Ok(AccountRemoveAuthorizedDepositorInput { badge }) =
                manifest_decode(&encoded_args)
            {
                self.authorized_depositors
                    .entry(address)
                    .or_default()
                    .insert(badge, Update::Remove);
            }
        } else if method_name == ACCOUNT_SET_DEFAULT_DEPOSIT_RULE_IDENT {
            if let Ok(AccountSetDefaultDepositRuleInput { default }) =
                manifest_decode(&encoded_args)
            {
                self.default_deposit_rules.insert(address, default);
            }
        }
    }
}

impl ExecutionSummaryCallback for AccountSettingsUpdateDetector {}

impl AccountSettingsUpdateDetector {
    pub fn is_valid(&self) -> bool {
        self.is_valid && self.account_settings_instruction_encountered
    }

    fn construct_fn_rules(address: &DynamicGlobalAddress) -> FnRules {
        match address {
            DynamicGlobalAddress::Named(..) => FnRules::all_disallowed(),
            DynamicGlobalAddress::Static(address) => {
                address
                    .as_node_id()
                    .entity_type()
                    .map(|entity_type| {
                        match entity_type {
                            EntityType::GlobalAccount
                            | EntityType::GlobalPreallocatedSecp256k1Account
                            | EntityType::GlobalPreallocatedEd25519Account => {
                                FnRules {
                                    allowed: &[
                                        /* Resource Preference */
                                        ACCOUNT_SET_RESOURCE_PREFERENCE_IDENT,
                                        ACCOUNT_REMOVE_RESOURCE_PREFERENCE_IDENT,
                                        /* Authorized Depositors */
                                        ACCOUNT_ADD_AUTHORIZED_DEPOSITOR,
                                        ACCOUNT_REMOVE_AUTHORIZED_DEPOSITOR,
                                        /* Default Deposit Rule */
                                        ACCOUNT_SET_DEFAULT_DEPOSIT_RULE_IDENT,
                                        /* Locking of fees */
                                        ACCOUNT_LOCK_FEE_IDENT,
                                        ACCOUNT_LOCK_CONTINGENT_FEE_IDENT,
                                    ],
                                    disallowed: &[],
                                    default: FnRule::Disallowed,
                                }
                            }
                            /* Disallowed */
                            EntityType::GlobalGenericComponent
                            | EntityType::GlobalIdentity
                            | EntityType::GlobalPreallocatedSecp256k1Identity
                            | EntityType::GlobalPreallocatedEd25519Identity
                            | EntityType::InternalGenericComponent
                            | EntityType::GlobalPackage
                            | EntityType::GlobalValidator
                            | EntityType::GlobalFungibleResourceManager
                            | EntityType::GlobalNonFungibleResourceManager
                            | EntityType::GlobalConsensusManager
                            | EntityType::InternalFungibleVault
                            | EntityType::InternalNonFungibleVault
                            | EntityType::InternalKeyValueStore
                            | EntityType::GlobalTransactionTracker
                            | EntityType::GlobalAccessController
                            | EntityType::GlobalOneResourcePool
                            | EntityType::GlobalTwoResourcePool
                            | EntityType::GlobalMultiResourcePool
                            | EntityType::GlobalAccountLocker => {
                                FnRules::all_disallowed()
                            }
                        }
                    })
                    .unwrap_or(FnRules::all_disallowed())
            }
        }
    }

    fn construct_specific_fn_rules(address: &DynamicGlobalAddress) -> FnRules {
        match address {
            DynamicGlobalAddress::Named(..) => FnRules::all_disallowed(),
            DynamicGlobalAddress::Static(address) => {
                address
                    .as_node_id()
                    .entity_type()
                    .map(|entity_type| {
                        match entity_type {
                            EntityType::GlobalAccount
                            | EntityType::GlobalPreallocatedSecp256k1Account
                            | EntityType::GlobalPreallocatedEd25519Account => {
                                FnRules {
                                    allowed: &[
                                        /* Resource Preference */
                                        ACCOUNT_SET_RESOURCE_PREFERENCE_IDENT,
                                        ACCOUNT_REMOVE_RESOURCE_PREFERENCE_IDENT,
                                        /* Authorized Depositors */
                                        ACCOUNT_ADD_AUTHORIZED_DEPOSITOR,
                                        ACCOUNT_REMOVE_AUTHORIZED_DEPOSITOR,
                                        /* Default Deposit Rule */
                                        ACCOUNT_SET_DEFAULT_DEPOSIT_RULE_IDENT,
                                    ],
                                    disallowed: &[],
                                    default: FnRule::Disallowed,
                                }
                            }
                            /* Disallowed */
                            EntityType::GlobalGenericComponent
                            | EntityType::GlobalIdentity
                            | EntityType::GlobalPreallocatedSecp256k1Identity
                            | EntityType::GlobalPreallocatedEd25519Identity
                            | EntityType::InternalGenericComponent
                            | EntityType::GlobalPackage
                            | EntityType::GlobalValidator
                            | EntityType::GlobalFungibleResourceManager
                            | EntityType::GlobalNonFungibleResourceManager
                            | EntityType::GlobalConsensusManager
                            | EntityType::InternalFungibleVault
                            | EntityType::InternalNonFungibleVault
                            | EntityType::InternalKeyValueStore
                            | EntityType::GlobalTransactionTracker
                            | EntityType::GlobalAccessController
                            | EntityType::GlobalOneResourcePool
                            | EntityType::GlobalTwoResourcePool
                            | EntityType::GlobalMultiResourcePool
                            | EntityType::GlobalAccountLocker => {
                                FnRules::all_disallowed()
                            }
                        }
                    })
                    .unwrap_or(FnRules::all_disallowed())
            }
        }
    }
}

impl Default for AccountSettingsUpdateDetector {
    fn default() -> Self {
        Self {
            is_valid: true,
            account_settings_instruction_encountered: false,
            resource_preferences: Default::default(),
            default_deposit_rules: Default::default(),
            authorized_depositors: Default::default(),
        }
    }
}
