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
pub struct EntitySecurifyAnalyzer {
    output: EntitySecurifyOutput,
    entities_by_owner_badge_id:
        IndexMap<NonFungibleLocalId, ManifestGlobalAddress>,
}

/// A securify operation and the security structure proposed for the entity.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntitySecurifyOperation {
    pub entity_address: ManifestGlobalAddress,
    pub rule_set: ManifestRuleSet,
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
                receiver: ManifestInvocationReceiver::GlobalMethod(receiver),
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
                    .insert(owner_badge_id.clone(), receiver.into());
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

                self.output.securify_operations.push(
                    EntitySecurifyOperation {
                        entity_address,
                        rule_set: input.rule_set.clone(),
                        timed_recovery_delay_in_minutes: input
                            .timed_recovery_delay_in_minutes,
                    },
                );
            }

            _ => return,
        }
    }
}

fn owner_badge_local_id(
    invocation_io_items: &InvocationIoItems,
) -> Option<&NonFungibleLocalId> {
    let mut owner_badge_ids = invocation_io_items
        .items_iter()
        .filter_map(|item| match item {
            InvocationIoItem::NonFungible(resource_address, ids)
                if resource_address == &ACCOUNT_OWNER_BADGE
                    || resource_address == &IDENTITY_OWNER_BADGE =>
            {
                Some(ids)
            }
            _ => None,
        })
        .flat_map(|ids| ids.iter());

    let owner_badge_id = owner_badge_ids.next()?;
    owner_badge_ids.next().is_none().then_some(owner_badge_id)
}

/// The static requirements for classifying a manifest as an entity securify.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct EntitySecurifyStaticRequirementState {
    pending_entity_count: usize,
    paired_entity_count: usize,
    created_access_controller_addresses: IndexSet<ManifestNamedAddress>,
    has_xrd_withdrawal: bool,
    has_xrd_contribution: bool,
    invalid_manifest: bool,
}

impl ManifestAnalyzerRequirementState for EntitySecurifyStaticRequirementState {
    fn requirement_state(&self) -> RequirementState {
        if self.invalid_manifest {
            RequirementState::PermanentlyUnfulfilled
        } else if self.paired_entity_count > 0
            && self.pending_entity_count == 0
            && self.has_xrd_withdrawal == self.has_xrd_contribution
        {
            RequirementState::Fulfilled
        } else {
            RequirementState::CurrentlyUnfulfilled
        }
    }

    fn process_instruction(&mut self, context: InstructionContext<'_>) {
        let InstructionContext::InvocationInstruction {
            typed_native_invocation: Some(typed_invocation),
            named_address_store,
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

                if let Some(address_reservation) = input.address_reservation {
                    // AllocateGlobalAddress assigns the reservation and named
                    // address for an allocation the same index.
                    let named_address =
                        ManifestNamedAddress(address_reservation.0);
                    let access_controller_blueprint = BlueprintId::new(
                        &ACCESS_CONTROLLER_PACKAGE,
                        ACCESS_CONTROLLER_BLUEPRINT,
                    );

                    if named_address_store.get(&named_address)
                        == Some(&access_controller_blueprint)
                    {
                        self.created_access_controller_addresses
                            .insert(named_address);
                    }
                }
            }
            TypedManifestNativeInvocation::AccountBlueprintInvocation(
                AccountBlueprintInvocation::Method(
                    AccountBlueprintMethod::Withdraw(
                        AccountWithdrawManifestInput {
                            resource_address: ManifestResourceAddress::Static(
                                XRD,
                            ),
                            ..
                        },
                    ),
                ),
            ) => {
                self.has_xrd_withdrawal = true;
            }
            TypedManifestNativeInvocation::AccessControllerBlueprintInvocation(
                AccessControllerBlueprintInvocation::Method(
                    AccessControllerBlueprintMethod::ContributeRecoveryFee(..),
                ),
            ) => {
                self.has_xrd_contribution = true;

                let is_created_access_controller = matches!(
                    &typed_invocation.receiver,
                    ManifestInvocationReceiver::GlobalMethod(
                        ResolvedManifestAddress::Named {
                            named_address,
                            ..
                        }
                    ) if self
                        .created_access_controller_addresses
                        .contains(named_address)
                );
                self.invalid_manifest |= !is_created_access_controller;
            }
            _ => {}
        }
    }
}

fn is_instruction_permitted(context: InstructionContext<'_>) -> bool {
    is_permitted_invocation(context)
        || is_xrd_withdraw(context)
        || is_xrd_contribution_to_named_access_controller(context)
        || is_permitted_non_invocation(context)
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
                        )
                        // Setting metadata such as the owner keys of the
                        // securified entity.
                        | TypedManifestNativeInvocation::MetadataBlueprintInvocation(
                            MetadataBlueprintInvocation::Method(
                                MetadataBlueprintMethod::Set(..),
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
                // These are the instructions for creating proofs with the owner
                // badge that was put on the worktop after securifying the
                // entity. They are mainly needed for setting the owner_keys
                // metadata on the entity.
                GroupedInstruction::ProofInstructions(..)
                | GroupedInstruction::TakeFromWorktopInstructions(..) => true,
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

fn is_xrd_withdraw(context: InstructionContext<'_>) -> bool {
    matches!(
        context,
        InstructionContext::InvocationInstruction {
            typed_native_invocation: Some(TypedNativeInvocation {
                invocation:
                    TypedManifestNativeInvocation::AccountBlueprintInvocation(
                        AccountBlueprintInvocation::Method(
                            AccountBlueprintMethod::Withdraw(
                                AccountWithdrawManifestInput {
                                    resource_address:
                                        ManifestResourceAddress::Static(XRD),
                                    ..
                                },
                            ),
                        ),
                    ),
                ..
            }),
            ..
        }
    )
}

fn is_xrd_contribution_to_named_access_controller(
    context: InstructionContext<'_>,
) -> bool {
    let InstructionContext::InvocationInstruction {
        typed_native_invocation:
            Some(TypedNativeInvocation {
                receiver:
                    ManifestInvocationReceiver::GlobalMethod(
                        ResolvedManifestAddress::Named { .. },
                    ),
                invocation:
                    TypedManifestNativeInvocation::AccessControllerBlueprintInvocation(
                        AccessControllerBlueprintInvocation::Method(
                            AccessControllerBlueprintMethod::ContributeRecoveryFee(..),
                        ),
                    ),
            }),
        static_analysis_invocation_io,
        ..
    } = context
    else {
        return false;
    };

    let specified_resources =
        static_analysis_invocation_io.input.specified_resources();
    !specified_resources.is_empty()
        && specified_resources
            .keys()
            .all(|resource_address| resource_address == &XRD)
        && !static_analysis_invocation_io
            .input
            .unspecified_resources()
            .may_be_present()
}
