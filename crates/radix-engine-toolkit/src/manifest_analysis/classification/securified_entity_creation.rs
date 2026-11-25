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

/// Analyzes manifests that create accounts or identities and put each one's
/// owner badge into an access controller they create.
///
/// Such a manifest is also classified `General`, which permits the function
/// calls it makes, so consumers should prefer the most specific class:
/// `SecurifiedEntityCreation` is the one that shows the rules each entity is
/// created under.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct SecurifiedEntityCreationAnalyzer {
    output: SecurifiedEntityCreationOutput,
    entities_by_owner_badge_id: IndexMap<NonFungibleLocalId, GlobalAddress>,
}

/// An entity created in the manifest, and the rules of the access controller
/// created to hold its owner badge.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreatedSecurifiedEntity {
    pub entity_address: GlobalAddress,
    pub rule_set: RuleSet,
    pub timed_recovery_delay_in_minutes: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct SecurifiedEntityCreationOutput {
    pub created_entities: Vec<CreatedSecurifiedEntity>,
}

impl ManifestStaticAnalyzer for SecurifiedEntityCreationAnalyzer {
    type Initializer = ();
    type Output = ();
    type PermissionState =
        CallbackPermissionState<PermissionStateStaticCallback>;

    type RequirementState = SecurifiedEntityCreationStaticRequirementState;

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
        // The entities created and their owner badges are only known from the
        // dynamic invocation IO, see the dynamic analyzer.
    }
}

impl ManifestDynamicAnalyzer for SecurifiedEntityCreationAnalyzer {
    type Output = SecurifiedEntityCreationOutput;

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

        match &typed_invocation.invocation {
            TypedManifestNativeInvocation::AccountBlueprintInvocation(
                AccountBlueprintInvocation::Function(
                    AccountBlueprintFunction::Create(..),
                ),
            )
            | TypedManifestNativeInvocation::IdentityBlueprintInvocation(
                IdentityBlueprintInvocation::Function(
                    IdentityBlueprintFunction::Create(..),
                ),
            ) => {
                let Some(owner_badge_id) = owner_badge_local_id(
                    &dynamic_analysis_invocation_io.output,
                ) else {
                    return;
                };
                let Some(entity_address) =
                    entity_address_of_owner_badge(owner_badge_id)
                else {
                    return;
                };

                self.entities_by_owner_badge_id
                    .insert(owner_badge_id.clone(), entity_address);
            }

            TypedManifestNativeInvocation::AccessControllerBlueprintInvocation(
                AccessControllerBlueprintInvocation::Function(
                    AccessControllerBlueprintFunction::Create(input),
                ),
            ) => {
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

                self.output.created_entities.push(CreatedSecurifiedEntity {
                    entity_address,
                    rule_set,
                    timed_recovery_delay_in_minutes: input
                        .timed_recovery_delay_in_minutes,
                });
            }

            _ => {}
        }
    }
}

/// The address of the entity an owner badge belongs to: the engine names the
/// owner badge of an account or identity it creates after its address.
fn entity_address_of_owner_badge(
    owner_badge_id: &NonFungibleLocalId,
) -> Option<GlobalAddress> {
    let NonFungibleLocalId::Bytes(bytes) = owner_badge_id else {
        return None;
    };
    let node_id = <[u8; NodeId::LENGTH]>::try_from(bytes.value()).ok()?;
    GlobalAddress::try_from(node_id).ok()
}

/// The static requirements for classifying a manifest as a securified entity
/// creation.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct SecurifiedEntityCreationStaticRequirementState {
    pending_entity_count: usize,
    paired_entity_count: usize,
    invalid_manifest: bool,
}

impl ManifestAnalyzerRequirementState
    for SecurifiedEntityCreationStaticRequirementState
{
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
                AccountBlueprintInvocation::Function(
                    AccountBlueprintFunction::Create(..),
                ),
            )
            | TypedManifestNativeInvocation::IdentityBlueprintInvocation(
                IdentityBlueprintInvocation::Function(
                    IdentityBlueprintFunction::Create(..),
                ),
            ) => self.pending_entity_count += 1,
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
                // The rules must be possible to show to the user.
                self.invalid_manifest |=
                    typed_rule_set(&input.rule_set).is_none();
                // Each access controller holds exactly one of the owner badges
                // of the entities created. The manifest can't get an owner
                // badge from anywhere else, so it's one of theirs.
                self.invalid_manifest |=
                    !is_one_owner_badge(&static_analysis_invocation_io.input);
            }
            _ => {}
        }
    }
}

/// Creating accounts or identities, the access controllers holding their owner
/// badges, and paying the fee. Nothing else is permitted, since the output
/// wouldn't show it: not creating an entity with `create_advanced`, whose
/// owner role isn't an owner badge, nor setting metadata, such as the owner
/// keys, nor topping up the recovery fee vault of an access controller.
fn is_instruction_permitted(context: InstructionContext<'_>) -> bool {
    is_permitted_invocation(context)
        || is_fee_payment(context)
        || is_permitted_non_invocation(context)
}

fn is_permitted_invocation(context: InstructionContext<'_>) -> bool {
    matches!(
        context,
        InstructionContext::InvocationInstruction {
            typed_native_invocation: Some(TypedNativeInvocation {
                receiver: ManifestInvocationReceiver::BlueprintFunction(..),
                invocation:
                    TypedManifestNativeInvocation::AccountBlueprintInvocation(
                        AccountBlueprintInvocation::Function(
                            AccountBlueprintFunction::Create(..),
                        ),
                    )
                    | TypedManifestNativeInvocation::IdentityBlueprintInvocation(
                        IdentityBlueprintInvocation::Function(
                            IdentityBlueprintFunction::Create(..),
                        ),
                    )
                    | TypedManifestNativeInvocation::AccessControllerBlueprintInvocation(
                        AccessControllerBlueprintInvocation::Function(
                            AccessControllerBlueprintFunction::Create(..),
                        ),
                    ),
            }),
            ..
        }
    )
}

fn is_permitted_non_invocation(context: InstructionContext<'_>) -> bool {
    match context {
        InstructionContext::NonInvocationInstruction {
            instruction, ..
        } => match instruction {
            // Taking the owner badge that creating the entity put on the
            // worktop, to pass it to the access controller created.
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
        },
        InstructionContext::InvocationInstruction { .. } => false,
    }
}
