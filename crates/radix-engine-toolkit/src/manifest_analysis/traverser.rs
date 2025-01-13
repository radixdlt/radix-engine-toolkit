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
    worktop_changes: Option<&WorktopChanges>,
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
    let static_invocation_io = static_analysis_invocation_io(manifest)?;
    let dynamic_invocation_io = worktop_changes.map(|worktop_changes| {
        dynamic_analysis_invocation_io(manifest, worktop_changes)
    });
    let invocation_io = InstructionIndexedInvocationIo::combine(
        &static_invocation_io,
        dynamic_invocation_io.as_ref(),
    );

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
        let invocation_io = invocation_io.for_instruction(&instruction_index);

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
            invocation_io,
            maybe_typed_invocation.as_ref(),
        );
    }

    Ok(())
}

fn static_analysis_invocation_io(
    manifest: &impl ReadableManifest,
) -> Result<
    IndexMap<InstructionIndex, InvocationIo<TrackedResources>>,
    TraverserError,
> {
    // The initial worktop state is only unknown if the manifest is a
    // subintent manifest. Otherwise, in the case of a v1 or v2 manifest the
    // initial worktop state is known to be zero since they can't be used as
    // subintents and can't be yielded into.
    let initial_worktop_state_is_unknown = manifest.is_subintent();
    let interpreter = StaticManifestInterpreter::new(
        ValidationRuleset::babylon_equivalent(),
        manifest,
    );
    let mut visitor =
        StaticResourceMovementsVisitor::new(initial_worktop_state_is_unknown);
    interpreter.validate_and_apply_visitor(&mut visitor)?;

    let invocation_io = visitor
        .output()
        .invocation_static_information
        .into_iter()
        .map(
            |(
                instruction_index,
                InvocationStaticInformation { input, output, .. },
            )| {
                let instruction_index = InstructionIndex::of(instruction_index);
                (instruction_index, InvocationIo { input, output })
            },
        )
        .collect::<IndexMap<_, _>>();
    Ok(invocation_io)
}

fn dynamic_analysis_invocation_io<'a>(
    manifest: &'a impl ReadableManifest,
    worktop_changes: &'a WorktopChanges,
) -> IndexMap<
    InstructionIndex,
    InvocationIo<
        IndexMap<&'a ResourceAddress, Vec<Tracked<ResourceQuantifier<'a>>>>,
    >,
> {
    let mut map = IndexMap::<
        _,
        InvocationIo<
            IndexMap<&'a ResourceAddress, Vec<Tracked<ResourceQuantifier<'a>>>>,
        >,
    >::new();
    let mut id_allocator = ManifestIdAllocator::new();
    let mut tracked_buckets = IndexMap::new();

    for (instruction_index, effect) in
        manifest.iter_instruction_effects().enumerate()
    {
        let instruction_index = InstructionIndex::of(instruction_index);

        match effect {
            // Bucket Creation
            ManifestInstructionEffect::CreateBucket { source_amount } => {
                let bucket_resource_address = source_amount.resource_address();
                let bucket = id_allocator.new_bucket_id();
                let bucket_quantifier = worktop_changes
                    .first_take(&instruction_index)
                    .map(ResourceQuantifier::from)
                    .unwrap_or(ResourceQuantifier::empty_static(
                        bucket_resource_address.is_fungible(),
                    ));
                let tracked_bucket_content = Tracked {
                    value: (bucket_resource_address, bucket_quantifier),
                    created_at: instruction_index,
                };
                tracked_buckets.insert(bucket, tracked_bucket_content);
            }
            // Bucket Consumption
            ManifestInstructionEffect::ConsumeBucket {
                consumed_bucket,
                destination,
            } => match destination {
                BucketDestination::Worktop | BucketDestination::Burned => {
                    let _ = tracked_buckets.swap_remove(&consumed_bucket);
                }
                BucketDestination::Invocation(..) => {
                    let tracked_bucket_contents = tracked_buckets
                        .swap_remove(&consumed_bucket)
                        .expect("Can't fail, the transaction committed successfully.");
                    map.entry(instruction_index)
                        .or_default()
                        .input
                        .entry(tracked_bucket_contents.0)
                        .or_default()
                        .push(tracked_bucket_contents.map(|(_, v)| v));
                }
            },
            ManifestInstructionEffect::Invocation { args, .. } => {
                // Handling the output.
                worktop_changes
                    .put_iterator(&instruction_index)
                    .map(|resource_specifier| {
                        (
                            resource_specifier.resource_address(),
                            ResourceQuantifier::from(resource_specifier),
                        )
                    })
                    .map(|resources| Tracked {
                        value: resources,
                        created_at: instruction_index,
                    })
                    .for_each(|output| {
                        map.entry(instruction_index)
                            .or_default()
                            .output
                            .entry(output.0)
                            .or_default()
                            .push(output.map(|(_, v)| v))
                    });

                // Handling the input.
                let indexed_value =
                    IndexedManifestValue::from_manifest_value(args);

                let buckets = indexed_value.buckets();
                let expressions = indexed_value.expressions();
                let has_entire_worktop_expression =
                    expressions.contains(&ManifestExpression::EntireWorktop);

                let buckets_tracked_resources = buckets.iter().map(|bucket| {
                    tracked_buckets.swap_remove(bucket).expect(
                        "Can't fail, the transaction committed successfully.",
                    )
                });
                let expression_tracked_resources = worktop_changes
                    .take_iterator(&instruction_index)
                    .map(|resource_specifier| {
                        (
                            resource_specifier.resource_address(),
                            ResourceQuantifier::from(resource_specifier),
                        )
                    })
                    .map(|resources| Tracked {
                        value: resources,
                        created_at: instruction_index,
                    });

                for tracked_invocation_input in buckets_tracked_resources {
                    map.entry(instruction_index)
                        .or_default()
                        .input
                        .entry(tracked_invocation_input.0)
                        .or_default()
                        .push(tracked_invocation_input.map(|(_, v)| v));
                }
                if has_entire_worktop_expression {
                    for tracked_invocation_input in expression_tracked_resources
                    {
                        map.entry(instruction_index)
                            .or_default()
                            .input
                            .entry(tracked_invocation_input.0)
                            .or_default()
                            .push(tracked_invocation_input.map(|(_, v)| v));
                    }
                }
            }
            // No effect on the resource movements
            ManifestInstructionEffect::CreateProof { .. }
            | ManifestInstructionEffect::ConsumeProof { .. }
            | ManifestInstructionEffect::CloneProof { .. }
            | ManifestInstructionEffect::DropManyProofs { .. }
            | ManifestInstructionEffect::CreateAddressAndReservation {
                ..
            }
            | ManifestInstructionEffect::ResourceAssertion { .. }
            | ManifestInstructionEffect::Verification { .. } => {}
        }
    }

    map
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
    StaticResourceMovementsError(StaticResourceMovementsError),
    TypedManifestNativeInvocationError(TypedManifestNativeInvocationError),
}

impl From<StaticResourceMovementsError> for TraverserError {
    fn from(v: StaticResourceMovementsError) -> Self {
        Self::StaticResourceMovementsError(v)
    }
}

impl From<TypedManifestNativeInvocationError> for TraverserError {
    fn from(v: TypedManifestNativeInvocationError) -> Self {
        Self::TypedManifestNativeInvocationError(v)
    }
}
