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

use transaction::prelude::*;
use transaction::validation::*;

use scrypto::blueprints::account::*;

use crate::instruction_visitor::core::error::*;
use crate::instruction_visitor::core::traits::*;
use crate::sbor::indexed_manifest_value::*;
use crate::utils::*;

#[derive(Default, Clone, Debug)]
pub struct TransferTransactionTypeVisitor {
    // Tracks the amount of resources currently in the worktop.
    worktop: Worktop,

    // Tracks the contents of the bucket.
    bucket_tracker: HashMap<ManifestBucket, (ResourceAddress, Resources)>,

    // Tracks which accounts were withdrawn from.
    account_withdrawn_from: Option<ComponentAddress>,

    // Tracks the accounts deposited into and the quantity.
    account_deposits: HashMap<ComponentAddress, HashMap<ResourceAddress, Resources>>,

    // Tracks if the visitor is currently in an illegal state or not.
    is_illegal_state: bool,
}

impl TransferTransactionTypeVisitor {
    #[allow(clippy::type_complexity)]
    pub fn output(
        self,
    ) -> Option<(
        ComponentAddress,
        HashMap<ComponentAddress, HashMap<ResourceAddress, Resources>>,
    )> {
        if self.is_illegal_state {
            None
        } else if let Some(account_withdrawn_from) = self.account_withdrawn_from {
            if !self.account_deposits.is_empty() {
                Some((account_withdrawn_from, self.account_deposits))
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl InstructionVisitor for TransferTransactionTypeVisitor {
    fn is_enabled(&self) -> bool {
        !self.is_illegal_state
    }

    fn visit_instruction(
        &mut self,
        instruction: &InstructionV1,
    ) -> Result<(), InstructionVisitorError> {
        match instruction {
            /* Method Calls */
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(address),
                method_name,
                args,
            } => {
                if !is_account(address) {
                    self.is_illegal_state = true;
                    return Ok(());
                }

                // This never panics. We have already checked that this is an account when we
                // called `is_account`.
                let component_address = ComponentAddress::new_or_panic(address.as_node_id().0);

                match method_name.as_str() {
                    ACCOUNT_WITHDRAW_IDENT => to_manifest_type(args)
                        .ok_or(TransferTransactionTypeError::InvalidArgs)
                        .and_then(|value| self.handle_account_withdraw(component_address, value))?,
                    ACCOUNT_WITHDRAW_NON_FUNGIBLES_IDENT => to_manifest_type(args)
                        .ok_or(TransferTransactionTypeError::InvalidArgs)
                        .and_then(|value| {
                            self.handle_account_withdraw_non_fungibles(component_address, value)
                        })?,
                    ACCOUNT_DEPOSIT_IDENT
                    | ACCOUNT_DEPOSIT_BATCH_IDENT
                    | ACCOUNT_TRY_DEPOSIT_OR_ABORT_IDENT
                    | ACCOUNT_TRY_DEPOSIT_BATCH_OR_REFUND_IDENT => {
                        self.handle_validation_and_account_deposits(component_address, args)?
                    }
                    _ => {
                        self.is_illegal_state = true;
                    }
                };
            }
            /* Worktop Take */
            InstructionV1::TakeFromWorktop {
                resource_address,
                amount,
            } => {
                let (bucket, resources) = self
                    .worktop
                    .take(*resource_address, *amount)
                    .map_err(TransferTransactionTypeError::WorktopError)?;
                self.bucket_tracker
                    .insert(bucket, (*resource_address, resources));
            }
            InstructionV1::TakeAllFromWorktop { resource_address } => {
                let (bucket, resources) = self
                    .worktop
                    .take_all(*resource_address)
                    .map_err(TransferTransactionTypeError::WorktopError)?;
                self.bucket_tracker
                    .insert(bucket, (*resource_address, resources));
            }
            InstructionV1::TakeNonFungiblesFromWorktop {
                resource_address,
                ids,
            } => {
                let (bucket, resources) = self
                    .worktop
                    .take_non_fungibles(*resource_address, &ids.iter().cloned().collect())
                    .map_err(TransferTransactionTypeError::WorktopError)?;
                self.bucket_tracker
                    .insert(bucket, (*resource_address, resources));
            }
            InstructionV1::ReturnToWorktop { bucket_id } => {
                let (resource_address, resources) = self
                    .bucket_tracker
                    .remove(bucket_id)
                    .ok_or(TransferTransactionTypeError::BucketNotFound(*bucket_id))?;
                self.worktop
                    .put(resource_address, resources)
                    .map_err(TransferTransactionTypeError::WorktopError)?;
            }
            /* Allowed Instructions */
            InstructionV1::AssertWorktopContains { .. }
            | InstructionV1::AssertWorktopContainsAny { .. }
            | InstructionV1::AssertWorktopContainsNonFungibles { .. } => {}
            /* Illegal Instructions */
            InstructionV1::CallMethod { .. }
            | InstructionV1::PopFromAuthZone
            | InstructionV1::PushToAuthZone { .. }
            | InstructionV1::CreateProofFromAuthZoneOfAmount { .. }
            | InstructionV1::CreateProofFromAuthZoneOfNonFungibles { .. }
            | InstructionV1::CreateProofFromAuthZoneOfAll { .. }
            | InstructionV1::DropNamedProofs
            | InstructionV1::DropAuthZoneProofs
            | InstructionV1::DropAuthZoneRegularProofs
            | InstructionV1::DropAuthZoneSignatureProofs
            | InstructionV1::CreateProofFromBucketOfAmount { .. }
            | InstructionV1::CreateProofFromBucketOfNonFungibles { .. }
            | InstructionV1::CreateProofFromBucketOfAll { .. }
            | InstructionV1::BurnResource { .. }
            | InstructionV1::CloneProof { .. }
            | InstructionV1::DropProof { .. }
            | InstructionV1::CallFunction { .. }
            | InstructionV1::CallRoyaltyMethod { .. }
            | InstructionV1::CallMetadataMethod { .. }
            | InstructionV1::CallRoleAssignmentMethod { .. }
            | InstructionV1::CallDirectVaultMethod { .. }
            | InstructionV1::DropAllProofs
            | InstructionV1::AllocateGlobalAddress { .. } => {
                self.is_illegal_state = true;
            }
        }

        Ok(())
    }
}

impl TransferTransactionTypeVisitor {
    fn handle_account_withdraw(
        &mut self,
        component_address: ComponentAddress,
        AccountWithdrawInput {
            resource_address,
            amount,
        }: AccountWithdrawInput,
    ) -> Result<(), TransferTransactionTypeError> {
        if let Some(account_withdrawn_from_in_the_past) = self.account_withdrawn_from {
            if component_address != account_withdrawn_from_in_the_past {
                self.is_illegal_state = true;
                return Ok(());
            }
        } else {
            self.account_withdrawn_from = Some(component_address)
        }

        self.worktop
            .put(resource_address, Resources::Amount(amount))?;
        Ok(())
    }

    fn handle_account_withdraw_non_fungibles(
        &mut self,
        component_address: ComponentAddress,
        AccountWithdrawNonFungiblesInput {
            resource_address,
            ids,
        }: AccountWithdrawNonFungiblesInput,
    ) -> Result<(), TransferTransactionTypeError> {
        if let Some(account_withdrawn_from_in_the_past) = self.account_withdrawn_from {
            if component_address != account_withdrawn_from_in_the_past {
                self.is_illegal_state = true;
                return Ok(());
            }
        } else {
            self.account_withdrawn_from = Some(component_address)
        }

        self.worktop.put(resource_address, Resources::Ids(ids))?;
        Ok(())
    }

    fn handle_validation_and_account_deposits(
        &mut self,
        component_address: ComponentAddress,
        args: &ManifestValue,
    ) -> Result<(), TransferTransactionTypeError> {
        // Validate that this follows either the deposit or deposit bach schema. If it does, then
        // extract all buckets and expressions and deal with them.

        let matches_deposit_schema = [
            validate_manifest_value_against_schema::<AccountDepositInput>(args),
            validate_manifest_value_against_schema::<AccountDepositBatchInput>(args),
            validate_manifest_value_against_schema::<AccountTryDepositOrAbortInput>(args),
            validate_manifest_value_against_schema::<AccountTryDepositBatchOrAbortInput>(args),
        ]
        .into_iter()
        .any(|result| result.is_ok());

        if !matches_deposit_schema {
            return Err(TransferTransactionTypeError::InvalidArgs);
        }

        let indexed_manifest_value = IndexedManifestValue::from_manifest_value(args);
        let buckets = indexed_manifest_value.buckets();
        let expressions = indexed_manifest_value.expressions();

        let resources_to_deposit = if !buckets.is_empty() {
            let mut vec = vec![];
            for bucket in buckets {
                let (resource_address, resources) = self
                    .bucket_tracker
                    .remove(bucket)
                    .ok_or(TransferTransactionTypeError::BucketNotFound(*bucket))?;

                vec.push((resource_address, resources))
            }
            Ok(vec)
        } else if !expressions.is_empty() {
            Ok(self.worktop.drain().collect())
        } else {
            Err(TransferTransactionTypeError::InvalidArgs)
        }?;

        for (resource_address, resources) in resources_to_deposit {
            let deposits_map = self.account_deposits.entry(component_address).or_default();
            if let Some(deposited_resources) = deposits_map.get_mut(&resource_address) {
                *deposited_resources = deposited_resources
                    .checked_add(&resources)
                    .ok_or(TransferTransactionTypeError::DepositError)?;
            } else {
                deposits_map.insert(resource_address, resources);
            }
        }

        Ok(())
    }
}

#[derive(Default, Clone, Debug)]
pub struct Worktop {
    id_allocator: ManifestIdAllocator,
    worktop: HashMap<ResourceAddress, Resources>,
}

impl Worktop {
    fn take_all(
        &mut self,
        resource_address: ResourceAddress,
    ) -> Result<(ManifestBucket, Resources), WorktopError> {
        self.worktop
            .remove(&resource_address)
            .map(|resources| {
                let bucket = self.id_allocator.new_bucket_id();
                (bucket, resources)
            })
            .map_or(
                Err(WorktopError::ResourceNotOnWorktop(resource_address)),
                Ok,
            )
    }

    fn take(
        &mut self,
        resource_address: ResourceAddress,
        amount: Decimal,
    ) -> Result<(ManifestBucket, Resources), WorktopError> {
        let worktop_contents =
            self.worktop
                .get_mut(&resource_address)
                .ok_or(WorktopError::ContentValidationError(
                    resource_address,
                    Resources::Amount(amount),
                ))?;

        let bucket = self.id_allocator.new_bucket_id();
        let resources = match worktop_contents {
            Resources::Amount(worktop_amount) => {
                if *worktop_amount >= amount {
                    *worktop_amount = worktop_amount
                        .safe_sub(amount)
                        .ok_or(WorktopError::TakeError)?;
                    Ok(Resources::Amount(amount))
                } else {
                    Err(WorktopError::TakeError)
                }
            }
            Resources::Ids(worktop_ids) => {
                let amount_to_take = amount
                    .to_string()
                    .parse::<usize>()
                    .map_err(|_| WorktopError::NonFungiblesCanNotBeFractionalized)?;

                if worktop_ids.len() >= amount_to_take {
                    let ids_to_take = worktop_ids
                        .iter()
                        .take(amount_to_take)
                        .cloned()
                        .collect::<BTreeSet<_>>();
                    for id in ids_to_take.iter() {
                        worktop_ids.remove(id);
                    }
                    Ok(Resources::Ids(ids_to_take))
                } else {
                    Err(WorktopError::TakeError)
                }
            }
        }?;

        Ok((bucket, resources))
    }

    fn take_non_fungibles(
        &mut self,
        resource_address: ResourceAddress,
        ids: &BTreeSet<NonFungibleLocalId>,
    ) -> Result<(ManifestBucket, Resources), WorktopError> {
        let worktop_contents =
            self.worktop
                .get_mut(&resource_address)
                .ok_or(WorktopError::ContentValidationError(
                    resource_address,
                    Resources::Ids(ids.clone()),
                ))?;

        *worktop_contents = worktop_contents
            .checked_sub_ids(ids)
            .ok_or(WorktopError::TakeError)?;

        let bucket = self.id_allocator.new_bucket_id();
        let resources = Resources::Ids(ids.clone());

        Ok((bucket, resources))
    }

    fn put(
        &mut self,
        resource_address: ResourceAddress,
        resources: Resources,
    ) -> Result<(), WorktopError> {
        if let Some(worktop_contents) = self.worktop.get_mut(&resource_address) {
            *worktop_contents = worktop_contents
                .checked_add(&resources)
                .ok_or(WorktopError::PutError)?;
        } else {
            self.worktop.insert(resource_address, resources);
        };

        Ok(())
    }

    fn drain(&mut self) -> std::collections::hash_map::Drain<ResourceAddress, Resources> {
        self.worktop.drain()
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Ord, Eq)]
pub enum Resources {
    Amount(Decimal),
    Ids(BTreeSet<NonFungibleLocalId>),
}

impl Resources {
    fn checked_add(&self, other: &Self) -> Option<Self> {
        match (self, other) {
            (Self::Amount(amount1), Self::Amount(amount2)) => {
                amount1.safe_add(*amount2).map(Self::Amount)
            }
            (Self::Ids(ids1), Self::Ids(ids2)) => Some(Self::Ids({
                let mut ids = ids1.clone();
                ids.extend(ids2.clone());
                ids
            })),
            _ => None,
        }
    }

    fn checked_sub_ids(&self, other: &BTreeSet<NonFungibleLocalId>) -> Option<Self> {
        match self {
            Self::Ids(ids) => {
                let mut ids = ids.clone();
                for id in other {
                    if !ids.remove(id) {
                        return None;
                    }
                }

                Some(Self::Ids(ids))
            }
            Self::Amount(..) => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum TransferTransactionTypeError {
    InvalidArgs,
    DepositError,
    WorktopError(WorktopError),
    BucketNotFound(ManifestBucket),
}

#[derive(Debug, Clone)]
pub enum WorktopError {
    PutError,
    TakeError,
    ResourceNotOnWorktop(ResourceAddress),
    ContentValidationError(ResourceAddress, Resources),
    NonFungiblesCanNotBeFractionalized,
}

impl From<WorktopError> for TransferTransactionTypeError {
    fn from(value: WorktopError) -> Self {
        Self::WorktopError(value)
    }
}
