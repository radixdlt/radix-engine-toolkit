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
pub struct ValidatorClaimAnalyzer(ValidatorClaimingXrdOutput);

impl ManifestStaticAnalyzer for ValidatorClaimAnalyzer {
    type Initializer = ();
    type Output = ();
    type PermissionState =
        CallbackPermissionState<PermissionStateStaticCallback>;
    type RequirementState = AllOfRequirement<(
        AccountOnlyNonFungibleWithdrawsRequirement,
        AccountWithdrawInstructionPresentRequirement,
        AccountDepositInstructionPresentRequirement,
        AccountsDepositedIntoSubsetOfWithdrawnFromRequirement,
        AccountResourcesWithdrawnAreNotDepositedBackRequirement,
        ValidatorClaimXrdInstructionPresentRequirement,
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

impl ManifestDynamicAnalyzer for ValidatorClaimAnalyzer {
    type Output = ValidatorClaimingXrdOutput;
    type RequirementState = ValidatorClaimDynamicRequirementState;

    fn new(
        _: Self::Initializer,
    ) -> (
        Self,
        <Self as ManifestStaticAnalyzer>::PermissionState,
        <Self as ManifestStaticAnalyzer>::RequirementState,
        <Self as ManifestDynamicAnalyzer>::RequirementState,
    ) {
        (
            Default::default(),
            CallbackPermissionState::new(is_instruction_permitted),
            Default::default(),
            Default::default(),
        )
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

    fn process_instruction(&mut self, context: AnalysisContext<'_>) {
        let AnalysisContext::InvocationInstruction {
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
                                ValidatorBlueprintMethod::ClaimXrd(..),
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

        let input = dynamic_analysis_invocation_io.input.items_iter().next();
        let output = dynamic_analysis_invocation_io.output.items_iter().next();

        if let Some((
            InvocationIoItem::NonFungible(
                claim_nft_resource_address,
                claim_nft_ids,
            ),
            InvocationIoItem::Fungible(XRD, xrd_amount),
        )) = input.zip(output)
        {
            self.0.claim_operations.push(ValidatorClaimOperation {
                validator_address,
                claim_nft_address: *claim_nft_resource_address,
                claim_nft_ids: (**claim_nft_ids).clone(),
                xrd_amount: **xrd_amount,
            });
        }
    }
}

/// The requirement state that must be fulfilled in order for a manifest to get
/// a validator claim detailed classification. The primary difference between
/// the static and dynamic requirement is that in the static requirement there
/// is no way for us to check the source and destination of the LSUs.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct ValidatorClaimDynamicRequirementState {
    /// This accumulator is similar to the [`accumulator`] found in the
    /// [`ValidatorStakeDynamicRequirementState`] type but this one works for
    /// multiple resources whereas the other one works only for XRD.
    ///
    /// The purpose of this accumulator is the same as the other one: we wish to
    /// verify that all of the liquid stake units sourced from the account are
    /// sinked into validator claim calls. Therefore, we track all withdraws
    /// here and:
    ///
    /// * With every withdraw from an account we increase this accumulator by
    ///   the amount withdrawn.
    /// * With every claim from a validator we decrease this accumulator by
    ///   the amount of claim NFTs that were claimed.
    ///
    /// In order for it to be classified as an unstaking transaction then all of
    /// the accumulators must be equal to zero.
    ///
    /// [`accumulator`]: ValidatorStakeDynamicRequirementState::accumulator
    accumulator: HashMap<ResourceAddress, Decimal>,
}

impl ManifestAnalyzerRequirementState
    for ValidatorClaimDynamicRequirementState
{
    fn requirement_state(&self) -> RequirementState {
        // All of the accumulators must equal to zero. In this case, it means
        // that all of the resources that were withdrawn were deposited or were
        // passed to the `claim` method on the validators.
        match self.accumulator.values().all(Decimal::is_zero) {
            true => RequirementState::Fulfilled,
            false => RequirementState::CurrentlyUnfulfilled,
        }
    }

    fn process_instruction(&mut self, context: AnalysisContext<'_>) {
        let AnalysisContext::InvocationInstruction {
            typed_native_invocation: Some(typed_native_invocation),
            dynamic_analysis_invocation_io: Some(dynamic_analysis_invocation_io),
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
            } => {
                *self.accumulator.entry(*resource_address).or_default() += *amount;
            }
            TypedNativeInvocation {
                receiver: ManifestInvocationReceiver::GlobalMethod(..),
                invocation:
                    TypedManifestNativeInvocation::AccountBlueprintInvocation(
                        AccountBlueprintInvocation::Method(
                            AccountBlueprintMethod::WithdrawNonFungibles(
                                AccountWithdrawNonFungiblesManifestInput {
                                    resource_address:
                                        ManifestResourceAddress::Static(resource_address),
                                    ids,
                                },
                            )
                            | AccountBlueprintMethod::LockFeeAndWithdrawNonFungibles(
                                AccountLockFeeAndWithdrawNonFungiblesManifestInput {
                                    resource_address:
                                        ManifestResourceAddress::Static(resource_address),
                                    ids,
                                    ..
                                },
                            ),
                        ),
                    ),
            } => {
                *self.accumulator.entry(*resource_address).or_default() +=
                    Decimal::from(ids.len());
            }
            TypedNativeInvocation {
                receiver: ManifestInvocationReceiver::GlobalMethod(..),
                invocation:
                    TypedManifestNativeInvocation::ValidatorBlueprintInvocation(
                        ValidatorBlueprintInvocation::Method(
                            ValidatorBlueprintMethod::ClaimXrd(..),
                        ),
                    ),
            } => {
                if let Some(claim_nft) =
                    dynamic_analysis_invocation_io.input.items_iter().next()
                {
                    *self
                        .accumulator
                        .entry(*claim_nft.resource_address())
                        .or_default() -= *claim_nft.amount();
                }
            }
            _ => {}
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct ValidatorClaimingXrdOutput {
    pub claim_operations: Vec<ValidatorClaimOperation>,
}

/// This type represents claim operations from the validators.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidatorClaimOperation {
    pub validator_address: ComponentAddress,
    /* Input */
    pub claim_nft_address: ResourceAddress,
    pub claim_nft_ids: IndexSet<NonFungibleLocalId>,
    /* Output */
    pub xrd_amount: Decimal,
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
                    VALIDATOR_CLAIM_XRD_IDENT,
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
        // Permitted Instructions
        GroupedInstruction::TakeFromWorktopInstructions(..)
        | GroupedInstruction::ReturnToWorktopInstructions(..)
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
