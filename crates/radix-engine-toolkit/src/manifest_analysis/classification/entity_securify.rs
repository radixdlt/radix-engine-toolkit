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

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct EntitySecurifyAnalyzer {
    output: EntitySecurifyOutput,
    entities_by_owner_badge_id: IndexMap<NonFungibleLocalId, GlobalAddress>,
}

/// A securify operation and the security structure proposed for the entity.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntitySecurifyOperation {
    pub entity_address: GlobalAddress,
    pub rule_set: RuleSet,
    pub timed_recovery_delay_in_minutes: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct EntitySecurifyOutput {
    pub securify_operations: Vec<EntitySecurifyOperation>,
}

impl ManifestStaticAnalyzer for EntitySecurifyAnalyzer {
    type Initializer = ();
    type Output = ();
    type PermissionState =
        CallbackPermissionState<PermissionStateStaticCallback>;

    type RequirementState = EntitySecurifyStaticRequirementState;

    fn new(
        _: Self::Initializer,
    ) -> (Self, Self::PermissionState, Self::RequirementState) {
        (
            Default::default(),
            CallbackPermissionState::new(is_instruction_permitted),
            Default::default(),
        )
    }

    fn output(self) -> Self::Output {}

    fn process_instruction(&mut self, _: InstructionContext<'_>) {
        // The static analyzer only determines whether the manifest matches the
        // entity securify classification. Operation extraction requires the
        // dynamic invocation IO and is performed by the dynamic analyzer.
    }
}

impl ManifestDynamicAnalyzer for EntitySecurifyAnalyzer {
    type Output = EntitySecurifyOutput;

    fn output(
        self,
    ) -> CombinedAnalysisOutput<
        <Self as ManifestStaticAnalyzer>::Output,
        <Self as ManifestDynamicAnalyzer>::Output,
    > {
        CombinedAnalysisOutput {
            static_analyzer_output: (),
            dynamic_analyzer_output: self.output,
        }
    }

    fn process_instruction(&mut self, context: InstructionContext<'_>) {
        let InstructionContext::InvocationInstruction {
            typed_native_invocation: Some(typed_invocation),
            dynamic_analysis_invocation_io: Some(dynamic_analysis_invocation_io),
            ..
        } = context
        else {
            return;
        };

        match typed_invocation {
            TypedNativeInvocation {
                receiver:
                    ManifestInvocationReceiver::GlobalMethod(
                        ResolvedManifestAddress::Static { static_address },
                    ),
                invocation:
                    TypedManifestNativeInvocation::AccountBlueprintInvocation(
                        AccountBlueprintInvocation::Method(
                            AccountBlueprintMethod::Securify(..),
                        ),
                    )
                    | TypedManifestNativeInvocation::IdentityBlueprintInvocation(
                        IdentityBlueprintInvocation::Method(
                            IdentityBlueprintMethod::Securify(..),
                        ),
                    ),
            }
            => {
                let Some(owner_badge_id) = owner_badge_local_id(
                    &dynamic_analysis_invocation_io.output,
                ) else {
                    return;
                };

                self.entities_by_owner_badge_id
                    .insert(owner_badge_id.clone(), *static_address);
            }

            TypedNativeInvocation {
                receiver: ManifestInvocationReceiver::BlueprintFunction(..),
                invocation:
                    TypedManifestNativeInvocation::AccessControllerBlueprintInvocation(
                        AccessControllerBlueprintInvocation::Function(
                            AccessControllerBlueprintFunction::Create(input),
                        ),
                    ),
            } => {
                // The static requirements only let an access controller be
                // created with exactly one owner badge.
                let Some(owner_badge_id) = owner_badge_local_id(
                    &dynamic_analysis_invocation_io.input,
                ) else {
                    return;
                };
                let Some(entity_address) = self
                    .entities_by_owner_badge_id
                    .swap_remove(owner_badge_id)
                else {
                    return;
                };
                // The static requirements reject rule sets that can't be
                // converted.
                let Some(rule_set) = typed_rule_set(&input.rule_set) else {
                    return;
                };

                self.output.securify_operations.push(
                    EntitySecurifyOperation {
                        entity_address,
                        rule_set,
                        timed_recovery_delay_in_minutes: input
                            .timed_recovery_delay_in_minutes,
                    },
                );
            }

            _ => return,
        }
    }
}

/// The static requirements for classifying a manifest as an entity securify.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct EntitySecurifyStaticRequirementState {
    pending_entity_count: usize,
    paired_entity_count: usize,
    invalid_manifest: bool,
}

impl ManifestAnalyzerRequirementState for EntitySecurifyStaticRequirementState {
    fn requirement_state(&self) -> RequirementState {
        if self.invalid_manifest {
            RequirementState::PermanentlyUnfulfilled
        } else if self.paired_entity_count > 0 && self.pending_entity_count == 0
        {
            RequirementState::Fulfilled
        } else {
            RequirementState::CurrentlyUnfulfilled
        }
    }

    fn process_instruction(&mut self, context: InstructionContext<'_>) {
        let InstructionContext::InvocationInstruction {
            typed_native_invocation: Some(typed_invocation),
            static_analysis_invocation_io,
            ..
        } = context
        else {
            return;
        };

        match &typed_invocation.invocation {
            TypedManifestNativeInvocation::AccountBlueprintInvocation(
                AccountBlueprintInvocation::Method(
                    AccountBlueprintMethod::Securify(..),
                ),
            )
            | TypedManifestNativeInvocation::IdentityBlueprintInvocation(
                IdentityBlueprintInvocation::Method(
                    IdentityBlueprintMethod::Securify(..),
                ),
            ) => {
                self.pending_entity_count += 1;
                // Only existing entities can be securified.
                self.invalid_manifest |= !matches!(
                    typed_invocation.receiver,
                    ManifestInvocationReceiver::GlobalMethod(
                        ResolvedManifestAddress::Static { .. }
                    )
                );
            }
            TypedManifestNativeInvocation::AccessControllerBlueprintInvocation(
                AccessControllerBlueprintInvocation::Function(
                    AccessControllerBlueprintFunction::Create(input),
                ),
            ) => {
                let Some(pending_entity_count) =
                    self.pending_entity_count.checked_sub(1)
                else {
                    self.invalid_manifest = true;
                    return;
                };

                self.pending_entity_count = pending_entity_count;
                self.paired_entity_count += 1;
                // The proposed rules must be possible to show to the user.
                self.invalid_manifest |= typed_rule_set(&input.rule_set).is_none();
                // Each access controller controls exactly one of the entities
                // securified. Otherwise one created with several owner badges,
                // or with none, would still be counted as one entity's, and
                // the entities it controls wouldn't be in the output.
                self.invalid_manifest |=
                    !is_one_owner_badge(&static_analysis_invocation_io.input);
            }
            _ => {}
        }
    }
}

/// Securifying, creating the access controllers, and paying the fee. Nothing
/// else is permitted, since the output wouldn't show it: setting metadata,
/// such as the entities' owner keys, and topping up the recovery fee vault of
/// an access controller created here are each done in a transaction of their
/// own, the latter of the recovery fee class.
fn is_instruction_permitted(context: InstructionContext<'_>) -> bool {
    is_permitted_invocation(context) || is_permitted_non_invocation(context)
}

fn is_permitted_invocation(context: InstructionContext<'_>) -> bool {
    matches!(
        context,
        InstructionContext::InvocationInstruction {
            typed_native_invocation: Some(
                TypedNativeInvocation {
                    receiver: ManifestInvocationReceiver::GlobalMethod(..),
                    invocation:
                        TypedManifestNativeInvocation::AccountBlueprintInvocation(
                            AccountBlueprintInvocation::Method(
                                AccountBlueprintMethod::Securify(..)
                                // Fee payment
                                | AccountBlueprintMethod::LockFee(..)
                                | AccountBlueprintMethod::LockContingentFee(..)
                                | AccountBlueprintMethod::CreateProofOfAmount(..)
                                | AccountBlueprintMethod::CreateProofOfNonFungibles(..),
                            ),
                        )
                        | TypedManifestNativeInvocation::IdentityBlueprintInvocation(
                            IdentityBlueprintInvocation::Method(
                                IdentityBlueprintMethod::Securify(..),
                            ),
                        )
                        // Normally used to create a proof for fee payment when
                        // the fee payer is already securified.
                        | TypedManifestNativeInvocation::AccessControllerBlueprintInvocation(
                            AccessControllerBlueprintInvocation::Method(
                                AccessControllerBlueprintMethod::CreateProof(..),
                            ),
                        ),
                }
                | TypedNativeInvocation {
                    receiver: ManifestInvocationReceiver::BlueprintFunction(..),
                    invocation:
                        TypedManifestNativeInvocation::AccessControllerBlueprintInvocation(
                            AccessControllerBlueprintInvocation::Function(
                                AccessControllerBlueprintFunction::Create(..),
                            ),
                        ),
                },
            ),
            ..
        }
    )
}

fn is_permitted_non_invocation(context: InstructionContext<'_>) -> bool {
    match context {
        InstructionContext::NonInvocationInstruction {
            instruction, ..
        } => {
            match instruction {
                // Taking the owner badge that securifying put on the worktop,
                // to pass it to the access controller created.
                GroupedInstruction::TakeFromWorktopInstructions(..) => true,
                // Address allocation for the new access controller.
                GroupedInstruction::AddressAllocationInstructions(
                    AddressAllocationInstructions::AllocateGlobalAddress(
                        AllocateGlobalAddress {
                            package_address,
                            blueprint_name,
                        },
                    ),
                ) => {
                    *package_address == ACCESS_CONTROLLER_PACKAGE
                        && blueprint_name == ACCESS_CONTROLLER_BLUEPRINT
                }
                _ => false,
            }
        }
        InstructionContext::InvocationInstruction { .. } => false,
    }
}
