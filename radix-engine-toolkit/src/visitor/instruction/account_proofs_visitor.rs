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
use crate::model::value::ast::ManifestAstValue;
use crate::visitor::InstructionVisitor;

/// A visitor whose main responsibility is analyzing the call-method instructions for proof creation
#[derive(Debug, Default)]
pub struct AccountProofsInstructionVisitor {
    /// The resource addresses of the created proofs
    pub created_proofs: BTreeSet<NetworkAwareNodeId>,
}

impl InstructionVisitor for AccountProofsInstructionVisitor {
    fn visit_call_method(
        &mut self,
        component_address: &mut ManifestAstValue,
        method_name: &mut ManifestAstValue,
        args: &mut Vec<ManifestAstValue>,
    ) -> Result<(), VisitorError> {
        // Checking for instructions that create proofs from accounts. Since all that is of interest
        // to us is the resource address then only the first argument needs to be analyzed.
        let args = args.clone();
        match (component_address, method_name, args.get(0)) {
            (
                ManifestAstValue::Address {
                    value: component_address,
                },
                ManifestAstValue::String {
                    value: ref method_name,
                },
                Some(ManifestAstValue::Address {
                    value: resource_address,
                }),
            ) if is_account(*component_address)
                && (method_name == ACCOUNT_CREATE_PROOF_IDENT
                    || method_name == ACCOUNT_CREATE_PROOF_OF_AMOUNT_IDENT
                    || method_name == ACCOUNT_CREATE_PROOF_OF_NON_FUNGIBLES_IDENT)
                && (resource_address
                    .node_id()
                    .is_global_fungible_resource_manager()
                    || resource_address
                        .node_id()
                        .is_global_non_fungible_resource_manager()) =>
            {
                self.created_proofs.insert(*resource_address);
            }
            _ => {}
        }

        Ok(())
    }
}
