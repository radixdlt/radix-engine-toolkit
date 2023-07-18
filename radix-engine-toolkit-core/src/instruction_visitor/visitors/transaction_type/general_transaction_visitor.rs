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

// TODO: There are a few asserts and panics in this module for cases when the RET encounters some
// form of an illegal state that is impossible to work with (e.g., take from worktop putting
// resources in the worktop). Is it reasonable to do that, or should this be a [`Result`] and error?

#![allow(clippy::match_like_matches_macro)]

use crate::instruction_visitor::core::error::InstructionVisitorError;
use crate::instruction_visitor::core::traits::InstructionVisitor;
use crate::sbor::indexed_manifest_value::IndexedManifestValue;
use crate::statics::ACCOUNT_WITHDRAW_METHODS;
use crate::utils::is_account;

use radix_engine::system::system_modules::execution_trace::ResourceSpecifier;
use radix_engine::system::system_modules::execution_trace::WorktopChange;
use radix_engine::transaction::*;
use radix_engine_common::prelude::*;
use scrypto::blueprints::access_controller::*;
use scrypto::blueprints::account::*;
use scrypto::prelude::*;
use transaction::prelude::*;
use transaction::validation::ManifestIdAllocator;

pub struct GeneralTransactionTypeVisitor<'r> {
    /// The execution trace from the preview receipt
    execution_trace: &'r TransactionExecutionTrace,

    /// Tracks if the visitor is currently in an illegal state or not.
    is_illegal_state: bool,

    /// The withdraws from the account
    account_withdraws: HashMap<ComponentAddress, Vec<ResourceTracker>>,

    /// The deposits to the accounts
    account_deposits: HashMap<ComponentAddress, Vec<ResourceTracker>>,

    /// Used to allocate new ids
    id_allocator: ManifestIdAllocator,

    /// Tracks the buckets and their contents
    bucket_tracker: HashMap<ManifestBucket, ResourceTracker>,

    /// The index of the current instruction
    instruction_index: usize,
}

impl<'r> InstructionVisitor for GeneralTransactionTypeVisitor<'r> {
    fn is_enabled(&self) -> bool {
        !self.is_illegal_state
    }

    fn post_visit(&mut self) -> Result<(), InstructionVisitorError> {
        self.instruction_index += 1;
        Ok(())
    }

    fn visit_instruction(
        &mut self,
        instruction: &InstructionV1,
    ) -> Result<(), InstructionVisitorError> {
        match instruction {
            /* Calling method or function */
            InstructionV1::CallFunction {
                package_address,
                blueprint_name,
                function_name,
                args,
            } => self
                .handle_call_function(package_address, blueprint_name, function_name, args)
                .map_err(|error| {
                    InstructionVisitorError::LocatedGeneralTransactionTypeError(
                        LocatedGeneralTransactionTypeError {
                            instruction_index: self.instruction_index,
                            error,
                        },
                    )
                })?,
            InstructionV1::CallMethod {
                address,
                method_name,
                args,
            } => self
                .handle_call_method(address, method_name, args)
                .map_err(|error| {
                    InstructionVisitorError::LocatedGeneralTransactionTypeError(
                        LocatedGeneralTransactionTypeError {
                            instruction_index: self.instruction_index,
                            error,
                        },
                    )
                })?,

            /* Worktop take and return */
            InstructionV1::TakeFromWorktop {
                resource_address,
                amount,
            } => self
                .handle_take_from_worktop(resource_address, amount)
                .map_err(|error| LocatedGeneralTransactionTypeError {
                    error,
                    instruction_index: self.instruction_index,
                })?,
            InstructionV1::TakeAllFromWorktop { resource_address } => self
                .handle_take_all_from_worktop(resource_address)
                .map_err(|error| LocatedGeneralTransactionTypeError {
                    error,
                    instruction_index: self.instruction_index,
                })?,
            InstructionV1::TakeNonFungiblesFromWorktop {
                resource_address,
                ids,
            } => self
                .handle_take_non_fungibles_from_worktop(
                    resource_address,
                    &ids.iter().cloned().collect(),
                )
                .map_err(|error| LocatedGeneralTransactionTypeError {
                    error,
                    instruction_index: self.instruction_index,
                })?,
            InstructionV1::ReturnToWorktop { bucket_id } => self
                .handle_return_to_worktop(bucket_id)
                .map_err(|error| LocatedGeneralTransactionTypeError {
                    error,
                    instruction_index: self.instruction_index,
                })?,

            /* Non-main module method put the visitor in illegal state */
            InstructionV1::CallRoyaltyMethod { .. }
            | InstructionV1::CallMetadataMethod { .. }
            | InstructionV1::CallAccessRulesMethod { .. } => {
                self.is_illegal_state = true;
            }

            /* Direct Vault method and recall put the visitor in illegal state */
            InstructionV1::BurnResource { .. } | InstructionV1::CallDirectVaultMethod { .. } => {
                self.is_illegal_state = true
            }

            /* Allowed Instructions */
            InstructionV1::AssertWorktopContains { .. }
            | InstructionV1::AssertWorktopContainsAny { .. }
            | InstructionV1::AssertWorktopContainsNonFungibles { .. }
            | InstructionV1::PopFromAuthZone
            | InstructionV1::PushToAuthZone { .. }
            | InstructionV1::ClearAuthZone
            | InstructionV1::CreateProofFromAuthZoneOfAmount { .. }
            | InstructionV1::CreateProofFromAuthZoneOfNonFungibles { .. }
            | InstructionV1::CreateProofFromAuthZoneOfAll { .. }
            | InstructionV1::ClearSignatureProofs
            | InstructionV1::CreateProofFromBucketOfAmount { .. }
            | InstructionV1::CreateProofFromBucketOfNonFungibles { .. }
            | InstructionV1::CreateProofFromBucketOfAll { .. }
            | InstructionV1::CloneProof { .. }
            | InstructionV1::DropProof { .. }
            | InstructionV1::DropAllProofs
            | InstructionV1::AllocateGlobalAddress { .. } => {}
        }
        Ok(())
    }
}

impl<'r> GeneralTransactionTypeVisitor<'r> {
    pub fn new(execution_trace: &'r TransactionExecutionTrace) -> Self {
        Self {
            execution_trace,
            is_illegal_state: Default::default(),
            account_withdraws: Default::default(),
            account_deposits: Default::default(),
            instruction_index: Default::default(),
            id_allocator: Default::default(),
            bucket_tracker: Default::default(),
        }
    }

    #[allow(clippy::type_complexity)]
    pub fn output(
        self,
    ) -> Option<(
        HashMap<ComponentAddress, Vec<ResourceTracker>>,
        HashMap<ComponentAddress, Vec<ResourceTracker>>,
    )> {
        if self.is_illegal_state {
            None
        } else {
            Some((self.account_withdraws, self.account_deposits))
        }
    }

    pub fn handle_call_function(
        &mut self,
        _: &DynamicPackageAddress,
        _: &str,
        _: &str,
        args: &ManifestValue,
    ) -> Result<(), GeneralTransactionTypeError> {
        // Handle passed buckets
        let indexed_manifest_value = IndexedManifestValue::from_manifest_value(args);
        for bucket in indexed_manifest_value.buckets() {
            self.bucket_tracker
                .remove(bucket)
                .ok_or(GeneralTransactionTypeError::UnknownBucket(*bucket))?;
        }

        Ok(())
    }

    pub fn handle_call_method(
        &mut self,
        global_address: &DynamicGlobalAddress,
        method_name: &str,
        args: &ManifestValue,
    ) -> Result<(), GeneralTransactionTypeError> {
        // Filter: We only permit static address - no dynamic or named addresses are allowed
        let global_address = if let DynamicGlobalAddress::Static(address) = global_address {
            address
        } else {
            self.is_illegal_state = true;
            return Ok(());
        };

        // Filter: Some method calls to certain objects put the visitor in an illegal state
        if !global_address
            .as_node_id()
            .entity_type()
            .map_or(false, |entity_type| match entity_type {
                /* Allowed */
                EntityType::GlobalGenericComponent
                | EntityType::GlobalAccount
                | EntityType::GlobalIdentity
                | EntityType::GlobalOneResourcePool
                | EntityType::GlobalTwoResourcePool
                | EntityType::GlobalMultiResourcePool
                | EntityType::GlobalVirtualSecp256k1Account
                | EntityType::GlobalVirtualSecp256k1Identity
                | EntityType::GlobalVirtualEd25519Account
                | EntityType::GlobalVirtualEd25519Identity
                | EntityType::InternalGenericComponent => true,

                /* Some are allowed */
                EntityType::GlobalAccessController => {
                    method_name == ACCESS_CONTROLLER_CREATE_PROOF_IDENT
                }

                /* Not Allowed */
                EntityType::GlobalPackage
                | EntityType::GlobalValidator
                | EntityType::GlobalFungibleResourceManager
                | EntityType::GlobalNonFungibleResourceManager
                | EntityType::InternalAccount
                | EntityType::GlobalConsensusManager
                | EntityType::InternalFungibleVault
                | EntityType::InternalNonFungibleVault
                | EntityType::InternalKeyValueStore
                | EntityType::GlobalTransactionTracker => false,
            })
        {
            self.is_illegal_state = true;
            return Ok(());
        }

        let component_address = ComponentAddress::new_or_panic(global_address.as_node_id().0);

        if ACCOUNT_WITHDRAW_METHODS.contains(&method_name.to_string()) && is_account(global_address)
        {
            let worktop_puts = self
                .execution_trace
                .worktop_changes()
                .get(&self.instruction_index)
                .and_then(|worktop_changes| worktop_changes.first())
                .and_then(|worktop_change| match worktop_change {
                    WorktopChange::Put(put) => Some(put),
                    WorktopChange::Take(..) => None,
                })
                .ok_or(GeneralTransactionTypeError::WorktopChangesError)
                .cloned()?;

            let resource_tracker = if method_name == ACCOUNT_WITHDRAW_IDENT {
                if let Ok(AccountWithdrawInput {
                    resource_address,
                    amount,
                }) = manifest_decode(&manifest_encode(&args).unwrap())
                {
                    match worktop_puts {
                        ResourceSpecifier::Amount(changes_resource_address, changes_amount) => {
                            assert_eq!(changes_resource_address, resource_address);
                            assert_eq!(changes_amount, amount);

                            ResourceTracker::Fungible {
                                resource_address,
                                amount: Source::Guaranteed(amount),
                            }
                        }
                        ResourceSpecifier::Ids(changes_resource_address, changes_ids) => {
                            assert_eq!(changes_resource_address, resource_address);
                            assert_eq!(usize_to_decimal(changes_ids.len()), amount);

                            ResourceTracker::NonFungible {
                                resource_address,
                                amount: Source::Guaranteed(amount),
                                ids: Source::Predicted(self.instruction_index, changes_ids),
                            }
                        }
                    }
                } else {
                    self.is_illegal_state = true;
                    return Ok(());
                }
            } else if method_name == ACCOUNT_LOCK_FEE_AND_WITHDRAW_IDENT {
                if let Ok(AccountLockFeeAndWithdrawInput {
                    resource_address,
                    amount,
                    ..
                }) = manifest_decode(&manifest_encode(&args).unwrap())
                {
                    match worktop_puts {
                        ResourceSpecifier::Amount(changes_resource_address, changes_amount) => {
                            assert_eq!(changes_resource_address, resource_address);
                            assert_eq!(changes_amount, amount);

                            ResourceTracker::Fungible {
                                resource_address,
                                amount: Source::Guaranteed(amount),
                            }
                        }
                        ResourceSpecifier::Ids(changes_resource_address, changes_ids) => {
                            assert_eq!(changes_resource_address, resource_address);
                            assert_eq!(usize_to_decimal(changes_ids.len()), amount);

                            ResourceTracker::NonFungible {
                                resource_address,
                                amount: Source::Guaranteed(amount),
                                ids: Source::Predicted(self.instruction_index, changes_ids),
                            }
                        }
                    }
                } else {
                    self.is_illegal_state = true;
                    return Ok(());
                }
            } else if method_name == ACCOUNT_WITHDRAW_NON_FUNGIBLES_IDENT {
                if let Ok(AccountWithdrawNonFungiblesInput {
                    resource_address,
                    ids,
                }) = manifest_decode(&manifest_encode(&args).unwrap())
                {
                    match worktop_puts {
                        ResourceSpecifier::Amount(..) => {
                            panic!("Account withdraw non-fungibles returned an amount!")
                        }
                        ResourceSpecifier::Ids(changes_resource_address, changes_ids) => {
                            assert_eq!(changes_resource_address, resource_address);
                            assert_eq!(ids, changes_ids);

                            ResourceTracker::NonFungible {
                                resource_address,
                                amount: Source::Guaranteed(usize_to_decimal(ids.len())),
                                ids: Source::Guaranteed(ids),
                            }
                        }
                    }
                } else {
                    self.is_illegal_state = true;
                    return Ok(());
                }
            } else if method_name == ACCOUNT_LOCK_FEE_AND_WITHDRAW_NON_FUNGIBLES_IDENT {
                if let Ok(AccountLockFeeAndWithdrawNonFungiblesInput {
                    resource_address,
                    ids,
                    ..
                }) = manifest_decode(&manifest_encode(&args).unwrap())
                {
                    match worktop_puts {
                        ResourceSpecifier::Amount(..) => {
                            panic!("Account withdraw non-fungibles returned an amount!")
                        }
                        ResourceSpecifier::Ids(changes_resource_address, changes_ids) => {
                            assert_eq!(changes_resource_address, resource_address);
                            assert_eq!(ids, changes_ids);

                            ResourceTracker::NonFungible {
                                resource_address,
                                amount: Source::Guaranteed(usize_to_decimal(ids.len())),
                                ids: Source::Guaranteed(ids),
                            }
                        }
                    }
                } else {
                    self.is_illegal_state = true;
                    return Ok(());
                }
            } else {
                self.is_illegal_state = true;
                return Ok(());
            };

            self.account_withdraws
                .entry(component_address)
                .or_default()
                .push(resource_tracker)
        } else if [
            ACCOUNT_DEPOSIT_IDENT,
            ACCOUNT_DEPOSIT_BATCH_IDENT,
            ACCOUNT_TRY_DEPOSIT_OR_ABORT_IDENT,
            ACCOUNT_TRY_DEPOSIT_BATCH_OR_ABORT_IDENT,
        ]
        .contains(&method_name)
            && is_account(global_address)
        {
            let indexed_manifest_value = IndexedManifestValue::from_manifest_value(args);

            let buckets = indexed_manifest_value.buckets();
            let expressions = indexed_manifest_value.expressions();
            if !expressions.is_empty() {
                let worktop_changes = self
                    .execution_trace
                    .worktop_changes()
                    .get(&self.instruction_index)
                    .map(|worktop_changes| {
                        worktop_changes
                            .iter()
                            .filter_map(|worktop_change| match worktop_change {
                                WorktopChange::Put(..) => None,
                                WorktopChange::Take(ResourceSpecifier::Amount(
                                    resource_address,
                                    amount,
                                )) => Some(ResourceTracker::Fungible {
                                    resource_address: *resource_address,
                                    amount: Source::Predicted(self.instruction_index, *amount),
                                }),
                                WorktopChange::Take(ResourceSpecifier::Ids(
                                    resource_address,
                                    ids,
                                )) => Some(ResourceTracker::NonFungible {
                                    resource_address: *resource_address,
                                    amount: Source::Predicted(
                                        self.instruction_index,
                                        usize_to_decimal(ids.len()),
                                    ),
                                    ids: Source::Predicted(self.instruction_index, ids.clone()),
                                }),
                            })
                            .collect::<Vec<_>>()
                    })
                    .ok_or(GeneralTransactionTypeError::WorktopChangesError)?;
                self.account_deposits
                    .entry(component_address)
                    .or_default()
                    .extend(worktop_changes)
            } else if !buckets.is_empty() {
                for bucket in buckets {
                    let bucket_amount = self
                        .bucket_tracker
                        .remove(bucket)
                        .ok_or(GeneralTransactionTypeError::UnknownBucket(*bucket))?;
                    self.account_deposits
                        .entry(component_address)
                        .or_default()
                        .push(bucket_amount)
                }
            }
        } else {
            let indexed_manifest_value = IndexedManifestValue::from_manifest_value(args);
            for bucket in indexed_manifest_value.buckets() {
                self.bucket_tracker
                    .remove(bucket)
                    .ok_or(GeneralTransactionTypeError::UnknownBucket(*bucket))?;
            }
        }

        Ok(())
    }

    pub fn handle_take_from_worktop(
        &mut self,
        resource_address: &ResourceAddress,
        amount: &Decimal,
    ) -> Result<(), GeneralTransactionTypeError> {
        // This depends on whether the resource is fungible or non-fungible. If the resource is
        // fungible, then we can just construct a ResourceTracker::Fungible of the amount given
        // here as guaranteed and move on. If it's non-fungible however, then the amount in here
        // is guaranteed but the non-fungible local ids are not guaranteed since we obtain them
        // by looking at the bucket snapshot from the receipt.
        let resource_tracker = match self
            .execution_trace
            .worktop_changes()
            .get(&self.instruction_index)
            .and_then(|worktop_changes| worktop_changes.first())
        {
            Some(WorktopChange::Put(..)) => {
                panic!("How did a call to TAKE from worktop PUT resources in the worktop?")
            }
            Some(WorktopChange::Take(ResourceSpecifier::Amount(_, changes_amount))) => {
                assert!(resource_address.is_fungible());
                assert_eq!(amount, changes_amount);

                ResourceTracker::Fungible {
                    resource_address: *resource_address,
                    amount: Source::Guaranteed(*amount),
                }
            }
            Some(WorktopChange::Take(ResourceSpecifier::Ids(_, ids))) => {
                assert!(resource_address
                    .as_node_id()
                    .is_global_non_fungible_resource_manager());
                assert_eq!(*amount, Decimal::from_str(&ids.len().to_string()).unwrap());

                ResourceTracker::NonFungible {
                    resource_address: *resource_address,
                    amount: Source::Guaranteed(*amount),
                    ids: Source::Predicted(self.instruction_index, ids.clone()),
                }
            }
            None if amount.is_zero() => {
                if resource_address.is_fungible() {
                    ResourceTracker::Fungible {
                        resource_address: *resource_address,
                        amount: Source::Guaranteed(Decimal::ZERO),
                    }
                } else {
                    ResourceTracker::NonFungible {
                        resource_address: *resource_address,
                        amount: Source::Guaranteed(Decimal::ZERO),
                        ids: Source::Guaranteed(Default::default()),
                    }
                }
            }
            None => {
                panic!("How can the worktop changes be None if the amount specified is not zero?")
            }
        };

        let bucket = self.id_allocator.new_bucket_id();
        self.bucket_tracker.insert(bucket, resource_tracker);

        Ok(())
    }

    pub fn handle_take_non_fungibles_from_worktop(
        &mut self,
        resource_address: &ResourceAddress,
        ids: &BTreeSet<NonFungibleLocalId>,
    ) -> Result<(), GeneralTransactionTypeError> {
        // In this case, the resource is non-fungible and the take from worktop is of a known set of
        // ids, which we can also say of a known set of amounts. Thus, everything about this can be
        // found statically and we can straight away map this to a resource tracker without the need
        // to look at the worktop changes.
        let resource_tracker = ResourceTracker::NonFungible {
            resource_address: *resource_address,
            amount: Source::Guaranteed(usize_to_decimal(ids.len())),
            ids: Source::Guaranteed(ids.clone()),
        };

        let bucket = self.id_allocator.new_bucket_id();
        self.bucket_tracker.insert(bucket, resource_tracker);
        Ok(())
    }

    pub fn handle_take_all_from_worktop(
        &mut self,
        resource_address: &ResourceAddress,
    ) -> Result<(), GeneralTransactionTypeError> {
        // This case changes slightly between fungible and non-fungible resources. However, all of
        // the cases result in everything being predicted and nothing being guaranteed. We observe
        // the worktop changes and then based on that construct the resource tracker of whatever
        // the observed bucket amounts/ids were.
        let resource_tracker = match self
            .execution_trace
            .worktop_changes()
            .get(&self.instruction_index)
            .and_then(|worktop_changes| worktop_changes.first())
        {
            Some(WorktopChange::Put(..)) => {
                panic!("How did a call to TAKE from worktop PUT resources in the worktop?")
            }
            Some(WorktopChange::Take(ResourceSpecifier::Amount(resource_address, amount))) => {
                assert!(resource_address.is_fungible());

                ResourceTracker::Fungible {
                    resource_address: *resource_address,
                    amount: Source::Predicted(self.instruction_index, *amount),
                }
            }
            Some(WorktopChange::Take(ResourceSpecifier::Ids(resource_address, ids))) => {
                assert!(resource_address
                    .as_node_id()
                    .is_global_non_fungible_resource_manager());

                ResourceTracker::NonFungible {
                    resource_address: *resource_address,
                    amount: Source::Predicted(self.instruction_index, usize_to_decimal(ids.len())),
                    ids: Source::Predicted(self.instruction_index, ids.clone()),
                }
            }
            None => {
                if resource_address.is_fungible() {
                    ResourceTracker::Fungible {
                        resource_address: *resource_address,
                        amount: Source::Predicted(self.instruction_index, Decimal::ZERO),
                    }
                } else {
                    ResourceTracker::NonFungible {
                        resource_address: *resource_address,
                        amount: Source::Predicted(self.instruction_index, Decimal::ZERO),
                        ids: Source::Predicted(self.instruction_index, Default::default()),
                    }
                }
            }
        };

        let bucket = self.id_allocator.new_bucket_id();
        self.bucket_tracker.insert(bucket, resource_tracker);
        Ok(())
    }

    pub fn handle_return_to_worktop(
        &mut self,
        bucket: &ManifestBucket,
    ) -> Result<(), GeneralTransactionTypeError> {
        self.bucket_tracker
            .remove(bucket)
            .ok_or(GeneralTransactionTypeError::UnknownBucket(*bucket))?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Source<T> {
    Guaranteed(T),
    Predicted(usize, T),
}

impl<T> std::ops::Deref for Source<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Guaranteed(target) | Self::Predicted(_, target) => target,
        }
    }
}

impl<T> std::ops::DerefMut for Source<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Guaranteed(target) | Self::Predicted(_, target) => target,
        }
    }
}

#[derive(Debug, Clone)]
pub enum GeneralTransactionTypeError {
    ReceiptOfAFailedOrRejectedTransaction,
    WorktopChangesError,
    UnknownBucket(ManifestBucket),
}

#[derive(Debug, Clone)]
pub struct LocatedGeneralTransactionTypeError {
    pub instruction_index: usize,
    pub error: GeneralTransactionTypeError,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResourceTracker {
    /// An account deposit of a fungible resources where the amount can either be guaranteed or
    /// predicted.
    Fungible {
        resource_address: ResourceAddress,
        amount: Source<Decimal>,
    },
    /// A set of tracked non-fungible resources. In this case, the amount and ids may be guaranteed
    /// or predicted. A valid non-fungible tracker may have a guaranteed amount but a
    /// non-guaranteed set of ids.
    NonFungible {
        resource_address: ResourceAddress,
        amount: Source<Decimal>,
        ids: Source<BTreeSet<NonFungibleLocalId>>,
    },
}

fn usize_to_decimal(num: usize) -> Decimal {
    num.to_string().parse().unwrap()
}
