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

use radix_transactions::prelude::*;
use scrypto::prelude::*;

use radix_engine::system::system_modules::execution_trace::ResourceSpecifier;
use radix_engine_interface::blueprints::account::*;
use radix_engine_interface::blueprints::pool::*;

use crate::contains;
use crate::transaction_types::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackedPoolRedemption {
    pub pool_address: ComponentAddress,
    /* Input */
    pub pool_units_resource_address: ResourceAddress,
    pub pool_units_amount: Decimal,
    /* Output */
    pub redeemed_resources: IndexMap<ResourceAddress, Decimal>,
}

pub struct PoolRedemptionDetector {
    is_valid: bool,
    required_method_called: bool,
    /// The pools encountered in this manifest that were redeemed from.
    pools: IndexSet<ComponentAddress>,
    /// Tracks the redemptions that occurred in the transaction.
    tracked_redemptions: Vec<TrackedPoolRedemption>,
}

impl PoolRedemptionDetector {
    pub fn output(
        self,
    ) -> Option<(IndexSet<ComponentAddress>, Vec<TrackedPoolRedemption>)> {
        if self.is_valid() {
            Some((self.pools, self.tracked_redemptions))
        } else {
            None
        }
    }
}

impl ManifestSummaryCallback for PoolRedemptionDetector {
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
            } if is_pool(address)
                && contains!(method_name => [
                    ONE_RESOURCE_POOL_REDEEM_IDENT,
                    TWO_RESOURCE_POOL_REDEEM_IDENT,
                    MULTI_RESOURCE_POOL_REDEEM_IDENT,
                ]) =>
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
                matches!(
                    entity_type,
                    EntityType::GlobalOneResourcePool
                        | EntityType::GlobalTwoResourcePool
                        | EntityType::GlobalMultiResourcePool
                )
            })
        {
            self.pools.insert(
                ComponentAddress::try_from(address).expect("Must succeed"),
            );
        }
    }
}

impl ExecutionSummaryCallback for PoolRedemptionDetector {
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
            } if is_pool(dynamic_address)
                && (method_name == ONE_RESOURCE_POOL_REDEEM_IDENT
                    || method_name == TWO_RESOURCE_POOL_REDEEM_IDENT
                    || method_name == MULTI_RESOURCE_POOL_REDEEM_IDENT) =>
            {
                let pool_address = ComponentAddress::try_from(*address)
                    .expect("Must be a valid component address");

                // The pool unit resource is the only input resource - if none
                // are found then an empty bucket of input resources was given
                // so we need to just ignore this operation.
                let Some(ResourceSpecifier::Amount(
                    pool_unit_resource_address,
                    pool_unit_amount,
                )) = input_resources.first()
                else {
                    return;
                };

                let mut tracked_redemption = TrackedPoolRedemption {
                    pool_address,
                    pool_units_resource_address: *pool_unit_resource_address,
                    pool_units_amount: *pool_unit_amount,
                    redeemed_resources: Default::default(),
                };

                for resource_specifier in output_resources {
                    let ResourceSpecifier::Amount(resource_address, amount) =
                        resource_specifier
                    else {
                        continue;
                    };
                    tracked_redemption
                        .redeemed_resources
                        .entry(*resource_address)
                        .or_default()
                        .add_assign(*amount);
                }
                self.tracked_redemptions.push(tracked_redemption)
            }
            _ => { /* No-op */ }
        }
    }
}

impl PoolRedemptionDetector {
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
                            | EntityType::GlobalVirtualEd25519Account => {
                                FnRules {
                                    allowed: &[
                                        /* All withdraw methods */
                                        ACCOUNT_WITHDRAW_IDENT,
                                        /* All deposit methods */
                                        ACCOUNT_DEPOSIT_IDENT,
                                        ACCOUNT_DEPOSIT_BATCH_IDENT,
                                        ACCOUNT_TRY_DEPOSIT_OR_ABORT_IDENT,
                                        ACCOUNT_TRY_DEPOSIT_BATCH_OR_ABORT_IDENT,
                                        /* Account Lock Fee */
                                        ACCOUNT_LOCK_FEE_IDENT,
                                        ACCOUNT_LOCK_CONTINGENT_FEE_IDENT,
                                        ACCOUNT_LOCK_FEE_AND_WITHDRAW_IDENT,
                                    ],
                                    disallowed: &[],
                                    default: FnRule::Disallowed,
                                }
                            }
                            EntityType::GlobalOneResourcePool => FnRules {
                                allowed: &[ONE_RESOURCE_POOL_REDEEM_IDENT],
                                disallowed: &[],
                                default: FnRule::Disallowed,
                            },
                            EntityType::GlobalTwoResourcePool => FnRules {
                                allowed: &[TWO_RESOURCE_POOL_REDEEM_IDENT],
                                disallowed: &[],
                                default: FnRule::Disallowed,
                            },
                            EntityType::GlobalMultiResourcePool => FnRules {
                                allowed: &[MULTI_RESOURCE_POOL_REDEEM_IDENT],
                                disallowed: &[],
                                default: FnRule::Disallowed,
                            },
                            /* Disallowed */
                            EntityType::GlobalGenericComponent
                            | EntityType::GlobalIdentity
                            | EntityType::GlobalVirtualSecp256k1Identity
                            | EntityType::GlobalVirtualEd25519Identity
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

fn is_pool(address: &DynamicGlobalAddress) -> bool {
    match address {
        DynamicGlobalAddress::Static(address) => address
            .as_node_id()
            .entity_type()
            .is_some_and(|entity_type| {
                matches!(
                    entity_type,
                    EntityType::GlobalOneResourcePool
                        | EntityType::GlobalTwoResourcePool
                        | EntityType::GlobalMultiResourcePool
                )
            }),
        DynamicGlobalAddress::Named(_) => false,
    }
}

impl Default for PoolRedemptionDetector {
    fn default() -> Self {
        Self {
            is_valid: true,
            required_method_called: false,
            pools: Default::default(),
            tracked_redemptions: Default::default(),
        }
    }
}
