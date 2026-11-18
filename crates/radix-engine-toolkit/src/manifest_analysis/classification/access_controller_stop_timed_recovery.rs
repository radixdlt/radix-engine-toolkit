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

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct AccessControllerStopTimedRecoveryAnalyzer(
    AccessControllerStopTimedRecoveryAnalyzerOutput,
);

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct AccessControllerStopTimedRecoveryAnalyzerOutput {
    pub access_controllers: Vec<ComponentAddress>,
}

impl ManifestStaticAnalyzer for AccessControllerStopTimedRecoveryAnalyzer {
    type Initializer = ();
    type Output = AccessControllerStopTimedRecoveryAnalyzerOutput;

    type PermissionState =
        CallbackPermissionState<PermissionStateStaticCallback>;

    type RequirementState = AccessControllerStopTimedRecoveryRequirementState;

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
        self.0
    }

    fn process_instruction(&mut self, context: InstructionContext<'_>) {
        let InstructionContext::InvocationInstruction {
            typed_native_invocation:
                Some(TypedNativeInvocation {
                    receiver:
                     ManifestInvocationReceiver::GlobalMethod(
                            ResolvedManifestAddress::Static {
                                static_address: ac_address,
                            },
                    ),

                    invocation:
                    TypedManifestNativeInvocation::AccessControllerBlueprintInvocation(
                        AccessControllerBlueprintInvocation::Method(
                            AccessControllerBlueprintMethod::StopTimedRecovery(..)
                        )
                    )
                }),
            ..
        } = context
        else {
            return;
        };

        let ac_address = ComponentAddress::try_from(*ac_address).expect(
            "Must succeed since the typed invocation conversion succeeded",
        );

        self.0.access_controllers.push(ac_address);
    }
}

/// Requires at least one timed recovery to be stopped, and every recovery
/// proposal cancelled in the manifest to belong to an access controller whose
/// timed recovery is also stopped in it.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct AccessControllerStopTimedRecoveryRequirementState {
    stopped_access_controllers:
        IndexSet<ResolvedManifestAddress<GlobalAddress>>,
    cancelled_access_controllers:
        IndexSet<ResolvedManifestAddress<GlobalAddress>>,
}

impl ManifestAnalyzerRequirementState
    for AccessControllerStopTimedRecoveryRequirementState
{
    fn requirement_state(&self) -> RequirementState {
        let all_cancellations_are_stopped = self
            .cancelled_access_controllers
            .is_subset(&self.stopped_access_controllers);

        if !self.stopped_access_controllers.is_empty()
            && all_cancellations_are_stopped
        {
            RequirementState::Fulfilled
        } else {
            RequirementState::CurrentlyUnfulfilled
        }
    }

    fn process_instruction(&mut self, context: InstructionContext<'_>) {
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
            return;
        };

        match method {
            AccessControllerBlueprintMethod::StopTimedRecovery(..) => {
                self.stopped_access_controllers.insert(receiver.clone());
            }
            AccessControllerBlueprintMethod::CancelPrimaryRoleRecoveryProposal(..)
            | AccessControllerBlueprintMethod::CancelRecoveryRoleRecoveryProposal(..) => {
                self.cancelled_access_controllers.insert(receiver.clone());
            }
            _ => {}
        }
    }
}

fn is_instruction_permitted(context: InstructionContext<'_>) -> bool {
    matches!(
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
                            // Stopping the timed recovery
                            | AccessControllerBlueprintMethod::StopTimedRecovery(..)
                            // Cancelling the stopped recovery proposal in the
                            // same transaction. The requirement state checks
                            // that it targets an access controller whose timed
                            // recovery is stopped.
                            | AccessControllerBlueprintMethod::CancelPrimaryRoleRecoveryProposal(..)
                            | AccessControllerBlueprintMethod::CancelRecoveryRoleRecoveryProposal(..),
                        ),
                    ),
            }),
            ..
        }
    )
}
