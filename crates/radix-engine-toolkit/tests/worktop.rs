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

use radix_engine_toolkit::transaction_types::ResourceSpecifierExt;
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
    let (manifest_summary, _) = test_runner.summarize(manifest);

    // Assert
    assert_eq!(manifest_summary.trusted_worktop_instructions.len(), 4);
    assert!(manifest_summary.trusted_worktop_instructions[0].trusted);
    assert!(manifest_summary.trusted_worktop_instructions[0]
        .resources
        .is_none());
    assert_eq!(
        manifest_summary.trusted_worktop_instructions[1]
            .resources
            .as_ref()
            .unwrap()
            .resource_address(),
        address
    );
    assert_eq!(
        *manifest_summary.trusted_worktop_instructions[1]
            .resources
            .as_ref()
            .unwrap()
            .amount()
            .unwrap(),
        dec!(10)
    );
    assert!(manifest_summary.trusted_worktop_instructions[1].trusted);
    assert!(manifest_summary.trusted_worktop_instructions[2]
        .resources
        .is_none());
    assert!(manifest_summary.trusted_worktop_instructions[2].trusted);
    assert!(manifest_summary.trusted_worktop_instructions[3]
        .resources
        .is_none());
    assert!(!manifest_summary.trusted_worktop_instructions[3].trusted);
}

#[test]
fn worktop_simple2() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().without_trace().build();
    let (_, _, account) = test_runner.new_allocated_account();
    let address = test_runner
        .create_freely_mintable_and_burnable_fungible_resource(
            OwnerRole::None,
            Some(dec!(100)),
            0,
            account,
        );

    //Act
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .withdraw_from_account(account, address, 10)
        .burn_all_from_worktop(address)
        .build();
    let (manifest_summary, _) = test_runner.summarize(manifest);

    // Assert
    assert_eq!(manifest_summary.trusted_worktop_instructions.len(), 4);
    assert!(manifest_summary.trusted_worktop_instructions[0]
        .resources
        .is_none());
    assert!(manifest_summary.trusted_worktop_instructions[0].trusted);
    assert_eq!(
        manifest_summary.trusted_worktop_instructions[1]
            .resources
            .as_ref()
            .unwrap()
            .resource_address(),
        address
    );
    assert_eq!(
        *manifest_summary.trusted_worktop_instructions[1]
            .resources
            .as_ref()
            .unwrap()
            .amount()
            .unwrap(),
        dec!(10)
    );
    assert!(manifest_summary.trusted_worktop_instructions[1].trusted);
    assert!(manifest_summary.trusted_worktop_instructions[2]
        .resources
        .is_none()); // automatically inserted instruction TakeAllFromWorktop
    assert!(manifest_summary.trusted_worktop_instructions[2].trusted);
    assert!(manifest_summary.trusted_worktop_instructions[3]
        .resources
        .is_none());
    assert!(manifest_summary.trusted_worktop_instructions[3].trusted);
}

#[test]
fn worktop_simple3() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().without_trace().build();
    let (_, _, account) = test_runner.new_allocated_account();
    let address = test_runner
        .create_freely_mintable_and_burnable_fungible_resource(
            OwnerRole::None,
            Some(dec!(100)),
            0,
            account,
        );

    //Act
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .withdraw_from_account(account, address, 10)
        .take_from_worktop(address, 6, "bucket_1")
        .return_to_worktop("bucket_1")
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();
    let (manifest_summary, _) = test_runner.summarize(manifest);

    // Assert
    assert_eq!(manifest_summary.trusted_worktop_instructions.len(), 5);
    assert!(manifest_summary.trusted_worktop_instructions[0]
        .resources
        .is_none());
    assert!(manifest_summary.trusted_worktop_instructions[0].trusted);

    assert_eq!(
        manifest_summary.trusted_worktop_instructions[1]
            .resources
            .as_ref()
            .unwrap()
            .resource_address(),
        address
    );
    assert_eq!(
        *manifest_summary.trusted_worktop_instructions[1]
            .resources
            .as_ref()
            .unwrap()
            .amount()
            .unwrap(),
        dec!(10)
    );
    assert!(manifest_summary.trusted_worktop_instructions[1].trusted);
    // assert_eq!(
    //     manifest_summary.trusted_worktop_instructions[2]
    //         .resources.as_ref()
    //         .unwrap()
    //         .resource_address(),
    //     address
    // );
    // assert_eq!(
    //     *manifest_summary.trusted_worktop_instructions[2]
    //         .resources.as_ref()
    //         .unwrap()
    //         .amount()
    //         .unwrap(),
    //     dec!(4)
    // );
    // assert!(manifest_summary.trusted_worktop_instructions[2].trusted);
    // assert_eq!(
    //     manifest_summary.trusted_worktop_instructions[3]
    //         .resources.as_ref()
    //         .unwrap()
    //         .resource_address(),
    //     address
    // );
    // assert_eq!(
    //     *manifest_summary.trusted_worktop_instructions[3]
    //         .resources.as_ref()
    //         .unwrap()
    //         .amount()
    //         .unwrap(),
    //     dec!(10)
    // );
    // assert!(!manifest_summary.trusted_worktop_instructions[3].trusted);
    // assert!(manifest_summary.trusted_worktop_instructions[4].resources
    //     .is_none());
    // assert!(manifest_summary.trusted_worktop_instructions[4].trusted);
}
