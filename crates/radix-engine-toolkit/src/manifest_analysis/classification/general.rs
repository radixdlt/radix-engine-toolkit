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

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GeneralAnalyzer {
    /// This flag changes some of the rules that this transaction type analyzer
    /// uses and it can be used to change between it being used for the general
    /// transaction type and the subintent general transaction type. This is
    /// implemented as a flag to allow for the majority of the code to be
    /// shared between the two transaction types.
    for_subintent: bool,
}

impl ManifestStaticAnalyzer for GeneralAnalyzer {
    type Initializer = GeneralInitializer;
    type Output = ();
    type PermissionState = SimplePermissionState;
    type RequirementState = GeneralRequirementState;

    fn new(
        GeneralInitializer { for_subintent }: Self::Initializer,
    ) -> (Self, Self::PermissionState, Self::RequirementState) {
        (
            Self { for_subintent },
            SimplePermissionState::default(),
            GeneralRequirementState::new(for_subintent),
        )
    }

    fn output(self) -> Self::Output {}

    fn process_permission(
        &mut self,
        permission_state: &mut Self::PermissionState,
        named_address_store: &NamedAddressStore,
        instruction: &GroupedInstruction,
        _: Option<(
            &ManifestInvocationReceiver,
            &TypedManifestNativeInvocation,
        )>,
    ) {
        // Compute if the next instruction is permitted or not.
        let is_next_instruction_permitted = match instruction {
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
                    ManifestGlobalAddress::Named(named_address) => {
                        named_address_store
                            .get(named_address)
                            .and_then(BlueprintId::entity_type)
                    }
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
                    )
                    | (
                        Some(GroupedEntityType::ValidatorEntities(..)),
                        VALIDATOR_STAKE_IDENT
                        | VALIDATOR_UNSTAKE_IDENT
                        | VALIDATOR_CLAIM_XRD_IDENT,
                    )
                    | (
                        // All of the pool blueprints have the same name for the
                        // contribute and redeem methods and I wanted to use a
                        // constant here so I chose to use the constants for the
                        // one resource pool blueprint. Again, they're all the
                        // same string for all of the pool blueprints and we do
                        // not need to have redundant strings here.
                        Some(GroupedEntityType::PoolEntities(..)),
                        ONE_RESOURCE_POOL_CONTRIBUTE_IDENT
                        | ONE_RESOURCE_POOL_REDEEM_IDENT,
                    ) => true,
                    // Permitted Invocations
                    (
                        Some(
                            GroupedEntityType::AccountLockerEntities(..)
                            | GroupedEntityType::GenericComponentEntities(..),
                        ),
                        _,
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
                            | GroupedEntityType::AccessControllerEntities(..),
                        )
                        | None,
                        _,
                    ) => false,
                }
            }
            GroupedInstruction::SubintentInstructions(
                SubintentInstructions::YieldToParent(..)
                | SubintentInstructions::VerifyParent(..),
            ) => self.for_subintent,
            // Permitted Instructions
            GroupedInstruction::TakeFromWorktopInstructions(..)
            | GroupedInstruction::ReturnToWorktopInstructions(..)
            | GroupedInstruction::AssertionInstructions(..)
            | GroupedInstruction::ProofInstructions(..)
            | GroupedInstruction::InvocationInstructions(
                InvocationInstructions::CallFunction(..),
            )
            | GroupedInstruction::BurnResourceInstructions(..)
            | GroupedInstruction::AddressAllocationInstructions(..) => true,
            // Disallowed Instructions
            GroupedInstruction::SubintentInstructions(
                SubintentInstructions::YieldToChild(..),
            )
            | GroupedInstruction::InvocationInstructions(
                InvocationInstructions::CallRoleAssignmentMethod(..)
                | InvocationInstructions::CallMetadataMethod(..)
                | InvocationInstructions::CallRoyaltyMethod(..)
                | InvocationInstructions::CallDirectVaultMethod(..),
            ) => false,
        };
        permission_state.next_instruction_status(is_next_instruction_permitted);
    }

    fn process_requirement(
        &mut self,
        requirement_state: &mut Self::RequirementState,
        _: &NamedAddressStore,
        instruction: &GroupedInstruction,
        _: Option<(
            &ManifestInvocationReceiver,
            &TypedManifestNativeInvocation,
        )>,
    ) {
        requirement_state.handle_instruction(instruction);
    }

    fn process_instruction(
        &mut self,
        _: &NamedAddressStore,
        _: &GroupedInstruction,
        _: Option<(
            &ManifestInvocationReceiver,
            &TypedManifestNativeInvocation,
        )>,
    ) {
    }
}

pub struct GeneralInitializer {
    /// This flag changes some of the rules that this transaction type Analyzer
    /// uses and it can be used to change between it being used for the general
    /// transaction type and the subintent general transaction type. This is
    /// implemented as a flag to allow for the majority of the code to be
    /// shared between the two transaction types.
    pub for_subintent: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GeneralRequirementState {
    for_subintent: bool,
    is_yield_to_parent_seen: bool,
    is_any_instruction_seen: bool,
}

impl ManifestAnalyzerRequirementState for GeneralRequirementState {
    fn all_requirements_met(&self) -> bool {
        if self.for_subintent {
            self.is_yield_to_parent_seen
        } else {
            self.is_any_instruction_seen
        }
    }
}

impl GeneralRequirementState {
    pub fn new(for_subintent: bool) -> Self {
        Self {
            for_subintent,
            is_yield_to_parent_seen: false,
            is_any_instruction_seen: false,
        }
    }

    pub fn handle_instruction(&mut self, instruction: &GroupedInstruction) {
        self.is_any_instruction_seen = true;
        self.is_yield_to_parent_seen |=
            instruction.as_yield_to_parent().is_some()
    }
}
