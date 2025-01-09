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

use crate::internal_prelude::*;

/// A callback for information that can be obtained statically from the manifest
/// without the need for the receipt or execution trace.
pub trait StaticAnalysisCallback {
    /// Called when the traverser starts going through a new instruction with
    /// the new instruction and the index that it is at.
    #[inline]
    fn on_instruction(
        &mut self,
        _instruction: &InstructionV2,
        _instruction_index: usize,
    ) {
    }

    /// Called when a proof is created out of an account.
    #[inline]
    fn on_create_proof(
        &mut self,
        _account: &ComponentAddress,
        _resource: &ResourceSpecifier,
    ) {
    }

    /// Called when a global entity is encountered in the manifest
    #[inline]
    fn on_global_entity_encounter(&mut self, _address: GlobalAddress) {}

    /// Called when the instructions in the manifest have finished.
    #[inline]
    fn on_finish(&mut self, _instructions_count: usize) {}
}

pub trait DynamicAnalysisCallback
where
    Self: StaticAnalysisCallback,
{
    /// Called when the traverser starts going through a new instruction with
    /// the new instruction and the index that it is at and information on the
    /// input resources that this instruction took and the output resources.
    #[inline]
    fn on_instruction(
        &mut self,
        _instruction: &InstructionV2,
        _instruction_index: usize,
        _input_resources: &[ResourceSpecifier],
        _output_resources: &[ResourceSpecifier],
    ) {
    }

    /// Called when resources are withdrawn from an account with the account and
    /// withdraw information.
    #[inline]
    fn on_account_withdraw(
        &mut self,
        _account: &ComponentAddress,
        _resource_indicator: &ResourceIndicator,
    ) {
    }

    /// Called when a deposit is performed into an account with the information
    /// of the deposited resources.
    #[inline]
    fn on_account_deposit(
        &mut self,
        _account: &ComponentAddress,
        _resource_indicator: &ResourceIndicator,
    ) {
    }
}
