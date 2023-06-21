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
    account_withdraws: HashMap<ComponentAddress, Vec<ResourceSpecifier>>,

    /// The deposits to the accounts
    account_deposits: HashMap<ComponentAddress, Vec<Source<ResourceSpecifier>>>,

    /// Used to allocate new ids
    id_allocator: ManifestIdAllocator,

    /// Tracks the buckets and their contents
    bucket_tracker: HashMap<ManifestBucket, Source<ResourceSpecifier>>,

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
            | InstructionV1::AssertWorktopContainsNonFungibles { .. }
            | InstructionV1::PopFromAuthZone
            | InstructionV1::PushToAuthZone { .. }
            | InstructionV1::ClearAuthZone
            | InstructionV1::CreateProofFromAuthZone { .. }
            | InstructionV1::CreateProofFromAuthZoneOfAmount { .. }
            | InstructionV1::CreateProofFromAuthZoneOfNonFungibles { .. }
            | InstructionV1::CreateProofFromAuthZoneOfAll { .. }
            | InstructionV1::ClearSignatureProofs
            | InstructionV1::CreateProofFromBucket { .. }
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
        HashMap<ComponentAddress, Vec<ResourceSpecifier>>,
        HashMap<ComponentAddress, Vec<Source<ResourceSpecifier>>>,
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
            let withdrawn_resources = self
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
            self.account_withdraws
                .entry(component_address)
                .or_default()
                .push(withdrawn_resources);
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
                                WorktopChange::Take(take) => {
                                    Some(Source::Predicted(self.instruction_index, take.clone()))
                                }
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
        let bucket = self.id_allocator.new_bucket_id();
        let resource_specifier = ResourceSpecifier::Amount(*resource_address, *amount);
        self.bucket_tracker
            .insert(bucket, Source::Guaranteed(resource_specifier));
        Ok(())
    }

    pub fn handle_take_non_fungibles_from_worktop(
        &mut self,
        resource_address: &ResourceAddress,
        ids: &BTreeSet<NonFungibleLocalId>,
    ) -> Result<(), GeneralTransactionTypeError> {
        let bucket = self.id_allocator.new_bucket_id();
        let resource_specifier = ResourceSpecifier::Ids(*resource_address, ids.clone());
        self.bucket_tracker
            .insert(bucket, Source::Guaranteed(resource_specifier));
        Ok(())
    }

    pub fn handle_take_all_from_worktop(
        &mut self,
        _: &ResourceAddress,
    ) -> Result<(), GeneralTransactionTypeError> {
        let bucket = self.id_allocator.new_bucket_id();
        let resource_specifier = self
            .execution_trace
            .worktop_changes()
            .get(&self.instruction_index)
            .and_then(|worktop_changes| worktop_changes.first())
            .and_then(|worktop_change| match worktop_change {
                WorktopChange::Take(take) => Some(take),
                WorktopChange::Put(..) => None,
            })
            .ok_or(GeneralTransactionTypeError::WorktopChangesError)
            .cloned()?;
        self.bucket_tracker.insert(
            bucket,
            Source::Predicted(self.instruction_index, resource_specifier),
        );
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
