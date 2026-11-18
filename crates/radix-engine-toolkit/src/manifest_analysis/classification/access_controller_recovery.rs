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

/// Classifies manifests that initiate the recovery of one or more access
/// controllers.
///
/// The output lists the access controllers for which a recovery is initiated.
/// The manifest may also confirm that recovery in the same transaction (quick
/// or timed confirmation), and may cancel or stop an older recovery proposal
/// that the new one replaces. Those instructions are permitted, but they don't
/// add access controllers to the output.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct AccessControllerRecoveryAnalyzer(AccessControllerRecoveryOutput);

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct AccessControllerRecoveryOutput {
    pub access_controllers: Vec<ComponentAddress>,
}

impl ManifestStaticAnalyzer for AccessControllerRecoveryAnalyzer {
    type Initializer = ();
    type Output = AccessControllerRecoveryOutput;

    type PermissionState =
        CallbackPermissionState<PermissionStateStaticCallback>;

    type RequirementState = AnyOfRequirement<(
        AccessControllerInitiateRecoveryAsPrimary,
        AccessControllerInitiateRecoveryAsRecovery,
    )>;

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
                            AccessControllerBlueprintMethod::InitiateRecoveryAsPrimary(..)
                        )
                    ) | TypedManifestNativeInvocation::AccessControllerBlueprintInvocation(
                        AccessControllerBlueprintInvocation::Method(
                            AccessControllerBlueprintMethod::InitiateRecoveryAsRecovery(..)
                        )
                    ),
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

fn is_instruction_permitted(context: InstructionContext<'_>) -> bool {
    // Lets the manifest be used as a subintent. Transaction manifests can't
    // contain these instructions, so this doesn't affect them.
    let is_permitted_subintent_instruction = matches!(
        context.instruction(),
        GroupedInstruction::SubintentInstructions(
            SubintentInstructions::YieldToParent(..)
                | SubintentInstructions::VerifyParent(..)
        )
    );

    is_permitted_subintent_instruction || matches!(
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
                    )
                    // Updating the metadata of the recovered entity, e.g. its
                    // owner keys.
                    | TypedManifestNativeInvocation::MetadataBlueprintInvocation(
                        MetadataBlueprintInvocation::Method(
                            MetadataBlueprintMethod::Set(..),
                        ),
                    ),
            }),
            ..
        }
    )
}
