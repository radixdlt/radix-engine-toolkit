pub mod manifest_summary {
    use crate::sbor::indexed_manifest_value::*;
    use crate::transaction_types::*;
    use crate::utils::*;
    use radix_engine_interface::blueprints::account::*;
    use transaction::prelude::*;

    pub fn traverse(
        callbacks: &mut [&mut dyn ManifestSummaryCallback],
        manifest: &TransactionManifestV1,
    ) {
        let TransactionManifestV1 { instructions, .. } = manifest;
        for (instruction_index, instruction) in instructions.iter().enumerate()
        {
            on_instruction(callbacks, instruction, instruction_index);
        }
        on_finish(callbacks, instructions.len());
    }

    pub(super) fn on_instruction(
        callbacks: &mut [&mut dyn ManifestSummaryCallback],
        instruction: &InstructionV1,
        instruction_index: usize,
    ) {
        // At the beginning of an instruction, call the on_instruction callback
        callbacks.iter_mut().for_each(|callback| {
            callback.on_instruction(instruction, instruction_index)
        });

        // Notify the callbacks of the created created account proofs
        handle_on_create_proof(callbacks, instruction);

        // Notify callbacks of the global entities encountered
        for node_id in IndexedManifestValue::from_typed(instruction)
            .static_addresses()
            .into_iter()
            .map(|item| *item.as_node_id())
        {
            let Ok(global_address) = GlobalAddress::try_from(node_id) else {
                continue;
            };
            callbacks.iter_mut().for_each(|callback| {
                callback.on_global_entity_encounter(global_address)
            });
        }
    }

    fn handle_on_create_proof(
        callbacks: &mut [&mut dyn ManifestSummaryCallback],
        instruction: &InstructionV1,
    ) {
        if let InstructionV1::CallMethod {
            address: dynamic_address @ DynamicGlobalAddress::Static(address),
            method_name,
            args,
        } = instruction
        {
            if !is_account(dynamic_address) {
                return;
            }

            if method_name == ACCOUNT_CREATE_PROOF_OF_AMOUNT_IDENT {
                if let Some(AccountCreateProofOfAmountInput {
                    resource_address,
                    ..
                }) = to_manifest_type(args)
                {
                    callbacks.iter_mut().for_each(|callback| {
                        callback.on_create_proof(&resource_address)
                    });
                }
            } else if method_name == ACCOUNT_CREATE_PROOF_OF_NON_FUNGIBLES_IDENT
            {
                if let Some(AccountCreateProofOfNonFungiblesInput {
                    resource_address,
                    ..
                }) = to_manifest_type(args)
                {
                    callbacks.iter_mut().for_each(|callback| {
                        callback.on_create_proof(&resource_address)
                    });
                }
            }
        }
    }

    pub(super) fn on_finish(
        callbacks: &mut [&mut dyn ManifestSummaryCallback],
        instructions_count: usize,
    ) {
        // After the iteration finishes inform the callbacks.
        callbacks
            .iter_mut()
            .for_each(|callback| callback.on_finish(instructions_count))
    }
}

pub mod execution_summary {
    use crate::sbor::indexed_manifest_value::IndexedManifestValue;
    use crate::transaction_types::*;
    use crate::utils::*;
    use radix_engine::system::system_modules::execution_trace::*;
    use radix_engine_interface::blueprints::account::*;
    use transaction::prelude::*;
    use transaction::validation::ManifestIdAllocator;

    pub fn traverse(
        callbacks: &mut [&mut dyn ExecutionSummaryCallback],
        manifest: &TransactionManifestV1,
        receipt: &TransactionTypesReceipt<'_>,
    ) {
        let TransactionManifestV1 { instructions, .. } = manifest;
        let mut id_allocator = ManifestIdAllocator::new();
        let mut bucket_tracker = Default::default();
        for (instruction_index, instruction) in instructions.iter().enumerate()
        {
            on_instruction(
                callbacks,
                instruction,
                instruction_index,
                receipt,
                &mut id_allocator,
                &mut bucket_tracker,
            );
        }
        on_finish(callbacks, instructions.len());
    }

    pub(super) fn on_instruction(
        callbacks: &mut [&mut dyn ExecutionSummaryCallback],
        instruction: &InstructionV1,
        instruction_index: usize,
        /* State */
        receipt: &TransactionTypesReceipt<'_>,
        id_allocator: &mut ManifestIdAllocator,
        bucket_tracker: &mut IndexMap<ManifestBucket, ResourceIndicator>,
    ) {
        // Calling the on_instruction handler of the `ManifestSummaryCallback`
        // to handle the manifest summary bits of this manifest and then we can
        // move on to the other aspects.
        super::manifest_summary::on_instruction(
            &mut callbacks
                .iter_mut()
                .map(|item| *item as &mut dyn ManifestSummaryCallback)
                .collect::<Vec<_>>(),
            instruction,
            instruction_index,
        );

        // Handling the instruction & Informing the callbacks
        handle_on_instruction(
            callbacks,
            instruction,
            instruction_index,
            receipt,
            bucket_tracker,
        );
        // Handling the account withdraws & Informing the callbacks.
        handle_account_withdraws(
            callbacks,
            instruction,
            instruction_index,
            receipt,
        );
        // Handling the account withdraws & Informing the callbacks.
        handle_account_deposits(
            callbacks,
            instruction,
            instruction_index,
            receipt,
            bucket_tracker,
        );
        // Handling and keeping track of the buckets.
        handle_buckets(
            instruction,
            instruction_index,
            receipt,
            id_allocator,
            bucket_tracker,
        );
    }

    pub(super) fn on_finish(
        callbacks: &mut [&mut dyn ExecutionSummaryCallback],
        instructions_count: usize,
    ) {
        super::manifest_summary::on_finish(
            &mut callbacks
                .iter_mut()
                .map(|item| *item as &mut dyn ManifestSummaryCallback)
                .collect::<Vec<_>>(),
            instructions_count,
        )
    }

    fn handle_on_instruction(
        callbacks: &mut [&mut dyn ExecutionSummaryCallback],
        instruction: &InstructionV1,
        instruction_index: usize,
        /* State */
        receipt: &TransactionTypesReceipt<'_>,
        bucket_tracker: &IndexMap<ManifestBucket, ResourceIndicator>,
    ) {
        let worktop_changes_entry = receipt
            .execution_trace()
            .worktop_changes()
            .get(&instruction_index)
            .map(Clone::clone)
            .unwrap_or_default();

        let inputs = {
            let mut inputs = worktop_changes_entry
                .iter()
                .filter_map(|item| {
                    if let WorktopChange::Take(resource_specifier) = item {
                        Some(resource_specifier.clone())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            match instruction {
                /* Sink methods - Input resources into the instruction */
                InstructionV1::CallFunction { args, .. }
                | InstructionV1::CallMethod { args, .. }
                | InstructionV1::CallRoyaltyMethod { args, .. }
                | InstructionV1::CallMetadataMethod { args, .. }
                | InstructionV1::CallRoleAssignmentMethod { args, .. }
                | InstructionV1::CallDirectVaultMethod { args, .. } => {
                    let manifest_value = IndexedManifestValue::from_typed(args);
                    let additional_resources = manifest_value
                        .buckets()
                        .into_iter()
                        .filter_map(|bucket| bucket_tracker.get(bucket))
                        .map(|resource_indicator| {
                            ResourceSpecifier::from(resource_indicator.clone())
                        });
                    inputs.extend(additional_resources)
                }
                InstructionV1::BurnResource { bucket_id }
                | InstructionV1::ReturnToWorktop { bucket_id } => {
                    if let Some(resource_indicator) =
                        bucket_tracker.get(bucket_id)
                    {
                        inputs.push(ResourceSpecifier::from(
                            resource_indicator.clone(),
                        ))
                    }
                }
                /* Non-sink methods */
                InstructionV1::TakeAllFromWorktop { .. }
                | InstructionV1::TakeFromWorktop { .. }
                | InstructionV1::TakeNonFungiblesFromWorktop { .. }
                | InstructionV1::AssertWorktopContainsAny { .. }
                | InstructionV1::AssertWorktopContains { .. }
                | InstructionV1::AssertWorktopContainsNonFungibles { .. }
                | InstructionV1::PopFromAuthZone
                | InstructionV1::PushToAuthZone { .. }
                | InstructionV1::CreateProofFromAuthZoneOfAmount { .. }
                | InstructionV1::CreateProofFromAuthZoneOfNonFungibles {
                    ..
                }
                | InstructionV1::CreateProofFromAuthZoneOfAll { .. }
                | InstructionV1::DropAuthZoneProofs
                | InstructionV1::DropAuthZoneRegularProofs
                | InstructionV1::DropAuthZoneSignatureProofs
                | InstructionV1::CreateProofFromBucketOfAmount { .. }
                | InstructionV1::CreateProofFromBucketOfNonFungibles {
                    ..
                }
                | InstructionV1::CreateProofFromBucketOfAll { .. }
                | InstructionV1::CloneProof { .. }
                | InstructionV1::DropProof { .. }
                | InstructionV1::DropNamedProofs
                | InstructionV1::DropAllProofs
                | InstructionV1::AllocateGlobalAddress { .. } => { /* No-Op */ }
            };

            inputs
        };
        let outputs = worktop_changes_entry
            .iter()
            .filter_map(|item| {
                if let WorktopChange::Put(resource_specifier) = item {
                    Some(resource_specifier.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        callbacks.iter_mut().for_each(|callback| {
            ExecutionSummaryCallback::on_instruction(
                *callback,
                instruction,
                instruction_index,
                &inputs,
                &outputs,
            )
        });
    }

    fn handle_account_withdraws(
        callbacks: &mut [&mut dyn ExecutionSummaryCallback],
        instruction: &InstructionV1,
        instruction_index: usize,
        receipt: &TransactionTypesReceipt<'_>,
    ) {
        let InstructionV1::CallMethod {
            address: dynamic_address @ DynamicGlobalAddress::Static(address),
            method_name,
            args,
        } = instruction
        else {
            return;
        };
        if !is_account(dynamic_address) {
            return;
        }

        let account =
            ComponentAddress::try_from(*address).expect("Must succeed");

        // In here we handle all of the cases, even the ones not allowed by the
        // callbacks so that this is blind to what the callbacks do internally
        // and just provides the information.
        let withdraw_information = if method_name == ACCOUNT_WITHDRAW_IDENT {
            let Some(AccountWithdrawInput {
                resource_address,
                amount,
            }) = to_manifest_type(&args)
            else {
                // TODO: Error? Panic?
                return;
            };

            if resource_address.is_fungible() {
                ResourceIndicator::Fungible(
                    resource_address,
                    FungibleResourceIndicator::Guaranteed(amount),
                )
            } else {
                ResourceIndicator::NonFungible(
                    resource_address,
                    NonFungibleResourceIndicator::ByAmount {
                        amount,
                        predicted_ids: Predicted {
                            value: predicted_non_fungible_ids_put_on_worktop(
                                receipt,
                                instruction_index,
                                resource_address,
                            )
                            .unwrap_or(Default::default()),
                            instruction_index,
                        },
                    },
                )
            }
        } else if method_name == ACCOUNT_LOCK_FEE_AND_WITHDRAW_IDENT {
            let Some(AccountLockFeeAndWithdrawInput {
                resource_address,
                amount,
                ..
            }) = to_manifest_type(&args)
            else {
                // TODO: Error? Panic?
                return;
            };

            if resource_address.is_fungible() {
                ResourceIndicator::Fungible(
                    resource_address,
                    FungibleResourceIndicator::Guaranteed(amount),
                )
            } else {
                ResourceIndicator::NonFungible(
                    resource_address,
                    NonFungibleResourceIndicator::ByAmount {
                        amount,
                        predicted_ids: Predicted {
                            value: predicted_non_fungible_ids_put_on_worktop(
                                receipt,
                                instruction_index,
                                resource_address,
                            )
                            .unwrap_or(Default::default()),
                            instruction_index,
                        },
                    },
                )
            }
        } else if method_name == ACCOUNT_WITHDRAW_NON_FUNGIBLES_IDENT {
            let Some(AccountWithdrawNonFungiblesInput {
                resource_address,
                ids,
                ..
            }) = to_manifest_type(&args)
            else {
                // TODO: Error? Panic?
                return;
            };

            ResourceIndicator::NonFungible(
                resource_address,
                NonFungibleResourceIndicator::ByIds(ids),
            )
        } else if method_name
            == ACCOUNT_LOCK_FEE_AND_WITHDRAW_NON_FUNGIBLES_IDENT
        {
            let Some(AccountLockFeeAndWithdrawNonFungiblesInput {
                resource_address,
                ids,
                ..
            }) = to_manifest_type(&args)
            else {
                // TODO: Error? Panic?
                return;
            };

            ResourceIndicator::NonFungible(
                resource_address,
                NonFungibleResourceIndicator::ByIds(ids),
            )
        } else {
            return;
        };

        callbacks.iter_mut().for_each(|callback| {
            callback.on_account_withdraw(&account, &withdraw_information)
        });
    }

    fn handle_account_deposits(
        callbacks: &mut [&mut dyn ExecutionSummaryCallback],
        instruction: &InstructionV1,
        instruction_index: usize,
        /* State */
        receipt: &TransactionTypesReceipt<'_>,
        bucket_tracker: &IndexMap<ManifestBucket, ResourceIndicator>,
    ) {
        if let InstructionV1::CallMethod {
            address: dynamic_address @ DynamicGlobalAddress::Static(address),
            method_name,
            args,
        } = instruction
        {
            if !is_account(dynamic_address) {
                return;
            }

            let account =
                ComponentAddress::try_from(*address).expect("Must succeed!");

            if crate::contains!(method_name => [
                ACCOUNT_DEPOSIT_IDENT,
                ACCOUNT_TRY_DEPOSIT_OR_ABORT_IDENT,
                ACCOUNT_TRY_DEPOSIT_OR_REFUND_IDENT,
                ACCOUNT_DEPOSIT_BATCH_IDENT,
                ACCOUNT_TRY_DEPOSIT_BATCH_OR_ABORT_IDENT,
                ACCOUNT_TRY_DEPOSIT_BATCH_OR_REFUND_IDENT,
            ]) {
                let indexed_manifest_value =
                    IndexedManifestValue::from_typed(args);

                // Handle all buckets
                for bucket in indexed_manifest_value.buckets() {
                    let Some(resource_indicator) = bucket_tracker.get(bucket)
                    else {
                        continue;
                    };
                    callbacks.iter_mut().for_each(|callback| {
                        callback
                            .on_account_deposit(&account, resource_indicator)
                    });
                }

                // Handle expressions
                if indexed_manifest_value
                    .expressions()
                    .contains(&ManifestExpression::EntireWorktop)
                {
                    if let Some(worktop_changes) = receipt
                        .execution_trace()
                        .worktop_changes()
                        .get(&instruction_index)
                    {
                        for resource_indicator in worktop_changes
                            .iter()
                            .filter_map(|worktop_changes| {
                                if let WorktopChange::Take(item) =
                                    worktop_changes
                                {
                                    Some(item)
                                } else {
                                    None
                                }
                            })
                            .map(
                                |resource_specifier| match resource_specifier {
                                    ResourceSpecifier::Amount(
                                        resource_address,
                                        amount,
                                    ) => ResourceIndicator::Fungible(
                                        *resource_address,
                                        FungibleResourceIndicator::Predicted(
                                            Predicted {
                                                value: *amount,
                                                instruction_index,
                                            },
                                        ),
                                    ),
                                    ResourceSpecifier::Ids(
                                        resource_address,
                                        ids,
                                    ) => ResourceIndicator::NonFungible(
                                        *resource_address,
                                        NonFungibleResourceIndicator::ByAll {
                                            predicted_amount: Predicted {
                                                value: usize_to_decimal(
                                                    ids.len(),
                                                ),
                                                instruction_index,
                                            },
                                            predicted_ids: Predicted {
                                                value: ids.clone(),
                                                instruction_index,
                                            },
                                        },
                                    ),
                                },
                            )
                        {
                            callbacks.iter_mut().for_each(|callback| {
                                callback.on_account_deposit(
                                    &account,
                                    &resource_indicator,
                                )
                            });
                        }
                    }
                }
            }
        }
    }

    fn handle_buckets(
        instruction: &InstructionV1,
        instruction_index: usize,
        /* State */
        receipt: &TransactionTypesReceipt<'_>,
        id_allocator: &mut ManifestIdAllocator,
        bucket_tracker: &mut IndexMap<ManifestBucket, ResourceIndicator>,
    ) {
        match instruction {
            /* Source */
            InstructionV1::TakeNonFungiblesFromWorktop {
                resource_address,
                ids,
            } => {
                let bucket = id_allocator.new_bucket_id();
                bucket_tracker.insert(
                    bucket,
                    ResourceIndicator::NonFungible(
                        *resource_address,
                        NonFungibleResourceIndicator::ByIds(
                            ids.iter().cloned().collect(),
                        ),
                    ),
                );
            }
            InstructionV1::TakeFromWorktop {
                resource_address,
                amount,
            } => {
                let bucket = id_allocator.new_bucket_id();
                let resource_indicator = if resource_address.is_fungible() {
                    ResourceIndicator::Fungible(
                        *resource_address,
                        FungibleResourceIndicator::Guaranteed(*amount),
                    )
                } else {
                    ResourceIndicator::NonFungible(
                        *resource_address,
                        NonFungibleResourceIndicator::ByAmount {
                            amount: *amount,
                            predicted_ids: Predicted {
                                value: predicted_non_fungible_ids_take_from_worktop(
                                    receipt,
                                    instruction_index,
                                    *resource_address,
                                )
                                .unwrap_or(Default::default()),
                                instruction_index,
                            },
                        },
                    )
                };
                bucket_tracker.insert(bucket, resource_indicator);
            }
            InstructionV1::TakeAllFromWorktop { resource_address } => {
                let bucket = id_allocator.new_bucket_id();
                let resource_indicator = if resource_address.is_fungible() {
                    ResourceIndicator::Fungible(
                        *resource_address,
                        FungibleResourceIndicator::Predicted(Predicted {
                            value: predicted_fungible_amount_take_from_worktop(
                                receipt,
                                instruction_index,
                                *resource_address,
                            )
                            .unwrap_or_default(),
                            instruction_index,
                        }),
                    )
                } else {
                    let predicted_ids =
                        predicted_non_fungible_ids_take_from_worktop(
                            receipt,
                            instruction_index,
                            *resource_address,
                        )
                        .unwrap_or(Default::default());
                    ResourceIndicator::NonFungible(
                        *resource_address,
                        NonFungibleResourceIndicator::ByAll {
                            predicted_amount: Predicted {
                                value: usize_to_decimal(predicted_ids.len()),
                                instruction_index,
                            },
                            predicted_ids: Predicted {
                                value: predicted_ids,
                                instruction_index,
                            },
                        },
                    )
                };
                bucket_tracker.insert(bucket, resource_indicator);
            }
            /* Sink */
            InstructionV1::ReturnToWorktop { bucket_id }
            | InstructionV1::BurnResource { bucket_id } => {
                // TODO: Do we want to check that the bucket was actually
                // present and then removed?
                bucket_tracker.remove(bucket_id);
            }
            InstructionV1::CallFunction { args, .. }
            | InstructionV1::CallMethod { args, .. }
            | InstructionV1::CallRoyaltyMethod { args, .. }
            | InstructionV1::CallMetadataMethod { args, .. }
            | InstructionV1::CallRoleAssignmentMethod { args, .. }
            | InstructionV1::CallDirectVaultMethod { args, .. } => {
                let manifest_value = IndexedManifestValue::from_typed(args);
                for bucket in manifest_value.buckets() {
                    // TODO: Do we want to check that the bucket was actually
                    // present and then removed?
                    bucket_tracker.remove(bucket);
                }
            }
            /* Neither */
            InstructionV1::AssertWorktopContainsAny { .. }
            | InstructionV1::AssertWorktopContains { .. }
            | InstructionV1::AssertWorktopContainsNonFungibles { .. }
            | InstructionV1::PopFromAuthZone
            | InstructionV1::PushToAuthZone { .. }
            | InstructionV1::CreateProofFromAuthZoneOfAmount { .. }
            | InstructionV1::CreateProofFromAuthZoneOfNonFungibles { .. }
            | InstructionV1::CreateProofFromAuthZoneOfAll { .. }
            | InstructionV1::DropAuthZoneProofs
            | InstructionV1::DropAuthZoneRegularProofs
            | InstructionV1::DropAuthZoneSignatureProofs
            | InstructionV1::CreateProofFromBucketOfAmount { .. }
            | InstructionV1::CreateProofFromBucketOfNonFungibles { .. }
            | InstructionV1::CreateProofFromBucketOfAll { .. }
            | InstructionV1::CloneProof { .. }
            | InstructionV1::DropProof { .. }
            | InstructionV1::DropNamedProofs
            | InstructionV1::DropAllProofs
            | InstructionV1::AllocateGlobalAddress { .. } => { /* No-op */ }
        }
    }

    fn predicted_fungible_amount_take_from_worktop(
        receipt: &TransactionTypesReceipt<'_>,
        instruction_index: usize,
        resource_address: ResourceAddress,
    ) -> Option<Decimal> {
        receipt
            .execution_trace()
            .worktop_changes()
            .entry(instruction_index)
            .or_default()
            .iter()
            .filter_map(|worktop_change| match worktop_change {
                WorktopChange::Take(ResourceSpecifier::Amount(
                    worktop_change_resource_address,
                    amount,
                )) if *worktop_change_resource_address == resource_address => {
                    Some(*amount)
                }
                _ => None,
            })
            .reduce(|acc, item| acc + item)
    }

    fn predicted_non_fungible_ids_take_from_worktop(
        receipt: &TransactionTypesReceipt<'_>,
        instruction_index: usize,
        resource_address: ResourceAddress,
    ) -> Option<IndexSet<NonFungibleLocalId>> {
        receipt
            .execution_trace()
            .worktop_changes()
            .entry(instruction_index)
            .or_default()
            .iter()
            .filter_map(|worktop_change| match worktop_change {
                WorktopChange::Take(ResourceSpecifier::Ids(
                    worktop_change_resource_address,
                    ids,
                )) if *worktop_change_resource_address == resource_address => {
                    Some(ids)
                }
                _ => None,
            })
            .cloned()
            .reduce(|mut acc, item| {
                acc.extend(item);
                acc
            })
    }

    fn predicted_non_fungible_ids_put_on_worktop(
        receipt: &TransactionTypesReceipt<'_>,
        instruction_index: usize,
        resource_address: ResourceAddress,
    ) -> Option<IndexSet<NonFungibleLocalId>> {
        receipt
            .execution_trace()
            .worktop_changes()
            .entry(instruction_index)
            .or_default()
            .iter()
            .filter_map(|worktop_change| match worktop_change {
                WorktopChange::Put(ResourceSpecifier::Ids(
                    worktop_change_resource_address,
                    ids,
                )) if *worktop_change_resource_address == resource_address => {
                    Some(ids)
                }
                _ => None,
            })
            .cloned()
            .reduce(|mut acc, item| {
                acc.extend(item);
                acc
            })
    }

    fn usize_to_decimal(num: usize) -> Decimal {
        Decimal(I192::from(num) * Decimal::ONE.0)
    }
}
