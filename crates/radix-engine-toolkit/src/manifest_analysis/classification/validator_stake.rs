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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ValidatorStakeAnalyzer(ValidatorStakingOutput);

impl ManifestStaticAnalyzer for ValidatorStakeAnalyzer {
    type Initializer = ();
    type Output = ();
    type PermissionState =
        CallbackPermissionState<PermissionStateStaticCallback>;
    type RequirementState = AllOfRequirement<(
        AccountOnlyXrdWithdrawsRequirement,
        AccountWithdrawInstructionPresentRequirement,
        AccountDepositInstructionPresentRequirement,
        AccountsDepositedIntoSubsetOfWithdrawnFromRequirement,
        AccountResourcesWithdrawnAreNotDepositedBackRequirement,
        ValidatorStakeInstructionPresentRequirement,
    )>;

    fn new(
        _: Self::Initializer,
    ) -> (Self, Self::PermissionState, Self::RequirementState) {
        (
            Default::default(),
            CallbackPermissionState::new(is_instruction_permitted),
            Default::default(),
        )
    }

    fn output(self) -> Self::Output {}

    fn process_instruction(&mut self, _: InstructionContext<'_>) {
        // No processing is done in the static analyzer. All of the processing
        // for this transaction type is done in the dynamic analyzer since it
        // requires us to monitor some invocations and resource movements.
    }
}

impl ManifestDynamicAnalyzer for ValidatorStakeAnalyzer {
    type Output = ValidatorStakingOutput;

    fn output(
        self,
    ) -> CombinedAnalysisOutput<
        <Self as ManifestStaticAnalyzer>::Output,
        <Self as ManifestDynamicAnalyzer>::Output,
    > {
        CombinedAnalysisOutput {
            static_analyzer_output: (),
            dynamic_analyzer_output: self.0,
        }
    }

    fn process_instruction(&mut self, context: InstructionContext<'_>) {
        let InstructionContext::InvocationInstruction {
            typed_native_invocation:
                Some(TypedNativeInvocation {
                    receiver:
                        ManifestInvocationReceiver::GlobalMethod(
                            ResolvedManifestAddress::Static {
                                static_address: validator_address,
                            },
                        ),
                    invocation:
                        TypedManifestNativeInvocation::ValidatorBlueprintInvocation(
                            ValidatorBlueprintInvocation::Method(
                                ValidatorBlueprintMethod::Stake(..),
                            ),
                        ),
                }),
            dynamic_analysis_invocation_io: Some(dynamic_analysis_invocation_io),
            ..
        } = context
        else {
            return;
        };

        let validator_address = ComponentAddress::try_from(*validator_address)
            .expect(
                "Must succeed since the typed invocation conversion succeeded",
            );

        let staked_xrd =
            dynamic_analysis_invocation_io.input.resource_amount(&XRD);
        if let Some(output) = dynamic_analysis_invocation_io.output.first() {
            self.0.stake_operations.push(ValidatorStakeOperation {
                validator_address,
                staked_xrd_amount: staked_xrd,
                liquid_stake_unit_resource_address: *output.resource_address(),
                liquid_stake_unit_amount: *output.amount(),
            });
        }
    }
}

/// The requirement state that is required in order for us to get a validator
/// stake classification. This is not the same as the requirements needed to
/// get a validator stake detailed classification.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ValidatorStakeStaticRequirementState {
    is_withdraws_just_xrd: bool,
    is_validator_stake_seen: bool,
}

impl Default for ValidatorStakeStaticRequirementState {
    fn default() -> Self {
        Self {
            is_withdraws_just_xrd: true,
            is_validator_stake_seen: false,
        }
    }
}

impl ManifestAnalyzerRequirementState for ValidatorStakeStaticRequirementState {
    fn requirement_state(&self) -> RequirementState {
        match self.is_withdraws_just_xrd && self.is_validator_stake_seen {
            true => RequirementState::Fulfilled,
            false => RequirementState::CurrentlyUnfulfilled,
        }
    }

    fn process_instruction(&mut self, context: InstructionContext<'_>) {
        let InstructionContext::InvocationInstruction {
            typed_native_invocation: Some(typed_native_invocation),
            ..
        } = context
        else {
            return;
        };

        match typed_native_invocation {
            TypedNativeInvocation {
                receiver: ManifestInvocationReceiver::GlobalMethod(..),
                invocation:
                    TypedManifestNativeInvocation::AccountBlueprintInvocation(
                        AccountBlueprintInvocation::Method(
                            AccountBlueprintMethod::Withdraw(
                                AccountWithdrawManifestInput {
                                    resource_address,
                                    ..
                                },
                            )
                            | AccountBlueprintMethod::LockFeeAndWithdraw(
                                AccountLockFeeAndWithdrawManifestInput {
                                    resource_address,
                                    ..
                                },
                            ),
                        ),
                    ),
            } => {
                self.is_withdraws_just_xrd &=
                    *resource_address == ManifestResourceAddress::Static(XRD)
            }
            TypedNativeInvocation {
                receiver: ManifestInvocationReceiver::GlobalMethod(..),
                invocation:
                    TypedManifestNativeInvocation::ValidatorBlueprintInvocation(
                        ValidatorBlueprintInvocation::Method(
                            ValidatorBlueprintMethod::Stake(..),
                        ),
                    ),
            } => self.is_validator_stake_seen = true,
            _ => {}
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ValidatorStakingOutput {
    pub stake_operations: Vec<ValidatorStakeOperation>,
}

/// This type represents a single validator stake operation. It contains the
/// [`ComponentAddress`] of the validator that was staked to. The amount of XRD,
/// as well as the resource address and amount of liquid stake units returned
/// back.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ValidatorStakeOperation {
    pub validator_address: ComponentAddress,
    pub staked_xrd_amount: Decimal,
    pub liquid_stake_unit_resource_address: ResourceAddress,
    pub liquid_stake_unit_amount: Decimal,
}

fn is_instruction_permitted(context: InstructionContext<'_>) -> bool {
    match context.instruction() {
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
                    | ACCOUNT_DEPOSIT_IDENT
                    | ACCOUNT_DEPOSIT_BATCH_IDENT
                    | ACCOUNT_TRY_DEPOSIT_OR_ABORT_IDENT
                    | ACCOUNT_TRY_DEPOSIT_BATCH_OR_ABORT_IDENT
                    | ACCOUNT_CREATE_PROOF_OF_AMOUNT_IDENT
                    | ACCOUNT_CREATE_PROOF_OF_NON_FUNGIBLES_IDENT
                    | ACCOUNT_LOCK_FEE_IDENT
                    | ACCOUNT_LOCK_CONTINGENT_FEE_IDENT
                    | ACCOUNT_LOCK_FEE_AND_WITHDRAW_IDENT,
                )
                | (
                    Some(GroupedEntityType::AccessControllerEntities(..)),
                    ACCESS_CONTROLLER_CREATE_PROOF_IDENT,
                )
                | (
                    Some(GroupedEntityType::ValidatorEntities(..)),
                    VALIDATOR_STAKE_IDENT,
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
                        | GroupedEntityType::GenericComponentEntities(..)
                        | GroupedEntityType::AccountLockerEntities(..),
                    )
                    | None,
                    _,
                ) => false,
            }
        }
        GroupedInstruction::TakeFromWorktopInstructions(..) => true,
        // Permitted Instructions
        GroupedInstruction::ReturnToWorktopInstructions(..)
        | GroupedInstruction::AssertionInstructions(..)
        | GroupedInstruction::ProofInstructions(..) => true,
        // Disallowed Instructions
        GroupedInstruction::SubintentInstructions(..)
        | GroupedInstruction::BurnResourceInstructions(..)
        | GroupedInstruction::AddressAllocationInstructions(..)
        | GroupedInstruction::InvocationInstructions(
            InvocationInstructions::CallFunction(..)
            | InvocationInstructions::CallRoyaltyMethod(..)
            | InvocationInstructions::CallMetadataMethod(..)
            | InvocationInstructions::CallDirectVaultMethod(..)
            | InvocationInstructions::CallRoleAssignmentMethod(..),
        ) => false,
    }
}
