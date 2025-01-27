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
pub struct PoolRedemptionAnalyzer(PoolRedemptionOutput);

impl ManifestStaticAnalyzer for PoolRedemptionAnalyzer {
    type Initializer = ();
    type Output = ();
    type PermissionState =
        CallbackPermissionState<PermissionStateStaticCallback>;
    type RequirementState = AllOfRequirement<(
        AccountOnlyFungibleWithdrawsRequirement,
        AccountWithdrawInstructionPresentRequirement,
        AccountDepositInstructionPresentRequirement,
        AccountsDepositedIntoSubsetOfWithdrawnFromRequirement,
        AccountResourcesWithdrawnAreNotDepositedBackRequirement,
        PoolRedeemInstructionPresentRequirement,
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

    fn process_instruction(&mut self, _: AnalysisContext<'_>) {
        // No processing is done in the static analyzer. All of the processing
        // for this transaction type is done in the dynamic analyzer since it
        // requires us to monitor some invocations and resource movements.
    }
}

impl ManifestDynamicAnalyzer for PoolRedemptionAnalyzer {
    type Output = PoolRedemptionOutput;

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

    fn process_instruction(&mut self, context: AnalysisContext<'_>) {
        if let AnalysisContext::InvocationInstruction {
            typed_native_invocation:
                Some(TypedNativeInvocation {
                    receiver:
                        ManifestInvocationReceiver::GlobalMethod(
                            ResolvedManifestAddress::Static {
                                static_address: pool_address,
                            },
                        ),
                    invocation:
                        TypedManifestNativeInvocation::OneResourcePoolBlueprintInvocation(
                            OneResourcePoolBlueprintInvocation::Method(
                                OneResourcePoolBlueprintMethod::Redeem(..),
                            ),
                        )
                        | TypedManifestNativeInvocation::TwoResourcePoolBlueprintInvocation(
                            TwoResourcePoolBlueprintInvocation::Method(
                                TwoResourcePoolBlueprintMethod::Redeem(..),
                            ),
                        )
                        | TypedManifestNativeInvocation::MultiResourcePoolBlueprintInvocation(
                            MultiResourcePoolBlueprintInvocation::Method(
                                MultiResourcePoolBlueprintMethod::Redeem(..),
                            ),
                        ),
                }),
            dynamic_analysis_invocation_io: Some(dynamic_analysis_invocation_io),
            ..
        } = context
        {
            let pool_address = ComponentAddress::try_from(*pool_address)
                .expect("Must succeed since the typed invocation conversion succeeded");

            let input = dynamic_analysis_invocation_io.input.items_iter().next();
            if let Some(pool_units_input) = input {
                self.0.redemption_operations.push(PoolRedemptionOperation {
                    pool_address,
                    redeemed_resources: dynamic_analysis_invocation_io
                        .output
                        .items_iter()
                        .map(|item| (*item.resource_address(), *item.amount()))
                        .collect(),
                    pool_units_resource_address: *pool_units_input.resource_address(),
                    pool_units_amount: *pool_units_input.amount(),
                })
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PoolRedemptionStaticRequirementState {
    is_pool_redemption_seen: bool,
}

impl ManifestAnalyzerRequirementState for PoolRedemptionStaticRequirementState {
    fn requirement_state(&self) -> RequirementState {
        match self.is_pool_redemption_seen {
            true => RequirementState::Fulfilled,
            false => RequirementState::CurrentlyUnfulfilled,
        }
    }

    fn process_instruction(&mut self, context: AnalysisContext<'_>) {
        if let AnalysisContext::InvocationInstruction {
            typed_native_invocation:
                Some(TypedNativeInvocation {
                    receiver: ManifestInvocationReceiver::GlobalMethod(..),
                    invocation:
                        TypedManifestNativeInvocation::OneResourcePoolBlueprintInvocation(
                            OneResourcePoolBlueprintInvocation::Method(
                                OneResourcePoolBlueprintMethod::Redeem(..),
                            ),
                        )
                        | TypedManifestNativeInvocation::TwoResourcePoolBlueprintInvocation(
                            TwoResourcePoolBlueprintInvocation::Method(
                                TwoResourcePoolBlueprintMethod::Redeem(..),
                            ),
                        )
                        | TypedManifestNativeInvocation::MultiResourcePoolBlueprintInvocation(
                            MultiResourcePoolBlueprintInvocation::Method(
                                MultiResourcePoolBlueprintMethod::Redeem(..),
                            ),
                        ),
                }),
            ..
        } = context
        {
            self.is_pool_redemption_seen = true
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct PoolRedemptionOutput {
    pub redemption_operations: Vec<PoolRedemptionOperation>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PoolRedemptionOperation {
    pub pool_address: ComponentAddress,
    /* Input */
    pub pool_units_resource_address: ResourceAddress,
    pub pool_units_amount: Decimal,
    /* Output */
    pub redeemed_resources: IndexMap<ResourceAddress, Decimal>,
}

fn is_instruction_permitted(context: AnalysisContext<'_>) -> bool {
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
                    // All of the pool blueprints have the same name for the
                    // contribute and redeem methods and I wanted to use a
                    // constant here so I chose to use the constants for the
                    // one resource pool blueprint. Again, they're all the
                    // same string for all of the pool blueprints and we do
                    // not need to have redundant strings here.
                    Some(GroupedEntityType::PoolEntities(..)),
                    ONE_RESOURCE_POOL_REDEEM_IDENT,
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
        GroupedInstruction::TakeFromWorktopInstructions(
            TakeFromWorktopInstructions::TakeFromWorktop(..)
            | TakeFromWorktopInstructions::TakeAllFromWorktop(..),
        ) => true,
        // Permitted Instructions
        GroupedInstruction::ReturnToWorktopInstructions(..)
        | GroupedInstruction::AssertionInstructions(..)
        | GroupedInstruction::ProofInstructions(..) => true,
        // Disallowed Instructions
        GroupedInstruction::TakeFromWorktopInstructions(
            TakeFromWorktopInstructions::TakeNonFungiblesFromWorktop(..),
        )
        | GroupedInstruction::SubintentInstructions(..)
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
