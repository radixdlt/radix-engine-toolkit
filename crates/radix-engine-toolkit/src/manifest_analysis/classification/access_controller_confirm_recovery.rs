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

use crate::internal_prelude::*;

/// Manifests that confirm recoveries already proposed on one or more access
/// controllers, so the proposed rules replace the current ones.
///
/// On each access controller the manifest confirms one recovery, quick or
/// timed. A confirmation names the rule set and timed recovery delay it
/// confirms, and fails unless they're the proposed ones, so the output shows
/// exactly the rules enacted. Confirming also unlocks the primary role and
/// discards the access controller's other recovery proposal, if any. Besides
/// the access controller calls, the manifest may only pay the fee.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct AccessControllerConfirmRecoveryClass;

pub type AccessControllerConfirmRecoveryAnalyzer =
    AccessControllerAnalyzer<AccessControllerConfirmRecoveryClass>;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct AccessControllerConfirmRecoveryOutput {
    pub confirmations: Vec<AccessControllerRecoveryConfirmation>,
}

/// A recovery confirmed on an access controller.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AccessControllerRecoveryConfirmation {
    pub access_controller: ComponentAddress,
    /// The role that proposed the recovery.
    pub proposer: Proposer,
    pub kind: RecoveryConfirmationKind,
    /// The rules that replace the access controller's current ones.
    pub rule_set: RuleSet,
    /// The timed recovery delay named in the proposal. The engine doesn't
    /// apply it: the access controller keeps the delay it was created with.
    pub timed_recovery_delay_in_minutes: Option<u32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RecoveryConfirmationKind {
    /// Confirmed by a role other than the proposer.
    Quick,
    /// Confirmed after the recovery role's timed recovery delay has passed,
    /// which needs no role.
    Timed,
}

impl AccessControllerClass for AccessControllerConfirmRecoveryClass {
    type Output = AccessControllerConfirmRecoveryOutput;

    fn classify(calls: &AccessControllerCalls) -> Option<Self::Output> {
        use AccessControllerCall::*;

        let confirmations = calls
            .by_access_controller()?
            .iter()
            .map(|(access_controller, calls)| {
                let (proposer, kind, proposal) = match calls.as_slice() {
                    [QuickConfirmRecovery { proposer, proposal }] => {
                        (*proposer, RecoveryConfirmationKind::Quick, proposal)
                    }
                    [TimedConfirmRecovery { proposal }] => (
                        Proposer::Recovery,
                        RecoveryConfirmationKind::Timed,
                        proposal,
                    ),
                    _ => return None,
                };
                Some(AccessControllerRecoveryConfirmation {
                    access_controller: *access_controller,
                    proposer,
                    kind,
                    rule_set: proposal.rule_set.clone(),
                    timed_recovery_delay_in_minutes: proposal
                        .timed_recovery_delay_in_minutes,
                })
            })
            .collect::<Option<_>>()?;

        Some(AccessControllerConfirmRecoveryOutput { confirmations })
    }
}
