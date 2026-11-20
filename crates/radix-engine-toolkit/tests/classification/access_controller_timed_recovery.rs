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

use super::access_controller_utils::*;
use crate::prelude::*;

const TIMED_RECOVERY: Option<ManifestClassification> =
    Some(ManifestClassification::AccessControllerTimedRecovery);

fn access_controller() -> ComponentAddress {
    TestAddressAllocator::new().new_access_controller_address()
}

#[test]
fn proposing_as_recovery_shows_the_proposed_rules() {
    for locks_primary_role in [false, true] {
        // Arrange
        let (mut ledger, account, access_controller) =
            ledger_with_access_controller();
        let mut builder = ManifestBuilder::new()
            .lock_fee(account, 10)
            .initiate_recovery(
                access_controller,
                Proposer::Recovery,
                proposal(new_rules(), Some(5)),
            );
        if locks_primary_role {
            builder = builder.lock_primary_role(access_controller);
        }

        // Act
        let class =
            detailed_access_controller_class(&mut ledger, builder.build());

        // Assert
        assert_eq!(
            class,
            DetailedManifestClassification::AccessControllerTimedRecovery(
                AccessControllerTimedRecoveryOutput {
                    timed_recoveries: vec![AccessControllerTimedRecovery {
                        access_controller,
                        rule_set: new_rules(),
                        timed_recovery_delay_in_minutes: Some(5),
                        locks_primary_role,
                    }],
                }
            )
        );
    }
}

#[test]
fn locking_the_primary_role_before_proposing_is_timed_recovery() {
    let access_controller = access_controller();
    let manifest = ManifestBuilder::new()
        .lock_primary_role(access_controller)
        .initiate_recovery(
            access_controller,
            Proposer::Recovery,
            proposal(allow_all(), None),
        )
        .build();
    assert_access_controller_class(manifest, TIMED_RECOVERY);
}

#[test]
fn starting_timed_recoveries_of_several_access_controllers_is_timed_recovery() {
    let mut allocator = TestAddressAllocator::new();
    let mut builder = ManifestBuilder::new();
    for _ in 0..2 {
        builder = builder.initiate_recovery(
            allocator.new_access_controller_address(),
            Proposer::Recovery,
            proposal(allow_all(), None),
        );
    }
    assert_access_controller_class(builder.build(), TIMED_RECOVERY);
}

/// A primary role proposal has no timed recovery; it waits for confirmation.
#[test]
fn proposing_as_primary_without_confirming_is_not_classified() {
    let manifest = ManifestBuilder::new()
        .initiate_recovery(
            access_controller(),
            Proposer::Primary,
            proposal(allow_all(), None),
        )
        .build();
    assert_access_controller_class(manifest, None);
}

#[test]
fn proposing_twice_on_the_same_access_controller_is_not_timed_recovery() {
    let access_controller = access_controller();
    for first in PROPOSERS {
        let manifest = ManifestBuilder::new()
            .initiate_recovery(
                access_controller,
                first,
                proposal(deny_all(), None),
            )
            .initiate_recovery(
                access_controller,
                Proposer::Recovery,
                proposal(allow_all(), None),
            )
            .build();
        assert_access_controller_class(manifest, None);
    }
}

#[test]
fn other_access_controller_calls_are_not_timed_recovery() {
    let access_controller = access_controller();
    let others: [fn(ManifestBuilder, ComponentAddress) -> ManifestBuilder; 6] = [
        |builder, access_controller| {
            builder.timed_confirm_recovery(
                access_controller,
                proposal(allow_all(), None),
            )
        },
        |builder, access_controller| {
            builder.stop_timed_recovery(
                access_controller,
                proposal(allow_all(), None),
            )
        },
        |builder, access_controller| {
            builder.cancel_recovery(access_controller, Proposer::Recovery)
        },
        |builder, access_controller| {
            builder.unlock_primary_role(access_controller)
        },
        |builder, access_controller| {
            builder
                .lock_primary_role(access_controller)
                .lock_primary_role(access_controller)
        },
        |builder, access_controller| {
            builder.quick_confirm_recovery(
                access_controller,
                Proposer::Primary,
                proposal(allow_all(), None),
            )
        },
    ];
    for other in others {
        for other_first in [true, false] {
            let mut builder = ManifestBuilder::new();
            if other_first {
                builder = other(builder, access_controller);
            }
            builder = builder.initiate_recovery(
                access_controller,
                Proposer::Recovery,
                proposal(allow_all(), None),
            );
            if !other_first {
                builder = other(builder, access_controller);
            }
            assert_access_controller_class(builder.build(), None);
        }
    }
}

/// Owner keys are set once the recovery is confirmed.
#[test]
fn setting_account_metadata_is_not_timed_recovery() {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .set_metadata(account, "owner_keys", "not important")
        .initiate_recovery(
            allocator.new_access_controller_address(),
            Proposer::Recovery,
            proposal(allow_all(), None),
        )
        .build();
    assert_access_controller_class(manifest, None);
}
