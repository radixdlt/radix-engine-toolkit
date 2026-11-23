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

//! Besides its access controller calls, a manifest of an access controller
//! class may only pay the fee: anything else it does wouldn't be in the
//! class's output. The recovery fee class, which also moves XRD, is tested in
//! `access_controller_recovery_fee`.

use super::access_controller_utils::*;
use crate::prelude::*;

type Calls = fn(ManifestBuilder, ComponentAddress) -> ManifestBuilder;

/// A manifest of each access controller class, by the calls it makes on an
/// access controller.
fn classes() -> [(ManifestClassification, Calls); 5] {
    [
        (
            ManifestClassification::AccessControllerRecovery,
            |builder, access_controller| {
                builder
                    .initiate_recovery(
                        access_controller,
                        Proposer::Recovery,
                        proposal(allow_all(), None),
                    )
                    .quick_confirm_recovery(
                        access_controller,
                        Proposer::Recovery,
                        proposal(allow_all(), None),
                    )
            },
        ),
        (
            ManifestClassification::AccessControllerTimedRecovery,
            |builder, access_controller| {
                builder.initiate_recovery(
                    access_controller,
                    Proposer::Recovery,
                    proposal(allow_all(), None),
                )
            },
        ),
        (
            ManifestClassification::AccessControllerConfirmRecovery,
            |builder, access_controller| {
                builder.timed_confirm_recovery(
                    access_controller,
                    proposal(allow_all(), None),
                )
            },
        ),
        (
            ManifestClassification::AccessControllerCancelRecovery,
            |builder, access_controller| {
                builder.cancel_recovery(access_controller, Proposer::Primary)
            },
        ),
        (
            ManifestClassification::AccessControllerPrimaryRoleLock,
            |builder, access_controller| {
                builder.lock_primary_role(access_controller)
            },
        ),
    ]
}

type Effect =
    fn(ManifestBuilder, ComponentAddress, ComponentAddress) -> ManifestBuilder;

/// What a manifest may do besides its access controller calls, by the account
/// and access controller involved.
fn fee_payments() -> [Effect; 5] {
    [
        |builder, account, _| builder.lock_fee(account, 10),
        |builder, account, _| builder.lock_contingent_fee(account, 10),
        |builder, account, _| {
            builder.create_proof_from_account_of_amount(account, XRD, 1)
        },
        |builder, _, access_controller| {
            builder.call_method(
                access_controller,
                ACCESS_CONTROLLER_CREATE_PROOF_IDENT,
                (),
            )
        },
        |builder, _, access_controller| {
            builder.call_method(
                access_controller,
                ACCESS_CONTROLLER_LOCK_RECOVERY_FEE_IDENT,
                AccessControllerLockRecoveryFeeManifestInput {
                    amount: Decimal::one(),
                },
            )
        },
    ]
}

/// What a manifest may not do besides its access controller calls.
fn other_effects() -> [Effect; 5] {
    [
        |builder, account, access_controller| {
            top_up_recovery_fee(builder, account, access_controller)
        },
        |builder, account, _| {
            builder.set_metadata(account, "owner_keys", "not important")
        },
        |builder, account, _| {
            builder.set_metadata(account, "name", "not important")
        },
        |builder, account, _| {
            builder
                .withdraw_from_account(account, XRD, 100)
                .try_deposit_entire_worktop_or_abort(account, None)
        },
        |builder, _, _| builder.set_metadata(XRD, "name", "not important"),
    ]
}

fn addresses() -> (ComponentAddress, ComponentAddress) {
    let mut allocator = TestAddressAllocator::new();
    (
        allocator.new_account_address(),
        allocator.new_access_controller_address(),
    )
}

#[test]
fn paying_the_fee_is_permitted_in_every_class() {
    let (account, access_controller) = addresses();
    for (class, calls) in classes() {
        for pay_fee in fee_payments() {
            let builder =
                pay_fee(ManifestBuilder::new(), account, access_controller);
            let manifest = calls(builder, access_controller).build();
            assert_access_controller_class(manifest, Some(class));
        }
    }
}

#[test]
fn doing_anything_else_is_not_of_any_class() {
    let (account, access_controller) = addresses();
    for (_, calls) in classes() {
        for effect in other_effects() {
            for effect_first in [true, false] {
                let mut builder = ManifestBuilder::new();
                if effect_first {
                    builder = effect(builder, account, access_controller);
                }
                builder = calls(builder, access_controller);
                if !effect_first {
                    builder = effect(builder, account, access_controller);
                }
                assert_access_controller_class(builder.build(), None);
            }
        }
    }
}
