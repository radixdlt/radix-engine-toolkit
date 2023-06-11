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

use radix_engine::system::system_modules::execution_trace::ResourceSpecifier;
use scrypto::blueprints::account::*;
use scrypto::prelude::*;
use transaction::validation::ManifestIdAllocator;

use crate::sbor::indexed_manifest_value::IndexedManifestValue;
use crate::utils::{self, to_manifest_type};
use crate::{instruction_visitor::core::traits::InstructionVisitor, utils::is_account};

#[derive(Default, Clone, Debug)]
pub struct TransferTransactionTypeVisitor {
    // The allocator that is used for allocating buckets.
    id_allocator: ManifestIdAllocator,

    // Tracks the amount of resources currently in the worktop.
    worktop_tracker: HashMap<ResourceAddress, ResourceQuantifier>,

    // Tracks the contents of the bucket.
    bucket_tracker: HashMap<ManifestBucket, ResourceSpecifier>,

    // Tracks which accounts were withdrawn from.
    account_withdrawn_from: Option<ComponentAddress>,

    // Tracks the accounts deposited into and the quantity.
    account_deposits: HashMap<ComponentAddress, HashMap<ResourceAddress, ResourceQuantifier>>,

    // Tracks if the visitor is currently in an illegal state or not.
    is_illegal_state: bool,
}

impl InstructionVisitor for TransferTransactionTypeVisitor {
    type Error = TransferTransactionTypeError;
    type Output = Option<(
        ComponentAddress,
        HashMap<ComponentAddress, HashMap<ResourceAddress, ResourceQuantifier>>,
    )>;

    fn output(self) -> Self::Output {
        if self.is_illegal_state {
            None
        } else if self.account_withdrawn_from.is_some() && !self.account_deposits.is_empty() {
            Some((self.account_withdrawn_from.unwrap(), self.account_deposits))
        } else {
            None
        }
    }

    fn is_enabled(&self) -> bool {
        !self.is_illegal_state
    }

    fn visit_call_method(
        &mut self,
        address: &GlobalAddress,
        method_name: &str,
        args: &ManifestValue,
    ) -> Result<(), Self::Error> {
        if is_account(address.as_node_id()) {
            let component_address = ComponentAddress::new_or_panic(address.as_node_id().0);

            // Only allowed account methods are: withdraw, withdraw_non_fungibles, deposit, and
            // deposit batch.
            if let (
                ACCOUNT_WITHDRAW_IDENT,
                Some(AccountWithdrawInput {
                    resource_address,
                    amount,
                }),
            ) = (method_name, to_manifest_type(args))
            {
                self.if_withdraw_account(component_address, |this| {
                    this.handle_worktop_fungible_deposit(resource_address, amount)
                });
            } else if let (
                ACCOUNT_LOCK_FEE_AND_WITHDRAW_IDENT,
                Some(AccountLockFeeAndWithdrawInput {
                    resource_address,
                    amount,
                    ..
                }),
            ) = (method_name, to_manifest_type(args))
            {
                self.if_withdraw_account(component_address, |this| {
                    this.handle_worktop_fungible_deposit(resource_address, amount)
                });
            } else if let (
                ACCOUNT_WITHDRAW_NON_FUNGIBLES_IDENT,
                Some(AccountWithdrawNonFungiblesInput {
                    resource_address,
                    ids,
                }),
            ) = (method_name, to_manifest_type(args))
            {
                self.if_withdraw_account(component_address, |this| {
                    this.handle_non_fungible_worktop_deposit(resource_address, ids)
                });
            } else if let (
                ACCOUNT_LOCK_FEE_AND_WITHDRAW_NON_FUNGIBLES_IDENT,
                Some(AccountLockFeeAndWithdrawNonFungiblesInput {
                    resource_address,
                    ids,
                    ..
                }),
            ) = (method_name, to_manifest_type(args))
            {
                self.if_withdraw_account(component_address, |this| {
                    this.handle_non_fungible_worktop_deposit(resource_address, ids)
                });
            } else if let ACCOUNT_DEPOSIT_IDENT
            | ACCOUNT_TRY_DEPOSIT_OR_ABORT_IDENT
            | ACCOUNT_DEPOSIT_BATCH_IDENT
            | ACCOUNT_TRY_DEPOSIT_BATCH_OR_ABORT_IDENT = method_name
            {
                // Validate against the schema
                let matches_deposit_schema = [
                    utils::validate_manifest_value_against_schema::<AccountDepositInput>(args),
                    utils::validate_manifest_value_against_schema::<AccountTryDepositOrAbortInput>(
                        args,
                    ),
                    utils::validate_manifest_value_against_schema::<AccountDepositBatchInput>(args),
                    utils::validate_manifest_value_against_schema::<
                        AccountTryDepositBatchOrAbortInput,
                    >(args),
                ]
                .into_iter()
                .any(|result| result.is_ok());

                if matches_deposit_schema {
                    let indexed_manifest_value = IndexedManifestValue::from_manifest_value(args);
                    if !indexed_manifest_value.expressions().is_empty() {
                        if let Some(deposits) = self.account_deposits.get_mut(&component_address) {
                            for (resource_address, worktop_amount) in self.worktop_tracker.drain() {
                                if let Some(resource_deposit) = deposits.get_mut(&resource_address)
                                {
                                    *resource_deposit =
                                        resource_deposit.checked_add(&worktop_amount)?;
                                } else {
                                    deposits.insert(resource_address, worktop_amount);
                                }
                            }
                        } else {
                            self.account_deposits
                                .insert(component_address, self.worktop_tracker.drain().collect());
                        }
                    } else if !indexed_manifest_value.buckets().is_empty() {
                        for bucket in indexed_manifest_value.buckets() {
                            let bucket_contents = self.bucket_tracker.remove(bucket).map_or_else(
                                || self.error(Self::Error::InvalidBucket(*bucket)),
                                Ok,
                            )?;

                            let resource_address = match bucket_contents {
                                ResourceSpecifier::Amount(address, ..) => address,
                                ResourceSpecifier::Ids(address, ..) => address,
                            };

                            if let Some(deposits) =
                                self.account_deposits.get_mut(&component_address)
                            {
                                if let Some(resource_deposits) = deposits.get_mut(&resource_address)
                                {
                                    match (bucket_contents, resource_deposits) {
                                        (
                                            ResourceSpecifier::Amount(_, bucket_amount),
                                            ResourceQuantifier::Amount(deposited_amount),
                                        ) => *deposited_amount += bucket_amount,
                                        (
                                            ResourceSpecifier::Ids(_, bucket_ids),
                                            ResourceQuantifier::Ids(deposited_ids),
                                        ) => {
                                            deposited_ids.extend(bucket_ids);
                                        }
                                        _ => {
                                            return self
                                                .error(Self::Error::InvalidWorktopAndBucketState)
                                        }
                                    }
                                } else {
                                    deposits.insert(resource_address, bucket_contents.into());
                                }
                            } else {
                                self.account_deposits.insert(
                                    component_address,
                                    [(resource_address, bucket_contents.into())].into(),
                                );
                            }
                        }
                    } else {
                        self.is_illegal_state = true
                    }
                } else {
                    self.is_illegal_state = true
                }
            } else {
                self.is_illegal_state = true;
            }
        } else {
            self.is_illegal_state = true
        };

        Ok(())
    }

    /* Bucket tracking methods */

    fn visit_take_from_worktop(
        &mut self,
        resource_address: &ResourceAddress,
        amount: &Decimal,
    ) -> Result<(), Self::Error> {
        if let Some(worktop_contents) = self.worktop_tracker.get_mut(resource_address) {
            let resource_specifier = match worktop_contents {
                ResourceQuantifier::Amount(ref mut worktop_amount) => {
                    if *worktop_amount < *amount {
                        return self.error(TransferTransactionTypeError::NotEnoughBalance);
                    }

                    *worktop_amount -= *amount;
                    ResourceSpecifier::Amount(*resource_address, *amount)
                }
                ResourceQuantifier::Ids(ref mut worktop_ids) => {
                    let amount_to_take = amount.to_string().parse::<usize>().map_err(|_| {
                        self.is_illegal_state = true;
                        Self::Error::CantFractionalizeNonFungible
                    })?;

                    if worktop_ids.len() < amount_to_take {
                        return self.error(TransferTransactionTypeError::NotEnoughBalance);
                    }

                    let ids = worktop_ids
                        .iter()
                        .take(amount_to_take)
                        .cloned()
                        .collect::<BTreeSet<NonFungibleLocalId>>();
                    for id in ids.iter() {
                        worktop_ids.remove(id);
                    }

                    ResourceSpecifier::Ids(*resource_address, ids)
                }
            };

            let bucket = self.id_allocator.new_bucket_id();
            self.bucket_tracker.insert(bucket, resource_specifier);

            Ok(())
        } else {
            self.error(TransferTransactionTypeError::NotEnoughBalance)
        }
    }

    fn visit_take_non_fungibles_from_worktop(
        &mut self,
        resource_address: &ResourceAddress,
        ids: &[NonFungibleLocalId],
    ) -> Result<(), Self::Error> {
        if let Some(worktop_contents) = self.worktop_tracker.get_mut(resource_address) {
            let resource_specifier = match worktop_contents {
                ResourceQuantifier::Ids(ref mut worktop_ids) => {
                    for id in ids {
                        if worktop_ids.remove(id) {
                            Ok(())
                        } else {
                            self.is_illegal_state = true;
                            Err(
                                TransferTransactionTypeError::NonFungibleLocalIdNotInWorktop(
                                    id.clone(),
                                ),
                            )
                        }?;
                    }

                    Ok(ResourceSpecifier::Ids(
                        *resource_address,
                        ids.iter().cloned().collect(),
                    ))
                }
                ResourceQuantifier::Amount(_) => {
                    self.error(Self::Error::NonFungiblesWereWithdrawnAsFungibles)
                }
            }?;

            let bucket = self.id_allocator.new_bucket_id();
            self.bucket_tracker.insert(bucket, resource_specifier);

            Ok(())
        } else {
            self.error(TransferTransactionTypeError::NotEnoughBalance)
        }
    }

    fn visit_take_all_from_worktop(
        &mut self,
        resource_address: &ResourceAddress,
    ) -> Result<(), Self::Error> {
        if let Some(worktop_contents) = self.worktop_tracker.get_mut(resource_address) {
            let resource_specifier = match worktop_contents {
                ResourceQuantifier::Ids(ref mut worktop_ids) => {
                    let ids = worktop_ids.clone();
                    worktop_ids.clear();
                    ResourceSpecifier::Ids(*resource_address, ids)
                }
                ResourceQuantifier::Amount(amount) => {
                    let resource_specifier = ResourceSpecifier::Amount(*resource_address, *amount);
                    *amount = Decimal::ZERO;
                    resource_specifier
                }
            };

            let bucket = self.id_allocator.new_bucket_id();
            self.bucket_tracker.insert(bucket, resource_specifier);

            Ok(())
        } else {
            self.error(TransferTransactionTypeError::NotEnoughBalance)
        }
    }

    fn visit_return_to_worktop(&mut self, bucket_id: &ManifestBucket) -> Result<(), Self::Error> {
        let resource_specifier = self
            .bucket_tracker
            .remove(bucket_id)
            .ok_or(TransferTransactionTypeError::InvalidBucket(*bucket_id))?;

        let resource_address = match resource_specifier {
            ResourceSpecifier::Amount(address, ..) => address,
            ResourceSpecifier::Ids(address, ..) => address,
        };

        if let Some(worktop_contents) = self.worktop_tracker.get_mut(&resource_address) {
            match (resource_specifier, worktop_contents) {
                (
                    ResourceSpecifier::Amount(_, bucket_amount),
                    ResourceQuantifier::Amount(worktop_amount),
                ) => {
                    *worktop_amount += bucket_amount;
                    Ok(())
                }
                (ResourceSpecifier::Ids(_, bucket_ids), ResourceQuantifier::Ids(worktop_ids)) => {
                    worktop_ids.extend(bucket_ids);
                    Ok(())
                }
                _ => self.error(Self::Error::InvalidWorktopAndBucketState),
            }
        } else {
            self.worktop_tracker
                .insert(resource_address, resource_specifier.into());

            Ok(())
        }
    }

    /* Illegal State Instructions */

    #[inline]
    fn visit_pop_from_auth_zone(&mut self) -> Result<(), Self::Error> {
        self.is_illegal_state = true;
        Ok(())
    }

    #[inline]
    fn visit_push_to_auth_zone(&mut self, _: &ManifestProof) -> Result<(), Self::Error> {
        self.is_illegal_state = true;
        Ok(())
    }

    #[inline]
    fn visit_clear_auth_zone(&mut self) -> Result<(), Self::Error> {
        self.is_illegal_state = true;
        Ok(())
    }

    #[inline]
    fn visit_create_proof_from_auth_zone(
        &mut self,
        _: &ResourceAddress,
    ) -> Result<(), Self::Error> {
        self.is_illegal_state = true;
        Ok(())
    }

    #[inline]
    fn visit_create_proof_from_auth_zone_of_amount(
        &mut self,
        _: &ResourceAddress,
        _: &Decimal,
    ) -> Result<(), Self::Error> {
        self.is_illegal_state = true;
        Ok(())
    }

    #[inline]
    fn visit_create_proof_from_auth_zone_of_non_fungibles(
        &mut self,
        _: &ResourceAddress,
        _: &[NonFungibleLocalId],
    ) -> Result<(), Self::Error> {
        self.is_illegal_state = true;
        Ok(())
    }

    #[inline]
    fn visit_create_proof_from_auth_zone_of_all(
        &mut self,
        _: &ResourceAddress,
    ) -> Result<(), Self::Error> {
        self.is_illegal_state = true;
        Ok(())
    }

    #[inline]
    fn visit_clear_signature_proofs(&mut self) -> Result<(), Self::Error> {
        self.is_illegal_state = true;
        Ok(())
    }

    #[inline]
    fn visit_create_proof_from_bucket(&mut self, _: &ManifestBucket) -> Result<(), Self::Error> {
        self.is_illegal_state = true;
        Ok(())
    }

    #[inline]
    fn visit_create_proof_from_bucket_of_amount(
        &mut self,
        _: &ManifestBucket,
        _: &Decimal,
    ) -> Result<(), Self::Error> {
        self.is_illegal_state = true;
        Ok(())
    }

    #[inline]
    fn visit_create_proof_from_bucket_of_non_fungibles(
        &mut self,
        _: &ManifestBucket,
        _: &[NonFungibleLocalId],
    ) -> Result<(), Self::Error> {
        self.is_illegal_state = true;
        Ok(())
    }

    #[inline]
    fn visit_create_proof_from_bucket_of_all(
        &mut self,
        _: &ManifestBucket,
    ) -> Result<(), Self::Error> {
        self.is_illegal_state = true;
        Ok(())
    }

    #[inline]
    fn visit_burn_resource(&mut self, _: &ManifestBucket) -> Result<(), Self::Error> {
        self.is_illegal_state = true;
        Ok(())
    }

    #[inline]
    fn visit_clone_proof(&mut self, _: &ManifestProof) -> Result<(), Self::Error> {
        self.is_illegal_state = true;
        Ok(())
    }

    #[inline]
    fn visit_drop_proof(&mut self, _: &ManifestProof) -> Result<(), Self::Error> {
        self.is_illegal_state = true;
        Ok(())
    }

    #[inline]
    fn visit_call_function(
        &mut self,
        _: &PackageAddress,
        _: &str,
        _: &str,
        _: &ManifestValue,
    ) -> Result<(), Self::Error> {
        self.is_illegal_state = true;
        Ok(())
    }

    #[inline]
    fn visit_call_royalty_method(
        &mut self,
        _: &GlobalAddress,
        _: &str,
        _: &ManifestValue,
    ) -> Result<(), Self::Error> {
        self.is_illegal_state = true;
        Ok(())
    }

    #[inline]
    fn visit_call_metadata_method(
        &mut self,
        _: &GlobalAddress,
        _: &str,
        _: &ManifestValue,
    ) -> Result<(), Self::Error> {
        self.is_illegal_state = true;
        Ok(())
    }

    #[inline]
    fn visit_call_access_rules_method(
        &mut self,
        _: &GlobalAddress,
        _: &str,
        _: &ManifestValue,
    ) -> Result<(), Self::Error> {
        self.is_illegal_state = true;
        Ok(())
    }

    #[inline]
    fn visit_call_direct_vault_method(
        &mut self,
        _: &InternalAddress,
        _: &str,
        _: &ManifestValue,
    ) -> Result<(), Self::Error> {
        self.is_illegal_state = true;
        Ok(())
    }

    #[inline]
    fn visit_drop_all_proofs(&mut self) -> Result<(), Self::Error> {
        self.is_illegal_state = true;
        Ok(())
    }
}

impl TransferTransactionTypeVisitor {
    fn error<T>(
        &mut self,
        error: <TransferTransactionTypeVisitor as InstructionVisitor>::Error,
    ) -> Result<T, <TransferTransactionTypeVisitor as InstructionVisitor>::Error> {
        self.is_illegal_state = true;
        Err(error)
    }

    fn handle_worktop_fungible_deposit(
        &mut self,
        resource_address: ResourceAddress,
        amount: Decimal,
    ) {
        if let Some(worktop_contents) = self.worktop_tracker.get_mut(&resource_address) {
            match worktop_contents {
                ResourceQuantifier::Amount(worktop_amount) => {
                    *worktop_amount += amount;
                }
                ResourceQuantifier::Ids(_) => {
                    self.is_illegal_state = true;
                }
            }
        } else {
            self.worktop_tracker
                .insert(resource_address, ResourceQuantifier::Amount(amount));
        }
    }

    fn handle_non_fungible_worktop_deposit(
        &mut self,
        resource_address: ResourceAddress,
        ids: BTreeSet<NonFungibleLocalId>,
    ) {
        if let Some(worktop_contents) = self.worktop_tracker.get_mut(&resource_address) {
            match worktop_contents {
                ResourceQuantifier::Ids(worktop_ids) => worktop_ids.extend(ids),
                ResourceQuantifier::Amount(_) => {
                    self.is_illegal_state = true;
                }
            }
        } else {
            self.worktop_tracker
                .insert(resource_address, ResourceQuantifier::Ids(ids));
        }
    }

    fn if_withdraw_account<F, T>(
        &mut self,
        component_address: ComponentAddress,
        callback: F,
    ) -> Option<T>
    where
        F: FnOnce(&mut Self) -> T,
    {
        if self
            .account_withdrawn_from
            .map_or(true, |account| account == component_address)
        {
            self.account_withdrawn_from = Some(component_address);
            Some(callback(self))
        } else {
            self.is_illegal_state = true;
            None
        }
    }
}

#[derive(Debug, Clone)]
pub enum TransferTransactionTypeError {
    NotEnoughBalance,
    NonFungibleLocalIdNotInWorktop(NonFungibleLocalId),
    CantFractionalizeNonFungible,
    NonFungiblesWereWithdrawnAsFungibles,
    InvalidBucket(ManifestBucket),
    InvalidWorktopAndBucketState,
    InvalidResources,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum ResourceQuantifier {
    Amount(Decimal),
    Ids(BTreeSet<NonFungibleLocalId>),
}

impl From<ResourceSpecifier> for ResourceQuantifier {
    fn from(value: ResourceSpecifier) -> Self {
        match value {
            ResourceSpecifier::Amount(_, amount) => Self::Amount(amount),
            ResourceSpecifier::Ids(_, ids) => Self::Ids(ids),
        }
    }
}

impl ResourceQuantifier {
    fn checked_add(&self, other: &Self) -> Result<Self, TransferTransactionTypeError> {
        match (self, other) {
            (Self::Amount(amount1), Self::Amount(amount2)) => Ok(Self::Amount(*amount1 + *amount2)),
            (Self::Ids(ids1), Self::Ids(ids2)) => {
                let mut ids = ids1.clone();
                ids.extend(ids2.clone());
                Ok(Self::Ids(ids))
            }
            _ => Err(TransferTransactionTypeError::InvalidResources),
        }
    }
}
