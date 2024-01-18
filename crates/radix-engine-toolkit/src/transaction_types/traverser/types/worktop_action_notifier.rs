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

use std::borrow::Borrow;
use std::borrow::BorrowMut;

use scrypto::prelude::*;
use transaction::manifest::ast::Instruction;
use transaction::prelude::*;

use radix_engine::system::system_modules::execution_trace::*;
use radix_engine_interface::blueprints::account::*;
use radix_engine_interface::blueprints::consensus_manager::*;
use radix_engine_common::prelude::rust::sync::Arc;

use crate::transaction_types::*;
use crate::utils::*;

#[derive(Clone, Debug)]
pub enum WorktopAction {
    Put { resource_specifier: ResourceSpecifier, 
        instruction_index: usize},
    Take {
        resource_specifier: ResourceSpecifier, 
        instruction_index: usize
    }
}

//pub type WorktopActionSubscriber = fn(action: WorktopAction);

pub trait WorktopActionSubscriber {
    fn action_called(&mut self, action: WorktopAction);
}

#[derive(Default)]
enum WorktopTrustStatus {
    #[default]
    Uninitialized,
    Trusted,
    NotTrusted
}

pub struct WorktopActionPublisher<'a, T: WorktopActionSubscriber> {

    subscribers: Vec<&'a mut T>,
    instruction_idx: usize,
    instuction_valid: WorktopTrustStatus,
}

impl<'a, T: WorktopActionSubscriber> WorktopActionPublisher<'a, T> {

    pub fn new() -> Self {
        Self { subscribers: Vec::new(), instruction_idx: 0, instuction_valid: WorktopTrustStatus::default() }
    }

    pub fn register_subscriber(&mut self, subscriber: &'a mut T) {
        self.subscribers.push(subscriber);
    }

    fn notify_subscribers_take(&mut self, resource_specifier: ResourceSpecifier, instruction_index: usize) {
        for subscriber in &mut self.subscribers {
            subscriber.action_called(WorktopAction::Take{resource_specifier: resource_specifier.clone(), instruction_index})
        }
    }

    fn notify_subscribers_put(&mut self, resource_specifier: ResourceSpecifier, instruction_index: usize) {
        for subscriber in &mut self.subscribers {
            subscriber.action_called(WorktopAction::Put{resource_specifier: resource_specifier.clone(), instruction_index})
        }
    }

}

impl<'a, T: WorktopActionSubscriber> ManifestSummaryCallback for WorktopActionPublisher<'a, T> {
    fn on_finish(&mut self, instructions_count: usize) {
    }

    fn on_instruction(&mut self, instruction: &InstructionV1, instruction_index: usize) {
        //println!( "  ***-> ins ({}, {}) {:?}", self.instruction_idx, instruction_index, instruction);
        match instruction {
            //InstructionV1::TakeAllFromWorktop { resource_address }
            InstructionV1::TakeFromWorktop { resource_address, amount } => 
                self.notify_subscribers_take( ResourceSpecifier::Amount(*resource_address, *amount), instruction_index ),
            InstructionV1::TakeNonFungiblesFromWorktop { resource_address, ids } => {
                let mut list: IndexSet<NonFungibleLocalId> = IndexSet::new();
                for item in ids {
                    list.insert(item.clone());
                }
                self.notify_subscribers_take( ResourceSpecifier::Ids(*resource_address, list), instruction_index);
            }
            InstructionV1::ReturnToWorktop { bucket_id } => {
                
            }

            _ => ()
        };
    }
}


impl<'a, T: WorktopActionSubscriber> ExecutionSummaryCallback for WorktopActionPublisher<'a, T> {

    fn on_instruction(
        &mut self,
        instruction: &InstructionV1,
        instruction_index: usize,
        _input_resources: &[ResourceSpecifier],
        _output_resources: &[ResourceSpecifier],
    ) {
        self.instruction_idx = instruction_index;

        // match instruction {
        //     InstructionV1::TakeFromWorktop { .. } => self.instuction_valid = true,
        //     _ => ()
        // }

        //println!(" ** Instruction_called ({}) ({}): {:?}", self.instruction_idx, self.instruction_valid, instruction );
    }

    fn on_account_withdraw(
        &mut self,
        _account: &ComponentAddress,
        _resource_indicator: &ResourceIndicator,
    ) {
        // self.notify_subscribers_put( ResourceSpecifier::Amount(*resource_address, *amount), instruction_index );

        println!(" ** withdraw ({}): {:?}", self.instruction_idx, _resource_indicator );
    }

    fn on_account_deposit(
        &mut self,
        _account: &ComponentAddress,
        _resource_indicator: &ResourceIndicator,
    ) {
        // self.notify_subscribers_take( ResourceSpecifier::Amount(*resource_address, *amount), instruction_index );

        println!(" ** deposit ({}): {:?}", self.instruction_idx, _resource_indicator );
    }
}

