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
use scrypto_test::prelude::*;
mod test_runner_extension;
use radix_engine::system::system_modules::execution_trace::ResourceSpecifier;
use radix_engine_toolkit::transaction_types::TrustedWorktopInstruction;
use test_runner_extension::LedgerSimulatorEDExt;

// helper function
fn validate(
    trusted_worktop_instructions: &Vec<TrustedWorktopInstruction>,
    instruction: usize,
    trusted: bool,
    resources: Option<ResourceSpecifier>,
) {
    assert_eq!(
        trusted_worktop_instructions[instruction].is_trusted, trusted,
        "Instruction: {} (wrong trusted state)",
        instruction
    );
    if resources.is_none() {
        assert!(
            trusted_worktop_instructions[instruction]
                .resources
                .is_empty(),
            "Instruction: {} (resoruce not specified)",
            instruction
        );
    } else {
        match resources.unwrap() {
            ResourceSpecifier::Amount(address, amount) => {
                assert!(
                    trusted_worktop_instructions[instruction]
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
                    trusted_worktop_instructions[instruction]
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
                    val,
                    amount
                );
            }
            ResourceSpecifier::Ids(address, ids) => {
                assert!(
                    trusted_worktop_instructions[instruction]
                        .resources
                        .iter()
                        .find(|item| item.resource_address() == address)
                        .is_some(),
                    "Instruction: {}, resource address not found: {:?}",
                    instruction,
                    address
                );
                assert!(
                    trusted_worktop_instructions[instruction]
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

// helper functions
fn validate_amount(
    trusted_worktop_instructions: &Vec<TrustedWorktopInstruction>,
    instruction: usize,
    trusted: bool,
    resource: &[(ResourceAddress, Decimal)],
) {
    resource.iter().for_each(|(resource_address, amount)| {
        validate(
            trusted_worktop_instructions,
            instruction,
            trusted,
            Some(ResourceSpecifier::Amount(*resource_address, *amount)),
        )
    });
}
fn validate_ids(
    trusted_worktop_instructions: &Vec<TrustedWorktopInstruction>,
    instruction: usize,
    trusted: bool,
    resource: &[(ResourceAddress, IndexSet<NonFungibleLocalId>)],
) {
    resource.iter().for_each(|(resource_address, ids)| {
        validate(
            trusted_worktop_instructions,
            instruction,
            trusted,
            Some(ResourceSpecifier::Ids(*resource_address, ids.clone())),
        )
    });
}

#[test]
fn trusted_worktop_deposit_fungible_from_bucket() {
    // Arrange
    let mut test_runner =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
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
    let twi = test_runner.validate_and_get_trusted_worktop(&manifest);

    // Assert
    assert_eq!(twi.len(), 4);
    validate(&twi, 0, true, None);
    validate_amount(&twi, 1, true, &[(address, dec!(10))]);
    validate_amount(&twi, 2, true, &[(address, dec!(10))]);
    validate_amount(&twi, 3, true, &[(address, dec!(10))]);
}

#[test]
fn trusted_worktop_deposit_non_fungible_from_bucket() {
    // Arrange
    let mut test_runner =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = test_runner.new_allocated_account();
    let (_, _, account2) = test_runner.new_allocated_account();
    let address = test_runner.create_non_fungible_resource(account);

    let id1 = NonFungibleLocalId::integer(1);
    let id2 = NonFungibleLocalId::integer(2);

    //Act
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .withdraw_non_fungibles_from_account(
            account,
            address,
            [id1.clone(), id2.clone()],
        )
        .take_non_fungibles_from_worktop(address, [id1.clone()], "bucket_1")
        .try_deposit_or_abort(account2, None, "bucket_1")
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();
    let twi = test_runner.validate_and_get_trusted_worktop(&manifest);

    // Assert
    assert_eq!(twi.len(), 5);
    validate(&twi, 0, true, None);
    validate_ids(
        &twi,
        1,
        true,
        &[(address, indexset![id1.clone(), id2.clone()])],
    );
    validate_ids(&twi, 2, true, &[(address, indexset![id1.clone()])]);
    validate_ids(&twi, 3, true, &[(address, indexset![id1])]);
    validate_ids(&twi, 4, true, &[(address, indexset![id2])]);
}

#[test]
fn trusted_worktop_deposit_empty_bucket() {
    // Arrange
    let mut test_runner =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = test_runner.new_allocated_account();
    let address = test_runner.create_fungible_resource(dec!(100), 0, account);

    //Act
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .take_all_from_worktop(address, "empty_bucket")
        .try_deposit_or_abort(account, None, "empty_bucket")
        .build();
    let twi = test_runner.validate_and_get_trusted_worktop(&manifest);

    // Assert
    assert_eq!(twi.len(), 3);
    validate(&twi, 0, true, None);
    validate_amount(&twi, 1, true, &[(address, dec!(0))]);
    validate_amount(&twi, 2, true, &[(address, dec!(0))]);
}

#[test]
fn trusted_worktop_take_empty() {
    // Arrange
    let mut test_runner =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = test_runner.new_allocated_account();
    let address_fungible =
        test_runner.create_fungible_resource(dec!(100), 0, account);
    let address_non_fungible =
        test_runner.create_non_fungible_resource(account);

    //Act
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .take_all_from_worktop(address_fungible, "empty_bucket")
        .take_all_from_worktop(address_non_fungible, "empty_bucket2")
        .try_deposit_or_abort(account, None, "empty_bucket")
        .try_deposit_or_abort(account, None, "empty_bucket2")
        .build();
    let twi = test_runner.validate_and_get_trusted_worktop(&manifest);

    // Assert
    assert_eq!(twi.len(), 5);
    validate(&twi, 0, true, None);
    validate_amount(&twi, 1, true, &[(address_fungible, dec!(0))]);
    validate_ids(&twi, 2, true, &[(address_non_fungible, indexset! {})]);
    validate_amount(&twi, 3, true, &[(address_fungible, dec!(0))]);
    validate_ids(&twi, 4, true, &[(address_non_fungible, indexset! {})]);
}

#[test]
fn trusted_worktop_take_fungible_zero() {
    // Arrange
    let mut test_runner =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = test_runner.new_allocated_account();
    let address = test_runner.create_fungible_resource(dec!(100), 0, account);

    //Act
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .take_from_worktop(address, 0, "empty_bucket")
        .try_deposit_or_abort(account, None, "empty_bucket")
        .build();
    let twi = test_runner.validate_and_get_trusted_worktop(&manifest);

    // Assert
    assert_eq!(twi.len(), 3);
    validate(&twi, 0, true, None);
    validate_amount(&twi, 1, true, &[(address, dec!(0))]);
    validate_amount(&twi, 2, true, &[(address, dec!(0))]);
}

#[test]
fn trusted_worktop_take_nonfungible_empty() {
    // Arrange
    let mut test_runner =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = test_runner.new_allocated_account();
    let address = test_runner.create_non_fungible_resource(account);

    //Act
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .take_from_worktop(address, 0, "empty_bucket")
        .take_non_fungibles_from_worktop(address, [], "empty_bucket2")
        .try_deposit_or_abort(account, None, "empty_bucket")
        .try_deposit_or_abort(account, None, "empty_bucket2")
        .build();
    let twi = test_runner.validate_and_get_trusted_worktop(&manifest);

    // Assert
    assert_eq!(twi.len(), 5);
    validate(&twi, 0, true, None);
    validate_ids(&twi, 1, true, &[(address, indexset! {})]);
    validate_ids(&twi, 2, true, &[(address, indexset! {})]);
    validate_ids(&twi, 3, true, &[(address, indexset! {})]);
    validate_ids(&twi, 4, true, &[(address, indexset! {})]);
}

#[test]
fn trusted_worktop_burn_all() {
    // Arrange
    let mut test_runner =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
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
    let twi = test_runner.validate_and_get_trusted_worktop(&manifest);

    // Assert
    assert_eq!(twi.len(), 4);
    validate(&twi, 0, true, None);
    validate_amount(&twi, 1, true, &[(address, dec!(10))]);
    validate_amount(&twi, 2, true, &[(address, dec!(10))]); // inserted instruction TakeAllFromWorktop by test framework
    validate_amount(&twi, 3, true, &[(address, dec!(10))]);
}

#[test]
fn trusted_worktop_burn_empty() {
    // Arrange
    let mut test_runner =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
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
    let twi = test_runner.validate_and_get_trusted_worktop(&manifest);

    // Assert
    assert_eq!(twi.len(), 3);
    validate(&twi, 0, true, None);
    validate_amount(&twi, 1, true, &[(address, dec!(0))]); // inserted instruction TakeAllFromWorktop by test framework
    validate_amount(&twi, 1, true, &[(address, dec!(0))]);
}

#[test]
fn trusted_worktop_deposit_entire_worktop() {
    // Arrange
    let mut test_runner =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
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
    let twi = test_runner.validate_and_get_trusted_worktop(&manifest);

    // Assert
    assert_eq!(twi.len(), 5);
    validate(&twi, 0, true, None);
    validate_amount(&twi, 1, true, &[(address, dec!(10))]);
    validate_amount(&twi, 2, true, &[(address, dec!(6))]);
    validate_amount(&twi, 3, true, &[(address, dec!(6))]);
    validate_amount(&twi, 4, true, &[(address, dec!(10))]);
}

#[test]
fn trusted_worktop_deposit_account_and_deposit_entire_worktop() {
    // Arrange
    let mut test_runner =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
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
    let twi = test_runner.validate_and_get_trusted_worktop(&manifest);

    // Assert
    assert_eq!(twi.len(), 5);
    validate(&twi, 0, true, None);
    validate_amount(&twi, 1, true, &[(address, dec!(10))]);
    validate_amount(&twi, 2, true, &[(address, dec!(6))]);
    validate_amount(&twi, 3, true, &[(address, dec!(6))]);
    validate_amount(&twi, 4, true, &[(address, dec!(4))]);
}

#[test]
fn trusted_worktop_deposit_batch_and_deposit_entire_worktop() {
    // Arrange
    let mut test_runner =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
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
    let twi = test_runner.validate_and_get_trusted_worktop(&manifest);

    // Assert
    assert_eq!(twi.len(), 7);
    validate_amount(&twi, 0, true, &[(XRD, dec!(10000))]);
    validate_amount(&twi, 1, true, &[(XRD, dec!(1000))]);
    validate_amount(&twi, 2, true, &[(XRD, dec!(2000))]);
    validate_amount(&twi, 3, true, &[(address, dec!(10))]);
    validate_amount(&twi, 4, true, &[(address, dec!(6))]);
    validate_amount(&twi, 5, true, &[(XRD, dec!(3000)), (address, dec!(6))]);
    validate_amount(&twi, 6, true, &[(XRD, dec!(7000)), (address, dec!(4))]);
}

#[test]
fn trusted_worktop_two_withdraws() {
    // Arrange
    let mut test_runner =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
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
    let twi = test_runner.validate_and_get_trusted_worktop(&manifest);

    // Assert
    assert_eq!(twi.len(), 5);
    validate(&twi, 0, true, None);
    validate_amount(&twi, 1, true, &[(address, dec!(10))]);
    validate_amount(&twi, 2, true, &[(address, dec!(20))]);
    validate_amount(&twi, 3, true, &[(address, dec!(30))]);
    validate_amount(&twi, 4, true, &[(address, dec!(30))]);
}

#[test]
fn trusted_worktop_mint_fungible() {
    // Arrange
    let mut test_runner =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
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
    let twi = test_runner.validate_and_get_trusted_worktop(&manifest);

    // Assert
    assert_eq!(twi.len(), 3);
    validate(&twi, 0, true, None);
    validate_amount(&twi, 1, true, &[(address, dec!(10))]);
    validate_amount(&twi, 2, true, &[(address, dec!(10))]);
}

#[test]
fn trusted_worktop_mint_fungible_two_resources() {
    // Arrange
    let mut test_runner =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
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
    let twi = test_runner.validate_and_get_trusted_worktop(&manifest);

    // Assert
    assert_eq!(twi.len(), 6);
    validate(&twi, 0, true, None);
    validate_amount(&twi, 1, true, &[(addr_1, dec!(10))]);
    validate_amount(&twi, 2, true, &[(addr_2, dec!(5))]);
    validate_amount(&twi, 3, true, &[(addr_1, dec!(20))]);
    validate_amount(&twi, 4, true, &[(addr_2, dec!(7))]);
    validate_amount(&twi, 5, true, &[(addr_1, dec!(30)), (addr_2, dec!(12))]);
}

#[test]
fn trusted_worktop_mint_fungible_two_resources_and_deposits() {
    // Arrange
    let mut test_runner =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
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
    let twi = test_runner.validate_and_get_trusted_worktop(&manifest);

    // Assert
    assert_eq!(twi.len(), 10);
    validate(&twi, 0, true, None);
    validate_amount(&twi, 1, true, &[(addr_1, dec!(10))]);
    validate_amount(&twi, 2, true, &[(addr_2, dec!(5))]);
    validate_amount(&twi, 3, true, &[(addr_1, dec!(20))]);
    validate_amount(&twi, 4, true, &[(addr_2, dec!(7))]);
    validate_amount(&twi, 5, true, &[(addr_1, dec!(25))]);
    validate_amount(&twi, 6, true, &[(addr_1, dec!(25))]);
    validate_amount(&twi, 7, true, &[(addr_2, dec!(1))]);
    validate_amount(&twi, 8, true, &[(addr_2, dec!(1))]);
    validate_amount(&twi, 9, true, &[(addr_1, dec!(5)), (addr_2, dec!(11))]);
}

#[test]
fn trusted_worktop_one_resource_pool() {
    // Arrange
    let mut test_runner =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
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
    let twi = test_runner.validate_and_get_trusted_worktop(&manifest);

    // Assert
    assert_eq!(twi.len(), 5);
    validate(&twi, 0, true, None);
    validate_amount(&twi, 1, true, &[(address, dec!(100))]);
    validate_amount(&twi, 2, true, &[(address, dec!(100))]);
    validate_amount(&twi, 3, true, &[(address, dec!(100))]);
    // Untrusted as we don't know what is returned by resource pool.
    validate(&twi, 4, false, None);
}

#[test]
fn trusted_worktop_one_resource_pool_redeem() {
    // Arrange
    let mut test_runner =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
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
    let twi = test_runner.validate_and_get_trusted_worktop(&manifest);

    // Assert
    assert_eq!(twi.len(), 12);
    validate(&twi, 0, true, None);
    validate_amount(&twi, 1, true, &[(address, dec!(100))]);
    validate_amount(&twi, 2, true, &[(address, dec!(50))]);
    validate_amount(&twi, 3, true, &[(address, dec!(20))]);
    validate_amount(&twi, 4, true, &[(address, dec!(30))]);
    validate_amount(&twi, 5, true, &[(address, dec!(50))]);
    // Untrasted as we don't know what is returned by resource pool.
    validate(&twi, 6, false, None);
    validate(&twi, 7, false, None);
    validate(&twi, 8, false, None);
    validate_amount(&twi, 9, true, &[(address, dec!(20))]);
    validate(&twi, 10, false, None);
    validate_amount(&twi, 11, true, &[(address, dec!(30))]);
}

#[test]
fn trusted_worktop_one_resource_protected_withdraw() {
    // Arrange
    let mut test_runner =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
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
    let twi = test_runner.validate_and_get_trusted_worktop(&manifest);

    // Assert
    assert_eq!(twi.len(), 9);
    validate(&twi, 0, true, None);
    validate_amount(&twi, 1, true, &[(address, dec!(50))]);
    validate_amount(&twi, 2, true, &[(address, dec!(50))]);
    validate_amount(&twi, 3, true, &[(address, dec!(50))]);
    // Untrusted as we don't know what is returned by resource pool.
    validate(&twi, 4, false, None);
    validate(&twi, 5, false, None);
    // we don't know the pool resource type
    validate(&twi, 6, false, None);
    validate(&twi, 7, false, None);
    validate(&twi, 8, false, None);
}

#[test]
fn trusted_worktop_one_resource_protected_deposit() {
    // Arrange
    let mut test_runner =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
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
    let twi = test_runner.validate_and_get_trusted_worktop(&manifest);

    // Assert
    assert_eq!(twi.len(), 4);
    validate(&twi, 0, true, None);
    validate_amount(&twi, 1, true, &[(address, dec!(50))]);
    validate_amount(&twi, 2, true, &[(address, dec!(50))]);
    validate_amount(&twi, 3, true, &[(address, dec!(50))]);
}

#[test]
fn trusted_worktop_create_proof_fungible() {
    // Arrange
    let mut test_runner =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = test_runner.new_allocated_account();

    //Act
    let manifest = ManifestBuilder::new()
        .get_free_xrd_from_faucet()
        .take_all_from_worktop(XRD, "bucket")
        .create_proof_from_bucket_of_amount("bucket", 10, "proof")
        .drop_all_proofs()
        .try_deposit_or_abort(account, None, "bucket")
        .build();
    let twi = test_runner.validate_and_get_trusted_worktop(&manifest);

    // Assert
    assert_eq!(twi.len(), 5);
    validate_amount(&twi, 0, true, &[(XRD, faucet::FAUCET_FREE_AMOUNT.into())]);
    validate_amount(&twi, 1, true, &[(XRD, faucet::FAUCET_FREE_AMOUNT.into())]);
    validate_amount(&twi, 2, true, &[(XRD, dec!(10))]);
    validate(&twi, 3, true, None);
    validate_amount(&twi, 4, true, &[(XRD, faucet::FAUCET_FREE_AMOUNT.into())]);
}

#[test]
fn trusted_worktop_create_proof_non_fungible() {
    // Arrange
    let mut test_runner =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = test_runner.new_account(true);
    let address = test_runner.create_non_fungible_resource(account);

    let id1 = NonFungibleLocalId::integer(1);
    let id2 = NonFungibleLocalId::integer(2);

    //Act
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .withdraw_non_fungibles_from_account(
            account,
            address,
            [id1.clone(), id2.clone()],
        )
        .take_all_from_worktop(address, "bucket")
        .create_proof_from_bucket_of_non_fungibles(
            "bucket",
            [id1.clone()],
            "proof",
        )
        .drop_all_proofs()
        .try_deposit_or_abort(account, None, "bucket")
        .build();
    let twi = test_runner.validate_and_get_trusted_worktop(&manifest);

    // Assert
    assert_eq!(twi.len(), 6);
    validate(&twi, 0, true, None);
    validate_ids(
        &twi,
        1,
        true,
        &[(address, indexset![id1.clone(), id2.clone()])],
    );
    validate_ids(
        &twi,
        2,
        true,
        &[(address, indexset![id1.clone(), id2.clone()])],
    );
    validate_ids(&twi, 3, true, &[(address, indexset![id1.clone()])]);
    validate(&twi, 4, true, None);
    validate_ids(
        &twi,
        5,
        true,
        &[(address, indexset![id1.clone(), id2.clone()])],
    );
}
