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
use scrypto::prelude::*;

#[derive(Default)]
pub struct AccountResourceMovementsDetector {
    /// Account withdraws
    withdraws: IndexMap<ComponentAddress, Vec<ResourceIndicator>>,
    /// Account deposits
    deposits: IndexMap<ComponentAddress, Vec<ResourceIndicator>>,
}

impl AccountResourceMovementsDetector {
    pub fn output(
        self,
    ) -> (
        IndexMap<ComponentAddress, Vec<ResourceIndicator>>,
        IndexMap<ComponentAddress, Vec<ResourceIndicator>>,
    ) {
        (self.withdraws, self.deposits)
    }
}

impl ManifestSummaryCallback for AccountResourceMovementsDetector {}

impl ExecutionSummaryCallback for AccountResourceMovementsDetector {
    fn on_account_deposit(
        &mut self,
        account: &ComponentAddress,
        resource_indicator: &ResourceIndicator,
    ) {
        self.deposits
            .entry(*account)
            .or_default()
            .push(resource_indicator.clone());
    }

    fn on_account_withdraw(
        &mut self,
        account: &ComponentAddress,
        resource_indicator: &ResourceIndicator,
    ) {
        self.withdraws
            .entry(*account)
            .or_default()
            .push(resource_indicator.clone());
    }
}
