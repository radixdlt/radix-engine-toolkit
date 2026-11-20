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

const CANCEL_RECOVERY: Option<ManifestClassification> =
    Some(ManifestClassification::AccessControllerCancelRecovery);

fn access_controller() -> ComponentAddress {
    TestAddressAllocator::new().new_access_controller_address()
}

#[test]
fn stopping_and_cancelling_shows_the_stopped_proposal() {
    // Arrange
    let (mut ledger, account, access_controller) =
        ledger_with_access_controller();
    // The access controller has a timed recovery delay, so the recovery role's
    // proposal starts a timed recovery.
    execute(&mut ledger, |builder| {
        builder
            .initiate_recovery(
                access_controller,
                Proposer::Recovery,
                proposal(new_rules(), Some(5)),
            )
            .lock_primary_role(access_controller)
    });
    let manifest = ManifestBuilder::new()
        .lock_fee(account, 10)
        .stop_timed_recovery(access_controller, proposal(new_rules(), Some(5)))
        .cancel_recovery(access_controller, Proposer::Recovery)
        .unlock_primary_role(access_controller)
        .build();

    // Act
    let class = detailed_access_controller_class(&mut ledger, manifest);

    // Assert
    assert_eq!(
        class,
        DetailedManifestClassification::AccessControllerCancelRecovery(
            AccessControllerCancelRecoveryOutput {
                cancellations: vec![AccessControllerRecoveryCancellation {
                    access_controller,
                    stopped_timed_recovery: Some(proposal(
                        new_rules(),
                        Some(5)
                    )),
                    cancelled_proposers: vec![Proposer::Recovery],
                    unlocks_primary_role: true,
                }],
            }
        )
    );
}

#[test]
fn cancelling_shows_the_cancelled_proposer() {
    for proposer in PROPOSERS {
        // Arrange
        let (mut ledger, account, access_controller) =
            ledger_with_access_controller();
        execute(&mut ledger, |builder| {
            builder.initiate_recovery(
                access_controller,
                proposer,
                proposal(new_rules(), None),
            )
        });
        let manifest = ManifestBuilder::new()
            .lock_fee(account, 10)
            .cancel_recovery(access_controller, proposer)
            .build();

        // Act
        let class = detailed_access_controller_class(&mut ledger, manifest);

        // Assert
        assert_eq!(
            class,
            DetailedManifestClassification::AccessControllerCancelRecovery(
                AccessControllerCancelRecoveryOutput {
                    cancellations: vec![AccessControllerRecoveryCancellation {
                        access_controller,
                        stopped_timed_recovery: None,
                        cancelled_proposers: vec![proposer],
                        unlocks_primary_role: false,
                    }],
                }
            )
        );
    }
}

/// Stopping leaves the proposal in place, to be confirmed or cancelled later.
#[test]
fn stopping_alone_is_cancel_recovery() {
    let manifest = ManifestBuilder::new()
        .stop_timed_recovery(access_controller(), proposal(allow_all(), None))
        .build();
    assert_access_controller_class(manifest, CANCEL_RECOVERY);
}

#[test]
fn cancelling_both_proposals_is_cancel_recovery() {
    let access_controller = access_controller();
    let manifest = ManifestBuilder::new()
        .cancel_recovery(access_controller, Proposer::Primary)
        .stop_timed_recovery(access_controller, proposal(allow_all(), None))
        .cancel_recovery(access_controller, Proposer::Recovery)
        .build();
    assert_access_controller_class(manifest, CANCEL_RECOVERY);
}

#[test]
fn cancelling_on_several_access_controllers_is_cancel_recovery() {
    let mut allocator = TestAddressAllocator::new();
    let manifest = ManifestBuilder::new()
        .stop_timed_recovery(
            allocator.new_access_controller_address(),
            proposal(allow_all(), None),
        )
        .cancel_recovery(
            allocator.new_access_controller_address(),
            Proposer::Primary,
        )
        .build();
    assert_access_controller_class(manifest, CANCEL_RECOVERY);
}

/// Once the proposal is cancelled there is no timed recovery left to stop.
#[test]
fn stopping_after_cancelling_the_recovery_role_proposal_is_not_cancel_recovery()
{
    let access_controller = access_controller();
    let manifest = ManifestBuilder::new()
        .cancel_recovery(access_controller, Proposer::Recovery)
        .stop_timed_recovery(access_controller, proposal(allow_all(), None))
        .build();
    assert_access_controller_class(manifest, None);
}

#[test]
fn repeating_a_call_is_not_cancel_recovery() {
    let access_controller = access_controller();
    let repeated: [fn(ManifestBuilder, ComponentAddress) -> ManifestBuilder;
        3] = [
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
            builder
                .cancel_recovery(access_controller, Proposer::Primary)
                .unlock_primary_role(access_controller)
        },
    ];
    for call in repeated {
        let builder = call(ManifestBuilder::new(), access_controller);
        let manifest = call(builder, access_controller).build();
        assert_access_controller_class(manifest, None);
    }
}

#[test]
fn other_access_controller_calls_are_not_cancel_recovery() {
    let access_controller = access_controller();
    let others: [fn(ManifestBuilder, ComponentAddress) -> ManifestBuilder; 4] = [
        |builder, access_controller| {
            builder.initiate_recovery(
                access_controller,
                Proposer::Recovery,
                proposal(allow_all(), None),
            )
        },
        |builder, access_controller| {
            builder.quick_confirm_recovery(
                access_controller,
                Proposer::Primary,
                proposal(allow_all(), None),
            )
        },
        |builder, access_controller| {
            builder.timed_confirm_recovery(
                access_controller,
                proposal(allow_all(), None),
            )
        },
        |builder, access_controller| {
            builder.lock_primary_role(access_controller)
        },
    ];
    for other in others {
        let manifest = other(
            ManifestBuilder::new()
                .cancel_recovery(access_controller, Proposer::Primary),
            access_controller,
        )
        .build();
        assert_access_controller_class(manifest, None);
    }
}

#[test]
fn setting_account_metadata_is_not_cancel_recovery() {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .cancel_recovery(
            allocator.new_access_controller_address(),
            Proposer::Primary,
        )
        .set_metadata(account, "owner_keys", "not important")
        .build();
    assert_access_controller_class(manifest, None);
}
