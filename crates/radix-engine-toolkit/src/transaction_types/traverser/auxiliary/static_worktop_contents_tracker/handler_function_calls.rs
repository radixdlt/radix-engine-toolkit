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
use crate::utils::*;
use radix_engine_interface::blueprints::{
    access_controller::*, account::*, consensus_manager::*, identity::*,
    package::*, pool::*,
};
use radix_transactions::prelude::*;
use scrypto::prelude::*;

use super::{StaticWorktopContentsTracker, TrackedResource};

impl StaticWorktopContentsTracker {
    fn unknown_function_call(&mut self) {
        self.bucket_tracker.enter_untracked_mode();
        self.worktop_content_tracker.enter_untracked_mode();
        self.add_new_instruction(TrackedResource::Unknown);
    }

    pub fn handle_call_functions(
        &mut self,
        address: &DynamicPackageAddress,
        blueprint_name: &str,
        function_name: &str,
        args: &ManifestValue,
    ) {
        if is_account(address) {
            match function_name {
                ACCOUNT_CREATE_ADVANCED_IDENT => self
                    .add_new_instruction(TrackedResource::StaticallyKnownNone),
                ACCOUNT_CREATE_IDENT => {
                    // returns unknown resources put on worktop
                    self.add_new_instruction(TrackedResource::Unknown);
                    self.worktop_content_tracker.enter_untracked_mode();
                }
                _ => self.unknown_function_call(),
            }
        } else if is_validator(address) {
            match function_name {
                CONSENSUS_MANAGER_CREATE_IDENT => self
                    .add_new_instruction(TrackedResource::StaticallyKnownNone),
                _ => self.unknown_function_call(),
            }
        } else if is_identity(address) {
            match function_name {
                IDENTITY_CREATE_ADVANCED_IDENT => {
                    self.add_new_instruction(
                        TrackedResource::StaticallyKnownNone,
                    );
                }
                IDENTITY_CREATE_IDENT => {
                    // resturns unknown resources put on worktop
                    self.add_new_instruction(TrackedResource::Unknown);
                    self.worktop_content_tracker.enter_untracked_mode();
                }
                _ => self.unknown_function_call(),
            }
        } else if is_access_controller(address) {
            match function_name {
                ACCESS_CONTROLLER_CREATE_IDENT => {
                    if !self.bucket_tracker.is_untracked_mode() {
                        // invalidate input bucket
                        let input_args = IndexedManifestValue::from_typed(args);
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
                        self.add_new_instruction(TrackedResource::Unknown);
                    }
                }
                _ => self.unknown_function_call(),
            }
        } else {
            match address {
                DynamicPackageAddress::Named(_) => {
                    self.unknown_function_call();
                }
                DynamicPackageAddress::Static(address) => self
                    .handle_static_package_address(
                        address,
                        blueprint_name,
                        function_name,
                        args,
                    ),
            }
        }
    }

    fn handle_static_package_address(
        &mut self,
        address: &PackageAddress,
        blueprint_name: &str,
        function_name: &str,
        args: &ManifestValue,
    ) {
        if *address == PACKAGE_PACKAGE {
            match function_name {
                PACKAGE_PUBLISH_WASM_ADVANCED_IDENT
                | PACKAGE_PUBLISH_NATIVE_IDENT => self
                    .add_new_instruction(TrackedResource::StaticallyKnownNone),
                PACKAGE_PUBLISH_WASM_IDENT => {
                    // resturns unknown resources put on worktop
                    self.add_new_instruction(TrackedResource::Unknown);
                    self.worktop_content_tracker.enter_untracked_mode();
                }
                _ => self.unknown_function_call(),
            }
        } else if address.as_node_id().is_global_fungible_resource_manager() {
            match function_name {
                FUNGIBLE_RESOURCE_MANAGER_CREATE_WITH_INITIAL_SUPPLY_IDENT => {
                    // resturns unknown resources put on worktop
                    self.add_new_instruction(TrackedResource::Unknown);
                    self.worktop_content_tracker.enter_untracked_mode();
                }
                _ => self.unknown_function_call(),
            }
        } else if address
            .as_node_id()
            .is_global_non_fungible_resource_manager()
        {
            match function_name {
                NON_FUNGIBLE_RESOURCE_MANAGER_CREATE_WITH_INITIAL_SUPPLY_IDENT
                | NON_FUNGIBLE_RESOURCE_MANAGER_CREATE_RUID_WITH_INITIAL_SUPPLY_IDENT =>
                {
                    // resturns unknown resources put on worktop
                    self.add_new_instruction(TrackedResource::Unknown);
                    self.worktop_content_tracker.enter_untracked_mode();
                }
                _ => self.unknown_function_call(),
            }
        } else if matches!(
            address.as_node_id().entity_type(),
            Some(EntityType::GlobalOneResourcePool)
        ) {
            match function_name {
                ONE_RESOURCE_POOL_INSTANTIATE_IDENT => {
                    self.add_new_instruction(
                        TrackedResource::StaticallyKnownNone,
                    );
                }
                _ => self.unknown_function_call(),
            }
        } else if matches!(
            address.as_node_id().entity_type(),
            Some(EntityType::GlobalTwoResourcePool)
        ) {
            match function_name {
                TWO_RESOURCE_POOL_INSTANTIATE_IDENT => {
                    self.add_new_instruction(
                        TrackedResource::StaticallyKnownNone,
                    );
                }
                _ => self.unknown_function_call(),
            }
        } else if matches!(
            address.as_node_id().entity_type(),
            Some(EntityType::GlobalMultiResourcePool)
        ) {
            match function_name {
                MULTI_RESOURCE_POOL_INSTANTIATE_IDENT => {
                    self.add_new_instruction(
                        TrackedResource::StaticallyKnownNone,
                    );
                }
                _ => self.unknown_function_call(),
            }
        } else {
            self.handle_global_generic_component_function_call(
                address,
                blueprint_name,
                function_name,
                args,
            );
        }
    }

    fn handle_global_generic_component_function_call(
        &mut self,
        address: &PackageAddress,
        _blueprint_name: &str,
        function_name: &str,
        args: &ManifestValue,
    ) {
        if FAUCET_PACKAGE == *address {
            if function_name == "new" {
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
                    self.add_new_instruction(TrackedResource::Unknown);
                }
            } else {
                self.unknown_function_call();
            }
        } else if TRANSACTION_TRACKER_PACKAGE == *address {
            // function 'create' is trusted as it doesn't change the worktop state
            self.add_new_instruction(TrackedResource::StaticallyKnownNone);
        } else if GENESIS_HELPER_PACKAGE == *address {
            self.unknown_function_call();
        } else {
            self.unknown_function_call();
        }
    }
}
