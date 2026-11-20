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

//! Helpers for the access controller classification tests.

use crate::prelude::*;
use radix_transactions::manifest::BuildableManifest;

pub const PROPOSERS: [Proposer; 2] = [Proposer::Primary, Proposer::Recovery];

/// Every access controller class.
pub const ACCESS_CONTROLLER_CLASSES: [ManifestClassification; 5] = [
    ManifestClassification::AccessControllerRecovery,
    ManifestClassification::AccessControllerTimedRecovery,
    ManifestClassification::AccessControllerConfirmRecovery,
    ManifestClassification::AccessControllerCancelRecovery,
    ManifestClassification::AccessControllerPrimaryRoleLock,
];

pub fn rule_set(rule: AccessRule) -> RuleSet {
    RuleSet {
        primary_role: rule.clone(),
        recovery_role: rule.clone(),
        confirmation_role: rule,
    }
}

pub fn allow_all() -> RuleSet {
    rule_set(rule!(allow_all))
}

pub fn deny_all() -> RuleSet {
    rule_set(rule!(deny_all))
}

/// Rules other than the ones access controllers are created with in the tests.
pub fn new_rules() -> RuleSet {
    RuleSet {
        primary_role: rule!(deny_all),
        recovery_role: rule!(allow_all),
        confirmation_role: rule!(allow_all),
    }
}

pub fn proposal(
    rule_set: RuleSet,
    timed_recovery_delay_in_minutes: Option<u32>,
) -> RecoveryProposal {
    RecoveryProposal {
        rule_set,
        timed_recovery_delay_in_minutes,
    }
}

/// Access controller calls, named by what they do.
pub trait AccessControllerManifestBuilder: Sized {
    fn call(
        self,
        access_controller: ComponentAddress,
        method: &str,
        arguments: impl ResolvableArguments,
    ) -> Self;

    fn initiate_recovery(
        self,
        access_controller: ComponentAddress,
        proposer: Proposer,
        proposal: RecoveryProposal,
    ) -> Self {
        let method = match proposer {
            Proposer::Primary => {
                ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_PRIMARY_IDENT
            }
            Proposer::Recovery => {
                ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_RECOVERY_IDENT
            }
        };
        self.call(
            access_controller,
            method,
            AccessControllerInitiateRecoveryAsPrimaryManifestInput {
                rule_set: proposal.rule_set.into(),
                timed_recovery_delay_in_minutes: proposal
                    .timed_recovery_delay_in_minutes,
            },
        )
    }

    /// Quick confirms the recovery proposed by `proposer`.
    fn quick_confirm_recovery(
        self,
        access_controller: ComponentAddress,
        proposer: Proposer,
        proposal: RecoveryProposal,
    ) -> Self {
        let method = match proposer {
            Proposer::Primary => {
                ACCESS_CONTROLLER_QUICK_CONFIRM_PRIMARY_ROLE_RECOVERY_PROPOSAL_IDENT
            }
            Proposer::Recovery => {
                ACCESS_CONTROLLER_QUICK_CONFIRM_RECOVERY_ROLE_RECOVERY_PROPOSAL_IDENT
            }
        };
        self.call(
            access_controller,
            method,
            AccessControllerQuickConfirmPrimaryRoleRecoveryProposalManifestInput {
                rule_set: proposal.rule_set.into(),
                timed_recovery_delay_in_minutes: proposal
                    .timed_recovery_delay_in_minutes,
            },
        )
    }

    fn timed_confirm_recovery(
        self,
        access_controller: ComponentAddress,
        proposal: RecoveryProposal,
    ) -> Self {
        self.call(
            access_controller,
            ACCESS_CONTROLLER_TIMED_CONFIRM_RECOVERY_IDENT,
            AccessControllerTimedConfirmRecoveryManifestInput {
                rule_set: proposal.rule_set.into(),
                timed_recovery_delay_in_minutes: proposal
                    .timed_recovery_delay_in_minutes,
            },
        )
    }

    fn stop_timed_recovery(
        self,
        access_controller: ComponentAddress,
        proposal: RecoveryProposal,
    ) -> Self {
        self.call(
            access_controller,
            ACCESS_CONTROLLER_STOP_TIMED_RECOVERY_IDENT,
            AccessControllerStopTimedRecoveryManifestInput {
                rule_set: proposal.rule_set.into(),
                timed_recovery_delay_in_minutes: proposal
                    .timed_recovery_delay_in_minutes,
            },
        )
    }

    /// Cancels the recovery proposed by `proposer`.
    fn cancel_recovery(
        self,
        access_controller: ComponentAddress,
        proposer: Proposer,
    ) -> Self {
        let method = match proposer {
            Proposer::Primary => {
                ACCESS_CONTROLLER_CANCEL_PRIMARY_ROLE_RECOVERY_PROPOSAL_IDENT
            }
            Proposer::Recovery => {
                ACCESS_CONTROLLER_CANCEL_RECOVERY_ROLE_RECOVERY_PROPOSAL_IDENT
            }
        };
        self.call(access_controller, method, ())
    }

    fn lock_primary_role(self, access_controller: ComponentAddress) -> Self {
        self.call(
            access_controller,
            ACCESS_CONTROLLER_LOCK_PRIMARY_ROLE_IDENT,
            (),
        )
    }

    fn unlock_primary_role(self, access_controller: ComponentAddress) -> Self {
        self.call(
            access_controller,
            ACCESS_CONTROLLER_UNLOCK_PRIMARY_ROLE_IDENT,
            (),
        )
    }
}

impl<M: BuildableManifest> AccessControllerManifestBuilder
    for ManifestBuilder<M>
where
    M::Instruction: From<InstructionV1>,
{
    fn call(
        self,
        access_controller: ComponentAddress,
        method: &str,
        arguments: impl ResolvableArguments,
    ) -> Self {
        self.call_method(access_controller, method, arguments)
    }
}

/// Withdraws XRD from `account` and contributes it to the recovery fee vault of
/// `access_controller`.
pub fn top_up_recovery_fee(
    builder: ManifestBuilder,
    account: ComponentAddress,
    access_controller: ComponentAddress,
) -> ManifestBuilder {
    builder
        .withdraw_from_account(account, XRD, 100)
        .take_from_worktop(XRD, 100, "xrd")
        .then(|builder| {
            let bucket = builder.bucket("xrd");
            builder.call_method(
                access_controller,
                ACCESS_CONTROLLER_CONTRIBUTE_RECOVERY_FEE_IDENT,
                AccessControllerContributeRecoveryFeeManifestInput { bucket },
            )
        })
}

/// The access controller classes of the manifest, statically.
pub fn access_controller_classes(
    manifest: &TransactionManifestV1,
) -> Vec<ManifestClassification> {
    let StaticAnalysis {
        manifest_classification,
        ..
    } = statically_analyze(manifest).unwrap();
    ACCESS_CONTROLLER_CLASSES
        .into_iter()
        .filter(|class| manifest_classification.contains(class))
        .collect()
}

/// Asserts that the manifest is of the access controller class `expected`
/// and of no other, or of none if `expected` is `None`.
#[track_caller]
pub fn assert_access_controller_class(
    manifest: impl Into<TransactionManifestV1>,
    expected: Option<ManifestClassification>,
) {
    assert_eq!(
        access_controller_classes(&manifest.into()),
        expected.into_iter().collect::<Vec<_>>()
    );
}

/// The detailed access controller class of the manifest, dynamically, which
/// must be the only one.
#[track_caller]
pub fn detailed_access_controller_class(
    ledger: &mut DefaultLedgerSimulator,
    manifest: TransactionManifestV1,
) -> DetailedManifestClassification {
    let (
        _,
        DynamicAnalysis {
            detailed_manifest_classification,
            ..
        },
    ) = ledger.analyze(manifest);
    let mut classes = detailed_manifest_classification
        .into_iter()
        .filter(|class| {
            matches!(
                class,
                DetailedManifestClassification::AccessControllerRecovery(..)
                    | DetailedManifestClassification::AccessControllerTimedRecovery(..)
                    | DetailedManifestClassification::AccessControllerConfirmRecovery(..)
                    | DetailedManifestClassification::AccessControllerCancelRecovery(..)
                    | DetailedManifestClassification::AccessControllerPrimaryRoleLock(..)
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(classes.len(), 1, "{classes:?}");
    classes.remove(0)
}

/// A ledger with an access controller of an account, whose roles allow all
/// and whose timed recovery delay is 10 minutes.
pub fn ledger_with_access_controller(
) -> (DefaultLedgerSimulator, ComponentAddress, ComponentAddress) {
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (account, access_controller) = ledger.new_allow_all_access_controller();
    (ledger, account, access_controller)
}

/// Executes `calls` on the ledger, e.g. to propose the recovery a manifest
/// under test confirms.
pub fn execute(
    ledger: &mut DefaultLedgerSimulator,
    calls: impl FnOnce(ManifestBuilder) -> ManifestBuilder,
) {
    let manifest = calls(ManifestBuilder::new().lock_fee_from_faucet()).build();
    ledger
        .execute_manifest(manifest, vec![])
        .expect_commit_success();
}

/// Moves the ledger's time forward by `minutes` and a millisecond.
pub fn advance_time(ledger: &mut DefaultLedgerSimulator, minutes: i64) {
    let now_ms = ledger.get_current_proposer_timestamp_ms();
    let round = ledger.get_consensus_manager_state().round;
    ledger.advance_to_round_at_timestamp(
        Round::of(round.number() + 1),
        now_ms + minutes * 60_000 + 1,
    );
}
