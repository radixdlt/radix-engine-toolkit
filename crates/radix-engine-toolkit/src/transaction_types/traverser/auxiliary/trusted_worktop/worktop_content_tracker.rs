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
            if let Some(res) =
                self.worktop_content.get(&resources.resource_address())
            {
                // if found then exted with passed values
                match res {
                    ResourceSpecifier::Amount(_address, amount) => {
                        self.worktop_content.insert(
                            resources.resource_address(),
                            ResourceSpecifier::Amount(
                                resources.resource_address(),
                                amount
                                    .checked_add(*resources.amount().unwrap())
                                    .unwrap(),
                            ),
                        );
                    }
                    ResourceSpecifier::Ids(_address, ids) => {
                        let mut new_ids = ids.clone();
                        new_ids.extend(resources.ids().unwrap().clone());
                        self.worktop_content.insert(
                            resources.resource_address(),
                            ResourceSpecifier::Ids(
                                resources.resource_address(),
                                new_ids,
                            ),
                        );
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
        if let Some(res) =
            self.worktop_content.get(&resources.resource_address())
        {
            // if found then subtract passed values
            match res {
                ResourceSpecifier::Amount(_address, amount) => {
                    if resources.resource_address().is_fungible() {
                        self.worktop_content.insert(
                            resources.resource_address(),
                            ResourceSpecifier::Amount(
                                resources.resource_address(),
                                amount
                                    .checked_sub(*resources.amount().unwrap())
                                    .unwrap(),
                            ),
                        );
                        true
                    } else {
                        // don't know which non fungibles will be taken
                        // not setting untracked worktop content mode, as other instructions can still be valid
                        false
                    }
                }
                ResourceSpecifier::Ids(_address, ids) => {
                    if !resources.resource_address().is_fungible() {
                        let mut new_ids = ids.clone();
                        new_ids.retain(|item| {
                            !resources.ids().unwrap().contains(item)
                        });
                        self.worktop_content.insert(
                            resources.resource_address(),
                            ResourceSpecifier::Ids(
                                resources.resource_address(),
                                new_ids,
                            ),
                        );
                        true
                    } else {
                        // cannot take fungible -> worktop content is untracked
                        self.untracked_mode = true;
                        false
                    }
                }
            }
        } else {
            false
        }
    }

    pub fn take_from_worktop_by_address(
        &mut self,
        resource_address: ResourceAddress,
    ) -> Option<ResourceSpecifier> {
        self.worktop_content
            .remove(&resource_address)
            .map(|item| item.clone())
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
