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
pub struct ReservedInstructionsVisitor(ReservedInstructionsOutput);

impl ReservedInstructionsVisitor {
    pub fn new() -> Self {
        Default::default()
    }
}

impl ManifestAnalysisVisitor for ReservedInstructionsVisitor {
    type Output = ReservedInstructionsOutput;
    type ValidityState = ConstManifestAnalysisVisitorValidityState<true>;

    fn output(self) -> Self::Output {
        self.0
    }

    fn validity_state(&self) -> &Self::ValidityState {
        &ConstManifestAnalysisVisitorValidityState::<true>
    }

    fn on_instruction(
        &mut self,
        named_address_store: &NamedAddressStore,
        grouped_instruction: &GroupedInstruction,
        _: &InstructionIndex,
        maybe_typed_invocation: Option<&TypedManifestNativeInvocation>,
        _: Option<&InvocationIo<InvocationIoItems>>,
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

        // Getting the grouped entity type for the address.
        let grouped_entity_type = match address {
            ManifestGlobalAddress::Named(named_address) => named_address_store
                .get(&named_address)
                .and_then(BlueprintId::entity_type),
            ManifestGlobalAddress::Static(static_address) => {
                static_address.as_node_id().entity_type()
            }
        }
        .map(GroupedEntityType::from);
        let is_account = grouped_entity_type.is_some_and(|entity_type| {
            entity_type.belongs_to_account_entities()
        });
        let is_identity = grouped_entity_type.is_some_and(|entity_type| {
            entity_type.belongs_to_identity_entities()
        });

        // Analyzing the invocation to determine if it's a reserved instruction.
        match maybe_typed_invocation {
            Some(TypedManifestNativeInvocation::AccountBlueprintInvocation(
                AccountBlueprintInvocation::Method(method),
            )) => match method {
                AccountBlueprintMethod::Securify(..) => {
                    self.0.account_securify_invocations.insert(address);
                }
                AccountBlueprintMethod::LockFee(..)
                | AccountBlueprintMethod::LockContingentFee(..)
                | AccountBlueprintMethod::LockFeeAndWithdraw(..)
                | AccountBlueprintMethod::LockFeeAndWithdrawNonFungibles(..) => {
                    self.0.account_lock_fee_invocations.insert(address);
                }
                AccountBlueprintMethod::Deposit(..)
                | AccountBlueprintMethod::DepositBatch(..)
                | AccountBlueprintMethod::Withdraw(..)
                | AccountBlueprintMethod::WithdrawNonFungibles(..)
                | AccountBlueprintMethod::Burn(..)
                | AccountBlueprintMethod::BurnNonFungibles(..)
                | AccountBlueprintMethod::CreateProofOfAmount(..)
                | AccountBlueprintMethod::CreateProofOfNonFungibles(..)
                | AccountBlueprintMethod::SetDefaultDepositRule(..)
                | AccountBlueprintMethod::SetResourcePreference(..)
                | AccountBlueprintMethod::RemoveResourcePreference(..)
                | AccountBlueprintMethod::TryDepositOrRefund(..)
                | AccountBlueprintMethod::TryDepositBatchOrRefund(..)
                | AccountBlueprintMethod::TryDepositOrAbort(..)
                | AccountBlueprintMethod::TryDepositBatchOrAbort(..)
                | AccountBlueprintMethod::AddAuthorizedDepositor(..)
                | AccountBlueprintMethod::RemoveAuthorizedDepositor(..) => {}
            },
            Some(TypedManifestNativeInvocation::IdentityBlueprintInvocation(
                IdentityBlueprintInvocation::Method(IdentityBlueprintMethod::Securify(
                    ..,
                )),
            )) => {
                self.0.identity_securify_invocations.insert(address);
            }
            Some(TypedManifestNativeInvocation::AccessControllerBlueprintInvocation(
                AccessControllerBlueprintInvocation::Method(..),
            )) => {
                self.0.access_controller_invocations.insert(address);
            }
            Some(TypedManifestNativeInvocation::MetadataBlueprintInvocation(
                MetadataBlueprintInvocation::Method(MetadataBlueprintMethod::Set(
                    MetadataSetInput { key, .. },
                )),
            )) if key == "owner_keys" => {
                if is_account {
                    self.0
                        .account_update_owner_keys_metadata_field_invocations
                        .insert(address);
                } else if is_identity {
                    self.0
                        .identity_update_owner_keys_metadata_field_invocations
                        .insert(address);
                }
            }
            Some(TypedManifestNativeInvocation::MetadataBlueprintInvocation(
                MetadataBlueprintInvocation::Method(MetadataBlueprintMethod::Lock(
                    MetadataLockInput { key, .. },
                )),
            )) if key == "owner_keys" => {
                if is_account {
                    self.0
                        .account_lock_owner_keys_metadata_field_invocations
                        .insert(address);
                } else if is_identity {
                    self.0
                        .identity_lock_owner_keys_metadata_field_invocations
                        .insert(address);
                }
            }
            _ => {}
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct ReservedInstructionsOutput {
    pub account_lock_fee_invocations: IndexSet<ManifestGlobalAddress>,
    pub account_securify_invocations: IndexSet<ManifestGlobalAddress>,
    pub account_lock_owner_keys_metadata_field_invocations:
        IndexSet<ManifestGlobalAddress>,
    pub account_update_owner_keys_metadata_field_invocations:
        IndexSet<ManifestGlobalAddress>,
    pub identity_securify_invocations: IndexSet<ManifestGlobalAddress>,
    pub identity_lock_owner_keys_metadata_field_invocations:
        IndexSet<ManifestGlobalAddress>,
    pub identity_update_owner_keys_metadata_field_invocations:
        IndexSet<ManifestGlobalAddress>,
    pub access_controller_invocations: IndexSet<ManifestGlobalAddress>,
}
