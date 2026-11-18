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

use super::common::*;
use crate::internal_prelude::*;

/// Classifies manifests that initiate the recovery of one or more access
/// controllers.
///
/// The output lists the recovery proposals initiated in the manifest. The
/// manifest may also confirm a recovery (quick or timed), cancel or stop an
/// older proposal that the new one replaces, and top up the recovery fee vault
/// with XRD, but only on access controllers whose recovery it initiates.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct AccessControllerRecoveryAnalyzer {
    recovery_proposals:
        IndexMap<(ComponentAddress, Proposer), (RuleSet, Option<u32>)>,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct AccessControllerRecoveryOutput {
    pub recovery_proposals: Vec<AccessControllerRecoveryProposal>,
}

/// A recovery proposal initiated on an access controller.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AccessControllerRecoveryProposal {
    pub access_controller: ComponentAddress,
    /// The role that initiated the recovery.
    pub proposer: Proposer,
    /// The proposed rules that replace the access controller's current ones.
    pub rule_set: RuleSet,
    pub timed_recovery_delay_in_minutes: Option<u32>,
}

impl ManifestStaticAnalyzer for AccessControllerRecoveryAnalyzer {
    type Initializer = ();
    type Output = AccessControllerRecoveryOutput;

    type PermissionState =
        CallbackPermissionState<PermissionStateStaticCallback>;

    type RequirementState = AccessControllerRecoveryRequirementState;

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
        AccessControllerRecoveryOutput {
            recovery_proposals: self
                .recovery_proposals
                .into_iter()
                .map(
                    |(
                        (access_controller, proposer),
                        (rule_set, timed_recovery_delay_in_minutes),
                    )| AccessControllerRecoveryProposal {
                        access_controller,
                        proposer,
                        rule_set,
                        timed_recovery_delay_in_minutes,
                    },
                )
                .collect(),
        }
    }

    fn process_instruction(&mut self, context: InstructionContext<'_>) {
        let Some((access_controller, proposer, rule_set, delay)) =
            initiated_recovery(context)
        else {
            return;
        };
        let ResolvedManifestAddress::Static { static_address } =
            access_controller
        else {
            return;
        };
        let Some(rule_set) = typed_rule_set(rule_set) else {
            return;
        };
        let access_controller = ComponentAddress::try_from(*static_address)
            .expect(
                "Must succeed since the typed invocation conversion succeeded",
            );

        self.recovery_proposals
            .insert((access_controller, proposer), (rule_set, delay));
    }
}

/// The requirements for classifying a manifest as an access controller
/// recovery:
///
/// * At least one recovery is initiated.
/// * Every confirmation, cancellation, stop and recovery fee contribution
///   targets an access controller whose recovery is initiated in the manifest.
/// * XRD is withdrawn from an account only to contribute it to the recovery
///   fee vault.
/// * Every proposed rule set can be shown to the user, i.e. it doesn't refer
///   to anything that only exists in the manifest.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct AccessControllerRecoveryRequirementState {
    initiated_access_controllers:
        IndexSet<ResolvedManifestAddress<GlobalAddress>>,
    other_targeted_access_controllers:
        IndexSet<ResolvedManifestAddress<GlobalAddress>>,
    has_xrd_withdrawal: bool,
    has_xrd_contribution: bool,
    invalid_manifest: bool,
}

impl ManifestAnalyzerRequirementState
    for AccessControllerRecoveryRequirementState
{
    fn requirement_state(&self) -> RequirementState {
        if self.invalid_manifest {
            RequirementState::PermanentlyUnfulfilled
        } else if !self.initiated_access_controllers.is_empty()
            && self
                .other_targeted_access_controllers
                .is_subset(&self.initiated_access_controllers)
            && self.has_xrd_withdrawal == self.has_xrd_contribution
        {
            RequirementState::Fulfilled
        } else {
            RequirementState::CurrentlyUnfulfilled
        }
    }

    fn process_instruction(&mut self, context: InstructionContext<'_>) {
        if let Some((access_controller, _, rule_set, _)) =
            initiated_recovery(context)
        {
            self.initiated_access_controllers
                .insert(access_controller.clone());
            self.invalid_manifest |= !matches!(
                access_controller,
                ResolvedManifestAddress::Static { .. }
            ) || typed_rule_set(rule_set).is_none();
        } else if let Some(access_controller) =
            xrd_recovery_fee_contribution_receiver(context)
        {
            self.has_xrd_contribution = true;
            self.other_targeted_access_controllers
                .insert(access_controller.clone());
        } else if is_xrd_withdraw(context) {
            self.has_xrd_withdrawal = true;
        } else if let InstructionContext::InvocationInstruction {
            typed_native_invocation:
                Some(TypedNativeInvocation {
                    receiver: ManifestInvocationReceiver::GlobalMethod(receiver),
                    invocation:
                        TypedManifestNativeInvocation::AccessControllerBlueprintInvocation(
                            AccessControllerBlueprintInvocation::Method(
                                AccessControllerBlueprintMethod::QuickConfirmPrimaryRoleRecoveryProposal(..)
                                | AccessControllerBlueprintMethod::QuickConfirmRecoveryRoleRecoveryProposal(..)
                                | AccessControllerBlueprintMethod::TimedConfirmRecovery(..)
                                | AccessControllerBlueprintMethod::CancelPrimaryRoleRecoveryProposal(..)
                                | AccessControllerBlueprintMethod::CancelRecoveryRoleRecoveryProposal(..)
                                | AccessControllerBlueprintMethod::StopTimedRecovery(..),
                            ),
                        ),
                }),
            ..
        } = context
        {
            self.other_targeted_access_controllers
                .insert(receiver.clone());
        }
    }
}

/// If the instruction initiates a recovery, returns the access controller, the
/// proposer, the proposed rule set and the timed recovery delay.
fn initiated_recovery<'a>(
    context: InstructionContext<'a>,
) -> Option<(
    &'a ResolvedManifestAddress<GlobalAddress>,
    Proposer,
    &'a ManifestRuleSet,
    Option<u32>,
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

    match method {
        AccessControllerBlueprintMethod::InitiateRecoveryAsPrimary(
            AccessControllerInitiateRecoveryAsPrimaryManifestInput {
                rule_set,
                timed_recovery_delay_in_minutes,
            },
        ) => Some((
            receiver,
            Proposer::Primary,
            rule_set,
            *timed_recovery_delay_in_minutes,
        )),
        AccessControllerBlueprintMethod::InitiateRecoveryAsRecovery(
            AccessControllerInitiateRecoveryAsRecoveryManifestInput {
                rule_set,
                timed_recovery_delay_in_minutes,
            },
        ) => Some((
            receiver,
            Proposer::Recovery,
            rule_set,
            *timed_recovery_delay_in_minutes,
        )),
        _ => None,
    }
}

fn is_instruction_permitted(context: InstructionContext<'_>) -> bool {
    is_permitted_subintent_instruction(context)
        || is_account_or_identity_metadata_set(context)
        || is_xrd_withdraw(context)
        || xrd_recovery_fee_contribution_receiver(context).is_some()
        || matches!(
            context,
            InstructionContext::NonInvocationInstruction {
                // Taking the withdrawn XRD from the worktop to contribute it
                // to the recovery fee vault.
                instruction: GroupedInstruction::TakeFromWorktopInstructions(..),
                ..
            }
        )
        || matches!(
            context,
            InstructionContext::InvocationInstruction {
                typed_native_invocation: Some(TypedNativeInvocation {
                    receiver: ManifestInvocationReceiver::GlobalMethod(..),
                    invocation:
                        // Fee payment
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
                                // Fee payment. The proof is needed when the fee
                                // payer is itself a securified account.
                                AccessControllerBlueprintMethod::CreateProof(..)
                                | AccessControllerBlueprintMethod::LockRecoveryFee(..)
                                // Initiating the recovery
                                | AccessControllerBlueprintMethod::InitiateRecoveryAsPrimary(..)
                                | AccessControllerBlueprintMethod::InitiateRecoveryAsRecovery(..)
                                // Confirming the recovery in the same transaction
                                | AccessControllerBlueprintMethod::QuickConfirmPrimaryRoleRecoveryProposal(..)
                                | AccessControllerBlueprintMethod::QuickConfirmRecoveryRoleRecoveryProposal(..)
                                | AccessControllerBlueprintMethod::TimedConfirmRecovery(..)
                                // Replacing an existing recovery proposal
                                | AccessControllerBlueprintMethod::CancelPrimaryRoleRecoveryProposal(..)
                                | AccessControllerBlueprintMethod::CancelRecoveryRoleRecoveryProposal(..)
                                | AccessControllerBlueprintMethod::StopTimedRecovery(..),
                            ),
                        ),
                }),
                ..
            }
        )
}
