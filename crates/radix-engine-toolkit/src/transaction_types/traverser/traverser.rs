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

pub mod static_analysis {
    use crate::sbor::indexed_manifest_value::*;
    use crate::transaction_types::*;
    use crate::utils::*;
    use radix_engine::system::system_modules::execution_trace::ResourceSpecifier;
    use radix_engine_interface::blueprints::account::*;
    use radix_transactions::prelude::manifest_instruction::*;
    use radix_transactions::prelude::*;
    use scrypto::prelude::*;

    pub fn traverse(
        callbacks: &mut [&mut dyn ManifestSummaryCallback],
        instructions: &[InstructionV2],
    ) {
        for (instruction_index, instruction) in instructions.iter().enumerate()
        {
            on_instruction(callbacks, instruction, instruction_index);
        }
        on_finish(callbacks, instructions.len());
    }

    pub(super) fn on_instruction(
        callbacks: &mut [&mut dyn ManifestSummaryCallback],
        instruction: &InstructionV2,
        instruction_index: usize,
    ) {
        // At the beginning of an instruction, call the on_instruction callback
        callbacks.iter_mut().for_each(|callback| {
            callback.on_instruction(instruction, instruction_index)
        });

        // Notify the callbacks of the created account proofs
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
        instruction: &InstructionV2,
    ) {
        if let InstructionV2::CallMethod(CallMethod {
            address: dynamic_address @ DynamicGlobalAddress::Static(address),
            method_name,
            args,
        }) = instruction
        {
            if !is_account(dynamic_address) {
                return;
            }

            let account =
                ComponentAddress::try_from(*address).expect("Must succeed");

            if method_name == ACCOUNT_CREATE_PROOF_OF_AMOUNT_IDENT {
                if let Some(AccountCreateProofOfAmountInput {
                    resource_address,
                    amount,
                }) = to_manifest_type(args)
                {
                    callbacks.iter_mut().for_each(|callback| {
                        callback.on_create_proof(
                            &account,
                            &ResourceSpecifier::Amount(
                                resource_address,
                                amount,
                            ),
                        )
                    });
                }
            } else if method_name == ACCOUNT_CREATE_PROOF_OF_NON_FUNGIBLES_IDENT
            {
                if let Some(AccountCreateProofOfNonFungiblesInput {
                    resource_address,
                    ids,
                }) = to_manifest_type(args)
                {
                    callbacks.iter_mut().for_each(|callback| {
                        callback.on_create_proof(
                            &account,
                            &ResourceSpecifier::Ids(
                                resource_address,
                                ids.clone(),
                            ),
                        )
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

pub mod dynamic_analysis {
    use crate::sbor::indexed_manifest_value::*;
    use crate::transaction_types::*;
    use crate::utils::*;
    use radix_common::prelude::*;
    use radix_engine::system::system_modules::execution_trace::{
        ResourceSpecifier, WorktopChange,
    };
    use radix_engine_interface::blueprints::account::*;
    use radix_transactions::prelude::manifest_instruction::*;
    use radix_transactions::prelude::*;
    use radix_transactions::validation::*;

    pub fn traverse(
        callbacks: &mut [&mut dyn ExecutionSummaryCallback],
        instructions: &[InstructionV2],
        receipt: &TransactionTypesReceipt<'_>,
    ) {
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
        instruction: &InstructionV2,
        instruction_index: usize,
        /* State */
        receipt: &TransactionTypesReceipt<'_>,
        id_allocator: &mut ManifestIdAllocator,
        bucket_tracker: &mut IndexMap<ManifestBucket, ResourceIndicator>,
    ) {
        // Calling the on_instruction handler of the `ManifestSummaryCallback`
        // to handle the manifest summary bits of this manifest and then we can
        // move on to the other aspects.
        super::static_analysis::on_instruction(
            // This is the reason why we need to depend on nightly builds of
            // rust. The manifest summary traverser takes in an array of
            // &mut [&mut dyn ManifestSummaryCallback] and the only way to cast
            // the individual callbacks into a dyn ManifestSummaryCallback is
            // through nightly rust.
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
        super::static_analysis::on_finish(
            // This is the reason why we need to depend on nightly builds of
            // rust. The manifest summary traverser takes in an array of
            // &mut [&mut dyn ManifestSummaryCallback] and the only way to cast
            // the individual callbacks into a dyn ManifestSummaryCallback is
            // through nightly rust.
            &mut callbacks
                .iter_mut()
                .map(|item| *item as &mut dyn ManifestSummaryCallback)
                .collect::<Vec<_>>(),
            instructions_count,
        )
    }

    fn handle_on_instruction(
        callbacks: &mut [&mut dyn ExecutionSummaryCallback],
        instruction: &InstructionV2,
        instruction_index: usize,
        /* State */
        receipt: &TransactionTypesReceipt<'_>,
        bucket_tracker: &IndexMap<ManifestBucket, ResourceIndicator>,
    ) {
        let worktop_changes_entry = receipt
            .worktop_changes()
            .get(&instruction_index)
            .cloned()
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
                InstructionV2::CallFunction(CallFunction { args, .. })
                | InstructionV2::CallMethod(CallMethod { args, .. })
                | InstructionV2::CallRoyaltyMethod(CallRoyaltyMethod {
                    args,
                    ..
                })
                | InstructionV2::CallMetadataMethod(CallMetadataMethod {
                    args,
                    ..
                })
                | InstructionV2::CallRoleAssignmentMethod(
                    CallRoleAssignmentMethod { args, .. },
                )
                | InstructionV2::CallDirectVaultMethod(
                    CallDirectVaultMethod { args, .. },
                )
                | InstructionV2::YieldToParent(YieldToParent { args })
                | InstructionV2::YieldToChild(YieldToChild { args, .. }) => {
                    let manifest_value = IndexedManifestValue::from_typed(args);
                    let additional_resources = manifest_value
                        .buckets()
                        .iter()
                        .filter_map(|bucket| bucket_tracker.get(bucket))
                        .map(|resource_indicator| {
                            ResourceSpecifier::from(resource_indicator.clone())
                        });
                    inputs.extend(additional_resources)
                }
                InstructionV2::BurnResource(BurnResource { bucket_id })
                | InstructionV2::ReturnToWorktop(ReturnToWorktop {
                    bucket_id,
                }) => {
                    if let Some(resource_indicator) =
                        bucket_tracker.get(bucket_id)
                    {
                        inputs.push(ResourceSpecifier::from(
                            resource_indicator.clone(),
                        ))
                    }
                }
                /* Non-sink methods */
                InstructionV2::TakeAllFromWorktop(..)
                | InstructionV2::TakeFromWorktop(..)
                | InstructionV2::TakeNonFungiblesFromWorktop(..)
                | InstructionV2::AssertWorktopContainsAny(..)
                | InstructionV2::AssertWorktopContains(..)
                | InstructionV2::AssertWorktopContainsNonFungibles(..)
                | InstructionV2::PopFromAuthZone(..)
                | InstructionV2::PushToAuthZone(..)
                | InstructionV2::CreateProofFromAuthZoneOfAmount(..)
                | InstructionV2::CreateProofFromAuthZoneOfNonFungibles(..)
                | InstructionV2::CreateProofFromAuthZoneOfAll(..)
                | InstructionV2::DropAuthZoneProofs(..)
                | InstructionV2::DropAuthZoneRegularProofs(..)
                | InstructionV2::DropAuthZoneSignatureProofs(..)
                | InstructionV2::CreateProofFromBucketOfAmount(..)
                | InstructionV2::CreateProofFromBucketOfNonFungibles(..)
                | InstructionV2::CreateProofFromBucketOfAll(..)
                | InstructionV2::CloneProof(..)
                | InstructionV2::DropProof(..)
                | InstructionV2::DropNamedProofs(..)
                | InstructionV2::DropAllProofs(..)
                | InstructionV2::AllocateGlobalAddress(..)
                | InstructionV2::AuthenticateParent(..) => { /* No-Op */ }
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
        instruction: &InstructionV2,
        instruction_index: usize,
        receipt: &TransactionTypesReceipt<'_>,
    ) {
        let InstructionV2::CallMethod(CallMethod {
            address: dynamic_address @ DynamicGlobalAddress::Static(address),
            method_name,
            args,
        }) = instruction
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
            }) = to_manifest_type(args)
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
                            .unwrap_or_default(),
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
            }) = to_manifest_type(args)
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
                            .unwrap_or_default(),
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
            }) = to_manifest_type(args)
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
            }) = to_manifest_type(args)
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
        instruction: &InstructionV2,
        instruction_index: usize,
        /* State */
        receipt: &TransactionTypesReceipt<'_>,
        bucket_tracker: &IndexMap<ManifestBucket, ResourceIndicator>,
    ) {
        if let InstructionV2::CallMethod(CallMethod {
            address: dynamic_address @ DynamicGlobalAddress::Static(address),
            method_name,
            args,
        }) = instruction
        {
            if !is_account(dynamic_address) {
                return;
            }

            let account =
                ComponentAddress::try_from(*address).expect("Must succeed!");

            // TODO: If we plan on supporting the or_refund methods to at least
            // recognizing them at this layer, then we must do something like
            // output - input to determine how much we're actually depositing
            // and not bouncing back. We would also ignore the call to deposit
            // all together if the deposit bounces.
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
                    if let Some(worktop_changes) =
                        receipt.worktop_changes().get(&instruction_index)
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
        instruction: &InstructionV2,
        instruction_index: usize,
        /* State */
        receipt: &TransactionTypesReceipt<'_>,
        id_allocator: &mut ManifestIdAllocator,
        bucket_tracker: &mut IndexMap<ManifestBucket, ResourceIndicator>,
    ) {
        match instruction {
            /* Source */
            InstructionV2::TakeNonFungiblesFromWorktop(
                TakeNonFungiblesFromWorktop {
                    resource_address,
                    ids,
                },
            ) => {
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
            InstructionV2::TakeFromWorktop(TakeFromWorktop {
                resource_address,
                amount,
            }) => {
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
                                .unwrap_or_default(),
                                instruction_index,
                            },
                        },
                    )
                };
                bucket_tracker.insert(bucket, resource_indicator);
            }
            InstructionV2::TakeAllFromWorktop(TakeAllFromWorktop {
                resource_address,
            }) => {
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
                        .unwrap_or_default();
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
            InstructionV2::ReturnToWorktop(ReturnToWorktop { bucket_id })
            | InstructionV2::BurnResource(BurnResource { bucket_id }) => {
                // TODO: Do we want to check that the bucket was actually
                // present and then removed?
                bucket_tracker.swap_remove(bucket_id);
            }
            InstructionV2::CallFunction(CallFunction { args, .. })
            | InstructionV2::CallMethod(CallMethod { args, .. })
            | InstructionV2::CallRoyaltyMethod(CallRoyaltyMethod {
                args,
                ..
            })
            | InstructionV2::CallMetadataMethod(CallMetadataMethod {
                args,
                ..
            })
            | InstructionV2::CallRoleAssignmentMethod(
                CallRoleAssignmentMethod { args, .. },
            )
            | InstructionV2::CallDirectVaultMethod(CallDirectVaultMethod {
                args,
                ..
            })
            | InstructionV2::YieldToParent(YieldToParent { args })
            | InstructionV2::YieldToChild(YieldToChild { args, .. }) => {
                let manifest_value = IndexedManifestValue::from_typed(args);
                for bucket in manifest_value.buckets() {
                    // TODO: Do we want to check that the bucket was actually
                    // present and then removed?
                    bucket_tracker.swap_remove(bucket);
                }
            }
            /* Neither */
            InstructionV2::AssertWorktopContainsAny(..)
            | InstructionV2::AssertWorktopContains(..)
            | InstructionV2::AssertWorktopContainsNonFungibles(..)
            | InstructionV2::PopFromAuthZone(..)
            | InstructionV2::PushToAuthZone(..)
            | InstructionV2::CreateProofFromAuthZoneOfAmount(..)
            | InstructionV2::CreateProofFromAuthZoneOfNonFungibles(..)
            | InstructionV2::CreateProofFromAuthZoneOfAll(..)
            | InstructionV2::DropAuthZoneProofs(..)
            | InstructionV2::DropAuthZoneRegularProofs(..)
            | InstructionV2::DropAuthZoneSignatureProofs(..)
            | InstructionV2::CreateProofFromBucketOfAmount(..)
            | InstructionV2::CreateProofFromBucketOfNonFungibles(..)
            | InstructionV2::CreateProofFromBucketOfAll(..)
            | InstructionV2::CloneProof(..)
            | InstructionV2::DropProof(..)
            | InstructionV2::DropNamedProofs(..)
            | InstructionV2::DropAllProofs(..)
            | InstructionV2::AllocateGlobalAddress(..)
            | InstructionV2::AuthenticateParent(..) => { /* No-op */ }
        }
    }

    fn predicted_fungible_amount_take_from_worktop(
        receipt: &TransactionTypesReceipt<'_>,
        instruction_index: usize,
        resource_address: ResourceAddress,
    ) -> Option<Decimal> {
        receipt
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
