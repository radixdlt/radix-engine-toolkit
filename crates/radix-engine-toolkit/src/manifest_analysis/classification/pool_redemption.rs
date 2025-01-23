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

// ---------------------------- IMPLEMENTATION NOTE ----------------------------
// The rules for the static and dynamic classifier differ for this type. Where
// the static classifier has weaker requirements for manifests to be classified
// as redemption transactions. The static classifier only checks if the required
// set of invocations are there and that no disallowed instructions are present.
// The dynamic analyzer on the other hand does a few more checks to ensure that
// the source and sink of the resources is what we want it to be .
// -----------------------------------------------------------------------------

use crate::internal_prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct PoolRedemptionAnalyzer(PoolRedemptionOutput);

impl ManifestStaticAnalyzer for PoolRedemptionAnalyzer {
    type Initializer = ();
    type Output = ();
    type PermissionState = SimplePermissionState;
    type RequirementState = PoolRedemptionStaticRequirementState;

    fn new(
        _: Self::Initializer,
    ) -> (Self, Self::PermissionState, Self::RequirementState) {
        Default::default()
    }

    fn output(self) -> Self::Output {}

    fn process_permission(
        &mut self,
        permission_state: &mut Self::PermissionState,
        named_address_store: &NamedAddressStore,
        instruction: &GroupedInstruction,
        _: Option<&TypedNativeInvocation>,
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
        };
        permission_state.next_instruction_status(is_next_instruction_permitted);
    }

    fn process_requirement(
        &mut self,
        requirement_state: &mut Self::RequirementState,
        _: &NamedAddressStore,
        _: &GroupedInstruction,
        typed_native_invocation: Option<&TypedNativeInvocation>,
    ) {
        requirement_state.on_instruction(typed_native_invocation)
    }

    fn process_instruction(
        &mut self,
        _: &NamedAddressStore,
        _: &GroupedInstruction,
        _: Option<&TypedNativeInvocation>,
    ) {
        // No processing is done in the static analyzer. All of the processing
        // for this transaction type is done in the dynamic analyzer since it
        // requires us to monitor some invocations and resource movements.
    }
}

impl ManifestDynamicAnalyzer for PoolRedemptionAnalyzer {
    type Output = PoolRedemptionOutput;
    type RequirementState = PoolRedemptionDynamicRequirementState;

    fn new(
        _: Self::Initializer,
    ) -> (
        Self,
        <Self as ManifestStaticAnalyzer>::PermissionState,
        <Self as ManifestStaticAnalyzer>::RequirementState,
        <Self as ManifestDynamicAnalyzer>::RequirementState,
    ) {
        Default::default()
    }

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

    fn process_requirement(
        &mut self,
        requirement_state: &mut <Self as ManifestDynamicAnalyzer>::RequirementState,
        _: &NamedAddressStore,
        _: &GroupedInstruction,
        invocation_io: &InvocationIo<InvocationIoItems>,
        typed_native_invocation: Option<&TypedNativeInvocation>,
    ) {
        requirement_state
            .on_instruction(invocation_io, typed_native_invocation);
    }

    fn process_instruction(
        &mut self,
        _: &NamedAddressStore,
        _: &GroupedInstruction,
        invocation_io: &InvocationIo<InvocationIoItems>,
        typed_native_invocation: Option<&TypedNativeInvocation>,
    ) {
        if let Some(TypedNativeInvocation {
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
        }) = typed_native_invocation
        {
            let pool_address = ComponentAddress::try_from(*pool_address)
                .expect(
                "Must succeed since the typed invocation conversion succeeded",
            );

            let input = invocation_io.input.items_iter().next();
            if let Some(pool_units_input) = input {
                self.0.redemption_operations.push(PoolRedemptionOperation {
                    pool_address,
                    redeemed_resources: invocation_io
                        .output
                        .items_iter()
                        .map(|item| (*item.resource_address(), *item.amount()))
                        .collect(),
                    pool_units_resource_address: *pool_units_input
                        .resource_address(),
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

impl PoolRedemptionStaticRequirementState {
    fn on_instruction(
        &mut self,
        typed_native_invocation: Option<&TypedNativeInvocation>,
    ) {
        if let Some(TypedNativeInvocation {
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
        }) = typed_native_invocation
        {
            self.is_pool_redemption_seen = true
        }
    }
}

impl ManifestAnalyzerRequirementState for PoolRedemptionStaticRequirementState {
    fn all_requirements_met(&self) -> bool {
        self.is_pool_redemption_seen
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct PoolRedemptionDynamicRequirementState {
    /// When resources are withdrawn from the account this accumulator increases
    /// and when they're redeemed they decrease. When we finish, all values
    /// must be equal to zero.
    accumulator: HashMap<ResourceAddress, Decimal>,
}

impl PoolRedemptionDynamicRequirementState {
    fn on_instruction(
        &mut self,
        invocation_io: &InvocationIo<InvocationIoItems>,
        typed_native_invocation: Option<&TypedNativeInvocation>,
    ) {
        match typed_native_invocation {
            Some(TypedNativeInvocation {
                receiver: ManifestInvocationReceiver::GlobalMethod(..),
                invocation:
                    TypedManifestNativeInvocation::AccountBlueprintInvocation(
                        AccountBlueprintInvocation::Method(
                            AccountBlueprintMethod::Withdraw(
                                AccountWithdrawManifestInput {
                                    resource_address:
                                        ManifestResourceAddress::Static(resource_address),
                                    amount,
                                },
                            )
                            | AccountBlueprintMethod::LockFeeAndWithdraw(
                                AccountLockFeeAndWithdrawManifestInput {
                                    resource_address:
                                        ManifestResourceAddress::Static(resource_address),
                                    amount,
                                    ..
                                },
                            ),
                        ),
                    ),
            }) => {
                *self.accumulator.entry(*resource_address).or_default() += *amount;
            }
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
            }) => {
                for item in invocation_io.input.items_iter() {
                    let resource_address = item.resource_address();
                    let amount = item.amount();
                    *self.accumulator.entry(*resource_address).or_default() -= *amount;
                }
            }
            _ => {}
        }
    }
}

impl ManifestAnalyzerRequirementState
    for PoolRedemptionDynamicRequirementState
{
    fn all_requirements_met(&self) -> bool {
        self.accumulator.values().all(Decimal::is_zero)
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
