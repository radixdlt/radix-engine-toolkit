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

use crate::transaction_types::*;
use radix_engine::system::system_modules::execution_trace::ResourceSpecifier;
use scrypto::prelude::*;

#[derive(Default)]
pub struct WorktopContentTracker {
    // Current worktop content
    worktop_content: IndexMap<ResourceAddress, ResourceSpecifier>,
    // Information if we are in 'untracked worktop' mode, which is enabled
    // when we don't know what was put or taken from the worktop, triggered
    // by the upper layer.
    untracked_mode: bool,
}

impl WorktopContentTracker {
    pub fn is_untracked_mode(&self) -> bool {
        self.untracked_mode
    }

    pub fn enter_untracked_mode(&mut self) {
        self.untracked_mode = true;
    }

    pub fn put_to_worktop(&mut self, resources: ResourceSpecifier) {
        if !self.untracked_mode {
            if let Some(resource_worktop_content) =
                self.worktop_content.get_mut(&resources.resource_address())
            {
                // if found then exted with passed values

                match (resources, resource_worktop_content) {
                    (
                        ResourceSpecifier::Amount(_, incomming_amount),
                        ResourceSpecifier::Amount(_, amount),
                    ) => {
                        if let Some(new_value) =
                            amount.checked_add(incomming_amount)
                        {
                            *amount = new_value;
                        } else {
                            // set untracked mode as amount was unable to subtract
                            self.untracked_mode = true;
                        }
                    }
                    (
                        ResourceSpecifier::Ids(_, incomming_ids),
                        ResourceSpecifier::Ids(_, ids),
                    ) => {
                        ids.extend(incomming_ids.clone());
                    }
                    _ => {
                        // set untracked mode as incomming resource is incompatible
                        self.untracked_mode = true;
                    }
                }
            } else {
                self.worktop_content
                    .insert(resources.resource_address(), resources);
            }
        }
    }

    // return true in case of success
    pub fn take_from_worktop(&mut self, resources: ResourceSpecifier) -> bool {
        if let Some(resource_worktop_content) =
            self.worktop_content.get_mut(&resources.resource_address())
        {
            // if found then subtract passed values
            let resource_address = resource_worktop_content.resource_address();

            match (resources, resource_worktop_content) {
                (
                    ResourceSpecifier::Amount(_, incomming_amount),
                    ResourceSpecifier::Amount(_, amount),
                ) => {
                    if resource_address.is_fungible() {
                        if let Some(new_value) =
                            amount.checked_sub(incomming_amount)
                        {
                            *amount = new_value;

                            if amount.is_zero() {
                                self.worktop_content
                                    .swap_remove(&resource_address);
                            }

                            return true;
                        }
                        // else set untracked mode as amount was unable to subtract
                        self.untracked_mode = true;
                    }
                    // else return false as we don't know which non fungibles will be taken
                    // not setting untracked worktop content mode, as other instructions can still be valid
                }
                (
                    ResourceSpecifier::Ids(_, incomming_ids),
                    ResourceSpecifier::Ids(_, ids),
                ) => {
                    ids.retain(|item| !incomming_ids.contains(item));

                    if ids.is_empty() {
                        self.worktop_content.swap_remove(&resource_address);
                    }
                    return true;
                }
                _ => {
                    // set untracked mode as incomming resource is incompatible
                    self.untracked_mode = true;
                }
            }
        }
        false
    }

    pub fn take_from_worktop_by_address(
        &mut self,
        resource_address: ResourceAddress,
    ) -> Option<ResourceSpecifier> {
        self.worktop_content.swap_remove(&resource_address)
    }

    pub fn take_all_from_worktop(&mut self) -> Vec<ResourceSpecifier> {
        let ret = self
            .worktop_content
            .iter()
            .map(|(_k, v)| v.to_owned())
            .collect();
        // worktop is cleared so we can start tracking it back (if untracked)
        self.untracked_mode = false;
        self.worktop_content.clear();
        ret
    }
}
