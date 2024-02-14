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

use radix_engine_interface::blueprints::pool::*;
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
        "Instruction: {} (wrong trusted state)",
        instruction
    );
    if resources.is_none() {
        assert!(
            manifest_summary.trusted_worktop_instructions[instruction]
                .resources
                .is_empty(),
            "Instruction: {} (resoruce not specified)",
            instruction
        );
    } else {
        match resources.unwrap() {
            ResourceSpecifier::Amount(address, amount) => {
                assert!(
                    manifest_summary.trusted_worktop_instructions[instruction]
                        .resources
                        .iter()
                        .find(|item| item.resource_address() == address)
                        .is_some(),
                    "Instruction: {}, resource address not found: {:?}",
                    instruction,
                    address
                );
                let mut val = dec!(0);
                assert!(
                    manifest_summary.trusted_worktop_instructions[instruction]
                        .resources
                        .iter()
                        .find(|item| {
                            val = *item.amount().unwrap();
                            item.resource_address() == address
                                && *item.amount().unwrap() == amount
                        })
                        .is_some(),
                    "Instruction: {}, amount not equal, is: {} required: {}",
                    instruction,
                    amount,
                    val
                );
            }
            ResourceSpecifier::Ids(address, ids) => {
                assert!(
                    manifest_summary.trusted_worktop_instructions[instruction]
                        .resources
                        .iter()
                        .find(|item| item.resource_address() == address)
                        .is_some(),
                    "Instruction: {}, resource address not found: {:?}",
                    instruction,
                    address
                );
                assert!(
                    manifest_summary.trusted_worktop_instructions[instruction]
                        .resources
                        .iter()
                        .find(|item| item.resource_address() == address
                            && ids.difference(item.ids().unwrap()).count() == 0)
                        .is_some(),
                    "Instruction: {} (ids not equal)",
                    instruction
                );
            }
        }
    }
}

// helper function
fn validate_amount(
    manifest_summary: &ManifestSummary,
    instruction: usize,
    trusted: bool,
    resource: &[(ResourceAddress, Decimal)],
) {
    resource.iter().for_each(|(resource_address, amount)| {
        validate(
            manifest_summary,
            instruction,
            trusted,
            Some(ResourceSpecifier::Amount(*resource_address, *amount)),
        )
    });
}

#[test]
fn trusted_worktop_deposit_from_bucket() {
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
    validate_amount(&manifest_summary, 1, true, &[(address, dec!(10))]);
    validate_amount(&manifest_summary, 2, true, &[(address, dec!(10))]);
    validate_amount(&manifest_summary, 3, true, &[(address, dec!(10))]);
}

#[test]
fn trusted_worktop_deposit_empty_bucket() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().without_trace().build();
    let (_, _, account) = test_runner.new_allocated_account();
    let address = test_runner.create_fungible_resource(dec!(100), 0, account);

    //Act
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .take_all_from_worktop(address, "empty_bucket")
        .try_deposit_or_abort(account, None, "empty_bucket")
        .build();
    let (manifest_summary, _) = test_runner.summarize(manifest);

    // Assert
    assert_eq!(manifest_summary.trusted_worktop_instructions.len(), 3);
    validate(&manifest_summary, 0, true, None);
    validate(&manifest_summary, 1, true, None);
    validate(&manifest_summary, 2, true, None);
}

#[test]
fn trusted_worktop_take_zero() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().without_trace().build();
    let (_, _, account) = test_runner.new_allocated_account();
    let address = test_runner.create_fungible_resource(dec!(100), 0, account);

    //Act
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .take_from_worktop(address, 0, "empty_bucket")
        .try_deposit_or_abort(account, None, "empty_bucket")
        .build();
    let (manifest_summary, _) = test_runner.summarize(manifest);

    // Assert
    assert_eq!(manifest_summary.trusted_worktop_instructions.len(), 3);
    validate(&manifest_summary, 0, true, None);
    validate(&manifest_summary, 1, true, None);
    validate(&manifest_summary, 2, true, None);
}

#[test]
fn trusted_worktop_burn_all() {
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
    validate_amount(&manifest_summary, 1, true, &[(address, dec!(10))]);
    validate_amount(&manifest_summary, 2, true, &[(address, dec!(10))]); // inserted instruction TakeAllFromWorktop by test framework
    validate_amount(&manifest_summary, 3, true, &[(address, dec!(10))]);
}

#[test]
fn trusted_worktop_burn_empty() {
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
        .burn_all_from_worktop(address)
        .build();
    let (manifest_summary, _) = test_runner.summarize(manifest);

    // Assert
    assert_eq!(manifest_summary.trusted_worktop_instructions.len(), 3);
    validate(&manifest_summary, 0, true, None);
    validate(&manifest_summary, 1, true, None); // inserted instruction TakeAllFromWorktop by test framework
    validate(&manifest_summary, 2, true, None);
}

#[test]
fn trusted_worktop_deposit_entire_worktop() {
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
    validate_amount(&manifest_summary, 1, true, &[(address, dec!(10))]);
    validate_amount(&manifest_summary, 2, true, &[(address, dec!(6))]);
    validate_amount(&manifest_summary, 3, true, &[(address, dec!(6))]);
    validate_amount(&manifest_summary, 4, true, &[(address, dec!(10))]);
}

#[test]
fn trusted_worktop_deposit_account_and_deposit_entire_worktop() {
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
        .deposit(account, "bucket_1")
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();
    let (manifest_summary, _) = test_runner.summarize(manifest);

    // Assert
    assert_eq!(manifest_summary.trusted_worktop_instructions.len(), 5);
    validate(&manifest_summary, 0, true, None);
    validate_amount(&manifest_summary, 1, true, &[(address, dec!(10))]);
    validate_amount(&manifest_summary, 2, true, &[(address, dec!(6))]);
    validate_amount(&manifest_summary, 3, true, &[(address, dec!(6))]);
    validate_amount(&manifest_summary, 4, true, &[(address, dec!(4))]);
}

#[test]
fn trusted_worktop_deposit_batch_and_deposit_entire_worktop() {
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
        .get_free_xrd_from_faucet()
        .take_from_worktop(XRD, dec!(1000), "bucket_1")
        .take_from_worktop(XRD, dec!(2000), "bucket_2")
        .withdraw_from_account(account, address, 10)
        .take_from_worktop(address, 6, "bucket_3")
        .try_deposit_batch_or_abort(
            account,
            ["bucket_1", "bucket_2", "bucket_3"],
            None,
        )
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();
    let (ms, _) = test_runner.summarize(manifest);

    // Assert
    assert_eq!(ms.trusted_worktop_instructions.len(), 7);
    validate_amount(&ms, 0, true, &[(XRD, dec!(10000))]);
    validate_amount(&ms, 1, true, &[(XRD, dec!(1000))]);
    validate_amount(&ms, 2, true, &[(XRD, dec!(2000))]);
    validate_amount(&ms, 3, true, &[(address, dec!(10))]);
    validate_amount(&ms, 4, true, &[(address, dec!(6))]);
    validate_amount(&ms, 5, true, &[(XRD, dec!(3000)), (address, dec!(6))]);
    validate_amount(&ms, 6, true, &[(XRD, dec!(7000)), (address, dec!(4))]);
}

#[test]
fn trusted_worktop_two_withdraws() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().without_trace().build();
    let (_, _, account) = test_runner.new_allocated_account();
    let (_, _, account2) = test_runner.new_allocated_account();
    let address = test_runner.create_fungible_resource(dec!(100), 0, account);

    //Act
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .withdraw_from_account(account, address, 10)
        .withdraw_from_account(account, address, 20)
        .take_from_worktop(address, 30, "bucket_1")
        .try_deposit_or_abort(account2, None, "bucket_1")
        .build();
    let (manifest_summary, _) = test_runner.summarize(manifest);

    // Assert
    assert_eq!(manifest_summary.trusted_worktop_instructions.len(), 5);
    validate(&manifest_summary, 0, true, None);
    validate_amount(&manifest_summary, 1, true, &[(address, dec!(10))]);
    validate_amount(&manifest_summary, 2, true, &[(address, dec!(20))]);
    validate_amount(&manifest_summary, 3, true, &[(address, dec!(30))]);
    validate_amount(&manifest_summary, 4, true, &[(address, dec!(30))]);
}

#[test]
fn trusted_worktop_mint_fungible() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().without_trace().build();
    let (_, _, account) = test_runner.new_allocated_account();
    let address = test_runner.create_freely_mintable_fungible_resource(
        OwnerRole::None,
        Some(dec!(100)),
        0,
        account,
    );

    //Act
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .mint_fungible(address, 10)
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();
    let (manifest_summary, _) = test_runner.summarize(manifest);

    // Assert
    assert_eq!(manifest_summary.trusted_worktop_instructions.len(), 3);
    validate(&manifest_summary, 0, true, None);
    validate_amount(&manifest_summary, 1, true, &[(address, dec!(10))]);
    validate_amount(&manifest_summary, 2, true, &[(address, dec!(10))]);
}

#[test]
fn trusted_worktop_mint_fungible_two_resources() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().without_trace().build();
    let (_, _, account) = test_runner.new_allocated_account();
    let addr_1 = test_runner.create_freely_mintable_fungible_resource(
        OwnerRole::None,
        Some(dec!(100)),
        0,
        account,
    );
    let addr_2 = test_runner.create_freely_mintable_fungible_resource(
        OwnerRole::None,
        Some(dec!(100)),
        0,
        account,
    );

    //Act
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .mint_fungible(addr_1, 10)
        .mint_fungible(addr_2, 5)
        .mint_fungible(addr_1, 20)
        .mint_fungible(addr_2, 7)
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();
    let (ms, _) = test_runner.summarize(manifest);

    // Assert
    assert_eq!(ms.trusted_worktop_instructions.len(), 6);
    validate(&ms, 0, true, None);
    validate_amount(&ms, 1, true, &[(addr_1, dec!(10))]);
    validate_amount(&ms, 2, true, &[(addr_2, dec!(5))]);
    validate_amount(&ms, 3, true, &[(addr_1, dec!(20))]);
    validate_amount(&ms, 4, true, &[(addr_2, dec!(7))]);
    validate_amount(&ms, 5, true, &[(addr_1, dec!(30)), (addr_2, dec!(12))]);
}

#[test]
fn trusted_worktop_mint_fungible_two_resources_and_deposits() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().without_trace().build();
    let (_, _, account) = test_runner.new_allocated_account();
    let addr_1 = test_runner.create_freely_mintable_fungible_resource(
        OwnerRole::None,
        Some(dec!(100)),
        0,
        account,
    );
    let addr_2 = test_runner.create_freely_mintable_fungible_resource(
        OwnerRole::None,
        Some(dec!(100)),
        0,
        account,
    );

    //Act
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .mint_fungible(addr_1, 10)
        .mint_fungible(addr_2, 5)
        .mint_fungible(addr_1, 20)
        .mint_fungible(addr_2, 7)
        .take_from_worktop(addr_1, 25, "bucket_1")
        .try_deposit_or_abort(account, None, "bucket_1")
        .take_from_worktop(addr_2, 1, "bucket_2")
        .try_deposit_or_abort(account, None, "bucket_2")
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();
    let (ms, _) = test_runner.summarize(manifest);

    // Assert
    assert_eq!(ms.trusted_worktop_instructions.len(), 10);
    validate(&ms, 0, true, None);
    validate_amount(&ms, 1, true, &[(addr_1, dec!(10))]);
    validate_amount(&ms, 2, true, &[(addr_2, dec!(5))]);
    validate_amount(&ms, 3, true, &[(addr_1, dec!(20))]);
    validate_amount(&ms, 4, true, &[(addr_2, dec!(7))]);
    validate_amount(&ms, 5, true, &[(addr_1, dec!(25))]);
    validate_amount(&ms, 6, true, &[(addr_1, dec!(25))]);
    validate_amount(&ms, 7, true, &[(addr_2, dec!(1))]);
    validate_amount(&ms, 8, true, &[(addr_2, dec!(1))]);
    validate_amount(&ms, 9, true, &[(addr_1, dec!(5)), (addr_2, dec!(11))]);
}

#[test]
fn trusted_worktop_one_resource_pool() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().without_trace().build();
    let (_, _, account) = test_runner.new_allocated_account();
    let address = test_runner.create_fungible_resource(dec!(100), 0, account);
    let (component_address, _resource_address) =
        test_runner.create_one_resource_pool(address, AccessRule::AllowAll);

    //Act
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .withdraw_from_account(account, address, 100)
        .take_from_worktop(address, 100, "bucket")
        .call_method_with_name_lookup(
            component_address,
            ONE_RESOURCE_POOL_CONTRIBUTE_IDENT,
            |lookup| OneResourcePoolContributeManifestInput {
                bucket: lookup.bucket("bucket"),
            },
        )
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();
    let (manifest_summary, _) = test_runner.summarize(manifest);

    // Assert
    assert_eq!(manifest_summary.trusted_worktop_instructions.len(), 5);
    validate(&manifest_summary, 0, true, None);
    validate_amount(&manifest_summary, 1, true, &[(address, dec!(100))]);
    validate_amount(&manifest_summary, 2, true, &[(address, dec!(100))]);
    validate_amount(&manifest_summary, 3, true, &[(address, dec!(100))]);
    // Untrusted as we don't know what is returned by resource pool.
    validate(&manifest_summary, 4, false, None);
}

#[test]
fn trusted_worktop_one_resource_pool_redeem() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().without_trace().build();
    let (_, _, account) = test_runner.new_allocated_account();
    let address = test_runner.create_fungible_resource(dec!(100), 0, account);
    let (component_address, resource_address) =
        test_runner.create_one_resource_pool(address, AccessRule::AllowAll);

    //Act
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .withdraw_from_account(account, address, 100)
        .take_from_worktop(address, 50, "bucket")
        .take_from_worktop(address, 20, "bucket_20")
        .take_from_worktop(address, 30, "bucket_30")
        .call_method_with_name_lookup(
            component_address,
            ONE_RESOURCE_POOL_CONTRIBUTE_IDENT,
            |lookup| OneResourcePoolContributeManifestInput {
                bucket: lookup.bucket("bucket"),
            },
        )
        .take_all_from_worktop(resource_address, "pool_units_bucket")
        .call_method_with_name_lookup(
            component_address,
            ONE_RESOURCE_POOL_REDEEM_IDENT,
            |lookup| OneResourcePoolRedeemManifestInput {
                bucket: lookup.bucket("pool_units_bucket"),
            },
        )
        .take_all_from_worktop(address, "returned_res_bucket")
        .deposit(account, "bucket_20")
        .deposit(account, "returned_res_bucket")
        .deposit(account, "bucket_30")
        .build();
    let (manifest_summary, _) = test_runner.summarize(manifest);

    // Assert
    assert_eq!(manifest_summary.trusted_worktop_instructions.len(), 12);
    validate(&manifest_summary, 0, true, None);
    validate_amount(&manifest_summary, 1, true, &[(address, dec!(100))]);
    validate_amount(&manifest_summary, 2, true, &[(address, dec!(50))]);
    validate_amount(&manifest_summary, 3, true, &[(address, dec!(20))]);
    validate_amount(&manifest_summary, 4, true, &[(address, dec!(30))]);
    validate_amount(&manifest_summary, 5, true, &[(address, dec!(50))]);
    // Untrasted as we don't know what is returned by resource pool.
    validate(&manifest_summary, 6, false, None);
    validate(&manifest_summary, 7, false, None);
    validate(&manifest_summary, 8, false, None);
    validate_amount(&manifest_summary, 9, true, &[(address, dec!(20))]);
    validate(&manifest_summary, 10, false, None);
    validate_amount(&manifest_summary, 11, true, &[(address, dec!(30))]);
}

#[test]
fn trusted_worktop_one_resource_protected_withdraw() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().without_trace().build();
    let (_, _, account) = test_runner.new_allocated_account();
    let address = test_runner.create_fungible_resource(dec!(100), 0, account);
    let (component_address, resource_address) =
        test_runner.create_one_resource_pool(address, AccessRule::AllowAll);

    //Act
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .withdraw_from_account(account, address, 50)
        .take_from_worktop(address, 50, "bucket")
        .call_method_with_name_lookup(
            component_address,
            ONE_RESOURCE_POOL_CONTRIBUTE_IDENT,
            |lookup| OneResourcePoolContributeManifestInput {
                bucket: lookup.bucket("bucket"),
            },
        )
        .take_all_from_worktop(resource_address, "pool_units_bucket")
        .deposit(account, "pool_units_bucket")
        .call_method(
            component_address,
            ONE_RESOURCE_POOL_PROTECTED_WITHDRAW_IDENT,
            OneResourcePoolProtectedWithdrawManifestInput {
                amount: dec!(10),
                withdraw_strategy: WithdrawStrategy::Exact,
            },
        )
        .take_all_from_worktop(address, "returned_res_bucket")
        .deposit(account, "returned_res_bucket")
        .build();
    let (manifest_summary, _) = test_runner.summarize(manifest);

    // Assert
    assert_eq!(manifest_summary.trusted_worktop_instructions.len(), 9);
    validate(&manifest_summary, 0, true, None);
    validate_amount(&manifest_summary, 1, true, &[(address, dec!(50))]);
    validate_amount(&manifest_summary, 2, true, &[(address, dec!(50))]);
    validate_amount(&manifest_summary, 3, true, &[(address, dec!(50))]);
    // Untrusted as we don't know what is returned by resource pool.
    validate(&manifest_summary, 4, false, None);
    validate(&manifest_summary, 5, false, None);
    // we don't know the pool resource type
    validate(&manifest_summary, 6, false, None);
    validate(&manifest_summary, 7, false, None);
    validate(&manifest_summary, 8, false, None);
}

#[test]
fn trusted_worktop_one_resource_protected_deposit() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().without_trace().build();
    let (_, _, account) = test_runner.new_allocated_account();
    let address = test_runner.create_fungible_resource(dec!(100), 0, account);
    let (component_address, _resource_address) =
        test_runner.create_one_resource_pool(address, AccessRule::AllowAll);

    //Act
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .withdraw_from_account(account, address, 50)
        .take_from_worktop(address, 50, "bucket")
        .call_method_with_name_lookup(
            component_address,
            ONE_RESOURCE_POOL_PROTECTED_DEPOSIT_IDENT,
            |lookup| OneResourcePoolProtectedDepositManifestInput {
                bucket: lookup.bucket("bucket"),
            },
        )
        .build();
    let (manifest_summary, _) = test_runner.summarize(manifest);

    // Assert
    assert_eq!(manifest_summary.trusted_worktop_instructions.len(), 4);
    validate(&manifest_summary, 0, true, None);
    validate_amount(&manifest_summary, 1, true, &[(address, dec!(50))]);
    validate_amount(&manifest_summary, 2, true, &[(address, dec!(50))]);
    validate_amount(&manifest_summary, 3, true, &[(address, dec!(50))]);
}
