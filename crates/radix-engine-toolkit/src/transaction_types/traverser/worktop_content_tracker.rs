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
use radix_engine::system::system_modules::execution_trace::*;
use scrypto::prelude::*;
use std::ops::{AddAssign, Deref, SubAssign};
use transaction::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum WorktopContentItem {
    Amount(Decimal),
    Ids(IndexSet<NonFungibleLocalId>),
}
impl WorktopContentItem {
    pub fn amount(&self) -> Option<Decimal> {
        match self {
            WorktopContentItem::Amount(d) => Some(*d),
            _ => None,
        }
    }
    pub fn ids(&self) -> Option<&IndexSet<NonFungibleLocalId>> {
        match self {
            WorktopContentItem::Ids(ids) => Some(ids),
            _ => None,
        }
    }

    fn as_resource_specifier(
        &self,
        address: &ResourceAddress,
    ) -> ResourceSpecifier {
        match self {
            WorktopContentItem::Amount(d) => {
                ResourceSpecifier::Amount(address.clone(), *d)
            }
            WorktopContentItem::Ids(ids) => {
                ResourceSpecifier::Ids(address.clone(), ids.clone())
            }
        }
    }
}

impl From<ResourceIndicator> for WorktopContentItem {
    fn from(value: ResourceIndicator) -> Self {
        match ResourceSpecifier::from(value) {
            ResourceSpecifier::Amount(_, value) => {
                WorktopContentItem::Amount(value)
            }
            ResourceSpecifier::Ids(_, value) => WorktopContentItem::Ids(value),
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct WorktopContent {
    pub content: IndexMap<ResourceAddress, WorktopContentItem>,
}
impl WorktopContent {
    fn add(&mut self, resource_indicator: &ResourceIndicator) {
        self.content
            .entry(resource_indicator.resource_address())
            .and_modify(|value| {
                match value {
                    WorktopContentItem::Amount(amount) => {
                        if let ResourceIndicator::Fungible(_, value) =
                            resource_indicator
                        {
                            amount.add_assign(*value.deref());
                        }
                    }
                    WorktopContentItem::Ids(list) => {
                        if let ResourceSpecifier::Ids(_, ids) =
                            ResourceSpecifier::from(resource_indicator.clone())
                        {
                            list.extend(ids);
                        }
                    }
                };
            })
            .or_insert(WorktopContentItem::from(resource_indicator.clone()));
    }

    fn remove_by_address(&mut self, resource_address: &ResourceAddress) {
        self.content.remove(resource_address);
    }

    fn remove(&mut self, resource_indicator: &ResourceIndicator) {
        if let Some(item) =
            self.content.get_mut(&resource_indicator.resource_address())
        {
            match item {
                WorktopContentItem::Amount(amount) => {
                    if let ResourceIndicator::Fungible(_, value) =
                        resource_indicator
                    {
                        amount.sub_assign(*value.deref());
                        if amount.is_zero() {
                            self.content
                                .remove(&resource_indicator.resource_address());
                        }
                    }
                }
                WorktopContentItem::Ids(list) => {
                    if let ResourceSpecifier::Ids(_, ids) =
                        ResourceSpecifier::from(resource_indicator.clone())
                    {
                        list.retain(|item| !ids.contains(item));
                        if list.is_empty() {
                            self.content
                                .remove(&resource_indicator.resource_address());
                        }
                    }
                }
            }
        }
    }

    pub fn as_resource_specifiers(&self) -> Vec<ResourceSpecifier> {
        self.content
            .iter()
            .map(|(resource_address, item)| {
                item.as_resource_specifier(resource_address)
            })
            .collect()
    }
}

pub trait WorktopContentTrackerObserver {
    fn on_instruction(
        &mut self,
        instruction: &InstructionV1,
        instruction_index: usize,
        worktop_content: &Vec<WorktopContent>,
    );
}

#[derive(Default)]
pub struct WorktopContentTracker<'a> {
    subscribers: Vec<Box<&'a mut dyn WorktopContentTrackerObserver>>,
    state_per_instruction: Vec<WorktopContent>,
}

impl<'a> WorktopContentTracker<'a> {
    pub fn register_subscriber(&mut self, subscriber: &'a mut dyn WorktopContentTrackerObserver) {
        self.subscribers.push(Box::new(subscriber));
    }

    fn notify_subscribers(
        &mut self,
        instruction: &InstructionV1,
        instruction_index: usize,
    ) {
        for i in (0..self.subscribers.len()).into_iter() {
            self.subscribers[i].on_instruction(
                instruction,
                instruction_index,
                &self.state_per_instruction)
        }
    }

    fn add_new_instruction(&mut self) {
        let new_item = if let Some(item) = self.state_per_instruction.last() {
            item.clone()
        } else {
            WorktopContent::default()
        };

        self.state_per_instruction.push(new_item);
    }

    pub fn get_results(&mut self) -> Vec<WorktopContent> {
        self.state_per_instruction.to_owned()
    }
}

impl<'a> ManifestSummaryCallback
    for WorktopContentTracker<'a>
{
}

impl<'a> ExecutionSummaryCallback
    for WorktopContentTracker<'a>
{
    fn on_instruction(
        &mut self,
        instruction: &InstructionV1,
        instruction_index: usize,
        _input_resources: &[ResourceSpecifier],
        _output_resources: &[ResourceSpecifier],
        bucket_list: &IndexMap<ManifestBucket, ResourceIndicator>,
    ) {
        assert_eq!(instruction_index, self.state_per_instruction.len());
        self.add_new_instruction();

        match instruction {
            InstructionV1::TakeAllFromWorktop { resource_address } => self
                .state_per_instruction
                .last_mut()
                .expect("Must succeed")
                .remove_by_address(&resource_address),
            InstructionV1::TakeFromWorktop {
                resource_address,
                amount,
            } => self
                .state_per_instruction
                .last_mut()
                .expect("Must succeed")
                .remove(&ResourceIndicator::Fungible(
                    *resource_address,
                    FungibleResourceIndicator::Guaranteed(*amount),
                )),
            InstructionV1::TakeNonFungiblesFromWorktop {
                resource_address,
                ids,
            } => {
                let mut list: IndexSet<NonFungibleLocalId> =
                    IndexSet::with_capacity(ids.len());
                ids.iter().for_each(|item| {
                    list.insert(item.clone());
                });
                self.state_per_instruction
                    .last_mut()
                    .expect("Must succeed")
                    .remove(&ResourceIndicator::NonFungible(
                        *resource_address,
                        NonFungibleResourceIndicator::ByIds(list),
                    ));
            }
            InstructionV1::ReturnToWorktop { bucket_id } => {
                let resource =
                    bucket_list.get(bucket_id).expect("Must succeed");
                self.state_per_instruction
                    .last_mut()
                    .expect("Must succeed")
                    .add(resource);
            }
            _ => (),
        }

        self.notify_subscribers(instruction, instruction_index);
    }

    fn on_account_withdraw(
        &mut self,
        _account: &ComponentAddress,
        resource_indicator: &ResourceIndicator,
    ) {
        // put resource to worktop
        self.state_per_instruction
            .last_mut()
            .expect("Must succeed")
            .add(resource_indicator);
    }

    fn on_account_deposit(
        &mut self,
        _account: &ComponentAddress,
        resource_indicator: &ResourceIndicator,
    ) {
        // take resource from worktop
        self.state_per_instruction
            .last_mut()
            .expect("Must succeed")
            .remove(resource_indicator);
    }
}
