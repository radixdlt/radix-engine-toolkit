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
use transaction::prelude::*;

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
        .take_from_worktop(address, 10, "bucket_1")
        .try_deposit_or_abort(account2, None, "bucket_1")
        .build();
    let (_, execution_summary) = test_runner.summarize(manifest);

    // Assert
    assert_eq!(execution_summary.trusted_worktop_content.len(), 4);
    assert!(execution_summary.trusted_worktop_content[0].0.content.get(&address).is_none());
    assert_eq!(execution_summary.trusted_worktop_content[1].0.content.get(&address).unwrap().amount().unwrap(), dec!(10));
    assert!(execution_summary.trusted_worktop_content[2].0.content.get(&address).is_none());
    assert!(execution_summary.trusted_worktop_content[3].0.content.get(&address).is_none());

    println!("worktop content:");
    for (i, val) in execution_summary.trusted_worktop_content.iter().enumerate() {
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
        .burn_all_from_worktop(address)
        .build();
    let (_, execution_summary) = test_runner.summarize(manifest);

    // Assert
    assert_eq!(execution_summary.trusted_worktop_content.len(), 4);
    assert!(execution_summary.trusted_worktop_content[0].0.content.get(&address).is_none());
    assert_eq!(execution_summary.trusted_worktop_content[1].0.content.get(&address).unwrap().amount().unwrap(), dec!(10));
    assert!(execution_summary.trusted_worktop_content[2].0.content.get(&address).is_none()); // automatically inserted instructino TakeAllFromWorktop
    assert!(execution_summary.trusted_worktop_content[3].0.content.get(&address).is_none());

    println!("\nworktop content:");
    for (i, val) in execution_summary.trusted_worktop_content.iter().enumerate() {
        println!("instruction {}: {:?}", i, val);
    }
}

#[test]
fn worktop_simple3() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().without_trace().build();
    let (_, _, account) = test_runner.new_allocated_account();
    let address = test_runner.create_freely_mintable_and_burnable_fungible_resource(OwnerRole::None, Some(dec!(100)), 0, account);

    //Act
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .withdraw_from_account(account, address, 10)
        .take_from_worktop(address, 6, "bucket_1")
        .return_to_worktop("bucket_1")
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();
    let (_, execution_summary) = test_runner.summarize(manifest);

    // Assert
    assert_eq!(execution_summary.trusted_worktop_content.len(), 5);
    assert!(execution_summary.trusted_worktop_content[0].0.content.get(&address).is_none());
    assert_eq!(execution_summary.trusted_worktop_content[1].0.content.get(&address).unwrap().amount().unwrap(), dec!(10));
    assert_eq!(execution_summary.trusted_worktop_content[2].0.content.get(&address).unwrap().amount().unwrap(), dec!(4));
    assert_eq!(execution_summary.trusted_worktop_content[3].0.content.get(&address).unwrap().amount().unwrap(), dec!(10));
    assert!(execution_summary.trusted_worktop_content[0].0.content.get(&address).is_none());
    println!("\nworktop content:");
    for (i, val) in execution_summary.trusted_worktop_content.iter().enumerate() {
        println!("instruction {}: {:?}", i, val);
    }
}
