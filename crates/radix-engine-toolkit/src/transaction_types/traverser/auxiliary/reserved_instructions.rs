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

use radix_transactions::manifest::*;
use radix_transactions::prelude::*;
use scrypto::prelude::*;

use crate::transaction_types::*;
use crate::utils::*;
use crate::*;

#[derive(Default)]
pub struct ReservedInstructionsDetector {
    reserved_instructions: IndexSet<ReservedInstruction>,
}

impl ReservedInstructionsDetector {
    pub fn output(self) -> IndexSet<ReservedInstruction> {
        self.reserved_instructions
    }
}

impl StaticAnalysisCallback for ReservedInstructionsDetector {
    fn on_instruction(&mut self, instruction: &InstructionV2, _: usize) {
        // TODO: Make use of the typed manifest invocations - they would make
        // some of the logic in here easier.
        match instruction {
            InstructionV2::CallMethod(CallMethod {
                address,
                method_name,
                ..
            }) => {
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
                } else if is_account(address)
                    && method_name == ACCOUNT_SECURIFY_IDENT
                {
                    self.reserved_instructions
                        .insert(ReservedInstruction::AccountSecurify);
                } else if is_identity(address)
                    && method_name == IDENTITY_SECURIFY_IDENT
                {
                    self.reserved_instructions
                        .insert(ReservedInstruction::AccountLockFee);
                } else if is_access_controller(address) {
                    self.reserved_instructions
                        .insert(ReservedInstruction::AccessControllerMethod);
                }
            }
            InstructionV2::CallMetadataMethod(CallMetadataMethod {
                address,
                method_name,
                args,
            }) => {
                if method_name == METADATA_SET_IDENT {
                    // Attempt to decode the args as a metadata set call. If we
                    // fail then we have not technically detected a violation of
                    // the reserved instructions and we can just ignore this.
                    let Some(MetadataSetInput { key, .. }) =
                        manifest_encode(args)
                            .ok()
                            .and_then(|encoded| manifest_decode(&encoded).ok())
                    else {
                        return;
                    };
                    let is_owner_keys_metadata_key = key == "owner_keys";
                    if is_account(address) && is_owner_keys_metadata_key {
                        self.reserved_instructions.insert(ReservedInstruction::AccountUpdateOwnerKeysMetadataField);
                    } else if is_identity(address) && is_owner_keys_metadata_key
                    {
                        self.reserved_instructions.insert(ReservedInstruction::IdentityUpdateOwnerKeysMetadataField);
                    }
                } else if method_name == METADATA_LOCK_IDENT {
                    // Attempt to decode the args as a metadata set call. If we
                    // fail then we have not technically detected a violation of
                    // the reserved instructions and we can just ignore this.
                    let Some(MetadataLockInput { key, .. }) =
                        manifest_encode(args)
                            .ok()
                            .and_then(|encoded| manifest_decode(&encoded).ok())
                    else {
                        return;
                    };
                    let is_owner_keys_metadata_key = key == "owner_keys";
                    if is_account(address) && is_owner_keys_metadata_key {
                        self.reserved_instructions.insert(ReservedInstruction::AccountLockOwnerKeysMetadataField);
                    } else if is_identity(address) && is_owner_keys_metadata_key
                    {
                        self.reserved_instructions.insert(ReservedInstruction::IdentityLockOwnerKeysMetadataField);
                    }
                }
            }
            _ => {}
        }
    }
}

impl DynamicAnalysisCallback for ReservedInstructionsDetector {}
