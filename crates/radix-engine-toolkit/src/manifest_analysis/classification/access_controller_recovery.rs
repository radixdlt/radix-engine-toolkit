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

/// Manifests that propose new rules for one or more access controllers and
/// confirm them right away, so the rules change in the transaction.
///
/// On each access controller the manifest proposes a recovery once, then
/// quick confirms that same proposal: the same proposer, rule set and timed
/// recovery delay. So the proposals in the output are exactly the rules the
/// manifest enacts. Confirming also unlocks the primary role and discards the
/// access controller's other recovery proposal, if any. Besides the access
/// controller calls, the manifest may only pay the fee.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct AccessControllerRecoveryClass;

pub type AccessControllerRecoveryAnalyzer =
    AccessControllerAnalyzer<AccessControllerRecoveryClass>;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct AccessControllerRecoveryOutput {
    pub recovery_proposals: Vec<AccessControllerRecoveryProposal>,
}

/// A recovery proposed and confirmed on an access controller.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AccessControllerRecoveryProposal {
    pub access_controller: ComponentAddress,
    /// The role that proposed the recovery.
    pub proposer: Proposer,
    /// The rules that replace the access controller's current ones.
    pub rule_set: RuleSet,
    /// The timed recovery delay named in the proposal. The engine doesn't
    /// apply it: the access controller keeps the delay it was created with.
    pub timed_recovery_delay_in_minutes: Option<u32>,
}

impl AccessControllerClass for AccessControllerRecoveryClass {
    type Output = AccessControllerRecoveryOutput;

    fn classify(calls: &AccessControllerCalls) -> Option<Self::Output> {
        let recovery_proposals = calls
            .by_access_controller()?
            .iter()
            .map(|(access_controller, calls)| match calls.as_slice() {
                [AccessControllerCall::InitiateRecovery {
                    proposer,
                    proposal,
                }, AccessControllerCall::QuickConfirmRecovery {
                    proposer: confirmed_proposer,
                    proposal: confirmed_proposal,
                }] if proposer == confirmed_proposer
                    && proposal == confirmed_proposal =>
                {
                    Some(AccessControllerRecoveryProposal {
                        access_controller: *access_controller,
                        proposer: *proposer,
                        rule_set: proposal.rule_set.clone(),
                        timed_recovery_delay_in_minutes: proposal
                            .timed_recovery_delay_in_minutes,
                    })
                }
                _ => None,
            })
            .collect::<Option<_>>()?;

        Some(AccessControllerRecoveryOutput { recovery_proposals })
    }
}
