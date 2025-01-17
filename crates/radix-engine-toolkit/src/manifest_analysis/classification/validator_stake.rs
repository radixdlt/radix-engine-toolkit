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
// as staking transactions. The static classifier only checks if the required
// set of invocations are there and that no disallowed instructions are present.
// The dynamic analyzer on the other hand does a few more checks to ensure that
// the source and sink of the resources is what we want it to be .
// -----------------------------------------------------------------------------

use crate::internal_prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ValidatorStakeAnalyzer(ValidatorStakingOutput);

impl ManifestStaticAnalyzer for ValidatorStakeAnalyzer {
    type Initializer = ();
    type Output = ();
    type PermissionState = SimplePermissionState;
    type RequirementState = ValidatorStakeStaticRequirementState;

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

impl ManifestDynamicAnalyzer for ValidatorStakeAnalyzer {
    type Output = ValidatorStakingOutput;
    type RequirementState = ValidatorStakeDynamicRequirementState;

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
                    ValidatorBlueprintMethod::Stake(..),
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

            let staked_xrd = invocation_io.input.resource_amount(&XRD);
            if let Some(output) = invocation_io.output.items_iter().next() {
                self.0.stake_operations.push(ValidatorStakeOperation {
                    validator_address,
                    staked_xrd_amount: staked_xrd,
                    liquid_stake_unit_resource_address: *output
                        .resource_address(),
                    liquid_stake_unit_amount: *output.amount(),
                });
            }
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

impl ValidatorStakeStaticRequirementState {
    fn on_instruction(
        &mut self,
        maybe_typed_invocation: Option<(
            &ManifestInvocationReceiver,
            &TypedManifestNativeInvocation,
        )>,
    ) {
        if let Some((
            ManifestInvocationReceiver::GlobalMethod(..),
            TypedManifestNativeInvocation::AccountBlueprintInvocation(
                AccountBlueprintInvocation::Method(
                    AccountBlueprintMethod::Withdraw(
                        AccountWithdrawManifestInput {
                            resource_address, ..
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
        )) = maybe_typed_invocation
        {
            self.is_withdraws_just_xrd &=
                *resource_address == ManifestResourceAddress::Static(XRD)
        }

        if let Some((
            ManifestInvocationReceiver::GlobalMethod(..),
            TypedManifestNativeInvocation::ValidatorBlueprintInvocation(
                ValidatorBlueprintInvocation::Method(
                    ValidatorBlueprintMethod::Stake(..),
                ),
            ),
        )) = maybe_typed_invocation
        {
            self.is_validator_stake_seen = true
        }
    }
}

impl ManifestAnalyzerRequirementState for ValidatorStakeStaticRequirementState {
    fn all_requirements_met(&self) -> bool {
        self.is_withdraws_just_xrd && self.is_validator_stake_seen
    }
}

/// The requirement state that must be fulfilled in order for a manifest to get
/// a validator stake detailed classification. The primary difference between
/// the static and dynamic requirement is that in the static requirement there
/// is no way for us to check the source and destination of the XRD.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ValidatorStakeDynamicRequirementState {
    /// This accumulator is used to track the source and destination of the XRD.
    /// A requirement that we have is that all of the XRD withdrawn from any
    /// account in the manifest must be staked and that all LSUs must be
    /// deposited into one or more account(s). Essentially, we want to ensure
    /// that in a stake transaction no transfers can take place.
    ///
    /// This accumulator performs this check in the following way:
    ///
    /// * When resources (XRD) is withdrawn from the account this accumulator
    ///   increases by the amount of XRD that was withdrawn.
    /// * When resources (XRD) are staked to a validator this accumulator
    ///   decreases by the amount of XRD that was staked to them.
    ///
    /// For a manifest to be considered a valid staking manifest then this
    /// accumulator must be zero by the end of it. So, this validates that all
    /// XRD sourced from an account are sinked into a validator stake call.
    accumulator: Decimal,
}

impl ValidatorStakeDynamicRequirementState {
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
                                    ManifestResourceAddress::Static(XRD),
                                amount,
                            },
                        )
                        | AccountBlueprintMethod::LockFeeAndWithdraw(
                            AccountLockFeeAndWithdrawManifestInput {
                                resource_address:
                                    ManifestResourceAddress::Static(XRD),
                                amount,
                                ..
                            },
                        ),
                    ),
                ),
            )) => {
                self.accumulator += *amount;
            }
            Some((
                ManifestInvocationReceiver::GlobalMethod(..),
                TypedManifestNativeInvocation::ValidatorBlueprintInvocation(
                    ValidatorBlueprintInvocation::Method(
                        ValidatorBlueprintMethod::Stake(..),
                    ),
                ),
            )) => {
                let staked_xrd = invocation_io.input.resource_amount(&XRD);
                self.accumulator -= staked_xrd;
            }
            _ => {}
        }
    }
}

impl ManifestAnalyzerRequirementState
    for ValidatorStakeDynamicRequirementState
{
    fn all_requirements_met(&self) -> bool {
        self.accumulator.is_zero()
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
