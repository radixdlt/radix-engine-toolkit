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

use crate::instruction_visitor::core::error::InstructionVisitorError;
use crate::instruction_visitor::core::traits::InstructionVisitor;
use crate::sbor::indexed_manifest_value::IndexedManifestValue;
use crate::utils::{is_account, is_validator};

use radix_engine::system::system_modules::execution_trace::{ResourceSpecifier, WorktopChange};
use radix_engine::transaction::*;
use radix_engine_common::prelude::*;
use radix_engine_interface::blueprints::consensus_manager::VALIDATOR_STAKE_IDENT;
use scrypto::blueprints::account::*;
use scrypto::prelude::*;
use transaction::prelude::*;
use transaction::validation::ManifestIdAllocator;

pub struct StakeVisitor<'r> {
    /// The execution trace from the preview receipt
    execution_trace: &'r TransactionExecutionTrace,

    /// The account withdrawn from - tracked to ensure that we deposit into the same account.
    account_withdrawn_from: Option<ComponentAddress>,

    /// Maps the validator component address to a map of the LSU's resource address and amount
    /// obtained as part of staking.
    validator_stake_mapping: HashMap<ComponentAddress, Stake>,

    /// Tracks if the visitor is currently in an illegal state or not.
    is_illegal_state: bool,

    /// Used to allocate new ids
    id_allocator: ManifestIdAllocator,

    /// Tracks the buckets and their contents
    bucket_tracker: HashMap<ManifestBucket, (ResourceAddress, Decimal)>,

    /// The index of the current instruction.
    instruction_index: usize,
}

impl<'r> StakeVisitor<'r> {
    pub fn new(execution_trace: &'r TransactionExecutionTrace) -> Self {
        Self {
            execution_trace,
            account_withdrawn_from: Default::default(),
            validator_stake_mapping: Default::default(),
            is_illegal_state: Default::default(),
            id_allocator: Default::default(),
            bucket_tracker: Default::default(),
            instruction_index: Default::default(),
        }
    }

    fn is_take_from_worktop_allowed(&self, resource_address: &ResourceAddress) -> bool {
        *resource_address == XRD
            || self.validator_stake_mapping.values().any(
                |Stake {
                     liquid_stake_units_resource_address,
                     ..
                 }| liquid_stake_units_resource_address == resource_address,
            )
    }

    pub fn output(self) -> Option<(ComponentAddress, HashMap<ComponentAddress, Stake>)> {
        match (
            self.is_illegal_state,
            self.validator_stake_mapping.is_empty(),
            self.account_withdrawn_from,
        ) {
            (false, false, Some(account)) => Some((account, self.validator_stake_mapping)),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Stake {
    pub staked_xrd: Decimal,
    pub liquid_stake_units_resource_address: ResourceAddress,
    pub liquid_stake_units_amount: Decimal,
}

impl<'r> InstructionVisitor for StakeVisitor<'r> {
    fn is_enabled(&self) -> bool {
        !self.is_illegal_state
    }

    fn post_visit(&mut self) -> Result<(), InstructionVisitorError> {
        self.instruction_index += 1;
        Ok(())
    }

    fn visit_instruction(
        &mut self,
        instruction: &InstructionV1,
    ) -> Result<(), InstructionVisitorError> {
        match instruction {
            InstructionV1::CallMethod {
                address,
                method_name,
                args,
            } => {
                // Filter: We only permit static address - no dynamic or named addresses are allowed
                let global_address = if let DynamicGlobalAddress::Static(address) = address {
                    address
                } else {
                    self.is_illegal_state = true;
                    return Ok(());
                };

                /* Only withdraw of XRD is allowed from account */
                if is_account(global_address) && method_name == ACCOUNT_WITHDRAW_IDENT {
                    // Ensure arguments are valid and that the resource withdrawn is XRD.
                    let Some(AccountWithdrawInput {
                        resource_address: XRD,
                        ..
                    }) = manifest_encode(&args)
                        .ok()
                        .and_then(|encoded| manifest_decode(&encoded).ok())
                    else {
                        self.is_illegal_state = true;
                        return Ok(());
                    };
                    // Ensure that this is either the first time we withdraw or that this is the
                    // account we withdraw from all throughout the manifest.
                    let account_address = ComponentAddress::try_from(*global_address)
                        .expect("We have checked that it's a component address");
                    if let Some(previous_withdraw_component_address) = self.account_withdrawn_from {
                        if previous_withdraw_component_address != account_address {
                            self.is_illegal_state = true;
                            return Ok(());
                        }
                    } else {
                        self.account_withdrawn_from = Some(
                            (*global_address)
                                .try_into()
                                .expect("We have checked that it's a component address"),
                        );
                    }
                }
                /*
                Only permit account deposits to the same account withdrawn from and only with authed
                methods.
                 */
                else if is_account(global_address)
                    && (method_name == ACCOUNT_DEPOSIT_IDENT
                        || method_name == ACCOUNT_DEPOSIT_BATCH_IDENT)
                {
                    match self.account_withdrawn_from {
                        Some(withdraw_account)
                            if withdraw_account.into_node_id() == global_address.into_node_id() => {
                        }
                        Some(..) | None => {
                            self.is_illegal_state = true;
                            return Ok(());
                        }
                    }
                    let indexed_manifest_value = IndexedManifestValue::from_manifest_value(args);
                    for bucket in indexed_manifest_value.buckets() {
                        if self.bucket_tracker.remove(bucket).is_none() {
                            self.is_illegal_state = true;
                            return Ok(());
                        }
                    }
                }
                /* Staking to a validator */
                else if is_validator(global_address) && method_name == VALIDATOR_STAKE_IDENT {
                    let validator_address = ComponentAddress::try_from(*global_address)
                        .expect("We have checked that it's a component address");

                    let Some((bucket @ ManifestBucket(..),)) = manifest_encode(&args)
                        .ok()
                        .and_then(|encoded| manifest_decode(&encoded).ok())
                    else {
                        self.is_illegal_state = true;
                        return Ok(());
                    };
                    let Some((XRD, xrd_staked_amount)) = self.bucket_tracker.remove(&bucket) else {
                        self.is_illegal_state = true;
                        return Ok(());
                    };

                    let (liquid_stake_units_resource_address, liquid_stake_units_amount) =
                        match self
                            .execution_trace
                            .worktop_changes()
                            .get(&self.instruction_index)
                            .map(|x| x.as_slice())
                        {
                            Some(
                                [WorktopChange::Put(ResourceSpecifier::Amount(
                                    resource_address,
                                    amount,
                                ))],
                            ) => (*resource_address, *amount),
                            Some([]) | None => {
                                return Ok(());
                            }
                            _ => {
                                self.is_illegal_state = true;
                                return Ok(());
                            }
                        };

                    let entry = self
                        .validator_stake_mapping
                        .entry(validator_address)
                        .or_insert(Stake {
                            liquid_stake_units_resource_address,
                            liquid_stake_units_amount: Default::default(),
                            staked_xrd: Default::default(),
                        });
                    entry.liquid_stake_units_amount += liquid_stake_units_amount;
                    entry.staked_xrd += xrd_staked_amount;
                }
            }

            InstructionV1::TakeAllFromWorktop { resource_address } => {
                if self.is_take_from_worktop_allowed(resource_address) {
                    let amount = match self
                        .execution_trace
                        .worktop_changes()
                        .get(&self.instruction_index)
                        .map(|vec| vec.as_slice())
                    {
                        Some(
                            [WorktopChange::Take(ResourceSpecifier::Amount(
                                take_resource_address,
                                amount,
                            ))],
                        ) if resource_address == take_resource_address => *amount,
                        Some([]) | None => Decimal::ZERO,
                        _ => {
                            self.is_illegal_state = true;
                            return Ok(());
                        }
                    };
                    let bucket_id = self.id_allocator.new_bucket_id();
                    self.bucket_tracker
                        .insert(bucket_id, (*resource_address, amount));
                } else {
                    self.is_illegal_state = true;
                    return Ok(());
                }
            }
            InstructionV1::TakeFromWorktop {
                resource_address,
                amount,
            } => {
                if self.is_take_from_worktop_allowed(resource_address) {
                    let bucket_id = self.id_allocator.new_bucket_id();
                    self.bucket_tracker
                        .insert(bucket_id, (*resource_address, *amount));
                } else {
                    self.is_illegal_state = true;
                    return Ok(());
                }
            }

            /* Disallowed Instructions */
            InstructionV1::CallFunction { .. }
            | InstructionV1::CallRoyaltyMethod { .. }
            | InstructionV1::CallMetadataMethod { .. }
            | InstructionV1::CallRoleAssignmentMethod { .. }
            | InstructionV1::CallDirectVaultMethod { .. }
            | InstructionV1::DropNamedProofs
            | InstructionV1::DropAllProofs
            | InstructionV1::DropAuthZoneProofs { .. }
            | InstructionV1::DropAuthZoneRegularProofs { .. }
            | InstructionV1::DropAuthZoneSignatureProofs { .. }
            | InstructionV1::CreateProofFromAuthZoneOfAll { .. }
            | InstructionV1::CreateProofFromBucketOfAmount { .. }
            | InstructionV1::CreateProofFromBucketOfNonFungibles { .. }
            | InstructionV1::CreateProofFromBucketOfAll { .. }
            | InstructionV1::BurnResource { .. }
            | InstructionV1::CloneProof { .. }
            | InstructionV1::DropProof { .. }
            | InstructionV1::TakeNonFungiblesFromWorktop { .. }
            | InstructionV1::ReturnToWorktop { .. }
            | InstructionV1::AssertWorktopContainsAny { .. }
            | InstructionV1::AssertWorktopContains { .. }
            | InstructionV1::AssertWorktopContainsNonFungibles { .. }
            | InstructionV1::PopFromAuthZone { .. }
            | InstructionV1::PushToAuthZone { .. }
            | InstructionV1::CreateProofFromAuthZoneOfAmount { .. }
            | InstructionV1::CreateProofFromAuthZoneOfNonFungibles { .. }
            | InstructionV1::AllocateGlobalAddress { .. } => {
                self.is_illegal_state = true;
                return Ok(());
            }
        };
        Ok(())
    }
}
