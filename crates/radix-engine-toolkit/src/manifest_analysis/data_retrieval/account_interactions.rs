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
pub struct AccountInteractionsVisitor(AccountInteractionsOutput);

impl AccountInteractionsVisitor {
    pub fn new() -> Self {
        Default::default()
    }
}

impl ManifestAnalysisVisitor for AccountInteractionsVisitor {
    type Output = AccountInteractionsOutput;
    type ValidityState = ConstManifestAnalysisVisitorValidityState<true>;

    fn output(self) -> Self::Output {
        self.0
    }

    fn validity_state(&self) -> &Self::ValidityState {
        &ConstManifestAnalysisVisitorValidityState::<true>
    }

    fn on_instruction(
        &mut self,
        _: &NamedAddressStore,
        grouped_instruction: &GroupedInstruction,
        _: &InstructionIndex,
        _: Option<&InvocationIo<InvocationIoItems>>,
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

        // We're only interested in account invocations and do not care about
        // any other kind of invocations.
        let Some(TypedManifestNativeInvocation::AccountBlueprintInvocation(
            AccountBlueprintInvocation::Method(account_method),
        )) = maybe_typed_invocation
        else {
            return;
        };

        let sets_to_add_to: &mut [_] = match account_method {
            AccountBlueprintMethod::Securify(..) => {
                &mut [&mut self.0.accounts_securified]
            }
            AccountBlueprintMethod::LockFee(..)
            | AccountBlueprintMethod::LockContingentFee(..) => {
                &mut [&mut self.0.accounts_locked_fees_from]
            }
            AccountBlueprintMethod::LockFeeAndWithdraw(..)
            | AccountBlueprintMethod::LockFeeAndWithdrawNonFungibles(..) => {
                &mut [
                    &mut self.0.accounts_locked_fees_from,
                    &mut self.0.accounts_withdrawn_from,
                ]
            }
            AccountBlueprintMethod::Withdraw(..)
            | AccountBlueprintMethod::WithdrawNonFungibles(..) => {
                &mut [&mut self.0.accounts_withdrawn_from]
            }
            AccountBlueprintMethod::Deposit(..)
            | AccountBlueprintMethod::DepositBatch(..)
            | AccountBlueprintMethod::TryDepositOrRefund(..)
            | AccountBlueprintMethod::TryDepositBatchOrRefund(..)
            | AccountBlueprintMethod::TryDepositOrAbort(..)
            | AccountBlueprintMethod::TryDepositBatchOrAbort(..) => {
                &mut [&mut self.0.accounts_deposited_into]
            }
            AccountBlueprintMethod::Burn(..)
            | AccountBlueprintMethod::BurnNonFungibles(..) => {
                &mut [&mut self.0.accounts_burned_from]
            }
            AccountBlueprintMethod::CreateProofOfAmount(..)
            | AccountBlueprintMethod::CreateProofOfNonFungibles(..) => {
                &mut [&mut self.0.accounts_created_proofs_from]
            }
            AccountBlueprintMethod::SetDefaultDepositRule(..) => {
                &mut [&mut self.0.accounts_set_default_deposit_rule_of]
            }
            AccountBlueprintMethod::SetResourcePreference(..) => {
                &mut [&mut self.0.accounts_set_resource_preference_into]
            }
            AccountBlueprintMethod::RemoveResourcePreference(..) => {
                &mut [&mut self.0.accounts_remove_resource_preference_from]
            }
            AccountBlueprintMethod::AddAuthorizedDepositor(..) => {
                &mut [&mut self.0.accounts_add_authorized_depositor_into]
            }
            AccountBlueprintMethod::RemoveAuthorizedDepositor(..) => {
                &mut [&mut self.0.accounts_remove_authorized_depositor_from]
            }
        };
        for set in sets_to_add_to {
            set.insert(address);
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct AccountInteractionsOutput {
    pub accounts_securified: IndexSet<ManifestGlobalAddress>,
    pub accounts_deposited_into: IndexSet<ManifestGlobalAddress>,
    pub accounts_withdrawn_from: IndexSet<ManifestGlobalAddress>,
    pub accounts_locked_fees_from: IndexSet<ManifestGlobalAddress>,
    pub accounts_created_proofs_from: IndexSet<ManifestGlobalAddress>,
    pub accounts_burned_from: IndexSet<ManifestGlobalAddress>,
    pub accounts_set_default_deposit_rule_of: IndexSet<ManifestGlobalAddress>,
    pub accounts_set_resource_preference_into: IndexSet<ManifestGlobalAddress>,
    pub accounts_remove_resource_preference_from:
        IndexSet<ManifestGlobalAddress>,
    pub accounts_add_authorized_depositor_into: IndexSet<ManifestGlobalAddress>,
    pub accounts_remove_authorized_depositor_from:
        IndexSet<ManifestGlobalAddress>,
}
