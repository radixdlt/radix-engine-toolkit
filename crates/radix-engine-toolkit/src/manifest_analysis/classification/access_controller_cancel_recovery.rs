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

/// Manifests that stop or cancel recoveries already proposed on one or more
/// access controllers.
///
/// On each access controller the manifest stops the timed recovery, cancels
/// recovery proposals, or both, each at most once, and stops before it
/// cancels the recovery role's proposal. Stopping only stops the timer: the
/// proposal stays, and can still be confirmed by another role. A stop names
/// the proposal it stops and fails unless it's the proposed one, while a
/// cancellation names nothing, so what it cancels is only known from the
/// ledger. A stopped timed recovery that's also cancelled is discarded. The
/// manifest may also unlock the primary role, and pay the fee.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct AccessControllerCancelRecoveryClass;

pub type AccessControllerCancelRecoveryAnalyzer =
    AccessControllerAnalyzer<AccessControllerCancelRecoveryClass>;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct AccessControllerCancelRecoveryOutput {
    pub cancellations: Vec<AccessControllerRecoveryCancellation>,
}

/// The recoveries stopped or cancelled on an access controller.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AccessControllerRecoveryCancellation {
    pub access_controller: ComponentAddress,
    /// The recovery role's proposal whose timed recovery is stopped, if any.
    pub stopped_timed_recovery: Option<RecoveryProposal>,
    /// The proposers whose recovery proposals are cancelled.
    pub cancelled_proposers: Vec<Proposer>,
    pub unlocks_primary_role: bool,
}

impl AccessControllerClass for AccessControllerCancelRecoveryClass {
    type Output = AccessControllerCancelRecoveryOutput;

    fn classify(calls: &AccessControllerCalls) -> Option<Self::Output> {
        let cancellations = calls
            .by_access_controller()?
            .iter()
            .map(|(access_controller, calls)| {
                cancellation(*access_controller, calls)
            })
            .collect::<Option<_>>()?;

        Some(AccessControllerCancelRecoveryOutput { cancellations })
    }
}

fn cancellation(
    access_controller: ComponentAddress,
    calls: &[AccessControllerCall],
) -> Option<AccessControllerRecoveryCancellation> {
    let mut cancellation = AccessControllerRecoveryCancellation {
        access_controller,
        stopped_timed_recovery: None,
        cancelled_proposers: Vec::new(),
        unlocks_primary_role: false,
    };

    for call in calls {
        match call {
            AccessControllerCall::StopTimedRecovery { proposal }
                if cancellation.stopped_timed_recovery.is_none()
                    && !cancellation
                        .cancelled_proposers
                        .contains(&Proposer::Recovery) =>
            {
                cancellation.stopped_timed_recovery = Some(proposal.clone());
            }
            AccessControllerCall::CancelRecovery { proposer }
                if !cancellation.cancelled_proposers.contains(proposer) =>
            {
                cancellation.cancelled_proposers.push(*proposer);
            }
            AccessControllerCall::UnlockPrimaryRole
                if !cancellation.unlocks_primary_role =>
            {
                cancellation.unlocks_primary_role = true;
            }
            _ => return None,
        }
    }

    (cancellation.stopped_timed_recovery.is_some()
        || !cancellation.cancelled_proposers.is_empty())
    .then_some(cancellation)
}
