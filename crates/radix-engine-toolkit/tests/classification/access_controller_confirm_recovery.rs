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

const CONFIRM_RECOVERY: Option<ManifestClassification> =
    Some(ManifestClassification::AccessControllerConfirmRecovery);

fn access_controller() -> ComponentAddress {
    TestAddressAllocator::new().new_access_controller_address()
}

#[test]
fn quick_confirming_shows_the_rules_enacted() {
    for proposer in PROPOSERS {
        // Arrange
        let (mut ledger, account, access_controller) =
            ledger_with_access_controller();
        execute(&mut ledger, |builder| {
            builder.initiate_recovery(
                access_controller,
                proposer,
                proposal(new_rules(), Some(5)),
            )
        });
        let manifest = ManifestBuilder::new()
            .lock_fee(account, 10)
            .quick_confirm_recovery(
                access_controller,
                proposer,
                proposal(new_rules(), Some(5)),
            )
            .build();

        // Act
        let class = detailed_access_controller_class(&mut ledger, manifest);

        // Assert
        assert_eq!(
            class,
            DetailedManifestClassification::AccessControllerConfirmRecovery(
                AccessControllerConfirmRecoveryOutput {
                    confirmations: vec![AccessControllerRecoveryConfirmation {
                        access_controller,
                        proposer,
                        kind: RecoveryConfirmationKind::Quick,
                        rule_set: new_rules(),
                        timed_recovery_delay_in_minutes: Some(5),
                    }],
                }
            )
        );
    }
}

#[test]
fn timed_confirming_shows_the_rules_enacted() {
    // Arrange
    let (mut ledger, account, access_controller) =
        ledger_with_access_controller();
    execute(&mut ledger, |builder| {
        builder.initiate_recovery(
            access_controller,
            Proposer::Recovery,
            proposal(new_rules(), Some(5)),
        )
    });
    // The access controller's timed recovery delay is 10 minutes.
    advance_time(&mut ledger, 10);
    let manifest = ManifestBuilder::new()
        .lock_fee(account, 10)
        .timed_confirm_recovery(
            access_controller,
            proposal(new_rules(), Some(5)),
        )
        .build();

    // Act
    let class = detailed_access_controller_class(&mut ledger, manifest);

    // Assert
    assert_eq!(
        class,
        DetailedManifestClassification::AccessControllerConfirmRecovery(
            AccessControllerConfirmRecoveryOutput {
                confirmations: vec![AccessControllerRecoveryConfirmation {
                    access_controller,
                    proposer: Proposer::Recovery,
                    kind: RecoveryConfirmationKind::Timed,
                    rule_set: new_rules(),
                    timed_recovery_delay_in_minutes: Some(5),
                }],
            }
        )
    );
}

#[test]
fn confirming_on_several_access_controllers_is_confirm_recovery() {
    let mut allocator = TestAddressAllocator::new();
    let manifest = ManifestBuilder::new()
        .quick_confirm_recovery(
            allocator.new_access_controller_address(),
            Proposer::Primary,
            proposal(allow_all(), None),
        )
        .timed_confirm_recovery(
            allocator.new_access_controller_address(),
            proposal(allow_all(), None),
        )
        .build();
    assert_access_controller_class(manifest, CONFIRM_RECOVERY);
}

/// Confirming resets the access controller, so a second confirmation would
/// fail: there is nothing left to confirm.
#[test]
fn confirming_twice_on_the_same_access_controller_is_not_confirm_recovery() {
    let access_controller = access_controller();
    let manifest = ManifestBuilder::new()
        .quick_confirm_recovery(
            access_controller,
            Proposer::Primary,
            proposal(allow_all(), None),
        )
        .quick_confirm_recovery(
            access_controller,
            Proposer::Recovery,
            proposal(allow_all(), None),
        )
        .build();
    assert_access_controller_class(manifest, None);
}

#[test]
fn other_access_controller_calls_are_not_confirm_recovery() {
    let access_controller = access_controller();
    let others: [fn(ManifestBuilder, ComponentAddress) -> ManifestBuilder; 5] = [
        |builder, access_controller| {
            builder.initiate_recovery(
                access_controller,
                Proposer::Recovery,
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
            builder.cancel_recovery(access_controller, Proposer::Primary)
        },
        |builder, access_controller| {
            builder.lock_primary_role(access_controller)
        },
        |builder, access_controller| {
            builder.unlock_primary_role(access_controller)
        },
    ];
    for other in others {
        let manifest = other(
            ManifestBuilder::new().timed_confirm_recovery(
                access_controller,
                proposal(allow_all(), None),
            ),
            access_controller,
        )
        .build();
        assert_access_controller_class(manifest, None);
    }
}
