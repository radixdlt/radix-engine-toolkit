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

use crate::internal_prelude::*;

pub type SimpleTransferTransactionTypeVisitor = SimpleTransferStateMachine;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SimpleTransferStateMachine {
    #[default]
    InitialState,
    AccessControllerProofCreated,
    FeeLockPerformed,
    ResourcesWithdrawn,
    ResourcesInBucket,
    DepositPerformed,
    InvalidState,
}

impl ManifestAnalysisVisitor for SimpleTransferStateMachine {
    type Output = bool;
    type ValidityState = Self;

    fn output(self) -> Self::Output {
        matches!(self, Self::DepositPerformed)
    }

    fn validity_state(&self) -> &Self::ValidityState {
        self
    }

    fn on_instruction(
        &mut self,
        _: &NamedAddressStore,
        instruction: &GroupedInstruction,
        _: &InstructionIndex,
        _: Option<&TypedManifestNativeInvocation>,
        _: Option<&InvocationIo<InvocationIoItems>>,
    ) {
        self.transition(instruction);
    }
}

impl ManifestAnalysisVisitorValidityState for SimpleTransferStateMachine {
    fn is_visitor_accepting_instructions(&self) -> bool {
        !matches!(self, Self::InvalidState)
    }
}

impl SimpleTransferStateMachine {
    pub fn transition(&mut self, instruction: &GroupedInstruction) {
        let next_state = match (&self, instruction) {
            // Initial State -> Access Controller Proof State transition.
            (
                Self::InitialState,
                GroupedInstruction::InvocationInstructions(
                    InvocationInstructions::CallMethod(CallMethod {
                        address: ManifestGlobalAddress::Static(address),
                        method_name,
                        ..
                    }),
                ),
            ) if matches!(
                address.as_node_id().entity_type().map(Into::into),
                Some(GroupedEntityType::AccessControllerEntities(..))
            ) && method_name == ACCESS_CONTROLLER_CREATE_PROOF_IDENT =>
            {
                Self::AccessControllerProofCreated
            }
            // (Initial State | Access Controller Proof Created) -> Lock Fee
            // State transition.
            (
                Self::InitialState | Self::AccessControllerProofCreated,
                GroupedInstruction::InvocationInstructions(
                    InvocationInstructions::CallMethod(CallMethod {
                        address: ManifestGlobalAddress::Static(address),
                        method_name,
                        ..
                    }),
                ),
            ) if matches!(
                address.as_node_id().entity_type().map(Into::into),
                Some(GroupedEntityType::AccountEntities(..))
            ) && method_name == ACCOUNT_LOCK_FEE_IDENT =>
            {
                Self::FeeLockPerformed
            }
            // Access Controller Proof Created -> Resources on Worktop
            (
                Self::AccessControllerProofCreated,
                GroupedInstruction::InvocationInstructions(
                    InvocationInstructions::CallMethod(CallMethod {
                        address: ManifestGlobalAddress::Static(address),
                        method_name,
                        ..
                    }),
                ),
            ) if matches!(
                address.as_node_id().entity_type().map(Into::into),
                Some(GroupedEntityType::AccountEntities(..))
            ) && matches!(
                method_name.as_str(),
                ACCOUNT_LOCK_FEE_AND_WITHDRAW_IDENT
                    | ACCOUNT_LOCK_FEE_AND_WITHDRAW_NON_FUNGIBLES_IDENT
            ) =>
            {
                Self::ResourcesWithdrawn
            }
            // Initial State -> Resources on Worktop State transition.
            (
                Self::InitialState,
                GroupedInstruction::InvocationInstructions(
                    InvocationInstructions::CallMethod(CallMethod {
                        address: ManifestGlobalAddress::Static(address),
                        method_name,
                        ..
                    }),
                ),
            ) if matches!(
                address.as_node_id().entity_type().map(Into::into),
                Some(GroupedEntityType::AccountEntities(..))
            ) && matches!(
                method_name.as_str(),
                ACCOUNT_WITHDRAW_IDENT
                    | ACCOUNT_WITHDRAW_NON_FUNGIBLES_IDENT
                    | ACCOUNT_LOCK_FEE_AND_WITHDRAW_IDENT
                    | ACCOUNT_LOCK_FEE_AND_WITHDRAW_NON_FUNGIBLES_IDENT
            ) =>
            {
                Self::ResourcesWithdrawn
            }
            // Fees Locked -> Resources on Worktop State transition.
            (
                Self::FeeLockPerformed,
                GroupedInstruction::InvocationInstructions(
                    InvocationInstructions::CallMethod(CallMethod {
                        address: ManifestGlobalAddress::Static(address),
                        method_name,
                        ..
                    }),
                ),
            ) if matches!(
                address.as_node_id().entity_type().map(Into::into),
                Some(GroupedEntityType::AccountEntities(..))
            ) && matches!(
                method_name.as_str(),
                ACCOUNT_WITHDRAW_IDENT | ACCOUNT_WITHDRAW_NON_FUNGIBLES_IDENT
            ) =>
            {
                Self::ResourcesWithdrawn
            }
            // Resources on Worktop -> Resources in Bucket State transition.
            (
                Self::ResourcesWithdrawn,
                GroupedInstruction::TakeFromWorktopInstructions(
                    TakeFromWorktopInstructions::TakeFromWorktop(..),
                ),
            ) => Self::ResourcesInBucket,
            // Resources in Bucket -> Resources Deposited State transition.
            (
                Self::ResourcesInBucket,
                GroupedInstruction::InvocationInstructions(
                    InvocationInstructions::CallMethod(CallMethod {
                        address: ManifestGlobalAddress::Static(address),
                        method_name,
                        ..
                    }),
                ),
            ) if matches!(
                address.as_node_id().entity_type().map(Into::into),
                Some(GroupedEntityType::AccountEntities(..))
            ) && matches!(
                method_name.as_str(),
                ACCOUNT_DEPOSIT_IDENT | ACCOUNT_TRY_DEPOSIT_OR_ABORT_IDENT
            ) =>
            {
                Self::DepositPerformed
            }
            _ => Self::InvalidState,
        };
        *self = next_state;
    }
}
