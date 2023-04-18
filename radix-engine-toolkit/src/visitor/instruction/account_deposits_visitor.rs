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

use crate::error::{Error, Result};
use crate::model::address::NetworkAwareNodeId;
use crate::model::engine_identifier::BucketId;
use crate::model::resource_specifier::ResourceSpecifier;
use crate::model::value::ast::{ManifestAstValue, ManifestAstValueKind};
use crate::utils::is_account;
use crate::visitor::{traverse_value, InstructionVisitor, ManifestAstValueVisitor};
use radix_engine::system::system_modules::execution_trace::{
    ResourceChange, ResourceSpecifier as NativeResourceSpecifier, WorktopChange,
};
use scrypto::blueprints::account::{ACCOUNT_DEPOSIT_BATCH_IDENT, ACCOUNT_DEPOSIT_IDENT};
use scrypto::prelude::ManifestExpression;
use toolkit_derive::serializable;

/// A visitor whose main responsibility is analyzing the call-method instructions for proof creation
#[derive(Debug)]
pub struct AccountDepositsInstructionVisitor {
    pub deposits: Vec<AccountDeposit>,
    pub resource_changes: HashMap<u32, Vec<ResourceChange>>,
    pub worktop_changes: HashMap<u32, Vec<WorktopChange>>,
    buckets: BTreeMap<BucketId, ExactnessSpecifier>,
    instruction_index: u32,
    network_id: u8,
}

impl AccountDepositsInstructionVisitor {
    pub fn new(
        network_id: u8,
        resource_changes: HashMap<u32, Vec<ResourceChange>>,
        worktop_changes: HashMap<u32, Vec<WorktopChange>>,
    ) -> Self {
        Self {
            resource_changes,
            worktop_changes,
            deposits: Default::default(),
            buckets: Default::default(),
            instruction_index: Default::default(),
            network_id,
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
    pub deposited: ExactnessSpecifier,
}

#[serializable]
#[derive(PartialEq, PartialOrd, Eq, Ord)]
#[serde(tag = "type")]
pub enum ExactnessSpecifier {
    Exact {
        /// A specifier of the amount or ids of resources.
        resource_specifier: ResourceSpecifier,
    },
    Estimate {
        /// The instruction index that that this amount originates from. This might either be an
        /// instruction where a bucket is created of all worktop resources or an instruction where
        /// a deposit is performed of an estimated amount.
        #[schemars(with = "String")]
        #[schemars(regex(pattern = "[0-9]+"))]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        instruction_index: u32,

        /// A specifier of the amount or ids of resources.
        resource_specifier: ResourceSpecifier,
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
        arguments: &mut Option<Vec<ManifestAstValue>>,
    ) -> crate::error::Result<()> {
        // Consuming buckets
        let consumed_buckets = {
            let mut arguments = arguments.clone().unwrap_or_default();
            let mut visitor = BucketValueVisitor::default();
            for value in arguments.iter_mut() {
                traverse_value(value, &mut [&mut visitor])?;
            }
            visitor.0
        };
        for bucket_id in consumed_buckets {
            self.buckets.remove(&bucket_id).map_or(
                Err(Error::InvalidBucketId {
                    bucket_id: bucket_id.clone(),
                }),
                |_| Ok(()),
            )?;
        }
        Ok(())
    }

    fn visit_call_method(
        &mut self,
        component_address: &mut ManifestAstValue,
        method_name: &mut ManifestAstValue,
        arguments: &mut Option<Vec<ManifestAstValue>>,
    ) -> crate::error::Result<()> {
        let arguments = arguments.clone().unwrap_or_default();

        // Checking for account deposits
        match (component_address, method_name, &arguments) {
            (
                ManifestAstValue::Address {
                    address: component_address,
                },
                ManifestAstValue::String { value: method_name },
                arguments,
            ) if is_account(*component_address) && method_name == ACCOUNT_DEPOSIT_IDENT => {
                if let Some(ManifestAstValue::Bucket {
                    identifier: bucket_id,
                }) = arguments.get(0)
                {
                    let bucket_info = self.buckets.get(bucket_id).map_or(
                        Err(Error::InvalidBucketId {
                            bucket_id: bucket_id.clone(),
                        }),
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
                    address: component_address,
                },
                ManifestAstValue::String { value: method_name },
                arguments,
            ) if is_account(*component_address) && method_name == ACCOUNT_DEPOSIT_BATCH_IDENT => {
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
                                Err(Error::NoResourceChangesForInstruction {
                                    instruction_index: self.instruction_index,
                                }),
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
                                deposited: ExactnessSpecifier::Estimate {
                                    instruction_index: self.instruction_index,
                                    resource_specifier: ResourceSpecifier::Amount {
                                        resource_address: NetworkAwareNodeId(
                                            resource_change.resource_address.as_node_id().0,
                                            self.network_id,
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
                                if let ManifestAstValue::Bucket { identifier } = bucket_id {
                                    resolved_bucket_ids.insert(identifier.clone());
                                } else { /* TODO: currently a no-op. Should be an error? */
                                }
                            }
                            resolved_bucket_ids
                        };
                        for bucket_id in bucket_ids {
                            let bucket_info = self.buckets.get(&bucket_id).map_or(
                                Err(Error::InvalidBucketId {
                                    bucket_id: bucket_id.clone(),
                                }),
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
                Err(Error::InvalidBucketId {
                    bucket_id: bucket_id.clone(),
                }),
                |_| Ok(()),
            )?;
        }
        Ok(())
    }

    fn visit_take_from_worktop(
        &mut self,
        resource_address: &mut ManifestAstValue,
        into_bucket: &mut ManifestAstValue,
    ) -> Result<()> {
        match (resource_address, into_bucket) {
            (
                ManifestAstValue::Address {
                    address: resource_address,
                },
                ManifestAstValue::Bucket {
                    identifier: bucket_id,
                },
            ) if resource_address.node_id().is_global_resource() => {
                if let Some(worktop_changes) = self.worktop_changes.get(&self.instruction_index) {
                    if let Some(WorktopChange::Take(resource_specifier)) = worktop_changes.get(0) {
                        self.add_bucket(
                            bucket_id.clone(),
                            ExactnessSpecifier::Estimate {
                                instruction_index: self.instruction_index,
                                resource_specifier: match resource_specifier {
                                    NativeResourceSpecifier::Amount(_, amount) => {
                                        ResourceSpecifier::Amount {
                                            resource_address: *resource_address,
                                            amount: *amount,
                                        }
                                    }
                                    NativeResourceSpecifier::Ids(_, ids) => {
                                        ResourceSpecifier::Ids {
                                            resource_address: *resource_address,
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

    fn visit_take_from_worktop_by_amount(
        &mut self,
        resource_address: &mut ManifestAstValue,
        amount: &mut ManifestAstValue,
        into_bucket: &mut ManifestAstValue,
    ) -> Result<()> {
        match (resource_address, amount, into_bucket) {
            (
                ManifestAstValue::Address {
                    address: resource_address,
                },
                ManifestAstValue::Decimal { value: amount },
                ManifestAstValue::Bucket {
                    identifier: bucket_id,
                },
            ) if resource_address.node_id().is_global_resource() => {
                self.add_bucket(
                    bucket_id.clone(),
                    ExactnessSpecifier::Exact {
                        resource_specifier: ResourceSpecifier::Amount {
                            resource_address: *resource_address,
                            amount: *amount,
                        },
                    },
                )?;
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn visit_take_from_worktop_by_ids(
        &mut self,
        resource_address: &mut ManifestAstValue,
        ids: &mut Vec<ManifestAstValue>,
        into_bucket: &mut ManifestAstValue,
    ) -> Result<()> {
        match (resource_address, ids, into_bucket) {
            (
                ManifestAstValue::Address {
                    address: resource_address,
                },
                ids,
                ManifestAstValue::Bucket {
                    identifier: bucket_id,
                },
            ) if resource_address.node_id().is_global_resource() => {
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
                    ExactnessSpecifier::Exact {
                        resource_specifier: ResourceSpecifier::Ids {
                            ids,
                            resource_address: *resource_address,
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

    fn visit_return_to_worktop(&mut self, bucket: &mut ManifestAstValue) -> Result<()> {
        if let ManifestAstValue::Bucket {
            identifier: bucket_id,
        } = bucket
        {
            self.buckets.remove(bucket_id).map_or(
                Err(Error::InvalidBucketId {
                    bucket_id: bucket_id.clone(),
                }),
                |_| Ok(()),
            )
        } else {
            // TODO: Should be an error?
            Ok(())
        }
    }

    fn visit_burn_resource(&mut self, bucket: &mut ManifestAstValue) -> Result<()> {
        if let ManifestAstValue::Bucket {
            identifier: bucket_id,
        } = bucket
        {
            self.buckets.remove(bucket_id).map_or(
                Err(Error::InvalidBucketId {
                    bucket_id: bucket_id.clone(),
                }),
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
    ) -> Result<()> {
        if let ManifestAstValue::Bucket {
            identifier: bucket_id,
        } = bucket
        {
            self.buckets.remove(bucket_id).map_or(
                Err(Error::InvalidBucketId {
                    bucket_id: bucket_id.clone(),
                }),
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

    fn post_visit(&mut self) -> Result<()> {
        self.instruction_index += 1;
        Ok(())
    }
}

impl AccountDepositsInstructionVisitor {
    pub fn add_bucket(&mut self, bucket_id: BucketId, specifier: ExactnessSpecifier) -> Result<()> {
        if !self.buckets.contains_key(&bucket_id) {
            self.buckets.insert(bucket_id, specifier);
            Ok(())
        } else {
            Err(Error::BucketExistsError { bucket_id })
        }
    }
}

#[derive(Default)]
struct BucketValueVisitor(Vec<BucketId>);

impl ManifestAstValueVisitor for BucketValueVisitor {
    fn visit_bucket(&mut self, bucket: &mut ManifestAstValue) -> Result<()> {
        if let ManifestAstValue::Bucket {
            identifier: bucket_id,
        } = bucket
        {
            self.0.push(bucket_id.to_owned());
        }
        Ok(())
    }
}
