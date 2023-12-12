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

use scrypto::prelude::*;
use transaction::prelude::*;

use radix_engine_interface::blueprints::account::*;

use crate::statics::*;
use crate::transaction_types::*;
use crate::utils::*;

pub struct TransferDetector {
    is_valid: bool,
    instructions_match_simple_transfer: bool,
}

impl ManifestSummaryCallback for TransferDetector {
    fn on_instruction(
        &mut self,
        instruction: &InstructionV1,
        instruction_index: usize,
    ) {
        /* Simple transfer accounting */
        self.instructions_match_simple_transfer &= (instruction_index == 0
            && matches!(instruction, InstructionV1::CallMethod { address, method_name, .. } if is_account(address) && ACCOUNT_WITHDRAW_METHODS.contains(method_name)))
            || (instruction_index == 1
                && matches!(
                    instruction,
                    InstructionV1::TakeFromWorktop { .. }
                        | InstructionV1::TakeNonFungiblesFromWorktop { .. }
                ))
            || (instruction_index == 2
                && matches!(
                    instruction,
                    InstructionV1::CallMethod { address, method_name, .. } if is_account(address) && ACCOUNT_DEPOSIT_METHODS.contains(method_name)
                ));

        /* Rules */
        // Control whether or not this is a transfer based on:
        // 1. Whether the instruction is allowed.
        // 2. Whether the instruction contents are allowed.
        self.is_valid &= match instruction {
            /* Maybe Permitted - Need more info */
            InstructionV1::CallMethod {
                address,
                method_name,
                ..
            } => {
                Self::construct_fn_rules(address).is_fn_permitted(method_name)
            }
            /* Not Permitted */
            InstructionV1::TakeFromWorktop { .. }
            | InstructionV1::TakeNonFungiblesFromWorktop { .. } => true,
            /* Not Permitted */
            InstructionV1::TakeAllFromWorktop { .. }
            | InstructionV1::ReturnToWorktop { .. }
            | InstructionV1::AssertWorktopContainsAny { .. }
            | InstructionV1::AssertWorktopContains { .. }
            | InstructionV1::AssertWorktopContainsNonFungibles { .. }
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
            | InstructionV1::BurnResource { .. }
            | InstructionV1::CloneProof { .. }
            | InstructionV1::DropProof { .. }
            | InstructionV1::CallFunction { .. }
            | InstructionV1::CallRoyaltyMethod { .. }
            | InstructionV1::CallMetadataMethod { .. }
            | InstructionV1::CallRoleAssignmentMethod { .. }
            | InstructionV1::CallDirectVaultMethod { .. }
            | InstructionV1::DropNamedProofs
            | InstructionV1::DropAllProofs
            | InstructionV1::AllocateGlobalAddress { .. } => false,
        }
    }

    fn on_finish(&mut self, instructions_count: usize) {
        if instructions_count != 3 {
            self.instructions_match_simple_transfer = false
        }
    }
}

impl ExecutionSummaryCallback for TransferDetector {}

impl TransferDetector {
    pub fn is_transfer(&self) -> bool {
        self.is_valid
    }

    pub fn is_simple_transfer(&self) -> bool {
        self.is_valid && self.instructions_match_simple_transfer
    }

    fn construct_fn_rules(address: &DynamicGlobalAddress) -> FnRules {
        match address {
            DynamicGlobalAddress::Named(..) => FnRules::all_disallowed(),
            DynamicGlobalAddress::Static(address) => {
                address.as_node_id().entity_type().map(|entity_type| {
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
                            ],
                            disallowed: &[
                                /* Securification */
                                ACCOUNT_SECURIFY_IDENT,
                                /* Direct Burn from Account */
                                ACCOUNT_BURN_IDENT,
                                ACCOUNT_BURN_NON_FUNGIBLES_IDENT,
                                /* Manipulation of the Authorized Depositors list */
                                ACCOUNT_ADD_AUTHORIZED_DEPOSITOR,
                                ACCOUNT_REMOVE_AUTHORIZED_DEPOSITOR,
                                /* Deposit or Refund Methods */
                                ACCOUNT_TRY_DEPOSIT_OR_REFUND_IDENT,
                                ACCOUNT_TRY_DEPOSIT_BATCH_OR_REFUND_IDENT,
                                /* Manipulation of the Resource Preferences */
                                ACCOUNT_SET_DEFAULT_DEPOSIT_RULE_IDENT,
                                ACCOUNT_SET_RESOURCE_PREFERENCE_IDENT,
                                ACCOUNT_REMOVE_RESOURCE_PREFERENCE_IDENT,
                                ACCOUNT_ADD_AUTHORIZED_DEPOSITOR,
                                ACCOUNT_REMOVE_AUTHORIZED_DEPOSITOR,
                                /* Deposit or Refund */
                                ACCOUNT_TRY_DEPOSIT_OR_REFUND_IDENT,
                                ACCOUNT_TRY_DEPOSIT_BATCH_OR_REFUND_IDENT,
                                /* All fee locking methods */
                                ACCOUNT_LOCK_FEE_IDENT,
                                ACCOUNT_LOCK_CONTINGENT_FEE_IDENT,
                                ACCOUNT_LOCK_FEE_AND_WITHDRAW_IDENT,
                                ACCOUNT_LOCK_FEE_AND_WITHDRAW_NON_FUNGIBLES_IDENT,
                                /* All proof creation methods */
                                ACCOUNT_CREATE_PROOF_OF_AMOUNT_IDENT,
                                ACCOUNT_CREATE_PROOF_OF_NON_FUNGIBLES_IDENT,
                            ],
                            default: FnRule::Disallowed,
                        },
                        /* Disallowed */
                        EntityType::GlobalPackage
                        | EntityType::GlobalValidator
                        | EntityType::GlobalFungibleResourceManager
                        | EntityType::GlobalNonFungibleResourceManager
                        | EntityType::GlobalConsensusManager
                        | EntityType::InternalFungibleVault
                        | EntityType::InternalNonFungibleVault
                        | EntityType::InternalKeyValueStore
                        | EntityType::GlobalTransactionTracker
                        | EntityType::GlobalAccessController
                        | EntityType::GlobalGenericComponent
                        | EntityType::GlobalIdentity
                        | EntityType::GlobalOneResourcePool
                        | EntityType::GlobalTwoResourcePool
                        | EntityType::GlobalMultiResourcePool
                        | EntityType::GlobalVirtualSecp256k1Identity
                        | EntityType::GlobalVirtualEd25519Identity
                        | EntityType::InternalGenericComponent => FnRules::all_disallowed(),
                    }
                }).unwrap_or(FnRules::all_disallowed())
            }
        }
    }
}
