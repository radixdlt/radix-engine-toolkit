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

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TransferAnalyzer;

impl ManifestStaticAnalyzer for TransferAnalyzer {
    type Initializer = ();
    type Output = ();
    type PermissionState = SimplePermissionState;
    type RequirementState = TransferRequirementState;

    fn new(
        _: Self::Initializer,
    ) -> (Self, Self::PermissionState, Self::RequirementState) {
        Default::default()
    }

    fn output(self) -> Self::Output {}

    fn process_permission(
        &self,
        permission_state: &mut Self::PermissionState,
        context: AnalysisContext<'_>,
    ) {
        // Compute if the next instruction is permitted or not.
        let is_next_instruction_permitted = match context.instruction() {
            // Selective Permissions
            GroupedInstruction::InvocationInstructions(
                InvocationInstructions::CallMethod(CallMethod {
                    address,
                    method_name,
                    ..
                }),
            ) => {
                let grouped_entity_type = match address {
                    ManifestGlobalAddress::Static(static_address) => {
                        static_address.as_node_id().entity_type()
                    }
                    ManifestGlobalAddress::Named(named_address) => context
                        .named_address_store()
                        .get(named_address)
                        .and_then(BlueprintId::entity_type),
                }
                .map(GroupedEntityType::from);

                match (grouped_entity_type, method_name.as_str()) {
                    // Selective Permissions
                    (
                        Some(GroupedEntityType::AccountEntities(..)),
                        ACCOUNT_WITHDRAW_IDENT
                        | ACCOUNT_WITHDRAW_NON_FUNGIBLES_IDENT
                        | ACCOUNT_DEPOSIT_IDENT
                        | ACCOUNT_DEPOSIT_BATCH_IDENT
                        | ACCOUNT_TRY_DEPOSIT_OR_ABORT_IDENT
                        | ACCOUNT_TRY_DEPOSIT_BATCH_OR_ABORT_IDENT
                        | ACCOUNT_CREATE_PROOF_OF_AMOUNT_IDENT
                        | ACCOUNT_CREATE_PROOF_OF_NON_FUNGIBLES_IDENT
                        | ACCOUNT_LOCK_FEE_IDENT
                        | ACCOUNT_LOCK_CONTINGENT_FEE_IDENT
                        | ACCOUNT_LOCK_FEE_AND_WITHDRAW_IDENT
                        | ACCOUNT_LOCK_FEE_AND_WITHDRAW_NON_FUNGIBLES_IDENT,
                    )
                    | (
                        Some(GroupedEntityType::AccessControllerEntities(..)),
                        ACCESS_CONTROLLER_CREATE_PROOF_IDENT,
                    ) => true,
                    // Disallowed Invocations
                    (
                        Some(
                            GroupedEntityType::IdentityEntities(..)
                            | GroupedEntityType::PoolEntities(..)
                            | GroupedEntityType::InternalEntities(..)
                            | GroupedEntityType::SystemEntities(..)
                            | GroupedEntityType::ResourceManagerEntities(..)
                            | GroupedEntityType::PackageEntities(..)
                            | GroupedEntityType::ValidatorEntities(..)
                            | GroupedEntityType::AccountEntities(..)
                            | GroupedEntityType::AccessControllerEntities(..)
                            | GroupedEntityType::AccountLockerEntities(..)
                            | GroupedEntityType::GenericComponentEntities(..),
                        )
                        | None,
                        _,
                    ) => false,
                }
            }
            // Permitted Instructions
            GroupedInstruction::TakeFromWorktopInstructions(..)
            | GroupedInstruction::ReturnToWorktopInstructions(..)
            | GroupedInstruction::AssertionInstructions(..)
            | GroupedInstruction::ProofInstructions(..)
            | GroupedInstruction::AddressAllocationInstructions(..) => true,
            // Disallowed Instructions
            GroupedInstruction::SubintentInstructions(..)
            | GroupedInstruction::BurnResourceInstructions(..)
            | GroupedInstruction::InvocationInstructions(
                InvocationInstructions::CallFunction(..)
                | InvocationInstructions::CallRoyaltyMethod(..)
                | InvocationInstructions::CallMetadataMethod(..)
                | InvocationInstructions::CallDirectVaultMethod(..)
                | InvocationInstructions::CallRoleAssignmentMethod(..),
            ) => false,
        };
        permission_state.next_instruction_status(is_next_instruction_permitted);
    }

    fn process_requirement(
        &self,
        requirement_state: &mut Self::RequirementState,
        context: AnalysisContext<'_>,
    ) {
        let AnalysisContext::InvocationInstruction {
            typed_native_invocation:
                Some(TypedNativeInvocation { invocation, .. }),
            ..
        } = context
        else {
            return;
        };
        requirement_state.handle_invocation(Some(invocation))
    }

    fn process_instruction(&mut self, _: AnalysisContext<'_>) {}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TransferRequirementState {
    is_withdraw_seen: bool,
    is_deposit_seen: bool,
}

impl ManifestAnalyzerRequirementState for TransferRequirementState {
    fn all_requirements_met(&self) -> bool {
        self.is_withdraw_seen && self.is_deposit_seen
    }
}

impl TransferRequirementState {
    pub fn handle_invocation(
        &mut self,
        typed_invocation: Option<&TypedManifestNativeInvocation>,
    ) {
        match typed_invocation {
            Some(
                TypedManifestNativeInvocation::AccountBlueprintInvocation(
                    AccountBlueprintInvocation::Method(
                        AccountBlueprintMethod::Withdraw(..)
                        | AccountBlueprintMethod::WithdrawNonFungibles(..)
                        | AccountBlueprintMethod::LockFeeAndWithdraw(..)
                        | AccountBlueprintMethod::LockFeeAndWithdrawNonFungibles(
                            ..,
                        ),
                    ),
                ),
            ) => self.is_withdraw_seen = true,
            Some(
                TypedManifestNativeInvocation::AccountBlueprintInvocation(
                    AccountBlueprintInvocation::Method(
                        AccountBlueprintMethod::Deposit(..)
                        | AccountBlueprintMethod::DepositBatch(..)
                        | AccountBlueprintMethod::TryDepositOrAbort(..)
                        | AccountBlueprintMethod::TryDepositBatchOrAbort(..),
                    ),
                ),
            ) => self.is_deposit_seen = true,
            _ => {}
        }
    }
}
