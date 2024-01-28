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

use crate::sbor::indexed_manifest_value::IndexedManifestValue;
use crate::transaction_types::*;
use crate::utils::*;
use radix_engine::system::system_modules::execution_trace::ResourceSpecifier;
use radix_engine_interface::api::node_modules::royalty::*;
use radix_engine_interface::blueprints::{
    access_controller::*, account::*, consensus_manager::*, identity::*,
    package::*, pool::*,
};
use scrypto::prelude::*;
use transaction::prelude::*;

#[derive(Default)]
pub struct TrustedWorktop {
    trusted_state_per_instruction: Vec<(bool, Option<ResourceSpecifier>)>,
}

impl TrustedWorktop {
    fn add_new_instruction(
        &mut self,
        trusted: bool,
        worktop_content: Option<ResourceSpecifier>,
    ) {
        self.trusted_state_per_instruction
            .push((trusted, worktop_content));
    }

    pub fn is_worktop_trusted(
        &self,
        instruction_index: usize,
    ) -> Option<(bool, Option<ResourceSpecifier>)> {
        self.trusted_state_per_instruction
            .get(instruction_index)
            .map(|value| value.clone())
    }

    fn handle_account_methods(
        &mut self,
        method_name: &String,
        args: &ManifestValue,
    ) {
        match method_name.as_str() {
            // withdraw resources from account by address and amount
            ACCOUNT_WITHDRAW_IDENT => {
                let input_args: AccountWithdrawInput =
                    to_manifest_type(args).expect("Must succeed");

                if input_args.resource_address.is_fungible() {
                    // put fungible by amount to worktop -> trusted
                    self.add_new_instruction(
                        true,
                        Some(ResourceSpecifier::Amount(
                            input_args.resource_address,
                            input_args.amount.clone(),
                        )),
                    );
                } else {
                    // put nonfungible by amount to worktop -> non trusted
                    self.add_new_instruction(false, None);
                }
            }

            // withdraw non fugible resources from account
            ACCOUNT_WITHDRAW_NON_FUNGIBLES_IDENT => {
                let input_args: AccountWithdrawNonFungiblesInput =
                    to_manifest_type(args).expect("Must succeed");
                self.add_new_instruction(
                    true,
                    Some(ResourceSpecifier::Ids(
                        input_args.resource_address,
                        input_args.ids.clone(),
                    )),
                );
            }

            // withdraw resources from account by address and amount
            ACCOUNT_LOCK_FEE_AND_WITHDRAW_IDENT => {
                let input_args: AccountLockFeeAndWithdrawInput =
                    to_manifest_type(args).expect("Must succeed");

                if input_args.resource_address.is_fungible() {
                    // put fungible by amount to worktop -> trusted
                    self.add_new_instruction(
                        true,
                        Some(ResourceSpecifier::Amount(
                            input_args.resource_address,
                            input_args.amount.clone(),
                        )),
                    );
                } else {
                    // put nonfungible by amount to worktop -> non trusted
                    self.add_new_instruction(false, None);
                }
            }

            // withdraw non fugible resources from account
            ACCOUNT_LOCK_FEE_AND_WITHDRAW_NON_FUNGIBLES_IDENT => {
                let input_args: AccountLockFeeAndWithdrawNonFungiblesInput =
                    to_manifest_type(args).expect("Must succeed");
                self.add_new_instruction(
                    true,
                    Some(ResourceSpecifier::Ids(
                        input_args.resource_address,
                        input_args.ids.clone(),
                    )),
                );
            }

            // deposits into an account
            ACCOUNT_DEPOSIT_IDENT
            | ACCOUNT_DEPOSIT_BATCH_IDENT
            | ACCOUNT_TRY_DEPOSIT_OR_ABORT_IDENT
            | ACCOUNT_TRY_DEPOSIT_OR_REFUND_IDENT
            | ACCOUNT_TRY_DEPOSIT_BATCH_OR_ABORT_IDENT
            | ACCOUNT_TRY_DEPOSIT_BATCH_OR_REFUND_IDENT => {
                let _input_args = IndexedManifestValue::from_typed(args);
                // non trusted as we currently don't know what is in the bucket
                self.add_new_instruction(false, None);
            }

            // all other methods are trusted as they doesn't change the worktop state
            _ => self.add_new_instruction(true, None),
        }
    }

    fn handle_validator_methods(
        &mut self,
        method_name: &String,
        args: &ManifestValue,
    ) {
        match method_name.as_str() {
            VALIDATOR_APPLY_REWARD_IDENT
            | VALIDATOR_APPLY_EMISSION_IDENT
            | VALIDATOR_LOCK_OWNER_STAKE_UNITS_IDENT => {
                // todo invalidate input bucket
                self.add_new_instruction(true, None);
            }

            VALIDATOR_FINISH_UNLOCK_OWNER_STAKE_UNITS_IDENT => {
                let _input_args = IndexedManifestValue::from_typed(args);
                // non trusted as we currently don't know what is in the output bucket

                // todo: store bucket id as not trusted (unknown content)
                self.add_new_instruction(false, None); // todo: instruciton is trusted but bucket is not trusted
            }

            // all other methods are trusted as they doesn't change the worktop state
            _ => self.add_new_instruction(true, None),
        }
    }

    fn handle_identity_methods(
        &mut self,
        method_name: &String,
        args: &ManifestValue,
    ) {
        match method_name.as_str() {
            IDENTITY_CREATE_IDENT | IDENTITY_SECURIFY_IDENT => {
                let _input_args = IndexedManifestValue::from_typed(args);
                // non trusted as we currently don't know what is in the output bucket
                self.add_new_instruction(false, None); // todo: instruciton is trusted but bucket is not trusted
            }

            // all other methods are trusted as they doesn't change the worktop state
            _ => self.add_new_instruction(true, None),
        }
    }

    fn handle_access_controller_methods(
        &mut self,
        method_name: &String,
        args: &ManifestValue,
    ) {
        match method_name.as_str() {
            ACCESS_CONTROLLER_CREATE_IDENT => {
                // invalidates passed bucket
                self.add_new_instruction(true, None); // todo: instruciton is trusted but bucket is not trusted
            }

            ACCESS_CONTROLLER_QUICK_CONFIRM_PRIMARY_ROLE_BADGE_WITHDRAW_ATTEMPT_IDENT
            | ACCESS_CONTROLLER_QUICK_CONFIRM_RECOVERY_ROLE_BADGE_WITHDRAW_ATTEMPT_IDENT
            | ACCESS_CONTROLLER_MINT_RECOVERY_BADGES_IDENT => {
                let _input_args = IndexedManifestValue::from_typed(args);
                // non trusted as we currently don't know what is in the output bucket
                self.add_new_instruction(false, None);
            }

            // all other methods are trusted as they doesn't change the worktop state
            _ => self.add_new_instruction(true, None),
        }
    }

    fn handle_package_methods(
        &mut self,
        method_name: &String,
        args: &ManifestValue,
    ) {
        match method_name.as_str() {
            PACKAGE_PUBLISH_WASM_IDENT | PACKAGE_CLAIM_ROYALTIES_IDENT => {
                let _input_args = IndexedManifestValue::from_typed(args);
                // non trusted as we currently don't know what is in the output bucket
                self.add_new_instruction(false, None);
            }

            // all other methods are trusted as they doesn't change the worktop state
            _ => self.add_new_instruction(true, None),
        }
    }

    fn handle_fungible_resource_manager_methods(
        &mut self,
        method_name: &String,
        args: &ManifestValue,
    ) {
        match method_name.as_str() {
            FUNGIBLE_RESOURCE_MANAGER_CREATE_WITH_INITIAL_SUPPLY_IDENT
            | FUNGIBLE_RESOURCE_MANAGER_MINT_IDENT => {
                let _input_args = IndexedManifestValue::from_typed(args);
                // creates buckets with resources
                self.add_new_instruction(true, None);
            }

            // all other methods are trusted as they doesn't change the worktop state
            _ => self.add_new_instruction(true, None),
        }
    }

    fn handle_non_fungible_resource_manager_methods(
        &mut self,
        method_name: &String,
        args: &ManifestValue,
    ) {
        match method_name.as_str() {
            NON_FUNGIBLE_RESOURCE_MANAGER_CREATE_WITH_INITIAL_SUPPLY_IDENT
            | NON_FUNGIBLE_RESOURCE_MANAGER_CREATE_RUID_WITH_INITIAL_SUPPLY_IDENT
            | NON_FUNGIBLE_RESOURCE_MANAGER_MINT_IDENT
            | NON_FUNGIBLE_RESOURCE_MANAGER_MINT_RUID_IDENT
            | NON_FUNGIBLE_RESOURCE_MANAGER_MINT_SINGLE_RUID_IDENT => {
                let _input_args = IndexedManifestValue::from_typed(args);
                // creates buckets with resources
                self.add_new_instruction(true, None);
            }

            // all other methods are trusted as they doesn't change the worktop state
            _ => self.add_new_instruction(true, None),
        }
    }

    fn handle_one_resource_pool_methods(
        &mut self,
        method_name: &String,
        args: &ManifestValue,
    ) {
        match method_name.as_str() {
            ONE_RESOURCE_POOL_CONTRIBUTE_IDENT
            | ONE_RESOURCE_POOL_REDEEM_IDENT
            | ONE_RESOURCE_POOL_PROTECTED_DEPOSIT_IDENT => {
                let _input_args = IndexedManifestValue::from_typed(args);
                // takes bucket as input and returns bucket
                self.add_new_instruction(true, None);
            }

            ONE_RESOURCE_POOL_PROTECTED_WITHDRAW_IDENT => {
                // returns bucket
                self.add_new_instruction(false, None);
            }

            // all other methods are trusted as they doesn't change the worktop state
            _ => self.add_new_instruction(true, None),
        }
    }

    fn handle_two_resource_pool_methods(
        &mut self,
        method_name: &String,
        args: &ManifestValue,
    ) {
        match method_name.as_str() {
            TWO_RESOURCE_POOL_CONTRIBUTE_IDENT
            | TWO_RESOURCE_POOL_REDEEM_IDENT
            | TWO_RESOURCE_POOL_PROTECTED_DEPOSIT_IDENT => {
                let _input_args = IndexedManifestValue::from_typed(args);
                // takes bucket as input and returns bucket
                self.add_new_instruction(true, None);
            }

            TWO_RESOURCE_POOL_PROTECTED_WITHDRAW_IDENT => {
                // returns bucket
                self.add_new_instruction(false, None);
            }

            // all other methods are trusted as they doesn't change the worktop state
            _ => self.add_new_instruction(true, None),
        }
    }

    fn handle_multi_resource_pool_methods(
        &mut self,
        method_name: &String,
        args: &ManifestValue,
    ) {
        match method_name.as_str() {
            MULTI_RESOURCE_POOL_CONTRIBUTE_IDENT
            | MULTI_RESOURCE_POOL_REDEEM_IDENT
            | MULTI_RESOURCE_POOL_PROTECTED_DEPOSIT_IDENT => {
                let _input_args = IndexedManifestValue::from_typed(args);
                // takes bucket as input and returns bucket
                self.add_new_instruction(true, None);
            }

            MULTI_RESOURCE_POOL_PROTECTED_WITHDRAW_IDENT => {
                // returns bucket
                self.add_new_instruction(false, None);
            }

            // all other methods are trusted as they doesn't change the worktop state
            _ => self.add_new_instruction(true, None),
        }
    }

    fn handle_call_methods(
        &mut self,
        address: &DynamicGlobalAddress,
        method_name: &String,
        args: &ManifestValue,
    ) {
        if is_account(address) {
            self.handle_account_methods(method_name, args);
        } else if is_validator(address) {
            self.handle_validator_methods(method_name, args);
        } else if is_identity(address) {
            self.handle_identity_methods(method_name, args);
        } else if is_access_controller(address) {
            self.handle_access_controller_methods(method_name, args);
        } else if address.is_static_global_package() {
            self.handle_package_methods(method_name, args);
        } else {
            match address {
                DynamicGlobalAddress::Named(_) => {
                    // unknown component call
                    self.add_new_instruction(false, None);
                }
                DynamicGlobalAddress::Static(address) => {
                    if address
                        .as_node_id()
                        .is_global_fungible_resource_manager()
                    {
                        self.handle_fungible_resource_manager_methods(
                            method_name,
                            args,
                        );
                    } else if address
                        .as_node_id()
                        .is_global_non_fungible_resource_manager()
                    {
                        self.handle_non_fungible_resource_manager_methods(
                            method_name,
                            args,
                        );
                    } else if matches!(
                        address.as_node_id().entity_type(),
                        Some(EntityType::GlobalOneResourcePool)
                    ) {
                        self.handle_one_resource_pool_methods(
                            method_name,
                            args,
                        );
                    } else if matches!(
                        address.as_node_id().entity_type(),
                        Some(EntityType::GlobalTwoResourcePool)
                    ) {
                        self.handle_two_resource_pool_methods(
                            method_name,
                            args,
                        );
                    } else if matches!(
                        address.as_node_id().entity_type(),
                        Some(EntityType::GlobalMultiResourcePool)
                    ) {
                        self.handle_multi_resource_pool_methods(
                            method_name,
                            args,
                        );
                    } else {
                        // other global or internal component call
                        self.add_new_instruction(false, None);
                    }
                }
            }
        }
    }

    fn handle_call_royalty_methods(
        &mut self,
        method_name: &String,
        _args: &ManifestValue,
    ) {
        match method_name.as_str() {
            COMPONENT_ROYALTY_CLAIM_ROYALTIES_IDENT => {
                // returns bucket
                self.add_new_instruction(false, None);
            }

            // all other methods are trusted as they doesn't change the worktop state
            _ => self.add_new_instruction(true, None),
        }
    }
}

impl ManifestSummaryCallback for TrustedWorktop {
    fn on_instruction(
        &mut self,
        instruction: &InstructionV1,
        instruction_index: usize,
    ) {
        match instruction {
            InstructionV1::TakeAllFromWorktop { .. }
            | InstructionV1::TakeFromWorktop { .. }
            | InstructionV1::TakeNonFungiblesFromWorktop { .. } => {
                self.add_new_instruction(true, None)
            }

            InstructionV1::ReturnToWorktop { .. } => {
                self.add_new_instruction(false, None)
            }

            InstructionV1::AssertWorktopContainsAny { .. }
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
            | InstructionV1::CloneProof { .. }
            | InstructionV1::DropProof { .. }
            | InstructionV1::DropNamedProofs
            | InstructionV1::DropAllProofs
            | InstructionV1::AllocateGlobalAddress { .. } => {
                self.add_new_instruction(true, None)
            }

            InstructionV1::CreateProofFromBucketOfAmount { .. }
            | InstructionV1::CreateProofFromBucketOfNonFungibles { .. }
            | InstructionV1::CreateProofFromBucketOfAll { .. }
            | InstructionV1::BurnResource { .. } => {
                // changes buckets
                self.add_new_instruction(true, None)
            }

            InstructionV1::CallMethod {
                address,
                method_name,
                args,
            } => self.handle_call_methods(address, method_name, args),

            // call of a function from unknown blueprint
            InstructionV1::CallFunction { .. } => {
                self.add_new_instruction(false, None)
            }

            InstructionV1::CallRoyaltyMethod {
                method_name, args, ..
            } => self.handle_call_royalty_methods(method_name, args),

            InstructionV1::CallRoleAssignmentMethod { .. }
            | InstructionV1::CallMetadataMethod { .. } => {
                self.add_new_instruction(true, None)
            }

            InstructionV1::CallDirectVaultMethod { .. } => {
                self.add_new_instruction(false, None)
            }
        }

        assert_eq!(
            self.trusted_state_per_instruction.len(),
            instruction_index + 1
        );
    }
}
