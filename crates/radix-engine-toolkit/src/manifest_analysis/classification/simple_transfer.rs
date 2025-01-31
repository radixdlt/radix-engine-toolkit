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

pub type SimpleTransferAnalyzer = SimpleTransferStateMachine;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SimpleTransferStateMachine {
    #[default]
    InitialState,
    AccessControllerProofCreated,
    FeeLockPerformed,
    ResourcesWithdrawn,
    ResourcesInBucket,
    DepositPerformed,
    InvalidState,
}

impl ManifestStaticAnalyzer for SimpleTransferStateMachine {
    type Initializer = ();
    type Output = ();
    type PermissionState = Self;
    type RequirementState = Self;

    fn new(
        _: Self::Initializer,
    ) -> (Self, Self::PermissionState, Self::RequirementState) {
        Default::default()
    }

    fn output(self) -> Self::Output {}

    fn process_instruction(&mut self, context: InstructionContext<'_>) {
        self.transition(context);
    }
}

impl ManifestAnalyzerRequirementState for SimpleTransferStateMachine {
    fn requirement_state(&self) -> RequirementState {
        match self {
            SimpleTransferStateMachine::DepositPerformed => {
                RequirementState::Fulfilled
            }
            SimpleTransferStateMachine::InitialState
            | SimpleTransferStateMachine::AccessControllerProofCreated
            | SimpleTransferStateMachine::FeeLockPerformed
            | SimpleTransferStateMachine::ResourcesWithdrawn
            | SimpleTransferStateMachine::ResourcesInBucket => {
                RequirementState::CurrentlyUnfulfilled
            }
            SimpleTransferStateMachine::InvalidState => {
                RequirementState::PermanentlyUnfulfilled
            }
        }
    }

    fn process_instruction(&mut self, context: InstructionContext<'_>) {
        self.transition(context);
    }
}

impl ManifestAnalyzerPermissionState for SimpleTransferStateMachine {
    fn all_instructions_permitted(&self) -> bool {
        !matches!(self, Self::InvalidState)
    }

    fn process_instruction(&mut self, context: InstructionContext<'_>) {
        self.transition(context);
    }
}

impl SimpleTransferStateMachine {
    pub fn transition(&mut self, context: InstructionContext<'_>) {
        let next_state = match (&self, context) {
            // Initial State -> Access Controller Proof State transition.
            (
                Self::InitialState,
                InstructionContext::InvocationInstruction {
                    typed_native_invocation:
                        Some(TypedNativeInvocation {
                            invocation:
                                TypedManifestNativeInvocation::AccessControllerBlueprintInvocation(
                                    AccessControllerBlueprintInvocation::Method(
                                        AccessControllerBlueprintMethod::CreateProof(..),
                                    ),
                                ),
                            ..
                        }),
                    ..
                },
            ) => Self::AccessControllerProofCreated,
            // (Initial State | Access Controller Proof Created) -> Lock Fee
            // State transition.
            (
                Self::InitialState | Self::AccessControllerProofCreated,
                InstructionContext::InvocationInstruction {
                    typed_native_invocation:
                        Some(TypedNativeInvocation {
                            invocation:
                                TypedManifestNativeInvocation::AccountBlueprintInvocation(
                                    AccountBlueprintInvocation::Method(
                                        AccountBlueprintMethod::LockFee(..),
                                    ),
                                ),
                            ..
                        }),
                    ..
                },
            ) => Self::FeeLockPerformed,
            // Access Controller Proof Created -> Resources on Worktop
            (
                Self::AccessControllerProofCreated,
                InstructionContext::InvocationInstruction {
                    typed_native_invocation:
                        Some(TypedNativeInvocation {
                            invocation:
                                TypedManifestNativeInvocation::AccountBlueprintInvocation(
                                    AccountBlueprintInvocation::Method(
                                        AccountBlueprintMethod::LockFeeAndWithdraw(..)
                                        | AccountBlueprintMethod::LockFeeAndWithdrawNonFungibles(..),
                                    ),
                                ),
                            ..
                        }),
                    ..
                },
            ) => Self::ResourcesWithdrawn,
            // Initial State -> Resources on Worktop State transition.
            (
                Self::InitialState,
                InstructionContext::InvocationInstruction {
                    typed_native_invocation:
                        Some(TypedNativeInvocation {
                            invocation:
                                TypedManifestNativeInvocation::AccountBlueprintInvocation(
                                    AccountBlueprintInvocation::Method(
                                        AccountBlueprintMethod::Withdraw(..)
                                        | AccountBlueprintMethod::WithdrawNonFungibles(..)
                                        | AccountBlueprintMethod::LockFeeAndWithdraw(..)
                                        | AccountBlueprintMethod::LockFeeAndWithdrawNonFungibles(..),
                                    ),
                                ),
                            ..
                        }),
                    ..
                },
            ) => Self::ResourcesWithdrawn,
            // Fees Locked -> Resources on Worktop State transition.
            (
                Self::FeeLockPerformed,
                InstructionContext::InvocationInstruction {
                    typed_native_invocation:
                        Some(TypedNativeInvocation {
                            invocation:
                                TypedManifestNativeInvocation::AccountBlueprintInvocation(
                                    AccountBlueprintInvocation::Method(
                                        AccountBlueprintMethod::Withdraw(..)
                                        | AccountBlueprintMethod::WithdrawNonFungibles(..),
                                    ),
                                ),
                            ..
                        }),
                    ..
                },
            ) => Self::ResourcesWithdrawn,
            // Resources on Worktop -> Resources in Bucket State transition.
            (
                Self::ResourcesWithdrawn,
                InstructionContext::NonInvocationInstruction {
                    instruction:
                        GroupedInstruction::TakeFromWorktopInstructions(
                            TakeFromWorktopInstructions::TakeFromWorktop(..),
                        ),
                    ..
                },
            ) => Self::ResourcesInBucket,
            // Resources in Bucket -> Resources Deposited State transition.
            (
                Self::ResourcesInBucket,
                InstructionContext::InvocationInstruction {
                    typed_native_invocation:
                        Some(TypedNativeInvocation {
                            invocation:
                                TypedManifestNativeInvocation::AccountBlueprintInvocation(
                                    AccountBlueprintInvocation::Method(
                                        AccountBlueprintMethod::Deposit(..)
                                        | AccountBlueprintMethod::TryDepositOrAbort(..),
                                    ),
                                ),
                            ..
                        }),
                    ..
                },
            ) => Self::DepositPerformed,
            _ => Self::InvalidState,
        };
        *self = next_state;
    }
}
