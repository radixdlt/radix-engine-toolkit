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
pub struct EntitiesRequiringAuthAnalyzer(EntitiesRequiringAuthOutput);

impl ManifestStaticAnalyzer for EntitiesRequiringAuthAnalyzer {
    type Initializer = ();
    type Output = EntitiesRequiringAuthOutput;
    type PermissionState = ConstState<true>;
    type RequirementState = ConstState<true>;

    fn new(
        _: Self::Initializer,
    ) -> (Self, Self::PermissionState, Self::RequirementState) {
        Default::default()
    }

    fn output(self) -> Self::Output {
        self.0
    }

    fn process_requirement(
        &self,
        _: &mut Self::RequirementState,
        _: AnalysisContext<'_>,
    ) {
    }

    fn process_instruction(&mut self, context: AnalysisContext<'_>) {
        // The primary set of invocations that require auth from either accounts
        // or identities are method calls. No function calls so far require it.
        // So, we match for this.
        let AnalysisContext::InvocationInstruction {
            typed_native_invocation: Some(typed_native_invocation),
            ..
        } = context
        else {
            return;
        };
        match typed_native_invocation {
            // Account
            TypedNativeInvocation {
                receiver: ManifestInvocationReceiver::GlobalMethod(receiver),
                invocation:
                    TypedManifestNativeInvocation::AccountBlueprintInvocation(
                        AccountBlueprintInvocation::Method(method),
                    ),
            } => match method {
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
                | AccountBlueprintMethod::RemoveAuthorizedDepositor(..) => {
                    self.0.accounts.insert(receiver.into());
                }
                AccountBlueprintMethod::TryDepositOrRefund(..)
                | AccountBlueprintMethod::TryDepositBatchOrRefund(..)
                | AccountBlueprintMethod::TryDepositOrAbort(..)
                | AccountBlueprintMethod::TryDepositBatchOrAbort(..)
                | AccountBlueprintMethod::Balance(..)
                | AccountBlueprintMethod::NonFungibleLocalIds(..)
                | AccountBlueprintMethod::HasNonFungible(..) => {}
            },
            TypedNativeInvocation {
                invocation:
                    TypedManifestNativeInvocation::AccountLockerBlueprintInvocation(
                        AccountLockerBlueprintInvocation::Method(method),
                    ),
                ..
            } => match method {
                AccountLockerBlueprintMethod::Claim(
                    AccountLockerClaimManifestInput { claimant, .. },
                )
                | AccountLockerBlueprintMethod::ClaimNonFungibles(
                    AccountLockerClaimNonFungiblesManifestInput { claimant, .. },
                ) => {
                    self.0.accounts.insert(claimant.0.into());
                }
                AccountLockerBlueprintMethod::Store(..)
                | AccountLockerBlueprintMethod::Airdrop(..)
                | AccountLockerBlueprintMethod::Recover(..)
                | AccountLockerBlueprintMethod::RecoverNonFungibles(..)
                | AccountLockerBlueprintMethod::GetAmount(..)
                | AccountLockerBlueprintMethod::GetNonFungibleLocalIds(..) => {}
            },
            // Identities
            TypedNativeInvocation {
                receiver: ManifestInvocationReceiver::GlobalMethod(receiver),
                invocation:
                    TypedManifestNativeInvocation::IdentityBlueprintInvocation(
                        IdentityBlueprintInvocation::Method(
                            IdentityBlueprintMethod::Securify(..),
                        ),
                    ),
            } => {
                self.0.identities.insert(receiver.into());
            }
            // Role Assignment Module
            TypedNativeInvocation {
                receiver: ManifestInvocationReceiver::GlobalMethod(receiver),
                invocation:
                    TypedManifestNativeInvocation::RoleAssignmentBlueprintInvocation(
                        RoleAssignmentBlueprintInvocation::Method(method),
                    ),
            } => match method {
                RoleAssignmentBlueprintMethod::SetOwner(..)
                | RoleAssignmentBlueprintMethod::LockOwner(..)
                | RoleAssignmentBlueprintMethod::Set(..) => {
                    if receiver.is_account() {
                        self.0.accounts.insert(receiver.into());
                    } else if receiver.is_identity() {
                        self.0.identities.insert(receiver.into());
                    }
                }
                RoleAssignmentBlueprintMethod::Get(..)
                | RoleAssignmentBlueprintMethod::GetOwnerRole(..) => {}
            },
            // Metadata Module
            TypedNativeInvocation {
                receiver: ManifestInvocationReceiver::GlobalMethod(receiver),
                invocation:
                    TypedManifestNativeInvocation::MetadataBlueprintInvocation(
                        MetadataBlueprintInvocation::Method(method),
                    ),
            } => match method {
                MetadataBlueprintMethod::Set(..)
                | MetadataBlueprintMethod::Lock(..)
                | MetadataBlueprintMethod::Remove(..) => {
                    if receiver.is_account() {
                        self.0.accounts.insert(receiver.into());
                    } else if receiver.is_identity() {
                        self.0.identities.insert(receiver.into());
                    }
                }
                MetadataBlueprintMethod::Get(..) => {}
            },
            // Royalty Module
            TypedNativeInvocation {
                receiver: ManifestInvocationReceiver::GlobalMethod(receiver),
                invocation:
                    TypedManifestNativeInvocation::ComponentRoyaltyBlueprintInvocation(
                        ComponentRoyaltyBlueprintInvocation::Method(method),
                    ),
            } => match method {
                ComponentRoyaltyBlueprintMethod::SetRoyalty(_)
                | ComponentRoyaltyBlueprintMethod::LockRoyalty(_)
                | ComponentRoyaltyBlueprintMethod::ClaimRoyalties(_) => {
                    if receiver.is_account() {
                        self.0.accounts.insert(receiver.into());
                    } else if receiver.is_identity() {
                        self.0.identities.insert(receiver.into());
                    }
                }
            },
            _ => {}
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct EntitiesRequiringAuthOutput {
    pub accounts: IndexSet<ManifestGlobalAddress>,
    pub identities: IndexSet<ManifestGlobalAddress>,
}
