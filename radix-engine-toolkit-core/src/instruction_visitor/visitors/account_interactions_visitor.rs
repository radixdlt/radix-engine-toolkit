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

use crate::instruction_visitor::core::error::InstructionVisitorError;
use crate::instruction_visitor::core::traits::InstructionVisitor;
use crate::utils::is_account;
use scrypto::prelude::*;
use transaction::prelude::GlobalAddress;

#[derive(Default, Clone, Debug)]
pub struct AccountInteractionsVisitor {
    accounts_requiring_auth: HashSet<ComponentAddress>,
    accounts_withdrawn_from: HashSet<ComponentAddress>,
    accounts_deposited_into: HashSet<ComponentAddress>,
}

impl AccountInteractionsVisitor {
    pub fn output(
        self,
    ) -> (
        HashSet<ComponentAddress>,
        HashSet<ComponentAddress>,
        HashSet<ComponentAddress>,
    ) {
        (
            self.accounts_requiring_auth,
            self.accounts_withdrawn_from,
            self.accounts_deposited_into,
        )
    }
}

impl InstructionVisitor for AccountInteractionsVisitor {
    fn visit_call_method(
        &mut self,
        address: &GlobalAddress,
        method_name: &str,
        _: &ManifestValue,
    ) -> Result<(), InstructionVisitorError> {
        if is_account(address) {
            let Ok(component_address) = ComponentAddress::try_from(address.as_node_id().0)
            else {
                return Ok(())
            };

            if crate::statics::ACCOUNT_METHODS_THAT_REQUIRE_AUTH
                .iter()
                .any(|ident| *ident == method_name)
            {
                self.accounts_requiring_auth.insert(component_address);
            }

            if crate::statics::ACCOUNT_DEPOSIT_METHODS.contains(&method_name) {
                self.accounts_deposited_into.insert(component_address);
            }

            if crate::statics::ACCOUNT_WITHDRAW_METHODS.contains(&method_name) {
                self.accounts_withdrawn_from.insert(component_address);
            }
        };
        Ok(())
    }

    fn visit_call_access_rules_method(
        &mut self,
        address: &GlobalAddress,
        method_name: &str,
        _: &ManifestValue,
    ) -> Result<(), InstructionVisitorError> {
        if is_account(address) {
            let Ok(component_address) = ComponentAddress::try_from(address.as_node_id().0)
            else {
                return Ok(())
            };

            if crate::statics::ACCESS_RULES_METHODS_THAT_REQUIRE_AUTH
                .iter()
                .any(|ident| *ident == method_name)
            {
                self.accounts_requiring_auth.insert(component_address);
            }
        }
        Ok(())
    }

    fn visit_call_metadata_method(
        &mut self,
        address: &GlobalAddress,
        method_name: &str,
        _: &ManifestValue,
    ) -> Result<(), InstructionVisitorError> {
        if is_account(address) {
            let Ok(component_address) = ComponentAddress::try_from(address.as_node_id().0)
            else {
                return Ok(())
            };

            if crate::statics::METADATA_METHODS_THAT_REQUIRE_AUTH
                .iter()
                .any(|ident| *ident == method_name)
            {
                self.accounts_requiring_auth.insert(component_address);
            }
        }
        Ok(())
    }

    fn visit_call_royalty_method(
        &mut self,
        address: &GlobalAddress,
        method_name: &str,
        _: &ManifestValue,
    ) -> Result<(), InstructionVisitorError> {
        if is_account(address) {
            let Ok(component_address) = ComponentAddress::try_from(address.as_node_id().0)
            else {
                return Ok(())
            };

            if crate::statics::ROYALTY_METHODS_THAT_REQUIRE_AUTH
                .iter()
                .any(|ident| *ident == method_name)
            {
                self.accounts_requiring_auth.insert(component_address);
            }
        }
        Ok(())
    }
}
