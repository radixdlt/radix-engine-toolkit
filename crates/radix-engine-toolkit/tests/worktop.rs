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
use radix_engine::system::system_modules::execution_trace::ResourceSpecifier;
use radix_engine_toolkit::transaction_types::ManifestSummary;
use test_runner_extension::TestRunnerEDExt;

// helper function
fn validate(
    manifest_summary: &ManifestSummary,
    instruction: usize,
    trusted: bool,
    resources: Option<ResourceSpecifier>,
) {
    assert_eq!(
        manifest_summary.trusted_worktop_instructions[instruction].trusted,
        trusted,
        "Instruction: {}",
        instruction
    );
    if resources.is_none() {
        assert!(
            manifest_summary.trusted_worktop_instructions[instruction]
                .resources
                .is_empty(),
            "Instruction: {}",
            instruction
        );
    } else {
        match resources.unwrap() {
            ResourceSpecifier::Amount(address, amount) => {
                assert_eq!(
                    manifest_summary.trusted_worktop_instructions[instruction]
                        .resources
                        .first()
                        .unwrap()
                        .resource_address(),
                    address,
                    "Instruction: {}",
                    instruction
                );
                assert_eq!(
                    *manifest_summary.trusted_worktop_instructions[instruction]
                        .resources
                        .first()
                        .unwrap()
                        .amount()
                        .unwrap(),
                    amount,
                    "Instruction: {}",
                    instruction
                );
            }
            ResourceSpecifier::Ids(address, _ids) => {
                assert_eq!(
                    manifest_summary.trusted_worktop_instructions[instruction]
                        .resources
                        .first()
                        .unwrap()
                        .resource_address(),
                    address,
                    "Instruction: {}",
                    instruction
                );
            }
        }
    }
}

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
    validate(&manifest_summary, 0, true, None);
    validate(
        &manifest_summary,
        1,
        true,
        Some(ResourceSpecifier::Amount(address, dec!(10))),
    );
    validate(
        &manifest_summary,
        2,
        true,
        Some(ResourceSpecifier::Amount(address, dec!(10))),
    );
    validate(
        &manifest_summary,
        3,
        true,
        Some(ResourceSpecifier::Amount(address, dec!(10))),
    );
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
    validate(&manifest_summary, 0, true, None);
    validate(
        &manifest_summary,
        1,
        true,
        Some(ResourceSpecifier::Amount(address, dec!(10))),
    );
    validate(
        &manifest_summary,
        2,
        true,
        Some(ResourceSpecifier::Amount(address, dec!(10))),
    ); // automatically inserted instruction TakeAllFromWorktop
    validate(&manifest_summary, 3, true, None);
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
    validate(&manifest_summary, 0, true, None);
    validate(
        &manifest_summary,
        1,
        true,
        Some(ResourceSpecifier::Amount(address, dec!(10))),
    );
    validate(
        &manifest_summary,
        2,
        true,
        Some(ResourceSpecifier::Amount(address, dec!(6))),
    );
    validate(
        &manifest_summary,
        3,
        true,
        Some(ResourceSpecifier::Amount(address, dec!(6))),
    );
    validate(&manifest_summary, 4, false, None);
}
