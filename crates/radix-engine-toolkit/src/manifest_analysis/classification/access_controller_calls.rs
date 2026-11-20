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

//! What the access controller classes share: every one of them permits the
//! same instructions and records the same access controller calls, and differs
//! only in which calls, in which order, it accepts on each access controller.
//! Besides access controller calls, the classes only permit paying the fee, so
//! a manifest of any of them withdraws nothing and changes nothing else.
//! Each class is an [`AccessControllerClass`], analyzed by an
//! [`AccessControllerAnalyzer`] of it.

use super::common::*;
use crate::internal_prelude::*;

/// A class of manifests that operate on access controllers.
pub trait AccessControllerClass {
    type Output: Default;

    /// The class's output, if the manifest's `calls` are of the class.
    fn classify(calls: &AccessControllerCalls) -> Option<Self::Output>;
}

/// An access controller call that changes the access controller's state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AccessControllerCall {
    InitiateRecovery {
        proposer: Proposer,
        proposal: RecoveryProposal,
    },
    QuickConfirmRecovery {
        /// The proposer of the recovery confirmed.
        proposer: Proposer,
        proposal: RecoveryProposal,
    },
    /// Confirms the recovery role's timed recovery.
    TimedConfirmRecovery {
        proposal: RecoveryProposal,
    },
    /// Stops the recovery role's timed recovery.
    StopTimedRecovery {
        proposal: RecoveryProposal,
    },
    CancelRecovery {
        /// The proposer of the recovery cancelled.
        proposer: Proposer,
    },
    LockPrimaryRole,
    UnlockPrimaryRole,
}

/// The access controller calls of a manifest.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct AccessControllerCalls {
    /// The calls on each access controller, in the order they're made.
    calls: IndexMap<ComponentAddress, Vec<AccessControllerCall>>,
    /// Whether a call can't be classified: it's made on an access controller
    /// that only exists in the manifest, or proposes rules that can't be shown
    /// to the user.
    invalid: bool,
}

impl AccessControllerCalls {
    /// The calls on each access controller, if the manifest calls at least
    /// one and every call can be classified.
    pub fn by_access_controller(
        &self,
    ) -> Option<&IndexMap<ComponentAddress, Vec<AccessControllerCall>>> {
        (!self.invalid && !self.calls.is_empty()).then_some(&self.calls)
    }

    fn process_instruction(&mut self, context: InstructionContext<'_>) {
        if let Some((receiver, call)) = access_controller_call(context) {
            match (static_access_controller(receiver), call) {
                (Some(access_controller), Some(call)) => {
                    self.calls.entry(access_controller).or_default().push(call)
                }
                _ => self.invalid = true,
            }
        }
    }
}

/// Analyzes manifests of the access controller class `C`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AccessControllerAnalyzer<C> {
    calls: AccessControllerCalls,
    class: PhantomData<C>,
}

impl<C> Default for AccessControllerAnalyzer<C> {
    fn default() -> Self {
        Self {
            calls: Default::default(),
            class: PhantomData,
        }
    }
}

impl<C: AccessControllerClass> ManifestStaticAnalyzer
    for AccessControllerAnalyzer<C>
{
    type Initializer = ();
    type Output = C::Output;

    type PermissionState =
        CallbackPermissionState<PermissionStateStaticCallback>;

    type RequirementState = AccessControllerRequirementState<C>;

    fn new(
        _: Self::Initializer,
    ) -> (Self, Self::PermissionState, Self::RequirementState) {
        (
            Default::default(),
            CallbackPermissionState::new(is_instruction_permitted),
            Default::default(),
        )
    }

    fn output(self) -> Self::Output {
        C::classify(&self.calls).unwrap_or_default()
    }

    fn process_instruction(&mut self, context: InstructionContext<'_>) {
        self.calls.process_instruction(context);
    }
}

/// Fulfilled when the manifest is of the access controller class `C`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AccessControllerRequirementState<C> {
    calls: AccessControllerCalls,
    class: PhantomData<C>,
}

impl<C> Default for AccessControllerRequirementState<C> {
    fn default() -> Self {
        Self {
            calls: Default::default(),
            class: PhantomData,
        }
    }
}

impl<C: AccessControllerClass> ManifestAnalyzerRequirementState
    for AccessControllerRequirementState<C>
{
    fn requirement_state(&self) -> RequirementState {
        if self.calls.invalid {
            RequirementState::PermanentlyUnfulfilled
        } else if C::classify(&self.calls).is_some() {
            RequirementState::Fulfilled
        } else {
            RequirementState::CurrentlyUnfulfilled
        }
    }

    fn process_instruction(&mut self, context: InstructionContext<'_>) {
        self.calls.process_instruction(context);
    }
}

/// The instructions every access controller class permits: its access
/// controller calls and paying the fee.
fn is_instruction_permitted(context: InstructionContext<'_>) -> bool {
    is_permitted_subintent_instruction(context)
        || access_controller_call(context).is_some()
        || is_fee_payment(context)
}

/// Locking the fee, and creating the proofs it or the access controller calls
/// need: of an access controller's badge when the fee payer is securified, or
/// of badges an access controller's roles require.
fn is_fee_payment(context: InstructionContext<'_>) -> bool {
    matches!(
        context,
        InstructionContext::InvocationInstruction {
            typed_native_invocation: Some(TypedNativeInvocation {
                receiver: ManifestInvocationReceiver::GlobalMethod(..),
                invocation:
                    TypedManifestNativeInvocation::AccountBlueprintInvocation(
                        AccountBlueprintInvocation::Method(
                            AccountBlueprintMethod::LockFee(..)
                            | AccountBlueprintMethod::LockContingentFee(..)
                            | AccountBlueprintMethod::CreateProofOfAmount(..)
                            | AccountBlueprintMethod::CreateProofOfNonFungibles(..),
                        ),
                    )
                    | TypedManifestNativeInvocation::AccessControllerBlueprintInvocation(
                        AccessControllerBlueprintInvocation::Method(
                            AccessControllerBlueprintMethod::CreateProof(..)
                            | AccessControllerBlueprintMethod::LockRecoveryFee(..),
                        ),
                    ),
            }),
            ..
        }
    )
}

/// If the instruction is an access controller call, returns the access
/// controller and the call, which is `None` when the call proposes rules that
/// can't be shown to the user.
fn access_controller_call(
    context: InstructionContext<'_>,
) -> Option<(
    &ResolvedManifestAddress<GlobalAddress>,
    Option<AccessControllerCall>,
)> {
    let InstructionContext::InvocationInstruction {
        typed_native_invocation:
            Some(TypedNativeInvocation {
                receiver: ManifestInvocationReceiver::GlobalMethod(receiver),
                invocation:
                    TypedManifestNativeInvocation::AccessControllerBlueprintInvocation(
                        AccessControllerBlueprintInvocation::Method(method),
                    ),
            }),
        ..
    } = context
    else {
        return None;
    };

    let call = match method {
        AccessControllerBlueprintMethod::InitiateRecoveryAsPrimary(
            AccessControllerInitiateRecoveryAsPrimaryManifestInput {
                rule_set,
                timed_recovery_delay_in_minutes,
            },
        ) => recovery_proposal(rule_set, *timed_recovery_delay_in_minutes).map(
            |proposal| AccessControllerCall::InitiateRecovery {
                proposer: Proposer::Primary,
                proposal,
            },
        ),
        AccessControllerBlueprintMethod::InitiateRecoveryAsRecovery(
            AccessControllerInitiateRecoveryAsRecoveryManifestInput {
                rule_set,
                timed_recovery_delay_in_minutes,
            },
        ) => recovery_proposal(rule_set, *timed_recovery_delay_in_minutes).map(
            |proposal| AccessControllerCall::InitiateRecovery {
                proposer: Proposer::Recovery,
                proposal,
            },
        ),
        AccessControllerBlueprintMethod::QuickConfirmPrimaryRoleRecoveryProposal(
            AccessControllerQuickConfirmPrimaryRoleRecoveryProposalManifestInput {
                rule_set,
                timed_recovery_delay_in_minutes,
            },
        ) => recovery_proposal(rule_set, *timed_recovery_delay_in_minutes).map(
            |proposal| AccessControllerCall::QuickConfirmRecovery {
                proposer: Proposer::Primary,
                proposal,
            },
        ),
        AccessControllerBlueprintMethod::QuickConfirmRecoveryRoleRecoveryProposal(
            AccessControllerQuickConfirmRecoveryRoleRecoveryProposalManifestInput {
                rule_set,
                timed_recovery_delay_in_minutes,
            },
        ) => recovery_proposal(rule_set, *timed_recovery_delay_in_minutes).map(
            |proposal| AccessControllerCall::QuickConfirmRecovery {
                proposer: Proposer::Recovery,
                proposal,
            },
        ),
        AccessControllerBlueprintMethod::TimedConfirmRecovery(
            AccessControllerTimedConfirmRecoveryManifestInput {
                rule_set,
                timed_recovery_delay_in_minutes,
            },
        ) => recovery_proposal(rule_set, *timed_recovery_delay_in_minutes)
            .map(|proposal| AccessControllerCall::TimedConfirmRecovery {
                proposal,
            }),
        AccessControllerBlueprintMethod::StopTimedRecovery(
            AccessControllerStopTimedRecoveryManifestInput {
                rule_set,
                timed_recovery_delay_in_minutes,
            },
        ) => recovery_proposal(rule_set, *timed_recovery_delay_in_minutes)
            .map(|proposal| AccessControllerCall::StopTimedRecovery {
                proposal,
            }),
        AccessControllerBlueprintMethod::CancelPrimaryRoleRecoveryProposal(
            ..,
        ) => Some(AccessControllerCall::CancelRecovery {
            proposer: Proposer::Primary,
        }),
        AccessControllerBlueprintMethod::CancelRecoveryRoleRecoveryProposal(
            ..,
        ) => Some(AccessControllerCall::CancelRecovery {
            proposer: Proposer::Recovery,
        }),
        AccessControllerBlueprintMethod::LockPrimaryRole(..) => {
            Some(AccessControllerCall::LockPrimaryRole)
        }
        AccessControllerBlueprintMethod::UnlockPrimaryRole(..) => {
            Some(AccessControllerCall::UnlockPrimaryRole)
        }
        _ => return None,
    };

    Some((receiver, call))
}

fn recovery_proposal(
    rule_set: &ManifestRuleSet,
    timed_recovery_delay_in_minutes: Option<u32>,
) -> Option<RecoveryProposal> {
    Some(RecoveryProposal {
        rule_set: typed_rule_set(rule_set)?,
        timed_recovery_delay_in_minutes,
    })
}

fn static_access_controller(
    address: &ResolvedManifestAddress<GlobalAddress>,
) -> Option<ComponentAddress> {
    let ResolvedManifestAddress::Static { static_address } = address else {
        return None;
    };
    ComponentAddress::try_from(*static_address).ok()
}
