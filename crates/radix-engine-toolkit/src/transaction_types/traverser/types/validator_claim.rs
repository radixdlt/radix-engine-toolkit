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

use radix_transactions::prelude::*;
use scrypto::prelude::*;

use radix_engine::system::system_modules::execution_trace::ResourceSpecifier;
use radix_engine_interface::blueprints::account::*;
use radix_engine_interface::blueprints::consensus_manager::*;

use crate::transaction_types::*;
use crate::utils::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackedValidatorClaim {
    pub validator_address: ComponentAddress,
    /* Input */
    pub claim_nft_address: ResourceAddress,
    pub claim_nft_ids: IndexSet<NonFungibleLocalId>,
    /* Output */
    pub xrd_amount: Decimal,
}

pub struct ValidatorClaimDetector {
    is_valid: bool,
    required_method_called: bool,
    /// The validators encountered in this manifest that were staked to.
    validators: IndexSet<ComponentAddress>,
    /// Tracks the claim operations in the transaction.
    tracked_claim: Vec<TrackedValidatorClaim>,
}

impl ValidatorClaimDetector {
    pub fn output(
        self,
    ) -> Option<(IndexSet<ComponentAddress>, Vec<TrackedValidatorClaim>)> {
        if self.is_valid() {
            Some((self.validators, self.tracked_claim))
        } else {
            None
        }
    }
}

impl ManifestSummaryCallback for ValidatorClaimDetector {
    fn on_finish(&mut self, instructions_count: usize) {
        if instructions_count == 0 {
            self.is_valid = false
        }
    }

    fn on_instruction(&mut self, instruction: &InstructionV1, _: usize) {
        self.is_valid &= match instruction {
            /* Maybe Permitted - Need more info */
            InstructionV1::CallMethod {
                address,
                method_name,
                ..
            } => Self::construct_fn_rules(address).is_fn_permitted(method_name),
            /* Permitted */
            InstructionV1::TakeFromWorktop { .. }
            | InstructionV1::TakeNonFungiblesFromWorktop { .. }
            | InstructionV1::TakeAllFromWorktop { .. }
            | InstructionV1::AssertWorktopContainsAny { .. }
            | InstructionV1::AssertWorktopContains { .. }
            | InstructionV1::AssertWorktopContainsNonFungibles { .. } => true,
            /* Not Permitted */
            InstructionV1::BurnResource { .. }
            | InstructionV1::CallRoyaltyMethod { .. }
            | InstructionV1::CallMetadataMethod { .. }
            | InstructionV1::CallRoleAssignmentMethod { .. }
            | InstructionV1::CallDirectVaultMethod { .. }
            | InstructionV1::AllocateGlobalAddress { .. }
            | InstructionV1::ReturnToWorktop { .. }
            | InstructionV1::PopFromAuthZone
            | InstructionV1::PushToAuthZone { .. }
            | InstructionV1::CreateProofFromAuthZoneOfAmount { .. }
            | InstructionV1::CreateProofFromAuthZoneOfNonFungibles { .. }
            | InstructionV1::CreateProofFromAuthZoneOfAll { .. }
            | InstructionV1::DropAuthZoneProofs
            | InstructionV1::DropAuthZoneRegularProofs
            | InstructionV1::DropAuthZoneSignatureProofs
            | InstructionV1::CreateProofFromBucketOfAmount { .. }
            | InstructionV1::CreateProofFromBucketOfNonFungibles { .. }
            | InstructionV1::CreateProofFromBucketOfAll { .. }
            | InstructionV1::CloneProof { .. }
            | InstructionV1::DropProof { .. }
            | InstructionV1::DropNamedProofs
            | InstructionV1::DropAllProofs
            | InstructionV1::CallFunction { .. } => false,
        };

        // Handle required method call
        match instruction {
            InstructionV1::CallMethod {
                address,
                method_name,
                ..
            } if is_validator(address)
                && method_name == VALIDATOR_CLAIM_XRD_IDENT =>
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

impl ExecutionSummaryCallback for ValidatorClaimDetector {
    fn on_instruction(
        &mut self,
        instruction: &InstructionV1,
        _: usize,
        input_resources: &[ResourceSpecifier],
        output_resources: &[ResourceSpecifier],
    ) {
        match instruction {
            InstructionV1::CallMethod {
                address: dynamic_address @ DynamicGlobalAddress::Static(address),
                method_name,
                ..
            } if is_validator(dynamic_address)
                && (method_name == VALIDATOR_CLAIM_XRD_IDENT) =>
            {
                let validator_component = ComponentAddress::try_from(*address)
                    .expect("Must succeed!");

                let Some(ResourceSpecifier::Ids(
                    claim_nft_resource_address,
                    claim_nft_ids,
                )) = input_resources.first()
                else {
                    return;
                };

                let Some(ResourceSpecifier::Amount(XRD, xrd_amount)) =
                    output_resources.first()
                else {
                    return;
                };

                self.tracked_claim.push(TrackedValidatorClaim {
                    validator_address: validator_component,
                    claim_nft_address: *claim_nft_resource_address,
                    claim_nft_ids: claim_nft_ids.clone(),
                    xrd_amount: *xrd_amount,
                });
            }
            _ => { /* No-op */ }
        }
    }
}

impl ValidatorClaimDetector {
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
                        | EntityType::GlobalVirtualSecp256k1Account
                        | EntityType::GlobalVirtualEd25519Account => FnRules {
                            allowed: &[
                                /* All withdraw methods */
                                ACCOUNT_WITHDRAW_IDENT,
                                ACCOUNT_WITHDRAW_NON_FUNGIBLES_IDENT,
                                /* All deposit methods */
                                ACCOUNT_DEPOSIT_IDENT,
                                ACCOUNT_DEPOSIT_BATCH_IDENT,
                                ACCOUNT_TRY_DEPOSIT_OR_ABORT_IDENT,
                                ACCOUNT_TRY_DEPOSIT_BATCH_OR_ABORT_IDENT,
                                /* Lock Fees */
                                ACCOUNT_LOCK_FEE_IDENT,
                                ACCOUNT_LOCK_CONTINGENT_FEE_IDENT,
                                ACCOUNT_LOCK_FEE_AND_WITHDRAW_IDENT,
                                ACCOUNT_LOCK_FEE_AND_WITHDRAW_NON_FUNGIBLES_IDENT,
                            ],
                            disallowed: &[],
                            default: FnRule::Disallowed,
                        },
                        EntityType::GlobalValidator => FnRules {
                            allowed: &[
                                VALIDATOR_CLAIM_XRD_IDENT
                            ],
                            disallowed: &[],
                            default: FnRule::Disallowed
                        },
                        /* Disallowed */
                        EntityType::GlobalGenericComponent
                        | EntityType::GlobalIdentity
                        | EntityType::GlobalVirtualSecp256k1Identity
                        | EntityType::GlobalVirtualEd25519Identity
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
                        | EntityType::GlobalAccountLocker
                         => FnRules::all_disallowed(),
                    }
                    })
                    .unwrap_or(FnRules::all_disallowed())
            }
        }
    }
}

impl Default for ValidatorClaimDetector {
    fn default() -> Self {
        Self {
            is_valid: true,
            required_method_called: false,
            validators: Default::default(),
            tracked_claim: Default::default(),
        }
    }
}
