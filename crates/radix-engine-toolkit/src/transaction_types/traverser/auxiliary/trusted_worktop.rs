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

use crate::transaction_types::*;
use scrypto::prelude::*;
use transaction::prelude::*;

#[derive(Default)]
pub struct TrustedWorktop {
    trusted_state_per_instruction: Vec<bool>,
}

impl TrustedWorktop {
    fn add_new_instruction(&mut self, trusted: bool) {
        self.trusted_state_per_instruction.push(trusted);
    }

    pub fn get_results(&self) -> Vec<bool> {
        self.trusted_state_per_instruction.to_owned()
    }

    pub fn is_worktop_trusted(&self, instruction_index: usize) -> Option<bool> {
        self.trusted_state_per_instruction
            .get(instruction_index)
            .map(|value| *value)
    }
}

impl WorktopContentTrackerObserver for TrustedWorktop {
    fn on_instruction(
        &mut self,
        instruction: &InstructionV1,
        instruction_index: usize,
        worktop_content: &Vec<WorktopContent>,
    ) {
        assert_eq!(self.trusted_state_per_instruction.len(), instruction_index);

        match instruction {
            InstructionV1::TakeAllFromWorktop { .. }
            | InstructionV1::TakeFromWorktop { .. }
            | InstructionV1::TakeNonFungiblesFromWorktop { .. } => {
                self.add_new_instruction(true)
            }

            InstructionV1::ReturnToWorktop { .. } => {
                self.add_new_instruction(false)
            }

            InstructionV1::AssertWorktopContainsAny { .. }
            | InstructionV1::AssertWorktopContains { .. }
            | InstructionV1::AssertWorktopContainsNonFungibles { .. }
            | InstructionV1::PopFromAuthZone
            | InstructionV1::PushToAuthZone { .. }
            | InstructionV1::CreateProofFromAuthZoneOfAmount { .. }
            | InstructionV1::CreateProofFromAuthZoneOfNonFungibles { .. }
            | InstructionV1::CreateProofFromAuthZoneOfAll { .. }
            | InstructionV1::DropAuthZoneProofs
            | InstructionV1::DropAuthZoneRegularProofs
            | InstructionV1::DropAuthZoneSignatureProofs
            | InstructionV1::CreateProofFromBucketOfAmount { .. }
            | InstructionV1::CreateProofFromBucketOfNonFungibles { .. }
            | InstructionV1::CreateProofFromBucketOfAll { .. }
            | InstructionV1::BurnResource { .. }
            | InstructionV1::CloneProof { .. }
            | InstructionV1::DropProof { .. }
            | InstructionV1::DropNamedProofs
            | InstructionV1::DropAllProofs
            | InstructionV1::AllocateGlobalAddress { .. } => {
                self.add_new_instruction(true)
            }

            InstructionV1::CallFunction { .. }
            | InstructionV1::CallMethod { .. }
            | InstructionV1::CallRoyaltyMethod { .. }
            | InstructionV1::CallMetadataMethod { .. }
            | InstructionV1::CallRoleAssignmentMethod { .. }
            | InstructionV1::CallDirectVaultMethod { .. } => {
                if instruction_index == 0 {
                    self.add_new_instruction(true)
                } else {
                    self.add_new_instruction(
                        worktop_content[instruction_index]
                            == worktop_content[instruction_index - 1],
                    );
                }
            }
        }
    }
}
