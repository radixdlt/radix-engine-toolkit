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

use crate::error::Result;
use crate::model::address::NetworkAwareComponentAddress;
use crate::model::value::ast::ManifestAstValue;
use crate::utils::is_account;
use crate::visitor::InstructionVisitor;

/// A visitor whose main responsibility is determining the kind of interactions involved with
/// accounts
#[derive(Debug, Default)]
pub struct AccountInteractionsInstructionVisitor {
    pub auth_required: BTreeSet<NetworkAwareComponentAddress>,
    pub accounts_withdrawn_from: BTreeSet<NetworkAwareComponentAddress>,
    pub accounts_deposited_into: BTreeSet<NetworkAwareComponentAddress>,
}

impl AccountInteractionsInstructionVisitor {
    const AUTH_REQUIRING_METHODS: &'static [&'static str] = &[
        ACCOUNT_LOCK_FEE_IDENT,
        ACCOUNT_LOCK_CONTINGENT_FEE_IDENT,
        ACCOUNT_WITHDRAW_IDENT,
        ACCOUNT_WITHDRAW_NON_FUNGIBLES_IDENT,
        ACCOUNT_LOCK_FEE_AND_WITHDRAW_IDENT,
        ACCOUNT_LOCK_FEE_AND_WITHDRAW_NON_FUNGIBLES_IDENT,
        ACCOUNT_CREATE_PROOF_IDENT,
        ACCOUNT_CREATE_PROOF_BY_AMOUNT_IDENT,
        ACCOUNT_CREATE_PROOF_BY_IDS_IDENT,
    ];
    const WITHDRAW_METHODS: &'static [&'static str] = &[
        ACCOUNT_WITHDRAW_IDENT,
        ACCOUNT_WITHDRAW_NON_FUNGIBLES_IDENT,
        ACCOUNT_LOCK_FEE_AND_WITHDRAW_IDENT,
        ACCOUNT_LOCK_FEE_AND_WITHDRAW_NON_FUNGIBLES_IDENT,
    ];
    const DEPOSIT_METHODS: &'static [&'static str] =
        &[ACCOUNT_DEPOSIT_IDENT, ACCOUNT_DEPOSIT_BATCH_IDENT];
}

impl InstructionVisitor for AccountInteractionsInstructionVisitor {
    fn visit_call_method(
        &mut self,
        component_address: &mut ManifestAstValue,
        method_name: &mut ManifestAstValue,
        _args: &mut Option<Vec<ManifestAstValue>>,
    ) -> Result<()> {
        // Checking for methods that require auth
        match (component_address, method_name) {
            (
                ManifestAstValue::Address {
                    address: component_address,
                },
                ManifestAstValue::String { value: method_name },
            ) if is_account(*component_address) => {
                if Self::AUTH_REQUIRING_METHODS.contains(&method_name.as_str()) {
                    self.auth_required.insert((*component_address).try_into()?);
                }
                if Self::WITHDRAW_METHODS.contains(&method_name.as_str()) {
                    self.accounts_withdrawn_from
                        .insert((*component_address).try_into()?);
                }
                if Self::DEPOSIT_METHODS.contains(&method_name.as_str()) {
                    self.accounts_deposited_into
                        .insert((*component_address).try_into()?);
                }
            }
            _ => {}
        }

        Ok(())
    }

    fn visit_set_metadata(
        &mut self,
        entity_address: &mut ManifestAstValue,
        _: &mut ManifestAstValue,
        _: &mut ManifestAstValue,
    ) -> Result<()> {
        match entity_address {
            ManifestAstValue::Address {
                address: component_address,
            } if is_account(*component_address) => {
                self.auth_required.insert((*component_address).try_into()?);
            }
            _ => {}
        }

        Ok(())
    }

    fn visit_set_component_royalty_config(
        &mut self,
        component_address: &mut crate::model::value::ast::ManifestAstValue,
        _: &mut crate::model::value::ast::ManifestAstValue,
    ) -> Result<()> {
        match component_address {
            ManifestAstValue::Address {
                address: component_address,
            } if is_account(*component_address) => {
                self.auth_required.insert((*component_address).try_into()?);
            }
            _ => {}
        }
        Ok(())
    }

    fn visit_claim_component_royalty(
        &mut self,
        component_address: &mut crate::model::value::ast::ManifestAstValue,
    ) -> Result<()> {
        match component_address {
            ManifestAstValue::Address {
                address: component_address,
            } if is_account(*component_address) => {
                self.auth_required.insert((*component_address).try_into()?);
            }
            _ => {}
        }
        Ok(())
    }

    fn visit_set_method_access_rule(
        &mut self,
        entity_address: &mut crate::model::value::ast::ManifestAstValue,
        _: &mut crate::model::value::ast::ManifestAstValue,
        _: &mut crate::model::value::ast::ManifestAstValue,
    ) -> Result<()> {
        match entity_address {
            ManifestAstValue::Address {
                address: component_address,
            } if is_account(*component_address) => {
                self.auth_required.insert((*component_address).try_into()?);
            }
            _ => {}
        }
        Ok(())
    }
}
