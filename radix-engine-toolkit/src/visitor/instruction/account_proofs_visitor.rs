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

use radix_engine::types::blueprints::account::*;

use crate::error::Result;
use crate::model::address::{EntityAddress, NetworkAwareResourceAddress};
use crate::model::value::ast::ManifestAstValue;
use crate::utils::is_account;
use crate::visitor::InstructionVisitor;

/// A visitor whose main responsibility is analyzing the call-method instructions for proof creation
#[derive(Debug, Default)]
pub struct AccountProofsInstructionVisitor {
    /// The resource addresses of the created proofs
    pub created_proofs: BTreeSet<NetworkAwareResourceAddress>,
}

impl InstructionVisitor for AccountProofsInstructionVisitor {
    fn visit_call_method(
        &mut self,
        component_address: &mut ManifestAstValue,
        method_name: &mut ManifestAstValue,
        args: &mut Option<Vec<ManifestAstValue>>,
    ) -> Result<()> {
        // Checking for instructions that create proofs from accounts. Since all that is of interest
        // to us is the resource address then only the first argument needs to be analyzed
        let args = args.clone();
        match (
            component_address,
            method_name,
            args.unwrap_or_default().get(0),
        ) {
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
                ManifestAstValue::String {
                    value: ref method_name,
                },
                Some(
                    ManifestAstValue::ResourceAddress {
                        address: resource_address,
                    }
                    | ManifestAstValue::Address {
                        address:
                            EntityAddress::ResourceAddress {
                                address: resource_address,
                            },
                    },
                ),
            ) if is_account(component_address)
                && (method_name == ACCOUNT_CREATE_PROOF_IDENT
                    || method_name == ACCOUNT_CREATE_PROOF_BY_AMOUNT_IDENT
                    || method_name == ACCOUNT_CREATE_PROOF_BY_IDS_IDENT) =>
            {
                self.created_proofs.insert(*resource_address);
            }
            _ => {}
        }

        Ok(())
    }
}
