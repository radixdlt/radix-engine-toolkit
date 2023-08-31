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

use radix_engine_interface::blueprints::account::*;
use radix_engine_interface::blueprints::identity::*;
use scrypto::prelude::*;

use crate::{
    instruction_visitor::core::traits::InstructionVisitor,
    utils::{is_access_controller, is_account},
};

#[derive(Clone, Debug, Hash, PartialEq, Eq, Copy)]
pub enum ReservedInstruction {
    AccountLockFee,
    AccountSecurify,
    IdentitySecurify,
    AccountUpdateSettings,
    AccessController,
}

#[derive(Clone, Debug, Default)]
pub struct ReservedInstructionsVisitor(HashSet<ReservedInstruction>);

impl ReservedInstructionsVisitor {
    pub fn output(self) -> HashSet<ReservedInstruction> {
        self.0
    }
}

impl InstructionVisitor for ReservedInstructionsVisitor {
    fn visit_call_method(
        &mut self,
        address: &transaction::prelude::DynamicGlobalAddress,
        method_name: &str,
        _: &radix_engine_common::prelude::ManifestValue,
    ) -> Result<(), crate::instruction_visitor::core::error::InstructionVisitorError> {
        // Case: Account lock fee calls
        if is_account(address)
            && (method_name == ACCOUNT_LOCK_FEE_IDENT
                || method_name == ACCOUNT_LOCK_CONTINGENT_FEE_IDENT
                || method_name == ACCOUNT_LOCK_FEE_AND_WITHDRAW_IDENT
                || method_name == ACCOUNT_LOCK_FEE_AND_WITHDRAW_NON_FUNGIBLES_IDENT)
        {
            self.0.insert(ReservedInstruction::AccountLockFee);
        }
        // Case: Account securify
        else if is_account(address) && method_name == ACCOUNT_SECURIFY_IDENT {
            self.0.insert(ReservedInstruction::AccountSecurify);
        }
        // Case: Identity securify
        else if is_account(address) && method_name == IDENTITY_SECURIFY_IDENT {
            self.0.insert(ReservedInstruction::IdentitySecurify);
        }
        // Case: Account Update Settings
        else if is_account(address)
            && (method_name == ACCOUNT_ADD_AUTHORIZED_DEPOSITOR
                || method_name == ACCOUNT_REMOVE_AUTHORIZED_DEPOSITOR
                || method_name == ACCOUNT_SET_DEFAULT_DEPOSIT_RULE_IDENT
                || method_name == ACCOUNT_SET_RESOURCE_PREFERENCE_IDENT
                || method_name == ACCOUNT_REMOVE_RESOURCE_PREFERENCE_IDENT)
        {
            self.0.insert(ReservedInstruction::AccountUpdateSettings);
        }
        // Case: Access Controller methods.
        else if is_access_controller(address) {
            self.0.insert(ReservedInstruction::AccessController);
        };
        Ok(())
    }
}
