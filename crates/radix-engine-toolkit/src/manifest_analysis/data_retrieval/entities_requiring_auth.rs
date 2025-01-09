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
pub struct EntitiesRequiringAuthVisitor(EntitiesRequiringAuthOutput);

impl EntitiesRequiringAuthVisitor {
    pub fn new() -> Self {
        Default::default()
    }
}

impl ManifestAnalysisVisitor for EntitiesRequiringAuthVisitor {
    type Output = EntitiesRequiringAuthOutput;
    type ValidityState = ConstVisitorValidityState<true>;

    fn output(self) -> Self::Output {
        self.0
    }

    fn validity_state(&self) -> &Self::ValidityState {
        &ConstVisitorValidityState::<true>
    }

    fn on_instruction(
        &mut self,
        named_address_store: &NamedAddressStore,
        grouped_instruction: &GroupedInstruction,
        _: &InstructionIndex,
        maybe_typed_invocation: Option<&TypedManifestNativeInvocation>,
    ) {
        // We're interested in invocations and in the invoked address so we
        // compute that. In the event that the instruction isn't an invocation
        // or that it's not one to a global entity then we return from this
        // method having done no work.
        let Some(address) = grouped_instruction
            .as_invocation_instructions()
            .and_then(|invocation| invocation.invoked_global_entity())
        else {
            return;
        };

        // Checking the typed invocation and deciding what requires auth and
        // what doesn't based on that.
        let address_requiring_auth = match maybe_typed_invocation {
            // Accounts
            Some(TypedManifestNativeInvocation::AccountBlueprintInvocation(
                AccountBlueprintInvocation::Method(method),
            )) => match method {
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
                | AccountBlueprintMethod::SetDefaultDepositRule(..)
                | AccountBlueprintMethod::SetResourcePreference(..)
                | AccountBlueprintMethod::RemoveResourcePreference(..)
                | AccountBlueprintMethod::AddAuthorizedDepositor(..)
                | AccountBlueprintMethod::RemoveAuthorizedDepositor(..) => Some(address),
                AccountBlueprintMethod::TryDepositOrRefund(..)
                | AccountBlueprintMethod::TryDepositBatchOrRefund(..)
                | AccountBlueprintMethod::TryDepositOrAbort(..)
                | AccountBlueprintMethod::TryDepositBatchOrAbort(..) => None,
            },
            Some(TypedManifestNativeInvocation::AccountLockerBlueprintInvocation(
                AccountLockerBlueprintInvocation::Method(method),
            )) => match method {
                AccountLockerBlueprintMethod::Claim(AccountLockerClaimManifestInput {
                    claimant,
                    ..
                })
                | AccountLockerBlueprintMethod::ClaimNonFungibles(
                    AccountLockerClaimNonFungiblesManifestInput { claimant, .. },
                ) => Some((*claimant).into()),
                AccountLockerBlueprintMethod::Store(..)
                | AccountLockerBlueprintMethod::Airdrop(..)
                | AccountLockerBlueprintMethod::Recover(..)
                | AccountLockerBlueprintMethod::RecoverNonFungibles(..)
                | AccountLockerBlueprintMethod::GetAmount(..)
                | AccountLockerBlueprintMethod::GetNonFungibleLocalIds(..) => None,
            },
            // Identities
            Some(TypedManifestNativeInvocation::IdentityBlueprintInvocation(
                IdentityBlueprintInvocation::Method(IdentityBlueprintMethod::Securify(..)),
            )) => Some(address),
            // Role Assignment Module
            Some(TypedManifestNativeInvocation::RoleAssignmentBlueprintInvocation(
                RoleAssignmentBlueprintInvocation::Method(method),
            )) => match method {
                RoleAssignmentBlueprintMethod::SetOwner(..)
                | RoleAssignmentBlueprintMethod::LockOwner(..)
                | RoleAssignmentBlueprintMethod::Set(..) => Some(address),
                RoleAssignmentBlueprintMethod::Get(..) => None,
            },
            // Metadata Module
            Some(TypedManifestNativeInvocation::MetadataBlueprintInvocation(
                MetadataBlueprintInvocation::Method(method),
            )) => match method {
                MetadataBlueprintMethod::Set(..)
                | MetadataBlueprintMethod::Lock(..)
                | MetadataBlueprintMethod::Remove(..) => Some(address),
                MetadataBlueprintMethod::Get(..) => None,
            },
            // Royalty Module
            Some(TypedManifestNativeInvocation::ComponentRoyaltyBlueprintInvocation(
                ComponentRoyaltyBlueprintInvocation::Method(method),
            )) => match method {
                ComponentRoyaltyBlueprintMethod::SetRoyalty(_)
                | ComponentRoyaltyBlueprintMethod::LockRoyalty(_)
                | ComponentRoyaltyBlueprintMethod::ClaimRoyalties(_) => Some(address),
            },
            // None of the following require auth from accounts or identities
            // controlled by the wallet.
            Some(
                TypedManifestNativeInvocation::AccessControllerBlueprintInvocation(..)
                | TypedManifestNativeInvocation::ConsensusManagerBlueprintInvocation(..)
                | TypedManifestNativeInvocation::ValidatorBlueprintInvocation(..)
                | TypedManifestNativeInvocation::PackageBlueprintInvocation(..)
                | TypedManifestNativeInvocation::OneResourcePoolBlueprintInvocation(..)
                | TypedManifestNativeInvocation::TwoResourcePoolBlueprintInvocation(..)
                | TypedManifestNativeInvocation::MultiResourcePoolBlueprintInvocation(..)
                | TypedManifestNativeInvocation::FungibleResourceManagerBlueprintInvocation(..)
                | TypedManifestNativeInvocation::NonFungibleResourceManagerBlueprintInvocation(..)
                | TypedManifestNativeInvocation::FungibleVaultBlueprintInvocation(..)
                | TypedManifestNativeInvocation::NonFungibleVaultBlueprintInvocation(..)
                | TypedManifestNativeInvocation::TransactionTrackerBlueprintInvocation(..)
                | TypedManifestNativeInvocation::AccountBlueprintInvocation(
                    AccountBlueprintInvocation::DirectMethod(..)
                    | AccountBlueprintInvocation::Function(..),
                )
                | TypedManifestNativeInvocation::AccountLockerBlueprintInvocation(
                    AccountLockerBlueprintInvocation::DirectMethod(..)
                    | AccountLockerBlueprintInvocation::Function(..),
                )
                | TypedManifestNativeInvocation::IdentityBlueprintInvocation(
                    IdentityBlueprintInvocation::DirectMethod(..)
                    | IdentityBlueprintInvocation::Function(..),
                )
                | TypedManifestNativeInvocation::RoleAssignmentBlueprintInvocation(
                    RoleAssignmentBlueprintInvocation::DirectMethod(..)
                    | RoleAssignmentBlueprintInvocation::Function(..),
                )
                | TypedManifestNativeInvocation::MetadataBlueprintInvocation(
                    MetadataBlueprintInvocation::DirectMethod(..)
                    | MetadataBlueprintInvocation::Function(..),
                )
                | TypedManifestNativeInvocation::ComponentRoyaltyBlueprintInvocation(
                    ComponentRoyaltyBlueprintInvocation::DirectMethod(..)
                    | ComponentRoyaltyBlueprintInvocation::Function(..),
                ),
            )
            | None => None,
        };

        // Do not continue if we do not need auth for the methods that we have.
        let Some(address_requiring_auth) = address_requiring_auth else {
            return;
        };

        // Getting the grouped entity type for the address.
        let grouped_entity_type = match address_requiring_auth {
            ManifestGlobalAddress::Named(named_address) => named_address_store
                .get(&named_address)
                .and_then(BlueprintId::entity_type),
            ManifestGlobalAddress::Static(static_address) => {
                static_address.as_node_id().entity_type()
            }
        }
        .map(GroupedEntityType::from);

        // Adding the address to the output depending on the address's type.
        if grouped_entity_type.is_some_and(|entity_type| {
            entity_type.belongs_to_account_entities()
        }) {
            self.0.accounts.insert(address_requiring_auth);
        } else if grouped_entity_type.is_some_and(|entity_type| {
            entity_type.belongs_to_identity_entities()
        }) {
            self.0.identities.insert(address_requiring_auth);
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct EntitiesRequiringAuthOutput {
    pub accounts: IndexSet<ManifestGlobalAddress>,
    pub identities: IndexSet<ManifestGlobalAddress>,
}
