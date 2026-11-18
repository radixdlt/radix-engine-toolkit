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
pub struct AccessControllerConfirmTimedRecoveryAnalyzer(
    AccessControllerConfirmTimedRecoveryOutput,
);

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct AccessControllerConfirmTimedRecoveryOutput {
    pub access_controllers: Vec<ComponentAddress>,
}

impl ManifestStaticAnalyzer for AccessControllerConfirmTimedRecoveryAnalyzer {
    type Initializer = ();
    type Output = AccessControllerConfirmTimedRecoveryOutput;

    type PermissionState =
        CallbackPermissionState<PermissionStateStaticCallback>;

    type RequirementState =
        AnyOfRequirement<(AccessControllerConfirmTimedRecovery,)>;

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
                            AccessControllerBlueprintMethod::TimedConfirmRecovery(..)
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
                            // Confirm timed recovery
                            | AccessControllerBlueprintMethod::TimedConfirmRecovery(..),
                        ),
                    ),
            }),
            ..
        }
    )
}
