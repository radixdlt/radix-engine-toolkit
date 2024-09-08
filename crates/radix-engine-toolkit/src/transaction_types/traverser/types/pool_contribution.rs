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

use radix_transactions::manifest::*;
use radix_transactions::prelude::*;
use scrypto::prelude::*;

use radix_engine::system::system_modules::execution_trace::ResourceSpecifier;
use radix_engine_interface::blueprints::account::*;
use radix_engine_interface::blueprints::pool::*;

use crate::contains;
use crate::transaction_types::types::*;
use crate::transaction_types::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackedPoolContribution {
    pub pool_address: ComponentAddress,
    /* Input */
    pub contributed_resources: IndexMap<ResourceAddress, Decimal>,
    /* Output */
    pub pool_units_resource_address: ResourceAddress,
    pub pool_units_amount: Decimal,
}

pub struct PoolContributionDetector {
    is_valid: bool,
    required_method_called: bool,
    /// The pools encountered in this manifest that were contributed to.
    pools: IndexSet<ComponentAddress>,
    /// Tracks the contributions that occurred in the transaction
    tracked_contributions: Vec<TrackedPoolContribution>,
}

impl PoolContributionDetector {
    pub fn output(
        self,
    ) -> Option<(IndexSet<ComponentAddress>, Vec<TrackedPoolContribution>)>
    {
        if self.is_valid() {
            Some((self.pools, self.tracked_contributions))
        } else {
            None
        }
    }
}

impl ManifestSummaryCallback for PoolContributionDetector {
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
            | InstructionV2::AssertWorktopIsEmpty { .. } => true,
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
            }) if is_pool(address)
                && contains!(method_name => [
                    ONE_RESOURCE_POOL_CONTRIBUTE_IDENT,
                    TWO_RESOURCE_POOL_CONTRIBUTE_IDENT,
                    MULTI_RESOURCE_POOL_CONTRIBUTE_IDENT,
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
                ComponentAddress::try_from(address).expect("Must succeed!"),
            );
        }
    }
}

impl ExecutionSummaryCallback for PoolContributionDetector {
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
            }) if is_pool(dynamic_address)
                && (method_name == ONE_RESOURCE_POOL_CONTRIBUTE_IDENT
                    || method_name == TWO_RESOURCE_POOL_CONTRIBUTE_IDENT
                    || method_name == MULTI_RESOURCE_POOL_CONTRIBUTE_IDENT) =>
            {
                let pool_address = ComponentAddress::try_from(*address)
                    .expect("Must be a valid component address");

                // Determine the output pool units. If we can't find them then
                // it means that no pool units were returned and that nothing
                // was contributed.
                let Some(ResourceSpecifier::Amount(
                    pool_unit_resource_address,
                    pool_unit_amount,
                )) = Self::pool_unit_resource_specifier(
                    input_resources,
                    output_resources,
                )
                else {
                    return;
                };

                let mut tracked_contribution = TrackedPoolContribution {
                    pool_address,
                    pool_units_resource_address: pool_unit_resource_address,
                    pool_units_amount: pool_unit_amount,
                    contributed_resources: Default::default(),
                };

                // Accounting for how much resources were contributed from the
                // input and the output (the change).
                for resource_specifier in input_resources.iter() {
                    let ResourceSpecifier::Amount(resource_address, amount) =
                        resource_specifier
                    else {
                        continue;
                    };

                    tracked_contribution
                        .contributed_resources
                        .entry(*resource_address)
                        .or_default()
                        .add_assign(*amount);
                }
                for resource_specifier in output_resources.iter() {
                    let ResourceSpecifier::Amount(resource_address, amount) =
                        resource_specifier
                    else {
                        continue;
                    };
                    let Some(entry) = tracked_contribution
                        .contributed_resources
                        .get_mut(resource_address)
                    else {
                        continue;
                    };
                    entry.sub_assign(*amount);
                }
                tracked_contribution.contributed_resources =
                    tracked_contribution
                        .contributed_resources
                        .into_iter()
                        .filter(|(_k, v)| !v.is_zero())
                        .collect();

                self.tracked_contributions.push(tracked_contribution);
            }
            _ => { /* No-op */ }
        }
    }
}

impl PoolContributionDetector {
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
                                        /* Lock Fee methods */
                                        ACCOUNT_LOCK_FEE_IDENT,
                                        ACCOUNT_LOCK_CONTINGENT_FEE_IDENT,
                                        ACCOUNT_LOCK_FEE_AND_WITHDRAW_IDENT,
                                    ],
                                    disallowed: &[],
                                    default: FnRule::Disallowed,
                                }
                            }
                            EntityType::GlobalOneResourcePool => FnRules {
                                allowed: &[ONE_RESOURCE_POOL_CONTRIBUTE_IDENT],
                                disallowed: &[],
                                default: FnRule::Disallowed,
                            },
                            EntityType::GlobalTwoResourcePool => FnRules {
                                allowed: &[TWO_RESOURCE_POOL_CONTRIBUTE_IDENT],
                                disallowed: &[],
                                default: FnRule::Disallowed,
                            },
                            EntityType::GlobalMultiResourcePool => FnRules {
                                allowed: &[MULTI_RESOURCE_POOL_CONTRIBUTE_IDENT],
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

    fn pool_unit_resource_specifier(
        input: &[ResourceSpecifier],
        output: &[ResourceSpecifier],
    ) -> Option<ResourceSpecifier> {
        // The pool unit resource specifier is that which is only present in the
        // output and not in the input. We also account for the pool returning
        // change so we do not use index based detection as it would't reliable
        // in this case.
        let input_resources = input
            .iter()
            .map(|specifier| specifier.resource_address())
            .collect::<IndexSet<_>>();

        output
            .iter()
            .find(|specifier| {
                !input_resources.contains(&specifier.resource_address())
            })
            .cloned()
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

impl Default for PoolContributionDetector {
    fn default() -> Self {
        Self {
            is_valid: true,
            required_method_called: false,
            pools: Default::default(),
            tracked_contributions: Default::default(),
        }
    }
}
