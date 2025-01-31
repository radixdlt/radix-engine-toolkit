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

use super::*;
use scrypto_test::prelude::*;

pub fn account_securify_manifest() -> (TransactionManifestV1, ComponentAddress)
{
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .call_method(
            account,
            ACCOUNT_SECURIFY_IDENT,
            AccountSecurifyManifestInput {},
        )
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();
    (manifest, account)
}

pub fn account_lock_fee_manifest() -> (TransactionManifestV1, ComponentAddress)
{
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new().lock_fee(account, 1).build();
    (manifest, account)
}

pub fn account_lock_contingent_fee_manifest(
) -> (TransactionManifestV1, ComponentAddress) {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .lock_contingent_fee(account, 1)
        .build();
    (manifest, account)
}

pub fn account_deposit_manifest() -> (TransactionManifestV1, ComponentAddress) {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .mint_fungible(XRD, 1)
        .take_all_from_worktop(XRD, "bucket")
        .deposit(account, "bucket")
        .build();
    (manifest, account)
}

pub fn account_deposit_batch_manifest(
) -> (TransactionManifestV1, ComponentAddress) {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .mint_fungible(XRD, 1)
        .deposit_batch(account, ManifestExpression::EntireWorktop)
        .build();
    (manifest, account)
}

pub fn account_try_deposit_or_abort_manifest(
) -> (TransactionManifestV1, ComponentAddress) {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .mint_fungible(XRD, 1)
        .take_all_from_worktop(XRD, "bucket")
        .try_deposit_or_abort(account, None, "bucket")
        .build();
    (manifest, account)
}

pub fn account_try_deposit_batch_or_abort_manifest(
) -> (TransactionManifestV1, ComponentAddress) {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .mint_fungible(XRD, 1)
        .try_deposit_batch_or_abort(
            account,
            ManifestExpression::EntireWorktop,
            None,
        )
        .build();
    (manifest, account)
}

pub fn account_try_deposit_or_refund_manifest(
) -> (TransactionManifestV1, ComponentAddress) {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .mint_fungible(XRD, 1)
        .take_all_from_worktop(XRD, "bucket")
        .try_deposit_or_refund(account, None, "bucket")
        .build();
    (manifest, account)
}

pub fn account_try_deposit_batch_or_refund_manifest(
) -> (TransactionManifestV1, ComponentAddress) {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .mint_fungible(XRD, 1)
        .try_deposit_batch_or_refund(
            account,
            ManifestExpression::EntireWorktop,
            None,
        )
        .build();
    (manifest, account)
}

pub fn account_withdraw_manifest() -> (TransactionManifestV1, ComponentAddress)
{
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account, XRD, 1)
        .deposit_batch(account, ManifestExpression::EntireWorktop)
        .build();
    (manifest, account)
}

pub fn account_withdraw_non_fungibles_manifest(
    resource_address: ResourceAddress,
) -> (TransactionManifestV1, ComponentAddress) {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .mint_non_fungible(
            resource_address,
            [(NonFungibleLocalId::integer(1), ())],
        )
        .deposit_batch(account, ManifestExpression::EntireWorktop)
        .withdraw_non_fungible_from_account(
            account,
            NonFungibleGlobalId::new(
                resource_address,
                NonFungibleLocalId::integer(1),
            ),
        )
        .deposit_batch(account, ManifestExpression::EntireWorktop)
        .build();
    (manifest, account)
}

pub fn account_lock_fee_and_withdraw_manifest(
) -> (TransactionManifestV1, ComponentAddress) {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .lock_fee_and_withdraw(account, 1, XRD, 1)
        .deposit_batch(account, ManifestExpression::EntireWorktop)
        .build();
    (manifest, account)
}

pub fn account_lock_fee_and_withdraw_non_fungibles_manifest(
    resource_address: ResourceAddress,
) -> (TransactionManifestV1, ComponentAddress) {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .mint_non_fungible(
            resource_address,
            [(NonFungibleLocalId::integer(1), ())],
        )
        .deposit_batch(account, ManifestExpression::EntireWorktop)
        .lock_fee_and_withdraw_non_fungibles(
            account,
            1,
            resource_address,
            [NonFungibleLocalId::integer(1)],
        )
        .deposit_batch(account, ManifestExpression::EntireWorktop)
        .build();
    (manifest, account)
}

pub fn account_burn_manifest() -> (TransactionManifestV1, ComponentAddress) {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .burn_in_account(account, XRD, 1)
        .build();
    (manifest, account)
}

pub fn account_burn_non_fungible_non_fungibles_manifest(
    resource_address: ResourceAddress,
) -> (TransactionManifestV1, ComponentAddress) {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .mint_non_fungible(
            resource_address,
            [(NonFungibleLocalId::integer(1), ())],
        )
        .deposit_batch(account, ManifestExpression::EntireWorktop)
        .burn_non_fungible_in_account(
            account,
            NonFungibleGlobalId::new(
                resource_address,
                NonFungibleLocalId::integer(1),
            ),
        )
        .build();
    (manifest, account)
}

pub fn account_create_proof_of_manifest(
) -> (TransactionManifestV1, ComponentAddress) {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_amount(account, XRD, 1)
        .deposit_batch(account, ManifestExpression::EntireWorktop)
        .build();
    (manifest, account)
}

pub fn account_create_proof_of_non_fungibles_manifest(
    resource_address: ResourceAddress,
) -> (TransactionManifestV1, ComponentAddress) {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .mint_non_fungible(
            resource_address,
            [(NonFungibleLocalId::integer(1), ())],
        )
        .deposit_batch(account, ManifestExpression::EntireWorktop)
        .create_proof_from_account_of_non_fungible(
            account,
            NonFungibleGlobalId::new(
                resource_address,
                NonFungibleLocalId::integer(1),
            ),
        )
        .deposit_batch(account, ManifestExpression::EntireWorktop)
        .build();
    (manifest, account)
}

pub fn account_set_default_deposit_rule_manifest(
) -> (TransactionManifestV1, ComponentAddress) {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .call_method(
            account,
            ACCOUNT_SET_DEFAULT_DEPOSIT_RULE_IDENT,
            AccountSetDefaultDepositRuleManifestInput {
                default: DefaultDepositRule::Accept,
            },
        )
        .build();
    (manifest, account)
}

pub fn account_set_resource_preference_manifest(
) -> (TransactionManifestV1, ComponentAddress) {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .call_method(
            account,
            ACCOUNT_SET_RESOURCE_PREFERENCE_IDENT,
            AccountSetResourcePreferenceManifestInput {
                resource_address: XRD.into(),
                resource_preference: ResourcePreference::Allowed,
            },
        )
        .build();
    (manifest, account)
}

pub fn account_remove_resource_preference_manifest(
) -> (TransactionManifestV1, ComponentAddress) {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .call_method(
            account,
            ACCOUNT_REMOVE_RESOURCE_PREFERENCE_IDENT,
            AccountRemoveResourcePreferenceManifestInput {
                resource_address: XRD.into(),
            },
        )
        .build();
    (manifest, account)
}

pub fn account_add_authorized_depositor_preference_manifest(
) -> (TransactionManifestV1, ComponentAddress) {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .call_method(
            account,
            ACCOUNT_ADD_AUTHORIZED_DEPOSITOR_IDENT,
            AccountAddAuthorizedDepositorManifestInput {
                badge: ManifestResourceOrNonFungible::Resource(XRD.into()),
            },
        )
        .build();
    (manifest, account)
}

pub fn account_remove_authorized_depositor_preference_manifest(
) -> (TransactionManifestV1, ComponentAddress) {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .call_method(
            account,
            ACCOUNT_REMOVE_AUTHORIZED_DEPOSITOR_IDENT,
            AccountRemoveAuthorizedDepositorManifestInput {
                badge: ManifestResourceOrNonFungible::Resource(XRD.into()),
            },
        )
        .build();
    (manifest, account)
}
