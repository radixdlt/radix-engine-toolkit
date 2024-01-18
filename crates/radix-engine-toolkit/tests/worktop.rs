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

use scrypto_unit::*;
use transaction::prelude::node_modules::ModuleConfig;
use transaction::prelude::*;
use radix_engine_toolkit::transaction_types::WorktopContentItem;

mod test_runner_extension;
use test_runner_extension::TestRunnerEDExt;

#[test]
fn worktop_simple() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().without_trace().build();
    let (_, _, account) = test_runner.new_allocated_account();
    let (_, _, account2) = test_runner.new_allocated_account();

    let address = test_runner.create_fungible_resource(dec!(100), 0, account);

    //Act

    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .withdraw_from_account(account, address, 10)
        .take_from_worktop(address, 10, "b1")
        //.burn_all_from_worktop(*address)
        //.try_deposit_entire_worktop_or_abort(account2, None)
        .try_deposit_or_abort(account2, None, "b1")
        //        .try_deposit_entire_worktop_or_abort(account, None)
        .build();
    let (_, execution_summary) = test_runner.summarize(manifest);

    // Assert
    println!("worktop content:");
    for (i, val) in execution_summary.worktop_content.iter().enumerate() {
        println!("instruction {}: {:?}", i, val);
    }
}

#[test]
fn worktop_simple2() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().without_trace().build();
    let (_, _, account) = test_runner.new_allocated_account();
    let address = test_runner.create_freely_mintable_and_burnable_fungible_resource(OwnerRole::None, Some(dec!(100)), 0, account);

    //Act

    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .withdraw_from_account(account, address, 10)
        //.take_from_worktop(address, 10, "b1")
        .burn_all_from_worktop(address)
        //.try_deposit_entire_worktop_or_abort(account, None)
        .build();
    let (_, execution_summary) = test_runner.summarize(manifest);

    // Assert
    println!("worktop content:");
    for (i, val) in execution_summary.worktop_content.iter().enumerate() {
        println!("instruction {}: {:?}", i, val);
    }
}
