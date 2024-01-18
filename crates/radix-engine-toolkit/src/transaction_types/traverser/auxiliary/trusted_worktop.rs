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


#[derive(Default)]
pub struct TrustedWorktop {
    trusted_state_per_instruction: Vec<bool>
}

impl TrustedWorktop {
    pub fn get_results(&self) -> Vec<bool> {
        self.trusted_state_per_instruction.to_owned()
    }

    pub fn is_worktop_trusted(&self, instruction_index: usize) -> Option<bool> {
        self.trusted_state_per_instruction.get(instruction_index).map(|value| *value)
    }
}

impl WorktopContentTrackerObserver for TrustedWorktop {
    fn on_instruction(
        &mut self,
        instruction: &InstructionV1,
        instruction_index: usize,
        worktop_content: &Vec<WorktopContent>,
    ) {
        self.trusted_state_per_instruction.push(false);
    }
}


