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
use crate::model::address::{
    EntityAddress, NetworkAwareComponentAddress, NetworkAwareResourceAddress,
};
use crate::model::engine_identifier::{BucketId, RENodeId};
use crate::model::resource_specifier::QuantitativeResourceSpecifier;
use crate::model::value::ast::{ManifestAstValue, ManifestAstValueKind};
use crate::request::ResourceChange;
use crate::utils::is_account;
use crate::visitor::{traverse_value, InstructionVisitor, ManifestAstValueVisitor};
use native_transaction::data::ManifestExpression;
use scrypto::radix_engine_interface::blueprints::account::{
    ACCOUNT_DEPOSIT_BATCH_IDENT, ACCOUNT_DEPOSIT_IDENT,
};
use toolkit_derive::serializable;

/// A visitor whose main responsibility is analyzing the call-method instructions for proof creation
#[derive(Debug)]
pub struct AccountDepositsInstructionVisitor {
    pub deposits: Vec<AccountDeposit>,
    pub resource_changes: HashMap<u32, Vec<ResourceChange>>,
    buckets: BTreeMap<BucketId, ExactnessResourceSpecifier>,
    instruction_index: u32,
}

impl AccountDepositsInstructionVisitor {
    pub fn new(resource_changes: HashMap<u32, Vec<ResourceChange>>) -> Self {
        Self {
            resource_changes,
            deposits: Default::default(),
            buckets: Default::default(),
            instruction_index: Default::default(),
        }
    }
}

#[serializable]
#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub struct AccountDeposit {
    #[schemars(with = "EntityAddress")]
    #[serde_as(as = "serde_with::TryFromInto<EntityAddress>")]
    pub component_address: NetworkAwareComponentAddress,

    #[serde(flatten)]
    pub deposited: ExactnessResourceSpecifier,
}

#[serializable]
#[derive(PartialEq, PartialOrd, Eq, Ord)]
#[serde(tag = "type")]
pub enum ExactnessResourceSpecifier {
    Exact {
        /// The resource address of the resources.
        #[schemars(with = "EntityAddress")]
        #[serde_as(as = "serde_with::TryFromInto<EntityAddress>")]
        resource_address: NetworkAwareResourceAddress,

        /// A specifier of the amount or ids of resources.
        resource_specifier: QuantitativeResourceSpecifier,
    },
    Estimate {
        /// The instruction index that that this amount originates from. This might either be an
        /// instruction where a bucket is created of all worktop resources or an instruction where
        /// a deposit is performed of an estimated amount.
        #[schemars(with = "String")]
        #[schemars(regex(pattern = "[0-9]+"))]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        instruction_index: u32,

        /// The resource address of the resources.
        #[schemars(with = "EntityAddress")]
        #[serde_as(as = "serde_with::TryFromInto<EntityAddress>")]
        resource_address: NetworkAwareResourceAddress,

        /// A specifier of the amount or ids of resources.
        resource_specifier: QuantitativeResourceSpecifier,
    },
}

impl InstructionVisitor for AccountDepositsInstructionVisitor {
    //===================
    // Consuming Buckets
    //===================

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
                ManifestAstValue::ComponentAddress {
                    address: ref component_address,
                }
                | ManifestAstValue::Address {
                    address:
                        EntityAddress::ComponentAddress {
                            address: ref component_address,
                        },
                },
                ManifestAstValue::String { value: method_name },
                arguments,
            ) if is_account(component_address) && method_name == ACCOUNT_DEPOSIT_IDENT => {
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
                        component_address: component_address.to_owned(),
                        deposited: bucket_info.to_owned(),
                    });
                }
            }
            (
                ManifestAstValue::ComponentAddress {
                    address: ref component_address,
                }
                | ManifestAstValue::Address {
                    address:
                        EntityAddress::ComponentAddress {
                            address: ref component_address,
                        },
                },
                ManifestAstValue::String { value: method_name },
                arguments,
            ) if is_account(component_address) && method_name == ACCOUNT_DEPOSIT_BATCH_IDENT => {
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
                                     owner_id, amount, ..
                                 }| {
                                    *owner_id
                                        == RENodeId::GlobalComponent {
                                            address: *component_address,
                                        }
                                        && amount.is_positive()
                                },
                            )
                            .collect::<Vec<&ResourceChange>>();

                        for resource_change in resource_changes {
                            self.deposits.push(AccountDeposit {
                                component_address: component_address.to_owned(),
                                deposited: ExactnessResourceSpecifier::Estimate {
                                    instruction_index: self.instruction_index,
                                    resource_address: resource_change.resource_address,
                                    resource_specifier: QuantitativeResourceSpecifier::Amount {
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
                                component_address: component_address.to_owned(),
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

    //==================
    // Creating Buckets
    //==================

    fn visit_take_from_worktop(
        &mut self,
        resource_address: &mut ManifestAstValue,
        into_bucket: &mut ManifestAstValue,
    ) -> Result<()> {
        if let (
            ManifestAstValue::ResourceAddress {
                address: resource_address,
            }
            | ManifestAstValue::Address {
                address:
                    EntityAddress::ResourceAddress {
                        address: resource_address,
                    },
            },
            ManifestAstValue::Bucket {
                identifier: bucket_id,
            },
        ) = (resource_address, into_bucket)
        {
            self.add_bucket(
                bucket_id.clone(),
                ExactnessResourceSpecifier::Estimate {
                    instruction_index: self.instruction_index,
                    resource_address: *resource_address,
                    resource_specifier: todo!("Need proper preview to understand this amount"),
                },
            )?;
        }
        Ok(())
    }

    fn visit_take_from_worktop_by_amount(
        &mut self,
        resource_address: &mut ManifestAstValue,
        amount: &mut ManifestAstValue,
        into_bucket: &mut ManifestAstValue,
    ) -> Result<()> {
        if let (
            ManifestAstValue::ResourceAddress {
                address: resource_address,
            }
            | ManifestAstValue::Address {
                address:
                    EntityAddress::ResourceAddress {
                        address: resource_address,
                    },
            },
            ManifestAstValue::Decimal { value: amount },
            ManifestAstValue::Bucket {
                identifier: bucket_id,
            },
        ) = (resource_address, amount, into_bucket)
        {
            self.add_bucket(
                bucket_id.clone(),
                ExactnessResourceSpecifier::Exact {
                    resource_address: *resource_address,
                    resource_specifier: QuantitativeResourceSpecifier::Amount { amount: *amount },
                },
            )?;
        }
        Ok(())
    }

    fn visit_take_from_worktop_by_ids(
        &mut self,
        resource_address: &mut ManifestAstValue,
        ids: &mut Vec<ManifestAstValue>,
        into_bucket: &mut ManifestAstValue,
    ) -> Result<()> {
        if let (
            ManifestAstValue::ResourceAddress {
                address: resource_address,
            }
            | ManifestAstValue::Address {
                address:
                    EntityAddress::ResourceAddress {
                        address: resource_address,
                    },
            },
            ids,
            ManifestAstValue::Bucket {
                identifier: bucket_id,
            },
        ) = (resource_address, ids, into_bucket)
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
                ExactnessResourceSpecifier::Exact {
                    resource_address: *resource_address,
                    resource_specifier: QuantitativeResourceSpecifier::Ids { ids },
                },
            )?;
        }
        Ok(())
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
    pub fn add_bucket(
        &mut self,
        bucket_id: BucketId,
        specifier: ExactnessResourceSpecifier,
    ) -> Result<()> {
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
