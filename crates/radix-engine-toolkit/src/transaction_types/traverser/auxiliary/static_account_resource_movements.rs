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

use crate::statics::*;
use crate::transaction_types::*;
use crate::utils::*;
use scrypto::prelude::*;
use radix_transactions::prelude::*;

#[derive(Default)]
pub struct StaticAccountResourceMovementsDetector {
    /// Account withdraws
    withdraws: IndexSet<ComponentAddress>,
    /// Account deposits
    deposits: IndexSet<ComponentAddress>,
}

impl StaticAccountResourceMovementsDetector {
    pub fn output(
        self,
    ) -> (IndexSet<ComponentAddress>, IndexSet<ComponentAddress>) {
        (self.withdraws, self.deposits)
    }
}

impl ManifestSummaryCallback for StaticAccountResourceMovementsDetector {
    fn on_instruction(&mut self, instruction: &InstructionV1, _: usize) {
        if let InstructionV1::CallMethod {
            address: dynamic_address @ DynamicGlobalAddress::Static(address),
            method_name,
            ..
        } = instruction
        {
            if !is_account(dynamic_address) {
                return;
            }
            let account =
                ComponentAddress::try_from(*address).expect("Must succeed!");

            if ACCOUNT_DEPOSIT_METHODS.contains(method_name) {
                self.deposits.insert(account);
            } else if ACCOUNT_WITHDRAW_METHODS.contains(method_name) {
                self.withdraws.insert(account);
            }
        }
    }
}

impl ExecutionSummaryCallback for StaticAccountResourceMovementsDetector {}
