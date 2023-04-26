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

use std::collections::BTreeSet;

use scrypto::blueprints::account::*;

use crate::error::VisitorError;
use crate::model::address::utils::is_account;
use crate::model::address::NetworkAwareNodeId;
use crate::model::resource_specifier::{ResourceManagerSpecifier, ResourceSpecifier};
use crate::model::value::ast::{ManifestAstValue, ManifestAstValueKind};
use crate::visitor::InstructionVisitor;
use toolkit_derive::serializable;

/// A visitor whose main responsibility is analyzing the call-method instructions for proof creation
#[derive(Debug, Default)]
pub struct AccountWithdrawsInstructionVisitor(pub Vec<AccountWithdraw>);

impl AccountWithdrawsInstructionVisitor {
    pub fn add(
        &mut self,
        component_address: NetworkAwareNodeId,
        resource_specifier: ResourceSpecifier,
    ) {
        self.0.push(AccountWithdraw {
            component_address,
            resource_specifier,
        });
    }
}

impl InstructionVisitor for AccountWithdrawsInstructionVisitor {
    fn visit_call_method(
        &mut self,
        component_address: &mut ManifestAstValue,
        method_name: &mut ManifestAstValue,
        args: &mut Option<Vec<ManifestAstValue>>,
    ) -> Result<(), VisitorError> {
        let args = args.clone().unwrap_or_default();
        match (
            component_address,
            method_name,
            args.get(0),
            args.get(1),
            args.get(2),
        ) {
            // Withdraw from account by amount
            (
                // Component Address
                ManifestAstValue::Address {
                    address: component_address,
                },
                // Method Name
                ManifestAstValue::String { value: method_name },
                // Resource Address to withdraw
                Some(ManifestAstValue::Address {
                    address: resource_address,
                }),
                // Amount to withdraw
                Some(ManifestAstValue::Decimal { value: amount }),
                None,
            ) if is_account(*component_address)
                && method_name == ACCOUNT_WITHDRAW_IDENT
                && resource_address.node_id().is_global_resource() =>
            {
                self.add(
                    *component_address,
                    ResourceSpecifier::Amount {
                        amount: amount.to_owned(),
                        resource_address: ResourceManagerSpecifier::Existing {
                            address: *resource_address,
                        },
                    },
                )
            }
            // Withdraw from account by ids
            (
                // Component Address
                ManifestAstValue::Address {
                    address: component_address,
                },
                // Method Name
                ManifestAstValue::String { value: method_name },
                // Resource Address to withdraw
                Some(ManifestAstValue::Address {
                    address: resource_address,
                }),
                // Set of non-fungible ids
                Some(ManifestAstValue::Array {
                    element_kind: ManifestAstValueKind::NonFungibleLocalId,
                    elements: ids,
                }),
                None,
            ) if is_account(*component_address)
                && method_name == ACCOUNT_WITHDRAW_NON_FUNGIBLES_IDENT
                && resource_address.node_id().is_global_resource() =>
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
                self.add(
                    *component_address,
                    ResourceSpecifier::Ids {
                        ids,
                        resource_address: ResourceManagerSpecifier::Existing {
                            address: *resource_address,
                        },
                    },
                )
            }
            // Lock fee and withdraw from account by amount
            (
                // Component Address
                ManifestAstValue::Address {
                    address: component_address,
                },
                // Method name
                ManifestAstValue::String { value: method_name },
                // Lock fee amount
                Some(ManifestAstValue::Decimal { .. }),
                // Resource address to withdraw
                Some(ManifestAstValue::Address {
                    address: resource_address,
                }),
                // Amount to withdraw
                Some(ManifestAstValue::Decimal { value: amount }),
            ) if is_account(*component_address)
                && method_name == ACCOUNT_LOCK_FEE_AND_WITHDRAW_IDENT
                && resource_address.node_id().is_global_resource() =>
            {
                self.add(
                    *component_address,
                    ResourceSpecifier::Amount {
                        amount: amount.to_owned(),
                        resource_address: ResourceManagerSpecifier::Existing {
                            address: *resource_address,
                        },
                    },
                )
            }
            // Lock fee and withdraw from account by ids
            (
                // Component Address
                ManifestAstValue::Address {
                    address: component_address,
                },
                // Method Name
                ManifestAstValue::String { value: method_name },
                // Amount to lock fee
                Some(ManifestAstValue::Decimal { .. }),
                // Resource Address
                Some(ManifestAstValue::Address {
                    address: resource_address,
                }),
                // Array of non-fungible ids
                Some(ManifestAstValue::Array {
                    element_kind: ManifestAstValueKind::NonFungibleLocalId,
                    elements: ids,
                }),
            ) if is_account(*component_address)
                && method_name == ACCOUNT_LOCK_FEE_AND_WITHDRAW_NON_FUNGIBLES_IDENT
                && resource_address.node_id().is_global_resource() =>
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
                self.add(
                    *component_address,
                    ResourceSpecifier::Ids {
                        ids,
                        resource_address: ResourceManagerSpecifier::Existing {
                            address: *resource_address,
                        },
                    },
                )
            }
            _ => { /* Ignore everything else */ }
        }
        Ok(())
    }
}

#[serializable]
#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub struct AccountWithdraw {
    /// The component address of the account that the resources were withdrawn from.
    #[schemars(with = "String")]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    component_address: NetworkAwareNodeId,

    /// A specifier used to specify what was withdrawn from the account - this could either be an
    /// amount or a set of non-fungible local ids.
    ///
    /// When this vector has more than one item, it means that multiple instructions performed a
    /// withdraw from the same account of the same resource.
    resource_specifier: ResourceSpecifier,
}
