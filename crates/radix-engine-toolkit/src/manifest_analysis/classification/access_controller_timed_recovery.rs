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

/// Manifests that start a timed recovery of one or more access controllers.
///
/// On each access controller the manifest proposes a recovery as the recovery
/// role, once, and doesn't confirm it. Unless another role confirms or stops
/// it first, the recovery can be confirmed by anyone once the access
/// controller's current timed recovery delay has passed, which is on the
/// ledger, not in the manifest. Without a delay, it waits for another role to
/// confirm it. The manifest may also lock the primary role until the recovery
/// is confirmed, and pay the fee.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct AccessControllerTimedRecoveryClass;

pub type AccessControllerTimedRecoveryAnalyzer =
    AccessControllerAnalyzer<AccessControllerTimedRecoveryClass>;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct AccessControllerTimedRecoveryOutput {
    pub timed_recoveries: Vec<AccessControllerTimedRecovery>,
}

/// A timed recovery started on an access controller.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AccessControllerTimedRecovery {
    pub access_controller: ComponentAddress,
    /// The rules that replace the access controller's current ones once the
    /// recovery is confirmed.
    pub rule_set: RuleSet,
    /// The timed recovery delay named in the proposal. It isn't the delay
    /// before the recovery can be confirmed, which is the access controller's
    /// own, and the engine doesn't apply it when the recovery is confirmed.
    pub timed_recovery_delay_in_minutes: Option<u32>,
    /// Whether the primary role is locked until the recovery is confirmed.
    pub locks_primary_role: bool,
}

impl AccessControllerClass for AccessControllerTimedRecoveryClass {
    type Output = AccessControllerTimedRecoveryOutput;

    fn classify(calls: &AccessControllerCalls) -> Option<Self::Output> {
        use AccessControllerCall::*;

        let timed_recoveries = calls
            .by_access_controller()?
            .iter()
            .map(|(access_controller, calls)| {
                let (proposal, locks_primary_role) = match calls.as_slice() {
                    [InitiateRecovery {
                        proposer: Proposer::Recovery,
                        proposal,
                    }] => (proposal, false),
                    [InitiateRecovery {
                        proposer: Proposer::Recovery,
                        proposal,
                    }, LockPrimaryRole]
                    | [LockPrimaryRole, InitiateRecovery {
                        proposer: Proposer::Recovery,
                        proposal,
                    }] => (proposal, true),
                    _ => return None,
                };
                Some(AccessControllerTimedRecovery {
                    access_controller: *access_controller,
                    rule_set: proposal.rule_set.clone(),
                    timed_recovery_delay_in_minutes: proposal
                        .timed_recovery_delay_in_minutes,
                    locks_primary_role,
                })
            })
            .collect::<Option<_>>()?;

        Some(AccessControllerTimedRecoveryOutput { timed_recoveries })
    }
}
