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

use radix_engine_interface::blueprints::account::*;

use crate::transaction_types::*;

pub struct GeneralDetector {
    is_valid: bool,
}

impl GeneralDetector {
    pub fn is_valid(&self) -> bool {
        self.is_valid
    }

    pub fn output(self) -> Option<()> {
        if self.is_valid() { Some(()) } else { None }
    }
}

impl StaticAnalysisCallback for GeneralDetector {
    fn on_finish(&mut self, instructions_count: usize) {
        if instructions_count == 0 {
            self.is_valid = false
        }
    }

    fn on_instruction(&mut self, instruction: &InstructionV2, _: usize) {
        // Control whether or not this is allowed or not based on:
        // 1. Whether the instruction is allowed.
        // 2. Whether the instruction contents are allowed.
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
            InstructionV2::TakeFromWorktop(..)
            | InstructionV2::TakeNonFungiblesFromWorktop(..)
            | InstructionV2::TakeAllFromWorktop(..)
            | InstructionV2::ReturnToWorktop(..)
            | InstructionV2::AssertWorktopContainsAny(..)
            | InstructionV2::AssertWorktopContains(..)
            | InstructionV2::AssertWorktopContainsNonFungibles(..)
            | InstructionV2::AssertWorktopIsEmpty(..)
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
            | InstructionV2::CallFunction(..) => true,
            /* Not Permitted */
            InstructionV2::BurnResource(..)
            | InstructionV2::CallRoyaltyMethod(..)
            | InstructionV2::CallMetadataMethod(..)
            | InstructionV2::CallRoleAssignmentMethod(..)
            | InstructionV2::CallDirectVaultMethod(..)
            | InstructionV2::AllocateGlobalAddress(..)
            | InstructionV2::YieldToParent(_)
            | InstructionV2::YieldToChild(_)
            | InstructionV2::VerifyParent(_) => false,
        }
    }
}

impl DynamicAnalysisCallback for GeneralDetector {}

impl GeneralDetector {
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
                                        /* All proof creation methods */
                                        ACCOUNT_CREATE_PROOF_OF_AMOUNT_IDENT,
                                        ACCOUNT_CREATE_PROOF_OF_NON_FUNGIBLES_IDENT,
                                        /* Locking of fees */
                                        ACCOUNT_LOCK_FEE_IDENT,
                                        ACCOUNT_LOCK_CONTINGENT_FEE_IDENT,
                                        ACCOUNT_LOCK_FEE_AND_WITHDRAW_IDENT,
                                        ACCOUNT_LOCK_FEE_AND_WITHDRAW_NON_FUNGIBLES_IDENT,
                                    ],
                                    disallowed: &[],
                                    default: FnRule::Disallowed,
                                }
                            }
                            EntityType::GlobalGenericComponent
                            | EntityType::GlobalIdentity
                            | EntityType::GlobalPreallocatedSecp256k1Identity
                            | EntityType::GlobalPreallocatedEd25519Identity
                            | EntityType::InternalGenericComponent
                            | EntityType::GlobalAccountLocker => FnRules::all_allowed(),
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
                            | EntityType::GlobalOneResourcePool
                            | EntityType::GlobalTwoResourcePool
                            | EntityType::GlobalMultiResourcePool => {
                                FnRules::all_disallowed()
                            }
                        }
                    })
                    .unwrap_or(FnRules::all_disallowed())
            }
        }
    }
}

impl Default for GeneralDetector {
    fn default() -> Self {
        Self { is_valid: true }
    }
}
