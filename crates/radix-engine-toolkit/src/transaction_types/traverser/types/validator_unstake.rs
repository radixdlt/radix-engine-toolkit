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

use radix_transactions::manifest::*;
use radix_transactions::prelude::*;
use scrypto::prelude::*;

use radix_engine::system::system_modules::execution_trace::ResourceSpecifier;
use radix_engine_interface::blueprints::account::*;

use crate::transaction_types::*;
use crate::utils::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackedValidatorUnstake {
    pub validator_address: ComponentAddress,
    /* Input */
    pub liquid_stake_unit_address: ResourceAddress,
    pub liquid_stake_unit_amount: Decimal,
    /* Output */
    pub claim_nft_address: ResourceAddress,
    pub claim_nft_ids: IndexSet<NonFungibleLocalId>,
}

pub struct ValidatorUnstakeDetector {
    is_valid: bool,
    required_method_called: bool,
    /// The validators encountered in this manifest that were staked to.
    validators: IndexSet<ComponentAddress>,
    /// Tracks the unstake operations in the transaction.
    tracked_unstake: Vec<TrackedValidatorUnstake>,
}

impl ValidatorUnstakeDetector {
    pub fn output(
        self,
    ) -> Option<(IndexSet<ComponentAddress>, Vec<TrackedValidatorUnstake>)>
    {
        if self.is_valid() {
            Some((self.validators, self.tracked_unstake))
        } else {
            None
        }
    }
}

impl StaticAnalysisCallback for ValidatorUnstakeDetector {
    fn on_finish(&mut self, instructions_count: usize) {
        if instructions_count == 0 {
            self.is_valid = false
        }
    }

    fn on_instruction(&mut self, instruction: &InstructionV2, _: usize) {
        self.is_valid &= match instruction {
            /* Maybe Permitted - Need more info */
            InstructionV2::CallMethod(CallMethod {
                address,
                method_name,
                ..
            }) => {
                Self::construct_fn_rules(address).is_fn_permitted(method_name)
            }
            /* Permitted */
            InstructionV2::TakeFromWorktop { .. }
            | InstructionV2::TakeNonFungiblesFromWorktop { .. }
            | InstructionV2::TakeAllFromWorktop { .. }
            | InstructionV2::AssertWorktopContainsAny { .. }
            | InstructionV2::AssertWorktopContains { .. }
            | InstructionV2::AssertWorktopContainsNonFungibles { .. }
            | InstructionV2::AssertWorktopResourcesOnly(..)
            | InstructionV2::AssertWorktopResourcesInclude(..)
            | InstructionV2::AssertNextCallReturnsOnly(..)
            | InstructionV2::AssertNextCallReturnsInclude(..)
            | InstructionV2::AssertBucketContents(..) => true,
            /* Not Permitted */
            InstructionV2::BurnResource { .. }
            | InstructionV2::CallRoyaltyMethod { .. }
            | InstructionV2::CallMetadataMethod { .. }
            | InstructionV2::CallRoleAssignmentMethod { .. }
            | InstructionV2::CallDirectVaultMethod { .. }
            | InstructionV2::AllocateGlobalAddress { .. }
            | InstructionV2::ReturnToWorktop { .. }
            | InstructionV2::PopFromAuthZone { .. }
            | InstructionV2::PushToAuthZone { .. }
            | InstructionV2::CreateProofFromAuthZoneOfAmount { .. }
            | InstructionV2::CreateProofFromAuthZoneOfNonFungibles { .. }
            | InstructionV2::CreateProofFromAuthZoneOfAll { .. }
            | InstructionV2::DropAuthZoneProofs { .. }
            | InstructionV2::DropAuthZoneRegularProofs { .. }
            | InstructionV2::DropAuthZoneSignatureProofs { .. }
            | InstructionV2::CreateProofFromBucketOfAmount { .. }
            | InstructionV2::CreateProofFromBucketOfNonFungibles { .. }
            | InstructionV2::CreateProofFromBucketOfAll { .. }
            | InstructionV2::CloneProof { .. }
            | InstructionV2::DropProof { .. }
            | InstructionV2::DropNamedProofs { .. }
            | InstructionV2::DropAllProofs { .. }
            | InstructionV2::CallFunction { .. }
            | InstructionV2::YieldToParent(_)
            | InstructionV2::YieldToChild(_)
            | InstructionV2::VerifyParent(_) => false,
        };

        // Handle required method call
        match instruction {
            InstructionV2::CallMethod(CallMethod {
                address,
                method_name,
                ..
            }) if is_validator(address)
                && method_name == VALIDATOR_UNSTAKE_IDENT =>
            {
                self.required_method_called = true
            }
            _ => {}
        };
    }

    fn on_global_entity_encounter(&mut self, address: GlobalAddress) {
        if address
            .as_node_id()
            .entity_type()
            .is_some_and(|entity_type| {
                matches!(entity_type, EntityType::GlobalValidator)
            })
        {
            self.validators.insert(
                ComponentAddress::try_from(address).expect("Must succeed!"),
            );
        }
    }
}

impl DynamicAnalysisCallback for ValidatorUnstakeDetector {
    fn on_instruction(
        &mut self,
        instruction: &InstructionV2,
        _: usize,
        input_resources: &[ResourceSpecifier],
        output_resources: &[ResourceSpecifier],
    ) {
        match instruction {
            InstructionV2::CallMethod(CallMethod {
                address: dynamic_address @ DynamicGlobalAddress::Static(address),
                method_name,
                ..
            }) if is_validator(dynamic_address)
                && (method_name == VALIDATOR_UNSTAKE_IDENT) =>
            {
                let validator_component = ComponentAddress::try_from(*address)
                    .expect("Must succeed!");

                let Some(ResourceSpecifier::Amount(
                    stake_unit_resource_address,
                    stake_unit_amount,
                )) = input_resources.first()
                else {
                    // Can happen if an empty bucket of LSUs is provided as
                    // input.
                    return;
                };

                let Some(ResourceSpecifier::Ids(
                    claim_nft_resource_address,
                    claim_nft_ids,
                )) = output_resources.first()
                else {
                    // TODO: Error? Panic? Is it ever possible that we get back
                    // no claim NFTs? If not, then I would say we do an unwrap
                    // here and force a crash if this ever happens.
                    return;
                };

                self.tracked_unstake.push(TrackedValidatorUnstake {
                    validator_address: validator_component,
                    liquid_stake_unit_address: *stake_unit_resource_address,
                    liquid_stake_unit_amount: *stake_unit_amount,
                    claim_nft_address: *claim_nft_resource_address,
                    claim_nft_ids: claim_nft_ids.clone(),
                });
            }
            _ => { /* No-op */ }
        }
    }
}

impl ValidatorUnstakeDetector {
    pub fn is_valid(&self) -> bool {
        self.is_valid && self.required_method_called
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
                                        /* All withdraw methods */
                                        ACCOUNT_WITHDRAW_IDENT,
                                        /* All deposit methods */
                                        ACCOUNT_DEPOSIT_IDENT,
                                        ACCOUNT_DEPOSIT_BATCH_IDENT,
                                        ACCOUNT_TRY_DEPOSIT_OR_ABORT_IDENT,
                                        ACCOUNT_TRY_DEPOSIT_BATCH_OR_ABORT_IDENT,
                                        /* Lock Fees */
                                        ACCOUNT_LOCK_FEE_IDENT,
                                        ACCOUNT_LOCK_CONTINGENT_FEE_IDENT,
                                        ACCOUNT_LOCK_FEE_AND_WITHDRAW_IDENT,
                                    ],
                                    disallowed: &[],
                                    default: FnRule::Disallowed,
                                }
                            }
                            EntityType::GlobalValidator => FnRules {
                                allowed: &[VALIDATOR_UNSTAKE_IDENT],
                                disallowed: &[],
                                default: FnRule::Disallowed,
                            },
                            /* Disallowed */
                            EntityType::GlobalGenericComponent
                            | EntityType::GlobalIdentity
                            | EntityType::GlobalPreallocatedSecp256k1Identity
                            | EntityType::GlobalPreallocatedEd25519Identity
                            | EntityType::InternalGenericComponent
                            | EntityType::GlobalPackage
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

impl Default for ValidatorUnstakeDetector {
    fn default() -> Self {
        Self {
            is_valid: true,
            required_method_called: false,
            validators: Default::default(),
            tracked_unstake: Default::default(),
        }
    }
}
