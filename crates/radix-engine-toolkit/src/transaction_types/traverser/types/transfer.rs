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

use radix_engine_interface::blueprints::account::*;

use crate::contains;
use crate::statics::*;
use crate::transaction_types::*;
use crate::utils::*;

pub struct TransferDetector {
    is_valid: bool,
    is_specific_instruction_encountered: bool,
    is_first_instruction_lock_fee: bool,
    instructions_match_simple_transfer: bool,
}

impl TransferDetector {
    pub fn is_valid(&self) -> bool {
        self.is_valid && self.is_specific_instruction_encountered
    }

    pub fn output(self) -> Option<bool> {
        if self.is_valid() {
            Some(self.instructions_match_simple_transfer)
        } else {
            None
        }
    }
}

impl ManifestSummaryCallback for TransferDetector {
    fn on_instruction(
        &mut self,
        instruction: &InstructionV1,
        instruction_index: usize,
    ) {
        if instruction_index == 0 {
            self.is_first_instruction_lock_fee = matches!(
                instruction,
                InstructionV1::CallMethod {
                    address,
                    method_name,
                    ..
                } if is_account(address)
                    && contains!(method_name => [
                        ACCOUNT_LOCK_FEE_IDENT,
                        ACCOUNT_LOCK_CONTINGENT_FEE_IDENT,
                        ACCOUNT_LOCK_FEE_AND_WITHDRAW_IDENT,
                        ACCOUNT_LOCK_FEE_AND_WITHDRAW_NON_FUNGIBLES_IDENT,
                    ])
            );
        }
        let offset = if self.is_first_instruction_lock_fee {
            1
        } else {
            0
        };

        /* Simple transfer accounting */
        self.instructions_match_simple_transfer &= (instruction_index == 0
            && matches!(
                instruction,
                InstructionV1::CallMethod {
                    address,
                    method_name,
                    ..
                } if is_account(address)
                    && contains!(method_name => [
                        ACCOUNT_LOCK_FEE_IDENT,
                        ACCOUNT_LOCK_CONTINGENT_FEE_IDENT,
                        ACCOUNT_LOCK_FEE_AND_WITHDRAW_IDENT,
                        ACCOUNT_LOCK_FEE_AND_WITHDRAW_NON_FUNGIBLES_IDENT,
                    ])
            ))
            || (instruction_index == offset
                && matches!(instruction, InstructionV1::CallMethod {
                    address,
                    method_name,
                    ..
                } if is_account(address)
                    && ACCOUNT_WITHDRAW_METHODS.contains(method_name)
                ))
            || (instruction_index == 1 + offset
                && matches!(
                    instruction,
                    InstructionV1::TakeFromWorktop { .. }
                ))
            || (instruction_index == 2 + offset
                && matches!(
                    instruction,
                    InstructionV1::CallMethod {
                        address,
                        method_name,
                        ..
                    } if is_account(address)
                        && ACCOUNT_DEPOSIT_METHODS.contains(method_name
                    )
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
            } => Self::construct_fn_rules(address).is_fn_permitted(method_name),
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
        };

        // Determine if the instruction is a transfer instruction.
        self.is_specific_instruction_encountered |=
            if let InstructionV1::CallMethod {
                address,
                method_name,
                ..
            } = instruction
            {
                Self::construct_specific_fn_rules(address)
                    .is_fn_permitted(method_name)
            } else {
                false
            };
    }

    fn on_finish(&mut self, instructions_count: usize) {
        if self.is_first_instruction_lock_fee {
            if instructions_count != 4 {
                self.instructions_match_simple_transfer = false
            }
        } else if instructions_count != 3 {
            self.instructions_match_simple_transfer = false
        }
        if instructions_count == 0 {
            self.is_valid = false
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
                                        ACCOUNT_WITHDRAW_NON_FUNGIBLES_IDENT,
                                        /* All deposit methods */
                                        ACCOUNT_DEPOSIT_IDENT,
                                        ACCOUNT_DEPOSIT_BATCH_IDENT,
                                        ACCOUNT_TRY_DEPOSIT_OR_ABORT_IDENT,
                                        ACCOUNT_TRY_DEPOSIT_BATCH_OR_ABORT_IDENT,
                                        /* Account Lock Fees */
                                        ACCOUNT_LOCK_FEE_IDENT,
                                        ACCOUNT_LOCK_CONTINGENT_FEE_IDENT,
                                        ACCOUNT_LOCK_FEE_AND_WITHDRAW_IDENT,
                                        ACCOUNT_LOCK_FEE_AND_WITHDRAW_NON_FUNGIBLES_IDENT,
                                    ],
                                    disallowed: &[],
                                    default: FnRule::Disallowed,
                                }
                            }
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
                            | EntityType::GlobalPreallocatedSecp256k1Identity
                            | EntityType::GlobalPreallocatedEd25519Identity
                            | EntityType::InternalGenericComponent
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
                                        /* All withdraw methods */
                                        ACCOUNT_WITHDRAW_IDENT,
                                        ACCOUNT_WITHDRAW_NON_FUNGIBLES_IDENT,
                                        /* All deposit methods */
                                        ACCOUNT_DEPOSIT_IDENT,
                                        ACCOUNT_DEPOSIT_BATCH_IDENT,
                                        ACCOUNT_TRY_DEPOSIT_OR_ABORT_IDENT,
                                        ACCOUNT_TRY_DEPOSIT_BATCH_OR_ABORT_IDENT,
                                        /* Account Lock Fees */
                                        ACCOUNT_LOCK_FEE_AND_WITHDRAW_IDENT,
                                        ACCOUNT_LOCK_FEE_AND_WITHDRAW_NON_FUNGIBLES_IDENT,
                                    ],
                                    disallowed: &[],
                                    default: FnRule::Disallowed,
                                }
                            }
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
                            | EntityType::GlobalPreallocatedSecp256k1Identity
                            | EntityType::GlobalPreallocatedEd25519Identity
                            | EntityType::InternalGenericComponent
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

impl Default for TransferDetector {
    fn default() -> Self {
        Self {
            is_valid: true,
            is_specific_instruction_encountered: false,
            is_first_instruction_lock_fee: false,
            instructions_match_simple_transfer: true,
        }
    }
}
