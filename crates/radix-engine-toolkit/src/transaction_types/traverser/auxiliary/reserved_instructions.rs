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
use transaction::prelude::*;

use crate::transaction_types::*;
use crate::utils::*;
use crate::*;

pub struct ReservedInstructionsDetector {
    reserved_instructions: IndexSet<ReservedInstruction>,
}

impl ManifestSummaryCallback for ReservedInstructionsDetector {
    fn on_instruction(
        &mut self,
        instruction: &InstructionV1,
        _instruction_index: usize,
    ) {
        let InstructionV1::CallMethod {
            address,
            method_name,
            ..
        } = instruction
        else {
            return;
        };

        if is_account(address)
            && contains!(
                method_name => [
                    ACCOUNT_LOCK_FEE_IDENT,
                    ACCOUNT_LOCK_FEE_AND_WITHDRAW_IDENT,
                    ACCOUNT_LOCK_FEE_AND_WITHDRAW_NON_FUNGIBLES_IDENT,
                ]
            )
        {
            self.reserved_instructions
                .insert(ReservedInstruction::AccountLockFee);
        } else if is_account(address) && method_name == ACCOUNT_SECURIFY_IDENT {
            self.reserved_instructions
                .insert(ReservedInstruction::AccountSecurify);
        } else if is_identity(address) && method_name == IDENTITY_SECURIFY_IDENT
        {
            self.reserved_instructions
                .insert(ReservedInstruction::AccountLockFee);
        } else if is_account(address)
            && contains!(
                method_name => [
                    ACCOUNT_ADD_AUTHORIZED_DEPOSITOR,
                    ACCOUNT_REMOVE_AUTHORIZED_DEPOSITOR,
                    ACCOUNT_SET_RESOURCE_PREFERENCE_IDENT,
                    ACCOUNT_REMOVE_RESOURCE_PREFERENCE_IDENT,
                    ACCOUNT_SET_DEFAULT_DEPOSIT_RULE_IDENT,
                ]
            )
        {
            self.reserved_instructions
                .insert(ReservedInstruction::AccountUpdateSettings);
        } else if is_access_controller(address) {
            self.reserved_instructions
                .insert(ReservedInstruction::AccessControllerMethod);
        }
    }
}

impl ExecutionSummaryCallback for ReservedInstructionsDetector {}
