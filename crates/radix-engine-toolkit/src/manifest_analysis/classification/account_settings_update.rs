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

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct AccountSettingsUpdateAnalyzer(AccountSettingsUpdateOutput);

impl ManifestStaticAnalyzer for AccountSettingsUpdateAnalyzer {
    type Initializer = ();
    type Output = AccountSettingsUpdateOutput;
    type PermissionState = SimplePermissionState;
    type RequirementState = AccountSettingsUpdateRequirementState;

    fn new(
        _: Self::Initializer,
    ) -> (Self, Self::PermissionState, Self::RequirementState) {
        Default::default()
    }

    fn output(self) -> Self::Output {
        self.0
    }

    fn process_permission(
        &self,
        permission_state: &mut Self::PermissionState,
        context: AnalysisContext<'_>,
    ) {
        // Compute if the next instruction is permitted or not.
        let is_next_instruction_permitted = match context.instruction() {
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
                        ACCOUNT_CREATE_PROOF_OF_AMOUNT_IDENT
                        | ACCOUNT_CREATE_PROOF_OF_NON_FUNGIBLES_IDENT
                        | ACCOUNT_LOCK_FEE_IDENT
                        | ACCOUNT_LOCK_CONTINGENT_FEE_IDENT
                        | ACCOUNT_SET_DEFAULT_DEPOSIT_RULE_IDENT
                        | ACCOUNT_SET_RESOURCE_PREFERENCE_IDENT
                        | ACCOUNT_REMOVE_RESOURCE_PREFERENCE_IDENT
                        | ACCOUNT_ADD_AUTHORIZED_DEPOSITOR_IDENT
                        | ACCOUNT_REMOVE_AUTHORIZED_DEPOSITOR_IDENT,
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
            // Allowed Instructions
            GroupedInstruction::ProofInstructions(..) => true,
            // Disallowed Instructions
            GroupedInstruction::TakeFromWorktopInstructions(..)
            | GroupedInstruction::ReturnToWorktopInstructions(..)
            | GroupedInstruction::AssertionInstructions(..)
            | GroupedInstruction::SubintentInstructions(..)
            | GroupedInstruction::AddressAllocationInstructions(..)
            | GroupedInstruction::BurnResourceInstructions(..)
            | GroupedInstruction::InvocationInstructions(
                InvocationInstructions::CallFunction(..)
                | InvocationInstructions::CallDirectVaultMethod(..)
                | InvocationInstructions::CallMetadataMethod(..)
                | InvocationInstructions::CallRoleAssignmentMethod(..)
                | InvocationInstructions::CallRoyaltyMethod(..),
            ) => false,
        };
        permission_state.next_instruction_status(is_next_instruction_permitted);
    }

    fn process_requirement(
        &self,
        requirement_state: &mut Self::RequirementState,
        context: AnalysisContext<'_>,
    ) {
        if let AnalysisContext::InvocationInstruction {
            typed_native_invocation:
                Some(TypedNativeInvocation {
                    receiver: ManifestInvocationReceiver::GlobalMethod(_),
                    invocation:
                        TypedManifestNativeInvocation::AccountBlueprintInvocation(
                            AccountBlueprintInvocation::Method(
                                AccountBlueprintMethod::SetDefaultDepositRule(..)
                                | AccountBlueprintMethod::SetResourcePreference(..)
                                | AccountBlueprintMethod::RemoveResourcePreference(..)
                                | AccountBlueprintMethod::AddAuthorizedDepositor(..)
                                | AccountBlueprintMethod::RemoveAuthorizedDepositor(..),
                            ),
                        ),
                }),
            ..
        } = context
        {
            requirement_state.is_any_account_settings_update_seen = true
        }
    }

    fn process_instruction(&mut self, context: AnalysisContext<'_>) {
        let AnalysisContext::InvocationInstruction {
            typed_native_invocation:
                Some(TypedNativeInvocation {
                    receiver: ManifestInvocationReceiver::GlobalMethod(receiver),
                    invocation:
                        TypedManifestNativeInvocation::AccountBlueprintInvocation(
                            AccountBlueprintInvocation::Method(method),
                        ),
                }),
            ..
        } = context
        else {
            return;
        };

        match method {
            AccountBlueprintMethod::SetDefaultDepositRule(
                AccountSetDefaultDepositRuleInput {
                    default: deposit_rule,
                },
            ) => {
                self.0
                    .default_deposit_rule_updates
                    .insert(receiver.into(), *deposit_rule);
            }
            AccountBlueprintMethod::SetResourcePreference(
                AccountSetResourcePreferenceManifestInput {
                    resource_address,
                    resource_preference,
                },
            ) => {
                self.0.resource_preference_updates.insert(
                    (receiver.into(), *resource_address),
                    Update::Set(*resource_preference),
                );
            }
            AccountBlueprintMethod::RemoveResourcePreference(
                AccountRemoveResourcePreferenceManifestInput {
                    resource_address,
                },
            ) => {
                self.0.resource_preference_updates.insert(
                    (receiver.into(), *resource_address),
                    Update::Remove,
                );
            }
            AccountBlueprintMethod::AddAuthorizedDepositor(
                AccountAddAuthorizedDepositorManifestInput { badge },
            ) => {
                self.0
                    .authorized_depositor_updates
                    .insert((receiver.into(), badge.clone()), Operation::Added);
            }
            AccountBlueprintMethod::RemoveAuthorizedDepositor(
                AccountRemoveAuthorizedDepositorManifestInput { badge },
            ) => {
                self.0.authorized_depositor_updates.insert(
                    (receiver.into(), badge.clone()),
                    Operation::Removed,
                );
            }
            AccountBlueprintMethod::Securify(..)
            | AccountBlueprintMethod::LockFee(..)
            | AccountBlueprintMethod::LockContingentFee(..)
            | AccountBlueprintMethod::Deposit(..)
            | AccountBlueprintMethod::DepositBatch(..)
            | AccountBlueprintMethod::Withdraw(..)
            | AccountBlueprintMethod::WithdrawNonFungibles(..)
            | AccountBlueprintMethod::Burn(..)
            | AccountBlueprintMethod::BurnNonFungibles(..)
            | AccountBlueprintMethod::LockFeeAndWithdraw(..)
            | AccountBlueprintMethod::LockFeeAndWithdrawNonFungibles(..)
            | AccountBlueprintMethod::CreateProofOfAmount(..)
            | AccountBlueprintMethod::CreateProofOfNonFungibles(..)
            | AccountBlueprintMethod::TryDepositOrRefund(..)
            | AccountBlueprintMethod::TryDepositBatchOrRefund(..)
            | AccountBlueprintMethod::TryDepositOrAbort(..)
            | AccountBlueprintMethod::TryDepositBatchOrAbort(..)
            | AccountBlueprintMethod::Balance(..)
            | AccountBlueprintMethod::NonFungibleLocalIds(..)
            | AccountBlueprintMethod::HasNonFungible(..) => {}
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct AccountSettingsUpdateRequirementState {
    is_any_account_settings_update_seen: bool,
}

impl ManifestAnalyzerRequirementState
    for AccountSettingsUpdateRequirementState
{
    fn all_requirements_met(&self) -> bool {
        self.is_any_account_settings_update_seen
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct AccountSettingsUpdateOutput {
    /// Updates to the account resource preference that took place in the
    /// transaction.
    pub resource_preference_updates: IndexMap<
        (ManifestGlobalAddress, ManifestResourceAddress),
        Update<ResourcePreference>,
    >,
    /// Updates to the default deposit rule of some account that took place in
    /// the transaction.
    pub default_deposit_rule_updates:
        IndexMap<ManifestGlobalAddress, DefaultDepositRule>,
    /// Updates to the set of authorized depositors of some account that took
    /// place in the transaction.
    pub authorized_depositor_updates: IndexMap<
        (ManifestGlobalAddress, ManifestResourceOrNonFungible),
        Operation,
    >,
}
