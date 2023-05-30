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

#![allow(clippy::map_entry)]

use std::collections::{BTreeMap, BTreeSet, HashMap};

use crate::error::{InstructionVisitorError, VisitorError};
use crate::model::address::utils::is_account;
use crate::model::address::NetworkAwareNodeId;
use crate::model::resource_quantifier::{ResourceManagerSpecifier, ResourceQuantifier};
use crate::model::value::ast::{ManifestAstValue, ManifestAstValueKind};
use crate::visitor::{traverse_value, InstructionVisitor, ManifestAstValueVisitor};
use radix_engine::system::system_modules::execution_trace::{
    ResourceChange, ResourceSpecifier as NativeResourceQuantifier, WorktopChange,
};
use radix_engine::types::ResourceAddress;
use scrypto::blueprints::account::*;
use scrypto::prelude::ManifestExpression;
use toolkit_derive::serializable;

/// A visitor whose main responsibility is analyzing the call-method instructions for proof creation
#[derive(Debug)]
pub struct AccountDepositsInstructionVisitor {
    pub deposits: Vec<AccountDeposit>,
    pub resource_changes: HashMap<u32, Vec<ResourceChange>>,
    pub worktop_changes: HashMap<u32, Vec<WorktopChange>>,
    newly_created_resources: Vec<ResourceAddress>,
    buckets: BTreeMap<String, ResourceSpecifier>,
    instruction_index: u32,
    network_id: u8,
}

impl AccountDepositsInstructionVisitor {
    pub fn new(
        network_id: u8,
        resource_changes: HashMap<u32, Vec<ResourceChange>>,
        worktop_changes: HashMap<u32, Vec<WorktopChange>>,
        newly_created_resources: Vec<ResourceAddress>,
    ) -> Self {
        Self {
            resource_changes,
            worktop_changes,
            deposits: Default::default(),
            buckets: Default::default(),
            instruction_index: Default::default(),
            network_id,
            newly_created_resources,
        }
    }
}

#[serializable]
#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub struct AccountDeposit {
    #[schemars(with = "String")]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub component_address: NetworkAwareNodeId,

    #[serde(flatten)]
    pub deposited: ResourceSpecifier,
}

#[serializable]
#[derive(PartialEq, PartialOrd, Eq, Ord)]
#[serde(tag = "type")]
pub enum ResourceSpecifier {
    Guaranteed {
        /// A specifier of the amount or ids of resources.
        resource_quantifier: ResourceQuantifier,
    },
    Predicted {
        /// The instruction index that that this amount originates from. This might either be an
        /// instruction where a bucket is created of all worktop resources or an instruction where
        /// a deposit is performed of an estimated amount.
        #[schemars(with = "String")]
        #[schemars(regex(pattern = "[0-9]+"))]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        instruction_index: u32,

        /// A specifier of the amount or ids of resources.
        resource_quantifier: ResourceQuantifier,
    },
}

impl InstructionVisitor for AccountDepositsInstructionVisitor {
    //===================
    // Consuming Buckets
    //===================

    fn visit_call_function(
        &mut self,
        _: &mut ManifestAstValue,
        _: &mut ManifestAstValue,
        _: &mut ManifestAstValue,
        arguments: &mut Vec<ManifestAstValue>,
    ) -> Result<(), VisitorError> {
        // Consuming buckets
        let consumed_buckets = {
            let mut arguments = arguments.clone();
            let mut visitor = BucketValueVisitor::default();
            for value in arguments.iter_mut() {
                traverse_value(value, &mut [&mut visitor])?;
            }
            visitor.0
        };
        for bucket_id in consumed_buckets {
            self.buckets.remove(&bucket_id).map_or(
                Err(AccountDepositsVisitorError::UseOfUndeclaredBucket(
                    bucket_id.clone(),
                )),
                |_| Ok(()),
            )?;
        }
        Ok(())
    }

    fn visit_call_method(
        &mut self,
        component_address: &mut ManifestAstValue,
        method_name: &mut ManifestAstValue,
        arguments: &mut Vec<ManifestAstValue>,
    ) -> Result<(), VisitorError> {
        let arguments = arguments.clone();

        // Checking for account deposits
        match (component_address, method_name, &arguments) {
            (
                ManifestAstValue::Address {
                    value: component_address,
                },
                ManifestAstValue::String { value: method_name },
                arguments,
            ) if is_account(*component_address)
                && (method_name == ACCOUNT_DEPOSIT_IDENT
                    || method_name == ACCOUNT_TRY_DEPOSIT_OR_ABORT_IDENT
                    || method_name == ACCOUNT_TRY_DEPOSIT_OR_REFUND_IDENT) =>
            {
                if let Some(ManifestAstValue::Bucket { value: bucket_id }) = arguments.get(0) {
                    let bucket_info = self.buckets.get(bucket_id).map_or(
                        Err(AccountDepositsVisitorError::UseOfUndeclaredBucket(
                            bucket_id.clone(),
                        )),
                        Ok,
                    )?;
                    self.deposits.push(AccountDeposit {
                        component_address: *component_address,
                        deposited: bucket_info.to_owned(),
                    });
                }
            }
            (
                ManifestAstValue::Address {
                    value: component_address,
                },
                ManifestAstValue::String { value: method_name },
                arguments,
            ) if is_account(*component_address)
                && (method_name == ACCOUNT_DEPOSIT_BATCH_IDENT
                    || method_name == ACCOUNT_TRY_DEPOSIT_BATCH_OR_ABORT_IDENT
                    || method_name == ACCOUNT_TRY_DEPOSIT_BATCH_OR_REFUND_IDENT) =>
            {
                match (arguments.len(), arguments.get(0)) {
                    (
                        1,
                        Some(ManifestAstValue::Expression {
                            value: ManifestExpression::EntireWorktop,
                        }),
                    ) => {
                        let resource_changes = self
                            .resource_changes
                            .get(&self.instruction_index)
                            .map_or(
                                Err(AccountDepositsVisitorError::NoCorrespondingResourceChangesForInstruction(
                                 self.instruction_index,
                                )),
                                Ok,
                            )?
                            .iter()
                            .filter(
                                |ResourceChange {
                                     node_id, amount, ..
                                 }| {
                                    *node_id == component_address.node_id() && amount.is_positive()
                                },
                            )
                            .collect::<Vec<&ResourceChange>>();

                        for resource_change in resource_changes {
                            self.deposits.push(AccountDeposit {
                                component_address: *component_address,
                                deposited: ResourceSpecifier::Predicted {
                                    instruction_index: self.instruction_index,
                                    resource_quantifier: ResourceQuantifier::Amount {
                                        resource_address: self
                                            .newly_created_resources
                                            .iter()
                                            .position(|address| {
                                                *address == resource_change.resource_address
                                            })
                                            .map_or(
                                                ResourceManagerSpecifier::Existing {
                                                    address: NetworkAwareNodeId(
                                                        resource_change
                                                            .resource_address
                                                            .as_node_id()
                                                            .0,
                                                        self.network_id,
                                                    ),
                                                },
                                                |index| ResourceManagerSpecifier::NewlyCreated {
                                                    index: index as u32,
                                                },
                                            ),
                                        amount: resource_change.amount,
                                    },
                                },
                            });
                        }
                    }
                    (
                        1,
                        Some(ManifestAstValue::Array {
                            element_kind: ManifestAstValueKind::Bucket,
                            elements: bucket_ids,
                        }),
                    ) => {
                        let bucket_ids = {
                            let mut resolved_bucket_ids = BTreeSet::new();
                            for bucket_id in bucket_ids {
                                if let ManifestAstValue::Bucket { value: identifier } = bucket_id {
                                    resolved_bucket_ids.insert(identifier.clone());
                                } else { /* TODO: currently a no-op. Should be an error? */
                                }
                            }
                            resolved_bucket_ids
                        };
                        for bucket_id in bucket_ids {
                            let bucket_info = self.buckets.get(&bucket_id).map_or(
                                Err(AccountDepositsVisitorError::UseOfUndeclaredBucket(
                                    bucket_id.clone(),
                                )),
                                Ok,
                            )?;
                            self.deposits.push(AccountDeposit {
                                component_address: *component_address,
                                deposited: bucket_info.to_owned(),
                            });
                        }
                    }
                    (_, _) => { /* No OP. TODO: Should be an error? */ }
                }
            }
            _ => {}
        }

        // Consuming buckets
        let consumed_buckets = {
            let mut arguments = arguments;
            let mut visitor = BucketValueVisitor::default();
            for value in arguments.iter_mut() {
                traverse_value(value, &mut [&mut visitor])?;
            }
            visitor.0
        };
        for bucket_id in consumed_buckets {
            self.buckets.remove(&bucket_id).map_or(
                Err(AccountDepositsVisitorError::UseOfUndeclaredBucket(
                    bucket_id.clone(),
                )),
                |_| Ok(()),
            )?;
        }
        Ok(())
    }

    fn visit_take_all_from_worktop(
        &mut self,
        resource_address: &mut ManifestAstValue,
        into_bucket: &mut ManifestAstValue,
    ) -> Result<(), VisitorError> {
        match (resource_address, into_bucket) {
            (
                ManifestAstValue::Address {
                    value: resource_address,
                },
                ManifestAstValue::Bucket { value: bucket_id },
            ) if (resource_address
                .node_id()
                .is_global_fungible_resource_manager()
                || resource_address
                    .node_id()
                    .is_global_non_fungible_resource_manager()) =>
            {
                if let Some(worktop_changes) = self.worktop_changes.get(&self.instruction_index) {
                    if let Some(WorktopChange::Take(resource_quantifier)) = worktop_changes.get(0) {
                        self.add_bucket(
                            bucket_id.clone(),
                            ResourceSpecifier::Predicted {
                                instruction_index: self.instruction_index,
                                resource_quantifier: match resource_quantifier {
                                    NativeResourceQuantifier::Amount(_, amount) => {
                                        ResourceQuantifier::Amount {
                                            resource_address: ResourceManagerSpecifier::Existing {
                                                address: *resource_address,
                                            },
                                            amount: *amount,
                                        }
                                    }
                                    NativeResourceQuantifier::Ids(_, ids) => {
                                        ResourceQuantifier::Ids {
                                            resource_address: ResourceManagerSpecifier::Existing {
                                                address: *resource_address,
                                            },
                                            ids: ids.clone(),
                                        }
                                    }
                                },
                            },
                        )?;
                    }
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn visit_take_from_worktop(
        &mut self,
        resource_address: &mut ManifestAstValue,
        amount: &mut ManifestAstValue,
        into_bucket: &mut ManifestAstValue,
    ) -> Result<(), VisitorError> {
        match (resource_address, amount, into_bucket) {
            (
                ManifestAstValue::Address {
                    value: resource_address,
                },
                ManifestAstValue::Decimal { value: amount },
                ManifestAstValue::Bucket { value: bucket_id },
            ) if (resource_address
                .node_id()
                .is_global_fungible_resource_manager()
                || resource_address
                    .node_id()
                    .is_global_non_fungible_resource_manager()) =>
            {
                self.add_bucket(
                    bucket_id.clone(),
                    ResourceSpecifier::Guaranteed {
                        resource_quantifier: ResourceQuantifier::Amount {
                            resource_address: ResourceManagerSpecifier::Existing {
                                address: *resource_address,
                            },
                            amount: *amount,
                        },
                    },
                )?;
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn visit_take_non_fungibles_from_worktop(
        &mut self,
        resource_address: &mut ManifestAstValue,
        ids: &mut Vec<ManifestAstValue>,
        into_bucket: &mut ManifestAstValue,
    ) -> Result<(), VisitorError> {
        match (resource_address, ids, into_bucket) {
            (
                ManifestAstValue::Address {
                    value: resource_address,
                },
                ids,
                ManifestAstValue::Bucket { value: bucket_id },
            ) if (resource_address
                .node_id()
                .is_global_fungible_resource_manager()
                || resource_address
                    .node_id()
                    .is_global_non_fungible_resource_manager()) =>
            {
                let ids = {
                    let mut resolved_ids = BTreeSet::new();
                    for id in ids {
                        if let ManifestAstValue::NonFungibleLocalId { value: id } = id {
                            resolved_ids.insert(id.clone());
                        } else { /* TODO: currently a no-op. Should be an error? */
                        }
                    }
                    resolved_ids
                };
                self.add_bucket(
                    bucket_id.clone(),
                    ResourceSpecifier::Guaranteed {
                        resource_quantifier: ResourceQuantifier::Ids {
                            ids,
                            resource_address: ResourceManagerSpecifier::Existing {
                                address: *resource_address,
                            },
                        },
                    },
                )?;
                Ok(())
            }
            _ => Ok(()),
        }
    }

    //==================
    // Creating Buckets
    //==================

    fn visit_return_to_worktop(
        &mut self,
        bucket: &mut ManifestAstValue,
    ) -> Result<(), VisitorError> {
        if let ManifestAstValue::Bucket { value: bucket_id } = bucket {
            self.buckets.remove(bucket_id).map_or(
                Err(AccountDepositsVisitorError::UseOfUndeclaredBucket(
                    bucket_id.clone(),
                ))?,
                |_| Ok(()),
            )
        } else {
            // TODO: Should be an error?
            Ok(())
        }
    }

    fn visit_burn_resource(&mut self, bucket: &mut ManifestAstValue) -> Result<(), VisitorError> {
        if let ManifestAstValue::Bucket { value: bucket_id } = bucket {
            self.buckets.remove(bucket_id).map_or(
                Err(AccountDepositsVisitorError::UseOfUndeclaredBucket(
                    bucket_id.clone(),
                ))?,
                |_| Ok(()),
            )
        } else {
            // TODO: Should be an error?
            Ok(())
        }
    }

    fn visit_create_access_controller(
        &mut self,
        bucket: &mut ManifestAstValue,
        _: &mut ManifestAstValue,
        _: &mut ManifestAstValue,
    ) -> Result<(), VisitorError> {
        if let ManifestAstValue::Bucket { value: bucket_id } = bucket {
            self.buckets.remove(bucket_id).map_or(
                Err(AccountDepositsVisitorError::UseOfUndeclaredBucket(
                    bucket_id.clone(),
                ))?,
                |_| Ok(()),
            )
        } else {
            // TODO: Should be an error?
            Ok(())
        }
    }

    //=================
    // Post Processing
    //=================

    fn post_visit(&mut self) -> Result<(), VisitorError> {
        self.instruction_index += 1;
        Ok(())
    }
}

impl AccountDepositsInstructionVisitor {
    pub fn add_bucket(
        &mut self,
        bucket_id: String,
        specifier: ResourceSpecifier,
    ) -> Result<(), VisitorError> {
        if !self.buckets.contains_key(&bucket_id) {
            self.buckets.insert(bucket_id, specifier);
            Ok(())
        } else {
            Err(AccountDepositsVisitorError::DuplicateBucketDeclaration(bucket_id).into())
        }
    }
}

#[derive(Default)]
struct BucketValueVisitor(Vec<String>);

impl ManifestAstValueVisitor for BucketValueVisitor {
    fn visit_bucket(&mut self, bucket: &mut ManifestAstValue) -> Result<(), VisitorError> {
        if let ManifestAstValue::Bucket { value: bucket_id } = bucket {
            self.0.push(bucket_id.to_owned());
        }
        Ok(())
    }
}

#[serializable]
pub enum AccountDepositsVisitorError {
    DuplicateBucketDeclaration(String),
    UseOfUndeclaredBucket(String),
    NoCorrespondingResourceChangesForInstruction(u32),
}

impl From<AccountDepositsVisitorError> for VisitorError {
    fn from(value: AccountDepositsVisitorError) -> Self {
        Self::InstructionVisitorError(InstructionVisitorError::AccountDepositsVisitorError(value))
    }
}
