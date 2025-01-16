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
// as unstaking transactions. The static classifier only checks if the required
// set of invocations are there and that no disallowed instructions are present.
// The dynamic analyzer on the other hand does a few more checks to ensure that
// the source and sink of the resources is what we want it to be .
// -----------------------------------------------------------------------------

use crate::internal_prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct ValidatorUnstakeAnalyzer(ValidatorUnstakingOutput);

impl ManifestStaticAnalyzer for ValidatorUnstakeAnalyzer {
    type Initializer = ();
    type Output = ();
    type PermissionState = SimplePermissionState;
    type RequirementState = ValidatorUnstakeStaticRequirementState;

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
                        VALIDATOR_UNSTAKE_IDENT,
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
        maybe_typed_invocation: Option<(
            &ManifestInvocationReceiver,
            &TypedManifestNativeInvocation,
        )>,
    ) {
        requirement_state.on_instruction(maybe_typed_invocation)
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
        // No processing is done in the static analyzer. All of the processing
        // for this transaction type is done in the dynamic analyzer since it
        // requires us to monitor some invocations and resource movements.
    }
}

impl ManifestDynamicAnalyzer for ValidatorUnstakeAnalyzer {
    type Output = ValidatorUnstakingOutput;
    type RequirementState = ValidatorUnstakeDynamicRequirementState;

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
        maybe_typed_invocation: Option<(
            &ManifestInvocationReceiver,
            &TypedManifestNativeInvocation,
        )>,
    ) {
        requirement_state.on_instruction(invocation_io, maybe_typed_invocation);
    }

    fn process_instruction(
        &mut self,
        _: &NamedAddressStore,
        _: &GroupedInstruction,
        invocation_io: &InvocationIo<InvocationIoItems>,
        maybe_typed_invocation: Option<(
            &ManifestInvocationReceiver,
            &TypedManifestNativeInvocation,
        )>,
    ) {
        if let Some((
            ManifestInvocationReceiver::GlobalMethod(
                ResolvedManifestAddress::Static {
                    static_address: validator_address,
                },
            ),
            TypedManifestNativeInvocation::ValidatorBlueprintInvocation(
                ValidatorBlueprintInvocation::Method(
                    ValidatorBlueprintMethod::Unstake(..),
                ),
            ),
        )) = maybe_typed_invocation
        {
            let validator_address = ComponentAddress::try_from(
                *validator_address,
            )
            .expect(
                "Must succeed since the typed invocation conversion succeeded",
            );

            let input = invocation_io.input.items_iter().next();
            let output = invocation_io.output.items_iter().next();

            if let Some((
                InvocationIoItem::Fungible(lsu_resource_address, lsu_amount),
                InvocationIoItem::NonFungible(
                    claim_nft_resource_address,
                    claim_nft_ids,
                ),
            )) = input.zip(output)
            {
                self.0.unstake_operations.push(ValidatorUnstakeOperation {
                    validator_address,
                    liquid_stake_unit_address: *lsu_resource_address,
                    liquid_stake_unit_amount: **lsu_amount,
                    claim_nft_address: *claim_nft_resource_address,
                    claim_nft_ids: (**claim_nft_ids).clone(),
                });
            }
        }
    }
}

/// The requirement state that is required in order for us to get a validator
/// unstake classification. This is not the same as the requirements needed to
/// get a validator unstake detailed classification.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ValidatorUnstakeStaticRequirementState {
    is_validator_unstake_seen: bool,
}

impl ValidatorUnstakeStaticRequirementState {
    fn on_instruction(
        &mut self,
        maybe_typed_invocation: Option<(
            &ManifestInvocationReceiver,
            &TypedManifestNativeInvocation,
        )>,
    ) {
        if let Some((
            ManifestInvocationReceiver::GlobalMethod(..),
            TypedManifestNativeInvocation::ValidatorBlueprintInvocation(
                ValidatorBlueprintInvocation::Method(
                    ValidatorBlueprintMethod::Unstake(..),
                ),
            ),
        )) = maybe_typed_invocation
        {
            self.is_validator_unstake_seen = true
        }
    }
}

impl ManifestAnalyzerRequirementState
    for ValidatorUnstakeStaticRequirementState
{
    fn all_requirements_met(&self) -> bool {
        self.is_validator_unstake_seen
    }
}

/// The requirement state that must be fulfilled in order for a manifest to get
/// a validator unstake detailed classification. The primary difference between
/// the static and dynamic requirement is that in the static requirement there
/// is no way for us to check the source and destination of the LSUs.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct ValidatorUnstakeDynamicRequirementState {
    /// This accumulator is similar to the [`accumulator`] found in the
    /// [`ValidatorStakeDynamicRequirementState`] type but this one works for
    /// multiple resources whereas the other one works only for XRD.
    ///
    /// The purpose of this accumulator is the same as the other one: we wish to
    /// verify that all of the liquid stake units sourced from the account are
    /// sinked into validator unstake calls. Therefore, we track all withdraws
    /// here and:
    ///
    /// * With every withdraw from an account we increase this accumulator by
    ///   the amount withdrawn.
    /// * With every unstake from a validator we decrease this accumulator by
    ///   the amount of LSUs that were unstaked.
    ///
    /// In order for it to be classified as an unstaking transaction then all of
    /// the accumulators must be equal to zero.
    ///
    /// [`accumulator`]: ValidatorStakeDynamicRequirementState::accumulator
    accumulator: HashMap<ResourceAddress, Decimal>,
}

impl ValidatorUnstakeDynamicRequirementState {
    fn on_instruction(
        &mut self,
        invocation_io: &InvocationIo<InvocationIoItems>,
        maybe_typed_invocation: Option<(
            &ManifestInvocationReceiver,
            &TypedManifestNativeInvocation,
        )>,
    ) {
        match maybe_typed_invocation {
            Some((
                ManifestInvocationReceiver::GlobalMethod(..),
                TypedManifestNativeInvocation::AccountBlueprintInvocation(
                    AccountBlueprintInvocation::Method(
                        AccountBlueprintMethod::Withdraw(
                            AccountWithdrawManifestInput {
                                resource_address:
                                    ManifestResourceAddress::Static(
                                        resource_address,
                                    ),
                                amount,
                            },
                        )
                        | AccountBlueprintMethod::LockFeeAndWithdraw(
                            AccountLockFeeAndWithdrawManifestInput {
                                resource_address:
                                    ManifestResourceAddress::Static(
                                        resource_address,
                                    ),
                                amount,
                                ..
                            },
                        ),
                    ),
                ),
            )) => {
                *self.accumulator.entry(*resource_address).or_default() +=
                    *amount;
            }
            Some((
                ManifestInvocationReceiver::GlobalMethod(..),
                TypedManifestNativeInvocation::ValidatorBlueprintInvocation(
                    ValidatorBlueprintInvocation::Method(
                        ValidatorBlueprintMethod::Unstake(..),
                    ),
                ),
            )) => {
                for lsu in invocation_io.input.items_iter() {
                    *self
                        .accumulator
                        .entry(*lsu.resource_address())
                        .or_default() -= *lsu.amount();
                }
            }
            _ => {}
        }
    }
}

impl ManifestAnalyzerRequirementState
    for ValidatorUnstakeDynamicRequirementState
{
    fn all_requirements_met(&self) -> bool {
        // All of the accumulators must equal to zero. In this case, it means
        // that all of the resources that were withdrawn were deposited or were
        // passed to the `unstake` method on the validators.
        self.accumulator.values().all(Decimal::is_zero)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct ValidatorUnstakingOutput {
    pub unstake_operations: Vec<ValidatorUnstakeOperation>,
}

/// This type represents unstake operations from the validators.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidatorUnstakeOperation {
    pub validator_address: ComponentAddress,
    /* Input */
    pub liquid_stake_unit_address: ResourceAddress,
    pub liquid_stake_unit_amount: Decimal,
    /* Output */
    pub claim_nft_address: ResourceAddress,
    pub claim_nft_ids: IndexSet<NonFungibleLocalId>,
}
