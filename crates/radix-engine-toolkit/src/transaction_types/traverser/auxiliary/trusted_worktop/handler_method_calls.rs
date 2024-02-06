use crate::sbor::indexed_manifest_value::IndexedManifestValue;
use crate::utils::*;
use radix_engine::system::system_modules::execution_trace::ResourceSpecifier;
use radix_engine_interface::api::node_modules::royalty::*;
use radix_engine_interface::blueprints::{
    access_controller::*, account::*, consensus_manager::*, identity::*,
    package::*, pool::*,
};
use scrypto::prelude::*;
use transaction::prelude::*;

use super::TrustedWorktop;

impl TrustedWorktop {
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
                    self.put_to_worktop(resources.clone());
                    self.add_new_instruction(true, Some(resources));
                } else {
                    // put nonfungible by amount to worktop -> non trusted
                    // we don't know what is on worktop so entering untracked mode
                    self.untrack_worktop_content = true;
                    self.add_new_instruction(false, None);
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

                self.put_to_worktop(resources.clone());
                self.add_new_instruction(true, Some(resources));
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
                    self.put_to_worktop(resources.clone());
                    self.add_new_instruction(true, Some(resources));
                } else {
                    // put non fungible by amount to worktop -> non trusted,
                    // we don't know what is on worktop so entering untracked mode
                    self.untrack_worktop_content = true;
                    self.add_new_instruction(false, None);
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

                self.put_to_worktop(resources.clone());
                self.add_new_instruction(true, Some(resources));
            }

            // deposits into an account
            ACCOUNT_DEPOSIT_IDENT | ACCOUNT_TRY_DEPOSIT_OR_ABORT_IDENT => {
                if !self.untrack_buckets {
                    let input_args = IndexedManifestValue::from_typed(args);

                    if input_args.expressions().len() > 0 {
                        assert_eq!(input_args.expressions().len(), 1);

                        match input_args
                            .expressions()
                            .first()
                            .expect("Expected expresion")
                        {
                            ManifestExpression::EntireWorktop => {
                                if !self.untrack_worktop_content {
                                    let resources =
                                        self.take_all_from_worktop();
                                    self.add_new_instruction_with_many_resources(true, resources);
                                } else {
                                    self.add_new_instruction(false, None);
                                }

                                // setting untracked buckets mode as we are not supporting handling vectors of buckets
                                //self.untrack_buckets = true;
                            }
                            _ => self.add_new_instruction(false, None),
                        }
                    } else {
                        assert_eq!(input_args.buckets().len(), 1);
                        let bucket_id = input_args
                            .buckets()
                            .first()
                            .expect("Expected bucket");
                        let resources = self
                            .bucket_consumed(bucket_id)
                            .expect("Bucket not found");
                        self.add_new_instruction(
                            resources.is_some(),
                            resources,
                        );
                    }
                } else {
                    self.add_new_instruction(false, None);
                }
            }
            ACCOUNT_DEPOSIT_BATCH_IDENT
            | ACCOUNT_TRY_DEPOSIT_BATCH_OR_ABORT_IDENT => {
                if !self.untrack_buckets {
                    let input_args = IndexedManifestValue::from_typed(args);

                    if input_args.expressions().len() > 0 {
                        match input_args
                            .expressions()
                            .first()
                            .expect("Expected expresion")
                        {
                            ManifestExpression::EntireWorktop => {
                                if !self.untrack_worktop_content {
                                    let resources =
                                        self.take_all_from_worktop();
                                    self.add_new_instruction_with_many_resources(true, resources);
                                } else {
                                    self.add_new_instruction(false, None);
                                }
                            }
                            _ => self.add_new_instruction(false, None),
                        }
                    } else {
                        let mut found_all_resources = true;
                        let mut resources =
                            Vec::with_capacity(input_args.buckets().len());
                        for bucket_id in input_args.buckets() {
                            if let Some(res) = self
                                .bucket_consumed(bucket_id)
                                .expect("Bucket not found")
                            {
                                resources.push(res);
                            } else {
                                // bucket with unknown resource -> untrusted instruction,
                                // iterate to consume rest of the buckets
                                found_all_resources = false;
                            }
                        }
                        if found_all_resources {
                            self.add_new_instruction_with_many_resources(
                                true,
                                Self::merge_same_resources(&resources),
                            );
                        } else {
                            self.add_new_instruction(false, None);
                        }
                    }
                } else {
                    self.add_new_instruction(false, None);
                }
            }
            ACCOUNT_TRY_DEPOSIT_OR_REFUND_IDENT
            | ACCOUNT_TRY_DEPOSIT_BATCH_OR_REFUND_IDENT => {
                // returns unknown bucket
                self.untrack_buckets = true;
                self.add_new_instruction(false, None);
            }

            // all other methods are trusted as they doesn't change the worktop state
            _ => self.add_new_instruction(true, None),
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
                if !self.untrack_buckets {
                    // invalidate input bucket
                    let input_args = IndexedManifestValue::from_typed(args);
                    assert_eq!(input_args.buckets().len(), 1);
                    let bucket_id =
                        input_args.buckets().first().expect("Expected bucket");
                    let resources = self
                        .bucket_consumed(bucket_id)
                        .expect("Bucket not found");
                    self.add_new_instruction(resources.is_some(), resources);
                } else {
                    self.add_new_instruction(false, None);
                }
            }

            VALIDATOR_FINISH_UNLOCK_OWNER_STAKE_UNITS_IDENT => {
                // returns unknown bucket
                self.untrack_buckets = true;
                self.add_new_instruction(false, None);
            }

            // all other methods are trusted as they doesn't change the worktop state
            _ => self.add_new_instruction(true, None),
        }
    }

    fn handle_identity_methods(
        &mut self,
        method_name: &str,
        _args: &ManifestValue,
    ) {
        match method_name {
            IDENTITY_SECURIFY_IDENT => {
                // returns unknown bucket
                self.new_bucket_unknown_resources();
                self.add_new_instruction(true, None);
            }

            // all other methods are trusted as they doesn't change the worktop state
            _ => self.add_new_instruction(true, None),
        }
    }

    fn handle_access_controller_methods(
        &mut self,
        method_name: &str,
        args: &ManifestValue,
    ) {
        match method_name {
            ACCESS_CONTROLLER_QUICK_CONFIRM_PRIMARY_ROLE_BADGE_WITHDRAW_ATTEMPT_IDENT
            | ACCESS_CONTROLLER_QUICK_CONFIRM_RECOVERY_ROLE_BADGE_WITHDRAW_ATTEMPT_IDENT
            | ACCESS_CONTROLLER_MINT_RECOVERY_BADGES_IDENT => {
                // returns unknown bucket
                self.untrack_buckets = true;
                self.add_new_instruction(false, None);
            }

            // all other methods are trusted as they doesn't change the worktop state
            _ => self.add_new_instruction(true, None),
        }
    }

    fn handle_package_methods(
        &mut self,
        method_name: &str,
        _args: &ManifestValue,
    ) {
        match method_name {
            PACKAGE_PUBLISH_WASM_IDENT | PACKAGE_CLAIM_ROYALTIES_IDENT => {
                // returns unknown bucket
                self.untrack_buckets = true;
                self.add_new_instruction(false, None);
            }

            // all other methods are trusted as they doesn't change the worktop state
            _ => self.add_new_instruction(true, None),
        }
    }

    fn handle_fungible_resource_manager_methods(
        &mut self,
        method_name: &str,
        _args: &ManifestValue,
    ) {
        match method_name {
            FUNGIBLE_RESOURCE_MANAGER_CREATE_WITH_INITIAL_SUPPLY_IDENT
            | FUNGIBLE_RESOURCE_MANAGER_MINT_IDENT => {
                // todo: mint: global address is res.addr and it is trusted
                // returns unknown bucket
                self.untrack_buckets = true;
                self.add_new_instruction(false, None);
            }

            // all other methods are trusted as they doesn't change the worktop state
            _ => self.add_new_instruction(true, None),
        }
    }

    fn handle_non_fungible_resource_manager_methods(
        &mut self,
        method_name: &str,
        _args: &ManifestValue,
    ) {
        match method_name {
            NON_FUNGIBLE_RESOURCE_MANAGER_CREATE_WITH_INITIAL_SUPPLY_IDENT
            | NON_FUNGIBLE_RESOURCE_MANAGER_CREATE_RUID_WITH_INITIAL_SUPPLY_IDENT
            | NON_FUNGIBLE_RESOURCE_MANAGER_MINT_IDENT // todo: trusted
            | NON_FUNGIBLE_RESOURCE_MANAGER_MINT_RUID_IDENT // don't know id so untrasted
            | NON_FUNGIBLE_RESOURCE_MANAGER_MINT_SINGLE_RUID_IDENT => {
                // returns unknown bucket
                self.untrack_buckets = true;
                self.add_new_instruction(false, None);
            }

            // all other methods are trusted as they doesn't change the worktop state
            _ => self.add_new_instruction(true, None),
        }
    }

    fn handle_one_resource_pool_methods(
        &mut self,
        method_name: &str,
        args: &ManifestValue,
    ) {
        match method_name {
            ONE_RESOURCE_POOL_CONTRIBUTE_IDENT
            | ONE_RESOURCE_POOL_REDEEM_IDENT
            | ONE_RESOURCE_POOL_PROTECTED_WITHDRAW_IDENT => {
                // returns unknown bucket
                self.untrack_buckets = true;
                self.add_new_instruction(false, None);
            }

            ONE_RESOURCE_POOL_PROTECTED_DEPOSIT_IDENT => {
                if !self.untrack_buckets {
                    // invalidate input bucket
                    let input_args: OneResourcePoolProtectedDepositManifestInput =
                        to_manifest_type(args).expect("Must succeed");
                    let resources = self
                        .bucket_consumed(&input_args.bucket)
                        .expect("Bucket not found");
                    self.add_new_instruction(resources.is_some(), resources);
                } else {
                    self.add_new_instruction(false, None);
                }
            }

            // all other methods are trusted as they doesn't change the worktop state
            _ => self.add_new_instruction(true, None),
        }
    }

    fn handle_two_resource_pool_methods(
        &mut self,
        method_name: &str,
        args: &ManifestValue,
    ) {
        match method_name {
            TWO_RESOURCE_POOL_CONTRIBUTE_IDENT
            | TWO_RESOURCE_POOL_REDEEM_IDENT
            | TWO_RESOURCE_POOL_PROTECTED_WITHDRAW_IDENT => {
                // returns unknown bucket
                self.untrack_buckets = true;
                self.add_new_instruction(false, None);
            }

            TWO_RESOURCE_POOL_PROTECTED_DEPOSIT_IDENT => {
                if !self.untrack_buckets {
                    // invalidate input bucket
                    let input_args: TwoResourcePoolProtectedDepositManifestInput =
                        to_manifest_type(args).expect("Must succeed");
                    let resources = self
                        .bucket_consumed(&input_args.bucket)
                        .expect("Bucket not found");
                    self.add_new_instruction(resources.is_some(), resources);
                } else {
                    self.add_new_instruction(false, None);
                }
            }

            // all other methods are trusted as they doesn't change the worktop state
            _ => self.add_new_instruction(true, None),
        }
    }

    fn handle_multi_resource_pool_methods(
        &mut self,
        method_name: &str,
        _args: &ManifestValue,
    ) {
        match method_name {
            MULTI_RESOURCE_POOL_CONTRIBUTE_IDENT
            | MULTI_RESOURCE_POOL_REDEEM_IDENT
            | MULTI_RESOURCE_POOL_PROTECTED_DEPOSIT_IDENT
            | MULTI_RESOURCE_POOL_PROTECTED_WITHDRAW_IDENT => {
                // todo: check withdrow according to strategy
                // returns unknown bucket
                self.untrack_buckets = true;
                self.add_new_instruction(false, None);
            }

            // all other methods are trusted as they doesn't change the worktop state
            _ => self.add_new_instruction(true, None),
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
                    // unknown component call, may return some unknown bucket
                    self.untrack_buckets = true;
                    self.untrack_worktop_content = true;
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
                // returns unknown bucket
                self.untrack_buckets = true;
                self.add_new_instruction(false, None);
            }

            // all other methods are trusted as they doesn't change the worktop state
            _ => self.add_new_instruction(true, None),
        }
    }

    fn handle_global_generic_component_method_call(
        &mut self,
        address: &GlobalAddress,
        method_name: &str,
        _args: &ManifestValue,
    ) {
        if FAUCET_COMPONENT.as_node_id() == address.as_node_id() {
            if method_name == "free" {
                // puts on worktop faucet::FAUCET_FREE_AMOUNT XRD count
                let resources = ResourceSpecifier::Amount(
                    XRD,
                    faucet::FAUCET_FREE_AMOUNT.into(),
                );
                self.put_to_worktop(resources.clone());
                self.add_new_instruction(true, Some(resources));
            } else {
                // method 'new' is trusted as it doesn't change the worktop state
                self.add_new_instruction(true, None);
            }
        } else if TRANSACTION_TRACKER.as_node_id() == address.as_node_id() {
            // method 'create' is trusted as it doesn't change the worktop state
            self.add_new_instruction(true, None);
        } else if GENESIS_HELPER.as_node_id() == address.as_node_id() {
            self.untrack_worktop_content = true;
            self.untrack_buckets = true;
            self.add_new_instruction(false, None);
        } else {
            // other unknown global or internal component call, may return some unknown bucket
            self.untrack_worktop_content = true;
            self.untrack_buckets = true;
            self.add_new_instruction(false, None);
        }
    }
}
