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

use super::{InstructionResource, StaticWorktopContentsTracker};

impl StaticWorktopContentsTracker {
    fn handle_account_methods(
        &mut self,
        method_name: &str,
        args: &ManifestValue,
    ) {
        match method_name {
            // withdraw resources from account by address and amount
            ACCOUNT_WITHDRAW_IDENT => {
                let input_args: AccountWithdrawInput =
                    to_manifest_type(args).expect("Must succeed");

                if input_args.resource_address.is_fungible() {
                    // put fungible by amount to worktop -> trusted
                    let resources = ResourceSpecifier::Amount(
                        input_args.resource_address,
                        input_args.amount.clone(),
                    );
                    self.worktop_content_tracker
                        .put_to_worktop(resources.clone());
                    self.add_new_instruction(
                        InstructionResource::StaticallyKnown(resources),
                    );
                } else {
                    // put nonfungible by amount to worktop -> non trusted
                    // we don't know what is on worktop so entering untracked mode
                    self.worktop_content_tracker.enter_untracked_mode();
                    self.add_new_instruction(InstructionResource::Unknown);
                }
            }

            // withdraw non fugible resources from account
            ACCOUNT_WITHDRAW_NON_FUNGIBLES_IDENT => {
                let input_args: AccountWithdrawNonFungiblesInput =
                    to_manifest_type(args).expect("Must succeed");
                let resources = ResourceSpecifier::Ids(
                    input_args.resource_address,
                    input_args.ids.clone(),
                );

                self.worktop_content_tracker
                    .put_to_worktop(resources.clone());
                self.add_new_instruction(InstructionResource::StaticallyKnown(
                    resources,
                ));
            }

            // withdraw resources from account by address and amount
            ACCOUNT_LOCK_FEE_AND_WITHDRAW_IDENT => {
                let input_args: AccountLockFeeAndWithdrawInput =
                    to_manifest_type(args).expect("Must succeed");

                if input_args.resource_address.is_fungible() {
                    // put fungible by amount to worktop -> trusted
                    let resources = ResourceSpecifier::Amount(
                        input_args.resource_address,
                        input_args.amount.clone(),
                    );
                    self.worktop_content_tracker
                        .put_to_worktop(resources.clone());
                    self.add_new_instruction(
                        InstructionResource::StaticallyKnown(resources),
                    );
                } else {
                    // put non fungible by amount to worktop -> non trusted,
                    // we don't know what is on worktop so entering untracked mode
                    self.worktop_content_tracker.enter_untracked_mode();
                    self.add_new_instruction(InstructionResource::Unknown);
                }
            }

            // withdraw non fugible resources from account
            ACCOUNT_LOCK_FEE_AND_WITHDRAW_NON_FUNGIBLES_IDENT => {
                let input_args: AccountLockFeeAndWithdrawNonFungiblesInput =
                    to_manifest_type(args).expect("Must succeed");
                let resources = ResourceSpecifier::Ids(
                    input_args.resource_address,
                    input_args.ids.clone(),
                );

                self.worktop_content_tracker
                    .put_to_worktop(resources.clone());
                self.add_new_instruction(InstructionResource::StaticallyKnown(
                    resources,
                ));
            }

            // deposits into an account
            ACCOUNT_DEPOSIT_IDENT | ACCOUNT_TRY_DEPOSIT_OR_ABORT_IDENT => {
                let input_args = IndexedManifestValue::from_typed(args);

                if !input_args.expressions().is_empty() {
                    assert_eq!(input_args.expressions().len(), 1);

                    match input_args
                        .expressions()
                        .first()
                        .expect("Expected expresion")
                    {
                        ManifestExpression::EntireWorktop => {
                            if !self.worktop_content_tracker.is_untracked_mode()
                            {
                                let resources = self
                                    .worktop_content_tracker
                                    .take_all_from_worktop();
                                self.add_new_instruction(
                                    InstructionResource::StaticallyKnownMany(
                                        &resources,
                                    ),
                                );
                            } else {
                                // take all from worktop will clear worktop so
                                // switch back to worktop tracked mode
                                self.worktop_content_tracker
                                    .take_all_from_worktop();
                                self.add_new_instruction(
                                    InstructionResource::Unknown,
                                );
                            }
                        }
                        _ => self
                            .add_new_instruction(InstructionResource::Unknown),
                    }
                } else {
                    if !self.bucket_tracker.is_untracked_mode() {
                        assert_eq!(input_args.buckets().len(), 1);
                        let bucket_id = input_args
                            .buckets()
                            .first()
                            .expect("Expected bucket");
                        let bucket = self
                            .bucket_tracker
                            .bucket_consumed(bucket_id)
                            .expect("Bucket not found");
                        self.add_new_instruction_from_bucket(&bucket);
                    } else {
                        self.add_new_instruction(InstructionResource::Unknown);
                    }
                }
            }
            ACCOUNT_DEPOSIT_BATCH_IDENT
            | ACCOUNT_TRY_DEPOSIT_BATCH_OR_ABORT_IDENT => {
                let input_args = IndexedManifestValue::from_typed(args);

                let (trusted, expression_resources) = if !input_args
                    .expressions()
                    .is_empty()
                {
                    match input_args
                        .expressions()
                        .first()
                        .expect("Expected expresion")
                    {
                        ManifestExpression::EntireWorktop => {
                            // handle worktop & buckets
                            if !self.worktop_content_tracker.is_untracked_mode()
                                && !self.bucket_tracker.is_untracked_mode()
                                && !self
                                    .bucket_tracker
                                    .is_any_bucket_with_unknown_resources()
                            {
                                let resources = self
                                    .worktop_content_tracker
                                    .take_all_from_worktop();

                                (
                                    true,
                                    Some(Self::merge_same_resources(
                                        &resources,
                                    )),
                                )
                            } else {
                                // take all from worktop will clear worktop so
                                // switch back to worktop tracked mode
                                self.worktop_content_tracker
                                    .take_all_from_worktop();
                                (false, None)
                            }
                        }
                        _ => (false, None),
                    }
                } else {
                    (false, None)
                };

                if !input_args.buckets().is_empty() {
                    if !self.bucket_tracker.is_untracked_mode() {
                        let mut found_all_resources = true;
                        let mut resources: Vec<ResourceSpecifier> =
                            Vec::with_capacity(input_args.buckets().len());
                        for bucket_id in input_args.buckets() {
                            let bucket = self
                                .bucket_tracker
                                .bucket_consumed(bucket_id)
                                .expect("Bucket not found");
                            if bucket.is_known_resources() {
                                if let Some(resource) = bucket.take_resources()
                                {
                                    // put resource on list only when bucket is not empty
                                    resources.push(resource);
                                }
                            } else {
                                // bucket with unknown resource -> untrusted instruction,
                                // iterate to consume rest of the buckets
                                found_all_resources = false;
                            }
                        }
                        if found_all_resources
                            && trusted
                            && expression_resources.is_some()
                        {
                            // merge resources from expression part
                            resources.extend(expression_resources.unwrap());

                            self.add_new_instruction(
                                InstructionResource::StaticallyKnownMany(
                                    &Self::merge_same_resources(&resources),
                                ),
                            );
                        } else if found_all_resources && !trusted {
                            self.add_new_instruction(
                                InstructionResource::StaticallyKnownMany(
                                    &Self::merge_same_resources(&resources),
                                ),
                            );
                        } else {
                            self.add_new_instruction(
                                InstructionResource::Unknown,
                            );
                        }
                    } else {
                        // even if expression was used we don't have list of all
                        // resources so instruction must be untrasted
                        self.add_new_instruction(InstructionResource::Unknown);
                    }
                } else {
                    // only expression was specified so use that data now
                    if let Some(resources) = expression_resources {
                        self.add_new_instruction(
                            InstructionResource::StaticallyKnownMany(
                                &resources,
                            ),
                        );
                    } else {
                        self.add_new_instruction(InstructionResource::Unknown);
                    }
                }
            }
            ACCOUNT_TRY_DEPOSIT_OR_REFUND_IDENT
            | ACCOUNT_TRY_DEPOSIT_BATCH_OR_REFUND_IDENT => {
                // resturns unknown resources put on worktop
                self.add_new_instruction(InstructionResource::Unknown);
                self.worktop_content_tracker.enter_untracked_mode();
            }

            // all other methods are trusted as they doesn't change the worktop state
            _ => self
                .add_new_instruction(InstructionResource::StaticallyKnownNone),
        }
    }

    fn handle_validator_methods(
        &mut self,
        method_name: &str,
        args: &ManifestValue,
    ) {
        match method_name {
            VALIDATOR_APPLY_REWARD_IDENT
            | VALIDATOR_APPLY_EMISSION_IDENT
            | VALIDATOR_LOCK_OWNER_STAKE_UNITS_IDENT => {
                if !self.bucket_tracker.is_untracked_mode() {
                    // invalidate input bucket
                    let input_args = IndexedManifestValue::from_typed(args);
                    assert_eq!(input_args.buckets().len(), 1);
                    let bucket_id =
                        input_args.buckets().first().expect("Expected bucket");
                    let bucket = self
                        .bucket_tracker
                        .bucket_consumed(bucket_id)
                        .expect("Bucket not found");
                    self.add_new_instruction_from_bucket(&bucket);
                } else {
                    self.add_new_instruction(InstructionResource::Unknown);
                }
            }

            VALIDATOR_FINISH_UNLOCK_OWNER_STAKE_UNITS_IDENT => {
                // resturns unknown resources put on worktop
                self.add_new_instruction(InstructionResource::Unknown);
                self.worktop_content_tracker.enter_untracked_mode();
            }

            // all other methods are trusted as they doesn't change the worktop state
            _ => self
                .add_new_instruction(InstructionResource::StaticallyKnownNone),
        }
    }

    fn handle_identity_methods(
        &mut self,
        method_name: &str,
        _args: &ManifestValue,
    ) {
        match method_name {
            IDENTITY_SECURIFY_IDENT => {
                // resturns unknown resources put on worktop
                self.add_new_instruction(InstructionResource::Unknown);
                self.worktop_content_tracker.enter_untracked_mode();
            }

            // all other methods are trusted as they doesn't change the worktop state
            _ => self
                .add_new_instruction(InstructionResource::StaticallyKnownNone),
        }
    }

    fn handle_access_controller_methods(
        &mut self,
        method_name: &str,
        _args: &ManifestValue,
    ) {
        match method_name {
            ACCESS_CONTROLLER_QUICK_CONFIRM_PRIMARY_ROLE_BADGE_WITHDRAW_ATTEMPT_IDENT
            | ACCESS_CONTROLLER_QUICK_CONFIRM_RECOVERY_ROLE_BADGE_WITHDRAW_ATTEMPT_IDENT
            | ACCESS_CONTROLLER_MINT_RECOVERY_BADGES_IDENT => {
                // resturns unknown resources put on worktop
                self.add_new_instruction(InstructionResource::Unknown);
                self.worktop_content_tracker.enter_untracked_mode();
            }

            // all other methods are trusted as they doesn't change the worktop state
            _ => self
                .add_new_instruction(InstructionResource::StaticallyKnownNone),
        }
    }

    fn handle_package_methods(
        &mut self,
        method_name: &str,
        _args: &ManifestValue,
    ) {
        match method_name {
            PACKAGE_CLAIM_ROYALTIES_IDENT => {
                // resturns unknown resources put on worktop
                self.add_new_instruction(InstructionResource::Unknown);
                self.worktop_content_tracker.enter_untracked_mode();
            }

            // all other methods are trusted as they doesn't change the worktop state
            _ => self
                .add_new_instruction(InstructionResource::StaticallyKnownNone),
        }
    }

    fn handle_fungible_resource_manager_methods(
        &mut self,
        address: ResourceAddress,
        method_name: &str,
        args: &ManifestValue,
    ) {
        match method_name {
            FUNGIBLE_RESOURCE_MANAGER_MINT_IDENT => {
                let input_args: FungibleResourceManagerMintInput =
                    to_manifest_type(args).expect("Must succeed");

                let r = ResourceSpecifier::Amount(address, input_args.amount);
                self.worktop_content_tracker.put_to_worktop(r.clone());
                self.add_new_instruction(InstructionResource::StaticallyKnown(
                    r,
                ));
            }

            // all other methods are trusted as they doesn't change the worktop state
            _ => self
                .add_new_instruction(InstructionResource::StaticallyKnownNone),
        }
    }

    fn handle_non_fungible_resource_manager_methods(
        &mut self,
        address: ResourceAddress,
        method_name: &str,
        args: &ManifestValue,
    ) {
        match method_name {
            NON_FUNGIBLE_RESOURCE_MANAGER_MINT_IDENT => {
                let input_args: NonFungibleResourceManagerMintManifestInput =
                    to_manifest_type(args).expect("Must succeed");

                let r = ResourceSpecifier::Ids(address, input_args.entries.keys().cloned().collect());
                self.worktop_content_tracker.put_to_worktop(r.clone());
                self.add_new_instruction( InstructionResource::StaticallyKnown(r));
            }
            NON_FUNGIBLE_RESOURCE_MANAGER_MINT_RUID_IDENT // don't know id so untrasted
            | NON_FUNGIBLE_RESOURCE_MANAGER_MINT_SINGLE_RUID_IDENT => {
                // resturns unknown resources put on worktop
                self.add_new_instruction( InstructionResource::Unknown);
                self.worktop_content_tracker.enter_untracked_mode();
            }

            // all other methods are trusted as they doesn't change the worktop state
            _ => self.add_new_instruction( InstructionResource::StaticallyKnownNone),
        }
    }

    fn handle_one_resource_pool_methods(
        &mut self,
        method_name: &str,
        args: &ManifestValue,
    ) {
        match method_name {
            ONE_RESOURCE_POOL_CONTRIBUTE_IDENT => {
                if !self.bucket_tracker.is_untracked_mode() {
                    let input_args: OneResourcePoolContributeManifestInput =
                        to_manifest_type(args).expect("Must succeed");

                    let bucket = self
                        .bucket_tracker
                        .bucket_consumed(&input_args.bucket)
                        .expect("Bucket not found");
                    self.add_new_instruction_from_bucket(&bucket);

                    // returned pool units are put on worktop, but we don't know exact resource type
                    // so we are entering untracked worktop mode
                    self.worktop_content_tracker.enter_untracked_mode();
                } else {
                    self.add_new_instruction(InstructionResource::Unknown);
                }
            }
            ONE_RESOURCE_POOL_REDEEM_IDENT => {
                if !self.bucket_tracker.is_untracked_mode() {
                    let input_args: OneResourcePoolRedeemManifestInput =
                        to_manifest_type(args).expect("Must succeed");

                    let bucket = self
                        .bucket_tracker
                        .bucket_consumed(&input_args.bucket)
                        .expect("Bucket not found");
                    self.add_new_instruction_from_bucket(&bucket);

                    // returned pool units are put on worktop, but we don't know exact resource type
                    // so we are entering untracked worktop mode
                    self.worktop_content_tracker.enter_untracked_mode();
                } else {
                    self.add_new_instruction(InstructionResource::Unknown);
                }
            }
            ONE_RESOURCE_POOL_PROTECTED_WITHDRAW_IDENT => {
                // returned resources are put on worktop, but we don't know exact resource address
                // so we are entering untracked worktop mode
                self.add_new_instruction(InstructionResource::Unknown);
                self.worktop_content_tracker.enter_untracked_mode();
            }
            ONE_RESOURCE_POOL_PROTECTED_DEPOSIT_IDENT => {
                if !self.bucket_tracker.is_untracked_mode() {
                    // invalidate input bucket
                    let input_args: OneResourcePoolProtectedDepositManifestInput =
                        to_manifest_type(args).expect("Must succeed");
                    let bucket = self
                        .bucket_tracker
                        .bucket_consumed(&input_args.bucket)
                        .expect("Bucket not found");
                    self.add_new_instruction_from_bucket(&bucket);
                } else {
                    self.add_new_instruction(InstructionResource::Unknown);
                }
            }

            // all other methods are trusted as they doesn't change the worktop state
            _ => self
                .add_new_instruction(InstructionResource::StaticallyKnownNone),
        }
    }

    fn handle_two_resource_pool_methods(
        &mut self,
        method_name: &str,
        args: &ManifestValue,
    ) {
        match method_name {
            TWO_RESOURCE_POOL_CONTRIBUTE_IDENT => {
                if !self.bucket_tracker.is_untracked_mode() {
                    let input_args: TwoResourcePoolContributeManifestInput =
                        to_manifest_type(args).expect("Must succeed");

                    let bucket_1 = self
                        .bucket_tracker
                        .bucket_consumed(&input_args.buckets.0)
                        .expect("Bucket not found");
                    let bucket_2 = self
                        .bucket_tracker
                        .bucket_consumed(&input_args.buckets.1)
                        .expect("Bucket not found");

                    if bucket_1.is_known_resources()
                        && bucket_2.is_known_resources()
                    {
                        let resource_1 = bucket_1.take_resources();
                        let resource_2 = bucket_2.take_resources();
                        if resource_1.is_some() && resource_2.is_some() {
                            self.add_new_instruction(
                                InstructionResource::StaticallyKnownMany(&[
                                    resource_1.unwrap(),
                                    resource_2.unwrap(),
                                ]),
                            );
                        } else {
                            self.add_new_instruction(
                                InstructionResource::Unknown,
                            );
                        }
                    } else {
                        self.add_new_instruction(InstructionResource::Unknown);
                    }

                    // returned pool units are put on worktop, but we don't know exact resource type
                    // so we are entering untracked worktop mode
                    self.worktop_content_tracker.enter_untracked_mode();
                } else {
                    self.add_new_instruction(InstructionResource::Unknown);
                }
            }
            TWO_RESOURCE_POOL_REDEEM_IDENT => {
                if !self.bucket_tracker.is_untracked_mode() {
                    let input_args: TwoResourcePoolRedeemManifestInput =
                        to_manifest_type(args).expect("Must succeed");

                    let bucket = self
                        .bucket_tracker
                        .bucket_consumed(&input_args.bucket)
                        .expect("Bucket not found");
                    self.add_new_instruction_from_bucket(&bucket);

                    // returned pool units are put on worktop, but we don't know exact resource type
                    // so we are entering untracked worktop mode
                    self.worktop_content_tracker.enter_untracked_mode();
                } else {
                    self.add_new_instruction(InstructionResource::Unknown);
                }
            }
            TWO_RESOURCE_POOL_PROTECTED_WITHDRAW_IDENT => {
                let input_args: TwoResourcePoolProtectedWithdrawManifestInput =
                    to_manifest_type(args).expect("Must succeed");

                if input_args.resource_address.is_fungible()
                    && matches!(
                        input_args.withdraw_strategy,
                        WithdrawStrategy::Exact
                    )
                {
                    let resource = ResourceSpecifier::Amount(
                        input_args.resource_address,
                        input_args.amount,
                    );
                    self.add_new_instruction(
                        InstructionResource::StaticallyKnown(resource.clone()),
                    );
                    self.worktop_content_tracker.put_to_worktop(resource);
                } else {
                    self.add_new_instruction(InstructionResource::Unknown);
                    self.worktop_content_tracker.enter_untracked_mode();
                }
            }
            TWO_RESOURCE_POOL_PROTECTED_DEPOSIT_IDENT => {
                if !self.bucket_tracker.is_untracked_mode() {
                    // invalidate input bucket
                    let input_args: TwoResourcePoolProtectedDepositManifestInput =
                        to_manifest_type(args).expect("Must succeed");
                    let bucket = self
                        .bucket_tracker
                        .bucket_consumed(&input_args.bucket)
                        .expect("Bucket not found");
                    self.add_new_instruction_from_bucket(&bucket);
                } else {
                    self.add_new_instruction(InstructionResource::Unknown);
                }
            }

            // all other methods are trusted as they doesn't change the worktop state
            _ => self
                .add_new_instruction(InstructionResource::StaticallyKnownNone),
        }
    }

    fn handle_multi_resource_pool_methods(
        &mut self,
        method_name: &str,
        args: &ManifestValue,
    ) {
        match method_name {
            MULTI_RESOURCE_POOL_CONTRIBUTE_IDENT => {
                if !self.bucket_tracker.is_untracked_mode() {
                    let input_args = IndexedManifestValue::from_typed(args);

                    if !input_args.expressions().is_empty() {
                        match input_args
                            .expressions()
                            .first()
                            .expect("Expected expresion")
                        {
                            ManifestExpression::EntireWorktop => {
                                if !self
                                    .worktop_content_tracker
                                    .is_untracked_mode()
                                {
                                    let resources = self
                                        .worktop_content_tracker
                                        .take_all_from_worktop();
                                    self.add_new_instruction(
                                         InstructionResource::StaticallyKnownMany(&Self::merge_same_resources(
                                            &resources
                                    )));
                                } else {
                                    // take all from worktop will clear worktop so
                                    // switch back to worktop tracked mode
                                    self.worktop_content_tracker
                                        .take_all_from_worktop();
                                    self.add_new_instruction(
                                        InstructionResource::Unknown,
                                    );
                                }
                            }
                            _ => self.add_new_instruction(
                                InstructionResource::Unknown,
                            ),
                        }
                    } else {
                        let input_args: MultiResourcePoolContributeManifestInput =
                            to_manifest_type(args).expect("Must succeed");

                        let resources: Vec<ResourceSpecifier> = input_args
                            .buckets
                            .iter()
                            .map(|bucket| {
                                self.bucket_tracker
                                    .bucket_consumed(&bucket)
                                    .expect("Bucket not found")
                            })
                            .filter(|bucket| {
                                !bucket.is_empty()
                                    && bucket.is_known_resources()
                            })
                            .map(|bucket| bucket.take_resources())
                            .flatten()
                            .collect();

                        // if we found all buckets in bucket tracker and all buckets has known resources
                        if resources.len() == input_args.buckets.len() {
                            self.add_new_instruction(
                                InstructionResource::StaticallyKnownMany(
                                    &resources,
                                ),
                            );
                        } else {
                            self.add_new_instruction(
                                InstructionResource::Unknown,
                            );
                        }
                    };

                    // returned pool units are put on worktop, but we don't know exact resource type
                    // so we are entering untracked worktop mode
                    self.worktop_content_tracker.enter_untracked_mode();
                } else {
                    self.add_new_instruction(InstructionResource::Unknown);
                }
            }
            MULTI_RESOURCE_POOL_REDEEM_IDENT => {
                // resturns unknown resources and puts them on worktop
                self.add_new_instruction(InstructionResource::Unknown);
                self.worktop_content_tracker.enter_untracked_mode();
            }
            MULTI_RESOURCE_POOL_PROTECTED_WITHDRAW_IDENT => {
                let input_args: MultiResourcePoolProtectedWithdrawManifestInput =
                    to_manifest_type(args).expect("Must succeed");

                if input_args.resource_address.is_fungible()
                    && matches!(
                        input_args.withdraw_strategy,
                        WithdrawStrategy::Exact
                    )
                {
                    self.add_new_instruction(
                        InstructionResource::StaticallyKnown(
                            ResourceSpecifier::Amount(
                                input_args.resource_address,
                                input_args.amount,
                            ),
                        ),
                    );
                } else {
                    self.add_new_instruction(InstructionResource::Unknown);
                }

                // returned pool units are put on worktop, but we don't know exact resource type
                // so we are entering untracked worktop mode
                self.worktop_content_tracker.enter_untracked_mode();
            }
            MULTI_RESOURCE_POOL_PROTECTED_DEPOSIT_IDENT => {
                if !self.bucket_tracker.is_untracked_mode() {
                    // invalidate input bucket
                    let input_args: MultiResourcePoolProtectedDepositManifestInput =
                        to_manifest_type(args).expect("Must succeed");
                    let bucket = self
                        .bucket_tracker
                        .bucket_consumed(&input_args.bucket)
                        .expect("Bucket not found");
                    self.add_new_instruction_from_bucket(&bucket);
                } else {
                    self.add_new_instruction(InstructionResource::Unknown);
                }
            }

            // all other methods are trusted as they doesn't change the worktop state
            _ => self
                .add_new_instruction(InstructionResource::StaticallyKnownNone),
        }
    }

    pub fn handle_call_methods(
        &mut self,
        address: &DynamicGlobalAddress,
        method_name: &str,
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
                    self.bucket_tracker.enter_untracked_mode();
                    self.worktop_content_tracker.enter_untracked_mode();
                    self.add_new_instruction(InstructionResource::Unknown);
                }
                DynamicGlobalAddress::Static(address) => {
                    if address
                        .as_node_id()
                        .is_global_fungible_resource_manager()
                    {
                        self.handle_fungible_resource_manager_methods(
                            ResourceAddress::new_or_panic(
                                address.as_node_id().0,
                            ),
                            method_name,
                            args,
                        );
                    } else if address
                        .as_node_id()
                        .is_global_non_fungible_resource_manager()
                    {
                        self.handle_non_fungible_resource_manager_methods(
                            ResourceAddress::new_or_panic(
                                address.as_node_id().0,
                            ),
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
                        self.handle_global_generic_component_method_call(
                            address,
                            method_name,
                            args,
                        );
                    }
                }
            }
        }
    }

    pub fn handle_call_royalty_methods(
        &mut self,
        method_name: &str,
        _args: &ManifestValue,
    ) {
        match method_name {
            COMPONENT_ROYALTY_CLAIM_ROYALTIES_IDENT => {
                // we don't know exactly what is put on worktop
                self.worktop_content_tracker.enter_untracked_mode();
                self.add_new_instruction(InstructionResource::Unknown);
            }

            // all other methods are trusted as they doesn't change the worktop state
            _ => self
                .add_new_instruction(InstructionResource::StaticallyKnownNone),
        }
    }

    fn handle_global_generic_component_method_call(
        &mut self,
        address: &GlobalAddress,
        method_name: &str,
        _args: &ManifestValue,
    ) {
        if FAUCET_COMPONENT.as_node_id() == address.as_node_id() {
            match method_name {
                "free" => {
                    // puts on worktop faucet::FAUCET_FREE_AMOUNT XRD count
                    let resources = ResourceSpecifier::Amount(
                        XRD,
                        faucet::FAUCET_FREE_AMOUNT.into(),
                    );
                    self.worktop_content_tracker
                        .put_to_worktop(resources.clone());
                    self.add_new_instruction(
                        InstructionResource::StaticallyKnown(resources),
                    );
                }
                "lock_fee" => {
                    self.add_new_instruction(
                        InstructionResource::StaticallyKnownNone,
                    );
                }
                _ => {
                    // unknown method call
                    self.add_new_instruction(InstructionResource::Unknown);
                }
            }
        } else if TRANSACTION_TRACKER.as_node_id() == address.as_node_id() {
            self.add_new_instruction(InstructionResource::StaticallyKnownNone);
        } else if GENESIS_HELPER.as_node_id() == address.as_node_id() {
            self.worktop_content_tracker.enter_untracked_mode();
            self.bucket_tracker.enter_untracked_mode();
            self.add_new_instruction(InstructionResource::Unknown);
        } else {
            // other unknown global or internal component call
            self.worktop_content_tracker.enter_untracked_mode();
            self.bucket_tracker.enter_untracked_mode();
            self.add_new_instruction(InstructionResource::Unknown);
        }
    }

    pub fn merge_same_resources(
        resources: &[ResourceSpecifier],
    ) -> Vec<ResourceSpecifier> {
        let mut set: IndexMap<ResourceAddress, Vec<&ResourceSpecifier>> =
            IndexMap::new();

        resources.iter().for_each(|resource| {
            if let Some((_, key, item)) =
                set.get_full_mut(&resource.resource_address())
            {
                assert_eq!(
                    resource.resource_address().is_fungible(),
                    key.is_fungible()
                );
                item.push(resource);
            } else {
                set.insert(resource.resource_address(), vec![resource]);
            }
        });

        let mut ret: Vec<ResourceSpecifier> = Vec::new();
        for (k, v) in set.iter() {
            if !v.is_empty() {
                ret.push(match v[0] {
                    ResourceSpecifier::Amount(_, _) => {
                        let mut amount = dec!(0);
                        for resource in v {
                            amount = amount
                                .checked_add(*resource.amount().unwrap())
                                .unwrap();
                        }
                        ResourceSpecifier::Amount(*k, amount)
                    }
                    ResourceSpecifier::Ids(_, _) => {
                        let mut new_ids: IndexSet<NonFungibleLocalId> =
                            IndexSet::new();
                        for resource in v {
                            new_ids.extend(resource.ids().unwrap().clone());
                        }
                        ResourceSpecifier::Ids(*k, new_ids)
                    }
                })
            }
        }
        ret
    }
}
