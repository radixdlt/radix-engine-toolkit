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

pub fn traverse(
    manifest: &impl ReadableManifest,
    worktop_changes: Option<&WorktopChanges<'_>>,
    visitor: &mut impl ManifestAnalysisVisitor,
) -> Result<(), TraverserError> {
    // This is a store of all of the named addresses that we encounter in the
    // manifest. It records the ManifestNamedAddress and maps it to the id of
    // blueprint seen in the manifest.
    let mut id_allocator = ManifestIdAllocator::new();
    let mut named_address_store = NamedAddressStore::new();

    // If the worktop changes, which is information we get from dynamic analysis
    // is available for this manifest then we compute the invocation IO which is
    // the composition of static and dynamic analysis into a single object type.
    let indexed_invocation_io = match worktop_changes {
        Some(worktop_changes) => {
            Some(IndexedInvocationIo::compute(manifest, worktop_changes)?)
        }
        None => None,
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
        /* Pre-Visitor Processing */
        // If we encounter an address allocation we store it in the named
        // address store.
        if let Some(AllocateGlobalAddress {
            package_address,
            blueprint_name,
        }) = instruction.as_allocate_global_address()
        {
            let named_address = id_allocator.new_address_id();
            let blueprint_id =
                BlueprintId::new(package_address, blueprint_name);
            named_address_store.insert(named_address, blueprint_id);
        }

        // Attempting to create a typed invocation from the instruction.
        let maybe_typed_invocation =
            resolve_typed_invocation(&instruction, &named_address_store)?;

        // Attempting to get the instruction's invocation IO.
        let maybe_invocation_io =
            indexed_invocation_io
                .as_ref()
                .and_then(|indexed_invocation_io| {
                    indexed_invocation_io.for_instruction(&instruction_index)
                });

        /* Visitor Processing */
        // If the visitor is no longer accepting anymore instructions then we
        // do not need to call it and can break out of the instructions iterator
        if !visitor.validity_state().is_visitor_accepting_instructions() {
            break;
        }
        visitor.on_instruction(
            &named_address_store,
            &instruction,
            &instruction_index,
            maybe_typed_invocation.as_ref(),
            maybe_invocation_io,
        );
    }

    Ok(())
}

fn resolve_typed_invocation(
    instruction: &GroupedInstruction,
    named_address_store: &NamedAddressStore,
) -> Result<Option<TypedManifestNativeInvocation>, TraverserError> {
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
            TypedManifestNativeInvocation::from_function_invocation(
                &blueprint_id,
                function_name,
                args,
            )
            .map_err(Into::into)
        }
        GroupedInstruction::InvocationInstructions(
            InvocationInstructions::CallDirectVaultMethod(
                CallDirectVaultMethod {
                    address,
                    method_name,
                    args,
                },
            ),
        ) => TypedManifestNativeInvocation::from_direct_method_invocation(
            &ResolvedDynamicAddress::StaticAddress(*address),
            method_name,
            args,
        )
        .map_err(Into::into),
        GroupedInstruction::InvocationInstructions(
            InvocationInstructions::CallMethod(CallMethod {
                address,
                method_name,
                args,
            }),
        ) => {
            let resolved_address = match address {
                ManifestGlobalAddress::Static(address) => {
                    ResolvedDynamicAddress::StaticAddress(*address)
                }
                ManifestGlobalAddress::Named(address) => {
                    let blueprint_id = named_address_store
                        .get(address)
                        .ok_or(TraverserError::InvalidNamedAddress(*address))?;
                    ResolvedDynamicAddress::BlueprintResolvedFromNamedAddress(
                        blueprint_id.clone(),
                    )
                }
            };
            TypedManifestNativeInvocation::from_method_invocation(
                &resolved_address,
                ModuleId::Main,
                method_name,
                args,
            )
            .map_err(Into::into)
        }
        GroupedInstruction::InvocationInstructions(
            InvocationInstructions::CallMetadataMethod(CallMetadataMethod {
                address,
                method_name,
                args,
            }),
        ) => {
            let resolved_address = match address {
                ManifestGlobalAddress::Static(address) => {
                    ResolvedDynamicAddress::StaticAddress(*address)
                }
                ManifestGlobalAddress::Named(address) => {
                    let blueprint_id = named_address_store
                        .get(address)
                        .ok_or(TraverserError::InvalidNamedAddress(*address))?;
                    ResolvedDynamicAddress::BlueprintResolvedFromNamedAddress(
                        blueprint_id.clone(),
                    )
                }
            };
            TypedManifestNativeInvocation::from_method_invocation(
                &resolved_address,
                ModuleId::Metadata,
                method_name,
                args,
            )
            .map_err(Into::into)
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
            let resolved_address = match address {
                ManifestGlobalAddress::Static(address) => {
                    ResolvedDynamicAddress::StaticAddress(*address)
                }
                ManifestGlobalAddress::Named(address) => {
                    let blueprint_id = named_address_store
                        .get(address)
                        .ok_or(TraverserError::InvalidNamedAddress(*address))?;
                    ResolvedDynamicAddress::BlueprintResolvedFromNamedAddress(
                        blueprint_id.clone(),
                    )
                }
            };
            TypedManifestNativeInvocation::from_method_invocation(
                &resolved_address,
                ModuleId::RoleAssignment,
                method_name,
                args,
            )
            .map_err(Into::into)
        }
        GroupedInstruction::InvocationInstructions(
            InvocationInstructions::CallRoyaltyMethod(CallRoyaltyMethod {
                address,
                method_name,
                args,
            }),
        ) => {
            let resolved_address = match address {
                ManifestGlobalAddress::Static(address) => {
                    ResolvedDynamicAddress::StaticAddress(*address)
                }
                ManifestGlobalAddress::Named(address) => {
                    let blueprint_id = named_address_store
                        .get(address)
                        .ok_or(TraverserError::InvalidNamedAddress(*address))?;
                    ResolvedDynamicAddress::BlueprintResolvedFromNamedAddress(
                        blueprint_id.clone(),
                    )
                }
            };
            TypedManifestNativeInvocation::from_method_invocation(
                &resolved_address,
                ModuleId::Royalty,
                method_name,
                args,
            )
            .map_err(Into::into)
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
    InvalidNamedAddress(ManifestNamedAddress),
    InvocationIoError(InvocationIoError),
    TypedManifestNativeInvocationError(TypedManifestNativeInvocationError),
}

impl From<TypedManifestNativeInvocationError> for TraverserError {
    fn from(v: TypedManifestNativeInvocationError) -> Self {
        Self::TypedManifestNativeInvocationError(v)
    }
}

impl From<InvocationIoError> for TraverserError {
    fn from(v: InvocationIoError) -> Self {
        Self::InvocationIoError(v)
    }
}
