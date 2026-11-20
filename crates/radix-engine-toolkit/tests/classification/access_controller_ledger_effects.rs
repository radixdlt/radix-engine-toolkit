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

//! Checks that each access controller class's output is what the manifest
//! does: the manifest is classified, then executed, and the access
//! controller's rules and state are read back from the ledger.

use super::access_controller_utils::*;
use crate::prelude::*;
use radix_engine::blueprints::access_controller::latest::{
    AccessControllerV2Field, AccessControllerV2StateFieldPayload,
};
use radix_engine::blueprints::access_controller::{
    PrimaryRoleLockingState, PrimaryRoleRecoveryAttemptState,
    RecoveryRoleRecoveryAttemptState, RecoveryRoleRecoveryState,
};
use radix_engine::object_modules::role_assignment::{
    RoleAssignmentAccessRuleEntryPayload, RoleAssignmentCollection,
};
use radix_engine::system::system_db_reader::{
    ObjectCollectionKey, SystemDatabaseReader,
};

/// What the ledger says about an access controller.
#[derive(Clone, Debug, PartialEq, Eq)]
struct AccessControllerOnLedger {
    rule_set: RuleSet,
    timed_recovery_delay_in_minutes: Option<u32>,
    primary_role_locked: bool,
    primary_role_proposal: Option<RecoveryProposal>,
    recovery_role_proposal: Option<RecoveryRoleProposal>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum RecoveryRoleProposal {
    Timed(RecoveryProposal),
    Untimed(RecoveryProposal),
}

fn read(
    ledger: &DefaultLedgerSimulator,
    access_controller: ComponentAddress,
) -> AccessControllerOnLedger {
    let reader = SystemDatabaseReader::new(ledger.substate_db());
    let node_id = access_controller.as_node_id();
    let role = |name: &str| -> AccessRule {
        reader
            .read_object_collection_entry::<_, RoleAssignmentAccessRuleEntryPayload>(
                node_id,
                ModuleId::RoleAssignment,
                ObjectCollectionKey::KeyValue(
                    RoleAssignmentCollection::AccessRuleKeyValue
                        .collection_index(),
                    &ModuleRoleKey::new(ModuleId::Main, name),
                ),
            )
            .unwrap()
            .unwrap()
            .fully_update_and_into_latest_version()
    };
    let substate = reader
        .read_typed_object_field::<AccessControllerV2StateFieldPayload>(
            node_id,
            ModuleId::Main,
            AccessControllerV2Field::State.field_index(),
        )
        .unwrap()
        .fully_update_and_into_latest_version();
    let (locking, primary_role_attempt, _, recovery_role_attempt, _) =
        substate.state;

    AccessControllerOnLedger {
        rule_set: RuleSet {
            primary_role: role("primary"),
            recovery_role: role("recovery"),
            confirmation_role: role("confirmation"),
        },
        timed_recovery_delay_in_minutes: substate
            .timed_recovery_delay_in_minutes,
        primary_role_locked: locking == PrimaryRoleLockingState::Locked,
        primary_role_proposal: match primary_role_attempt {
            PrimaryRoleRecoveryAttemptState::NoRecoveryAttempt => None,
            PrimaryRoleRecoveryAttemptState::RecoveryAttempt(proposal) => {
                Some(proposal)
            }
        },
        recovery_role_proposal: match recovery_role_attempt {
            RecoveryRoleRecoveryAttemptState::NoRecoveryAttempt => None,
            RecoveryRoleRecoveryAttemptState::RecoveryAttempt(
                RecoveryRoleRecoveryState::TimedRecovery { proposal, .. },
            ) => Some(RecoveryRoleProposal::Timed(proposal)),
            RecoveryRoleRecoveryAttemptState::RecoveryAttempt(
                RecoveryRoleRecoveryState::UntimedRecovery(proposal),
            ) => Some(RecoveryRoleProposal::Untimed(proposal)),
        },
    }
}

/// The access controller of the tests as created: every role allows all and
/// the timed recovery delay is 10 minutes.
fn created() -> AccessControllerOnLedger {
    AccessControllerOnLedger {
        rule_set: allow_all(),
        timed_recovery_delay_in_minutes: Some(10),
        primary_role_locked: false,
        primary_role_proposal: None,
        recovery_role_proposal: None,
    }
}

/// Classifies the manifest made of `calls`, executes it, and returns its
/// class and the access controller as it is afterwards.
fn classify_and_execute(
    ledger: &mut DefaultLedgerSimulator,
    account: ComponentAddress,
    access_controller: ComponentAddress,
    calls: impl Fn(ManifestBuilder) -> ManifestBuilder,
) -> (DetailedManifestClassification, AccessControllerOnLedger) {
    let class = detailed_access_controller_class(
        ledger,
        calls(ManifestBuilder::new().lock_fee(account, 10)).build(),
    );
    execute(ledger, calls);
    (class, read(ledger, access_controller))
}

#[test]
fn recovery_enacts_the_rules_shown() {
    for proposer in PROPOSERS {
        // Arrange
        let (mut ledger, account, access_controller) =
            ledger_with_access_controller();
        // A pending proposal of each role, which the recovery discards.
        execute(&mut ledger, |builder| {
            builder
                .initiate_recovery(
                    access_controller,
                    Proposer::Primary,
                    proposal(deny_all(), None),
                )
                .initiate_recovery(
                    access_controller,
                    Proposer::Recovery,
                    proposal(deny_all(), None),
                )
                .lock_primary_role(access_controller)
                .cancel_recovery(access_controller, proposer)
        });

        // Act
        let (class, after) = classify_and_execute(
            &mut ledger,
            account,
            access_controller,
            |builder| {
                builder
                    .initiate_recovery(
                        access_controller,
                        proposer,
                        proposal(new_rules(), Some(5)),
                    )
                    .quick_confirm_recovery(
                        access_controller,
                        proposer,
                        proposal(new_rules(), Some(5)),
                    )
            },
        );

        // Assert
        let DetailedManifestClassification::AccessControllerRecovery(
            AccessControllerRecoveryOutput { recovery_proposals },
        ) = class
        else {
            panic!("Not a recovery: {class:?}")
        };
        let [shown] = recovery_proposals.as_slice() else {
            panic!("Not one proposal: {recovery_proposals:?}")
        };
        // Only the rules change: the engine doesn't apply the proposal's
        // timed recovery delay, so the access controller keeps its own.
        assert_eq!(
            after,
            AccessControllerOnLedger {
                rule_set: shown.rule_set.clone(),
                ..created()
            }
        );
        assert_eq!(shown.timed_recovery_delay_in_minutes, Some(5));
        assert_eq!(after.rule_set, new_rules());
    }
}

#[test]
fn timed_recovery_leaves_the_proposal_shown_pending() {
    for locks_primary_role in [false, true] {
        // Arrange
        let (mut ledger, account, access_controller) =
            ledger_with_access_controller();

        // Act
        let (class, after) = classify_and_execute(
            &mut ledger,
            account,
            access_controller,
            |builder| {
                let builder = builder.initiate_recovery(
                    access_controller,
                    Proposer::Recovery,
                    proposal(new_rules(), Some(5)),
                );
                if locks_primary_role {
                    builder.lock_primary_role(access_controller)
                } else {
                    builder
                }
            },
        );

        // Assert
        let DetailedManifestClassification::AccessControllerTimedRecovery(
            AccessControllerTimedRecoveryOutput { timed_recoveries },
        ) = class
        else {
            panic!("Not a timed recovery: {class:?}")
        };
        let [shown] = timed_recoveries.as_slice() else {
            panic!("Not one timed recovery: {timed_recoveries:?}")
        };
        assert_eq!(
            after,
            AccessControllerOnLedger {
                primary_role_locked: shown.locks_primary_role,
                recovery_role_proposal: Some(RecoveryRoleProposal::Timed(
                    proposal(
                        shown.rule_set.clone(),
                        shown.timed_recovery_delay_in_minutes
                    )
                )),
                ..created()
            }
        );
        assert_eq!(shown.locks_primary_role, locks_primary_role);
    }
}

#[test]
fn confirm_recovery_enacts_the_rules_shown() {
    for (proposer, timed) in [
        (Proposer::Primary, false),
        (Proposer::Recovery, false),
        (Proposer::Recovery, true),
    ] {
        // Arrange
        let (mut ledger, account, access_controller) =
            ledger_with_access_controller();
        let other = match proposer {
            Proposer::Primary => Proposer::Recovery,
            Proposer::Recovery => Proposer::Primary,
        };
        // The proposal confirmed, and one of the other role, which confirming
        // discards.
        execute(&mut ledger, |builder| {
            builder
                .initiate_recovery(
                    access_controller,
                    other,
                    proposal(deny_all(), None),
                )
                .initiate_recovery(
                    access_controller,
                    proposer,
                    proposal(new_rules(), Some(5)),
                )
                .lock_primary_role(access_controller)
        });
        if timed {
            advance_time(&mut ledger, 10);
        }

        // Act
        let (class, after) = classify_and_execute(
            &mut ledger,
            account,
            access_controller,
            |builder| {
                if timed {
                    builder.timed_confirm_recovery(
                        access_controller,
                        proposal(new_rules(), Some(5)),
                    )
                } else {
                    builder.quick_confirm_recovery(
                        access_controller,
                        proposer,
                        proposal(new_rules(), Some(5)),
                    )
                }
            },
        );

        // Assert
        let DetailedManifestClassification::AccessControllerConfirmRecovery(
            AccessControllerConfirmRecoveryOutput { confirmations },
        ) = class
        else {
            panic!("Not a confirmation: {class:?}")
        };
        let [shown] = confirmations.as_slice() else {
            panic!("Not one confirmation: {confirmations:?}")
        };
        // Only the rules change: the engine doesn't apply the proposal's
        // timed recovery delay, so the access controller keeps its own.
        assert_eq!(
            after,
            AccessControllerOnLedger {
                rule_set: shown.rule_set.clone(),
                ..created()
            }
        );
        assert_eq!(shown.timed_recovery_delay_in_minutes, Some(5));
        assert_eq!((shown.proposer, after.rule_set), (proposer, new_rules()));
    }
}

/// The engine rejects a confirmation of anything but the pending proposal, so
/// the rules a confirmation shows can't differ from the rules it enacts.
#[test]
fn confirming_other_rules_than_the_pending_ones_fails() {
    // Arrange
    let (mut ledger, _, access_controller) = ledger_with_access_controller();
    execute(&mut ledger, |builder| {
        builder.initiate_recovery(
            access_controller,
            Proposer::Primary,
            proposal(new_rules(), Some(5)),
        )
    });

    for confirmed in
        [proposal(deny_all(), Some(5)), proposal(new_rules(), None)]
    {
        // Act
        let manifest = ManifestBuilder::new()
            .lock_fee_from_faucet()
            .quick_confirm_recovery(
                access_controller,
                Proposer::Primary,
                confirmed,
            )
            .build();
        let receipt = ledger.execute_manifest(manifest, vec![]);

        // Assert
        receipt.expect_commit_failure();
    }
    assert_eq!(read(&ledger, access_controller).rule_set, allow_all());
}

#[test]
fn stopping_leaves_the_proposal_shown_pending() {
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

    // Act
    let (class, after) = classify_and_execute(
        &mut ledger,
        account,
        access_controller,
        |builder| {
            builder.stop_timed_recovery(
                access_controller,
                proposal(new_rules(), Some(5)),
            )
        },
    );

    // Assert
    let DetailedManifestClassification::AccessControllerCancelRecovery(
        AccessControllerCancelRecoveryOutput { cancellations },
    ) = class
    else {
        panic!("Not a cancellation: {class:?}")
    };
    let [shown] = cancellations.as_slice() else {
        panic!("Not one cancellation: {cancellations:?}")
    };
    assert_eq!(
        after,
        AccessControllerOnLedger {
            recovery_role_proposal: Some(RecoveryRoleProposal::Untimed(
                shown.stopped_timed_recovery.clone().unwrap()
            )),
            ..created()
        }
    );
    assert!(shown.cancelled_proposers.is_empty());
}

#[test]
fn cancel_recovery_discards_the_proposals_shown() {
    // Arrange
    let (mut ledger, account, access_controller) =
        ledger_with_access_controller();
    execute(&mut ledger, |builder| {
        builder
            .initiate_recovery(
                access_controller,
                Proposer::Primary,
                proposal(deny_all(), None),
            )
            .initiate_recovery(
                access_controller,
                Proposer::Recovery,
                proposal(new_rules(), Some(5)),
            )
            .lock_primary_role(access_controller)
    });

    // Act
    let (class, after) = classify_and_execute(
        &mut ledger,
        account,
        access_controller,
        |builder| {
            builder
                .stop_timed_recovery(
                    access_controller,
                    proposal(new_rules(), Some(5)),
                )
                .cancel_recovery(access_controller, Proposer::Recovery)
                .cancel_recovery(access_controller, Proposer::Primary)
                .unlock_primary_role(access_controller)
        },
    );

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
                    cancelled_proposers: vec![
                        Proposer::Recovery,
                        Proposer::Primary
                    ],
                    unlocks_primary_role: true,
                }],
            }
        )
    );
    assert_eq!(after, created());
}

#[test]
fn primary_role_lock_does_what_it_shows() {
    // Arrange
    let (mut ledger, account, access_controller) =
        ledger_with_access_controller();

    for (lock, locked) in [(true, true), (false, false)] {
        // Act
        let (class, after) = classify_and_execute(
            &mut ledger,
            account,
            access_controller,
            |builder| {
                if lock {
                    builder.lock_primary_role(access_controller)
                } else {
                    builder.unlock_primary_role(access_controller)
                }
            },
        );

        // Assert
        let DetailedManifestClassification::AccessControllerPrimaryRoleLock(
            AccessControllerPrimaryRoleLockOutput { operations },
        ) = class
        else {
            panic!("Not a primary role lock: {class:?}")
        };
        let [shown] = operations.as_slice() else {
            panic!("Not one operation: {operations:?}")
        };
        assert_eq!(
            shown.operation == PrimaryRoleLockOperation::Lock,
            after.primary_role_locked
        );
        assert_eq!(
            after,
            AccessControllerOnLedger {
                primary_role_locked: locked,
                ..created()
            }
        );
    }
}
