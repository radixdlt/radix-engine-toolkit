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

use radix_engine::system::system_modules::execution_trace::*;
use scrypto::prelude::*;

use crate::transaction_types::*;

#[derive(Default)]
pub struct PresentedProofsDetector {
    presented_proofs: IndexMap<ComponentAddress, Vec<ResourceSpecifier>>,
}

impl PresentedProofsDetector {
    pub fn output(self) -> IndexMap<ComponentAddress, Vec<ResourceSpecifier>> {
        self.presented_proofs
    }
}

impl ManifestSummaryCallback for PresentedProofsDetector {
    fn on_create_proof(
        &mut self,
        account: &ComponentAddress,
        resource: &ResourceSpecifier,
    ) {
        self.presented_proofs.entry(*account).and_modify(|item| {
            if let Some(res) = item.iter().find(|res| {
                res.resource_address() == resource.resource_address()
            }) {
                match res {
                    ResourceSpecifier::Amount(_, mut amount) => {
                        match resource {
                            ResourceSpecifier::Amount(_, new_amount) => {
                                amount = amount
                                    .checked_add(*new_amount)
                                    .expect("Overflow");
                            }
                            ResourceSpecifier::Ids(_, _) => (),
                        }
                    }
                    ResourceSpecifier::Ids(_, mut ids) => match resource {
                        ResourceSpecifier::Amount(_, _) => (),
                        ResourceSpecifier::Ids(_, new_ids) => {
                            ids.extend(new_ids.clone());
                        }
                    },
                }
            } else {
                item.push(resource.clone());
            }
        });
    }
}

impl ExecutionSummaryCallback for PresentedProofsDetector {}
