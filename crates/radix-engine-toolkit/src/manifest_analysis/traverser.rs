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

pub fn static_analyzer_traverse<A: ManifestStaticAnalyzer>(
    manifest: &impl ReadableManifest,
    analyzer_initializer: A::Initializer,
) -> Result<StaticAnalyzerState<A>, TraverserError> {
    // Instantiating the analyzer based on the initializer passed to this
    // function.
    let mut analyzer_state =
        StaticAnalyzerState::<A>::new(analyzer_initializer);

    // This named address store will be used to the named address to blueprint
    // id mapping of the named addresses.
    let mut named_address_store = NamedAddressStore::new();

    let invocation_static_information = {
        // The initial worktop state is only unknown if the manifest is a
        // subintent manifest. Otherwise, in the case of a v1 or v2 manifest the
        // initial worktop state is known to be zero since they can't be used as
        // subintents and can't be yielded into.
        let initial_worktop_state_is_unknown = manifest.is_subintent();
        let interpreter = StaticManifestInterpreter::new(
            ValidationRuleset::babylon_equivalent(),
            manifest,
        );
        let mut visitor = StaticResourceMovementsVisitor::new(
            initial_worktop_state_is_unknown,
        );
        interpreter.validate_and_apply_visitor(&mut visitor)?;
        let output = visitor.output();
        output.invocation_static_information
    };

    // Iterating over all of the instructions in the manifest and processing
    // them in preparation for calling the visitor.
    for (instruction_index, instruction) in manifest
        .iter_cloned_instructions()
        .map(GroupedInstruction::from)
        .enumerate()
    {
        let instruction_index = InstructionIndex::of(instruction_index);

        let static_analysis_invocation_io = InvocationIo {
            input: invocation_static_information
                .get(instruction_index.value())
                .map(|value| value.input.clone())
                .unwrap_or_default(),
            output: invocation_static_information
                .get(instruction_index.value())
                .map(|value| value.output.clone())
                .unwrap_or_default(),
        };

        /* Pre-Visitor Processing */
        // If we encounter an address allocation we store it in the named
        // address store.
        if let Some(AllocateGlobalAddress {
            package_address,
            blueprint_name,
        }) = instruction.as_allocate_global_address()
        {
            let blueprint_id =
                BlueprintId::new(package_address, blueprint_name);
            named_address_store.insert(blueprint_id);
        }

        // Attempting to create a typed invocation from the instruction.
        let typed_native_invocation =
            resolve_typed_invocation(&instruction, &named_address_store)?.map(
                |(receiver, invocation)| TypedNativeInvocation {
                    receiver,
                    invocation,
                },
            );

        let context = if instruction.belongs_to_invocation_instructions() {
            AnalysisContext::InvocationInstruction {
                named_address_store: &named_address_store,
                instruction_index: &instruction_index,
                instruction: &instruction,
                typed_native_invocation: typed_native_invocation.as_ref(),
                static_analysis_invocation_io: &static_analysis_invocation_io,
                dynamic_analysis_invocation_io: None,
            }
        } else {
            AnalysisContext::NonInvocationInstruction {
                named_address_store: &named_address_store,
                instruction_index: &instruction_index,
                instruction: &instruction,
            }
        };

        /* Visitor processing */
        analyzer_state
            .static_permission_state
            .process_instruction(context);
        if !analyzer_state
            .static_permission_state
            .all_instructions_permitted()
        {
            break;
        }
        analyzer_state
            .static_requirement_state
            .process_instruction(context);
        ManifestStaticAnalyzer::process_instruction(
            &mut analyzer_state.analyzer,
            context,
        );
    }

    Ok(analyzer_state)
}

pub fn dynamic_analyzer_traverse<A: ManifestDynamicAnalyzer>(
    manifest: &impl ReadableManifest,
    worktop_changes: &WorktopChanges,
    analyzer_initializer: A::Initializer,
) -> Result<DynamicAnalyzerState<A>, TraverserError> {
    // Instantiating the analyzer based on the initializer passed to this
    // function.
    let mut analyzer_state =
        DynamicAnalyzerState::<A>::new(analyzer_initializer);

    // This named address store will be used to the named address to blueprint
    // id mapping of the named addresses.
    let mut named_address_store = NamedAddressStore::new();

    // If the worktop changes, which is information we get from dynamic analysis
    // is available for this manifest then we compute the invocation IO which is
    // the composition of static and dynamic analysis into a single object type.
    let indexed_invocation_io =
        IndexedInvocationIo::compute(manifest, worktop_changes)?;

    let invocation_static_information = {
        // The initial worktop state is only unknown if the manifest is a
        // subintent manifest. Otherwise, in the case of a v1 or v2 manifest the
        // initial worktop state is known to be zero since they can't be used as
        // subintents and can't be yielded into.
        let initial_worktop_state_is_unknown = manifest.is_subintent();
        let interpreter = StaticManifestInterpreter::new(
            ValidationRuleset::babylon_equivalent(),
            manifest,
        );
        let mut visitor = StaticResourceMovementsVisitor::new(
            initial_worktop_state_is_unknown,
        );
        interpreter.validate_and_apply_visitor(&mut visitor)?;
        let output = visitor.output();
        output.invocation_static_information
    };

    // Iterating over all of the instructions in the manifest and processing
    // them in preparation for calling the visitor.
    let instructions_iterator = manifest
        .iter_cloned_instructions()
        .enumerate()
        .map(|(i, instruction)| {
            (
                InstructionIndex::of(i),
                GroupedInstruction::from(instruction),
            )
        });
    for (instruction_index, instruction) in instructions_iterator {
        let static_analysis_invocation_io = InvocationIo {
            input: invocation_static_information
                .get(instruction_index.value())
                .map(|value| value.input.clone())
                .unwrap_or_default(),
            output: invocation_static_information
                .get(instruction_index.value())
                .map(|value| value.output.clone())
                .unwrap_or_default(),
        };

        /* Pre-Visitor Processing */
        // If we encounter an address allocation we store it in the named
        // address store.
        if let Some(AllocateGlobalAddress {
            package_address,
            blueprint_name,
        }) = instruction.as_allocate_global_address()
        {
            let blueprint_id =
                BlueprintId::new(package_address, blueprint_name);
            named_address_store.insert(blueprint_id);
        }

        // Attempting to create a typed invocation from the instruction.
        let typed_native_invocation =
            resolve_typed_invocation(&instruction, &named_address_store)?.map(
                |(receiver, invocation)| TypedNativeInvocation {
                    receiver,
                    invocation,
                },
            );

        // Attempting to get the dynamic invocation io of this invocation.
        let invocation_io =
            indexed_invocation_io.for_instruction(&instruction_index);

        let context = if instruction.belongs_to_invocation_instructions() {
            AnalysisContext::InvocationInstruction {
                named_address_store: &named_address_store,
                instruction_index: &instruction_index,
                instruction: &instruction,
                typed_native_invocation: typed_native_invocation.as_ref(),
                static_analysis_invocation_io: &static_analysis_invocation_io,
                dynamic_analysis_invocation_io: Some(invocation_io),
            }
        } else {
            AnalysisContext::NonInvocationInstruction {
                named_address_store: &named_address_store,
                instruction_index: &instruction_index,
                instruction: &instruction,
            }
        };

        /* Visitor processing */
        analyzer_state
            .static_permission_state
            .process_instruction(context);
        if !analyzer_state
            .static_permission_state
            .all_instructions_permitted()
        {
            break;
        }
        analyzer_state
            .static_requirement_state
            .process_instruction(context);
        ManifestStaticAnalyzer::process_instruction(
            &mut analyzer_state.analyzer,
            context,
        );
        ManifestDynamicAnalyzer::process_instruction(
            &mut analyzer_state.analyzer,
            context,
        );
    }

    Ok(analyzer_state)
}

pub struct StaticAnalyzerState<A: ManifestStaticAnalyzer> {
    pub analyzer: A,
    pub static_permission_state: <A as ManifestStaticAnalyzer>::PermissionState,
    pub static_requirement_state:
        <A as ManifestStaticAnalyzer>::RequirementState,
}

impl<A: ManifestStaticAnalyzer> StaticAnalyzerState<A> {
    fn new(initializer: A::Initializer) -> Self {
        let (analyzer, static_permission_state, static_requirement_state) =
            <A as ManifestStaticAnalyzer>::new(initializer);
        Self {
            analyzer,
            static_permission_state,
            static_requirement_state,
        }
    }
}

pub struct DynamicAnalyzerState<A: ManifestDynamicAnalyzer> {
    pub analyzer: A,
    pub static_permission_state: <A as ManifestStaticAnalyzer>::PermissionState,
    pub static_requirement_state:
        <A as ManifestStaticAnalyzer>::RequirementState,
}

impl<A: ManifestDynamicAnalyzer> DynamicAnalyzerState<A> {
    fn new(initializer: A::Initializer) -> Self {
        let (analyzer, static_permission_state, static_requirement_state) =
            <A as ManifestStaticAnalyzer>::new(initializer);
        Self {
            analyzer,
            static_permission_state,
            static_requirement_state,
        }
    }
}

/// Resolves a [`GroupedInstruction`] into [`TypedManifestNativeInvocation`]
/// if the given instruction is an invocation. Otherwise, [`None`] is returned.
fn resolve_typed_invocation(
    instruction: &GroupedInstruction,
    named_address_store: &NamedAddressStore,
) -> Result<
    Option<(ManifestInvocationReceiver, TypedManifestNativeInvocation)>,
    TraverserError,
> {
    match instruction {
        GroupedInstruction::InvocationInstructions(
            InvocationInstructions::CallFunction(CallFunction {
                package_address: ManifestPackageAddress::Static(package_address),
                blueprint_name,
                function_name,
                args,
            }),
        ) => {
            let blueprint_id =
                BlueprintId::new(package_address, blueprint_name);
            let receiver = ManifestInvocationReceiver::BlueprintFunction(
                blueprint_id.clone(),
            );
            TypedManifestNativeInvocation::from_function_invocation(
                &blueprint_id,
                function_name,
                args,
            )
            .map_err(Into::into)
            .map(|typed_invocation| {
                typed_invocation
                    .map(|typed_invocation| (receiver, typed_invocation))
            })
        }
        GroupedInstruction::InvocationInstructions(
            InvocationInstructions::CallDirectVaultMethod(
                CallDirectVaultMethod {
                    address,
                    method_name,
                    args,
                },
            ),
        ) => {
            let receiver = ManifestInvocationReceiver::DirectAccess(*address);
            TypedManifestNativeInvocation::from_direct_method_invocation(
                &ResolvedDynamicAddress::StaticAddress(*address),
                method_name,
                args,
            )
            .map_err(Into::into)
            .map(|typed_invocation| {
                typed_invocation
                    .map(|typed_invocation| (receiver, typed_invocation))
            })
        }
        GroupedInstruction::InvocationInstructions(
            InvocationInstructions::CallMethod(CallMethod {
                address,
                method_name,
                args,
            }),
        ) => {
            let resolved_address =
                ResolvedManifestAddress::from_manifest_global_address(
                    address,
                    named_address_store,
                )
                .ok_or(TraverserError::InvalidNamedAddress)?;
            let receiver = ManifestInvocationReceiver::GlobalMethod(
                resolved_address.clone(),
            );
            TypedManifestNativeInvocation::from_method_invocation(
                &resolved_address.into(),
                ModuleId::Main,
                method_name,
                args,
            )
            .map_err(Into::into)
            .map(|typed_invocation| {
                typed_invocation
                    .map(|typed_invocation| (receiver, typed_invocation))
            })
        }
        GroupedInstruction::InvocationInstructions(
            InvocationInstructions::CallMetadataMethod(CallMetadataMethod {
                address,
                method_name,
                args,
            }),
        ) => {
            let resolved_address =
                ResolvedManifestAddress::from_manifest_global_address(
                    address,
                    named_address_store,
                )
                .ok_or(TraverserError::InvalidNamedAddress)?;
            let receiver = ManifestInvocationReceiver::GlobalMethod(
                resolved_address.clone(),
            );
            TypedManifestNativeInvocation::from_method_invocation(
                &resolved_address.into(),
                ModuleId::Metadata,
                method_name,
                args,
            )
            .map_err(Into::into)
            .map(|typed_invocation| {
                typed_invocation
                    .map(|typed_invocation| (receiver, typed_invocation))
            })
        }
        GroupedInstruction::InvocationInstructions(
            InvocationInstructions::CallRoleAssignmentMethod(
                CallRoleAssignmentMethod {
                    address,
                    method_name,
                    args,
                },
            ),
        ) => {
            let resolved_address =
                ResolvedManifestAddress::from_manifest_global_address(
                    address,
                    named_address_store,
                )
                .ok_or(TraverserError::InvalidNamedAddress)?;
            let receiver = ManifestInvocationReceiver::GlobalMethod(
                resolved_address.clone(),
            );
            TypedManifestNativeInvocation::from_method_invocation(
                &resolved_address.into(),
                ModuleId::RoleAssignment,
                method_name,
                args,
            )
            .map_err(Into::into)
            .map(|typed_invocation| {
                typed_invocation
                    .map(|typed_invocation| (receiver, typed_invocation))
            })
        }
        GroupedInstruction::InvocationInstructions(
            InvocationInstructions::CallRoyaltyMethod(CallRoyaltyMethod {
                address,
                method_name,
                args,
            }),
        ) => {
            let resolved_address =
                ResolvedManifestAddress::from_manifest_global_address(
                    address,
                    named_address_store,
                )
                .ok_or(TraverserError::InvalidNamedAddress)?;
            let receiver = ManifestInvocationReceiver::GlobalMethod(
                resolved_address.clone(),
            );
            TypedManifestNativeInvocation::from_method_invocation(
                &resolved_address.into(),
                ModuleId::Royalty,
                method_name,
                args,
            )
            .map_err(Into::into)
            .map(|typed_invocation| {
                typed_invocation
                    .map(|typed_invocation| (receiver, typed_invocation))
            })
        }
        GroupedInstruction::TakeFromWorktopInstructions(..)
        | GroupedInstruction::ReturnToWorktopInstructions(..)
        | GroupedInstruction::AssertionInstructions(..)
        | GroupedInstruction::ProofInstructions(..)
        | GroupedInstruction::SubintentInstructions(..)
        | GroupedInstruction::AddressAllocationInstructions(..)
        | GroupedInstruction::BurnResourceInstructions(..)
        | GroupedInstruction::InvocationInstructions(
            InvocationInstructions::CallFunction(CallFunction {
                package_address: ManifestPackageAddress::Named(..),
                ..
            }),
        ) => Ok(None),
    }
}

#[derive(Debug)]
pub enum TraverserError {
    InvalidNamedAddress,
    StaticResourceMovementsError(Box<StaticResourceMovementsError>),
    TypedManifestNativeInvocationError(Box<TypedManifestNativeInvocationError>),
}

impl From<StaticResourceMovementsError> for TraverserError {
    fn from(v: StaticResourceMovementsError) -> Self {
        Self::StaticResourceMovementsError(Box::new(v))
    }
}

impl From<Box<StaticResourceMovementsError>> for TraverserError {
    fn from(v: Box<StaticResourceMovementsError>) -> Self {
        Self::StaticResourceMovementsError(v)
    }
}

impl From<TypedManifestNativeInvocationError> for TraverserError {
    fn from(v: TypedManifestNativeInvocationError) -> Self {
        Self::TypedManifestNativeInvocationError(Box::new(v))
    }
}
