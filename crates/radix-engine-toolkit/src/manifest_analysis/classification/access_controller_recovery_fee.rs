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

use super::common::*;
use crate::internal_prelude::*;

/// Analyzes manifests that move XRD into or out of the recovery fee vaults of
/// access controllers.
///
/// The manifest goes one way only. It either tops up: withdraws XRD from
/// accounts and contributes it to access controllers, depositing nothing; or
/// it withdraws: withdraws XRD from access controllers and deposits it into
/// accounts, withdrawing nothing from accounts. Since the worktop must end
/// empty, the XRD contributed is the XRD withdrawn from accounts, and the XRD
/// deposited is the XRD withdrawn from access controllers. Besides that, the
/// manifest may only pay the fee: it makes no other access controller calls.
/// A subintent may not even pay the fee, see `SubintentRestrictions`.
///
/// The output is only known from dynamic analysis: it names the amounts the
/// preview moved.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct AccessControllerRecoveryFeeAnalyzer {
    contributions: IndexMap<ComponentAddress, Decimal>,
    withdrawals: IndexMap<ComponentAddress, Decimal>,
    account_withdrawals: IndexMap<ComponentAddress, Decimal>,
    account_deposits: IndexMap<ComponentAddress, Decimal>,
}

/// The XRD a manifest moves into or out of recovery fee vaults. Only one side
/// is filled: `contributions` and `account_withdrawals` when it tops up,
/// `withdrawals` and `account_deposits` when it withdraws.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct AccessControllerRecoveryFeeOutput {
    /// The XRD contributed to each access controller's recovery fee vault.
    pub contributions: Vec<AccessControllerRecoveryFeeTransfer>,
    /// The XRD withdrawn from each access controller's recovery fee vault.
    pub withdrawals: Vec<AccessControllerRecoveryFeeTransfer>,
    /// The XRD withdrawn from each account, to contribute.
    pub account_withdrawals: Vec<AccountRecoveryFeeTransfer>,
    /// The XRD deposited into each account, from what was withdrawn.
    pub account_deposits: Vec<AccountRecoveryFeeTransfer>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AccessControllerRecoveryFeeTransfer {
    pub access_controller: ComponentAddress,
    pub xrd_amount: Decimal,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AccountRecoveryFeeTransfer {
    pub account: ComponentAddress,
    pub xrd_amount: Decimal,
}

impl ManifestStaticAnalyzer for AccessControllerRecoveryFeeAnalyzer {
    type Initializer = ();
    type Output = ();
    type PermissionState =
        CallbackPermissionState<PermissionStateStaticCallback>;
    type RequirementState = AccessControllerRecoveryFeeRequirementState;

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
        // The amounts are only known from dynamic analysis.
    }
}

impl ManifestDynamicAnalyzer for AccessControllerRecoveryFeeAnalyzer {
    type Output = AccessControllerRecoveryFeeOutput;

    fn output(
        self,
    ) -> CombinedAnalysisOutput<
        <Self as ManifestStaticAnalyzer>::Output,
        <Self as ManifestDynamicAnalyzer>::Output,
    > {
        let access_controller_transfers = |amounts: IndexMap<_, _>| {
            amounts
                .into_iter()
                .map(|(access_controller, xrd_amount)| {
                    AccessControllerRecoveryFeeTransfer {
                        access_controller,
                        xrd_amount,
                    }
                })
                .collect()
        };
        let account_transfers = |amounts: IndexMap<_, _>| {
            amounts
                .into_iter()
                .map(|(account, xrd_amount)| AccountRecoveryFeeTransfer {
                    account,
                    xrd_amount,
                })
                .collect()
        };

        CombinedAnalysisOutput {
            static_analyzer_output: (),
            dynamic_analyzer_output: AccessControllerRecoveryFeeOutput {
                contributions: access_controller_transfers(self.contributions),
                withdrawals: access_controller_transfers(self.withdrawals),
                account_withdrawals: account_transfers(
                    self.account_withdrawals,
                ),
                account_deposits: account_transfers(self.account_deposits),
            },
        }
    }

    fn process_instruction(&mut self, context: InstructionContext<'_>) {
        let InstructionContext::InvocationInstruction {
            typed_native_invocation: Some(typed_native_invocation),
            dynamic_analysis_invocation_io: Some(invocation_io),
            ..
        } = context
        else {
            return;
        };
        let Some((movement, address)) = xrd_movement(typed_native_invocation)
        else {
            return;
        };
        // The requirements reject movements of anything else than XRD, and
        // into or out of components only the manifest knows.
        let Some(address) = static_component(address) else {
            return;
        };

        let (amounts, xrd) = match movement {
            XrdMovement::Contribution => {
                (&mut self.contributions, &invocation_io.input)
            }
            XrdMovement::Withdrawal => {
                (&mut self.withdrawals, &invocation_io.output)
            }
            XrdMovement::AccountWithdrawal => {
                (&mut self.account_withdrawals, &invocation_io.output)
            }
            XrdMovement::AccountDeposit => {
                (&mut self.account_deposits, &invocation_io.input)
            }
        };
        *amounts.entry(address).or_default() += xrd.resource_amount(&XRD);
    }
}

/// Fulfilled when the manifest moves XRD into or out of recovery fee vaults,
/// one way only.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct AccessControllerRecoveryFeeRequirementState {
    contributes: bool,
    withdraws: bool,
    withdraws_from_accounts: bool,
    deposits_into_accounts: bool,
    /// Whether the manifest moves something else than XRD, or moves it into
    /// or out of a component only the manifest knows.
    invalid: bool,
    subintent_restrictions: SubintentRestrictions,
}

impl ManifestAnalyzerRequirementState
    for AccessControllerRecoveryFeeRequirementState
{
    fn requirement_state(&self) -> RequirementState {
        let tops_up_and_withdraws =
            self.contributes && (self.withdraws || self.deposits_into_accounts);
        let withdraws_and_tops_up =
            self.withdraws && self.withdraws_from_accounts;

        if self.invalid
            || self.subintent_restrictions.are_violated()
            || tops_up_and_withdraws
            || withdraws_and_tops_up
        {
            RequirementState::PermanentlyUnfulfilled
        } else if self.contributes || self.withdraws {
            RequirementState::Fulfilled
        } else {
            RequirementState::CurrentlyUnfulfilled
        }
    }

    fn process_instruction(&mut self, context: InstructionContext<'_>) {
        self.subintent_restrictions
            .process_instruction(context, is_fee_payment(context));

        let InstructionContext::InvocationInstruction {
            typed_native_invocation: Some(typed_native_invocation),
            static_analysis_invocation_io,
            ..
        } = context
        else {
            return;
        };
        let Some((movement, address)) = xrd_movement(typed_native_invocation)
        else {
            return;
        };

        let moved_resources = match movement {
            XrdMovement::Contribution => {
                self.contributes = true;
                Some(&static_analysis_invocation_io.input)
            }
            XrdMovement::Withdrawal => {
                self.withdraws = true;
                None
            }
            // The permissions only let accounts withdraw XRD.
            XrdMovement::AccountWithdrawal => {
                self.withdraws_from_accounts = true;
                None
            }
            XrdMovement::AccountDeposit => {
                self.deposits_into_accounts = true;
                Some(&static_analysis_invocation_io.input)
            }
        };

        self.invalid |= static_component(address).is_none()
            || moved_resources.is_some_and(|resources| !is_only_xrd(resources));
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum XrdMovement {
    /// Into an access controller's recovery fee vault.
    Contribution,
    /// Out of an access controller's recovery fee vault.
    Withdrawal,
    AccountWithdrawal,
    AccountDeposit,
}

/// If the invocation moves resources the class permits moving, how it moves
/// them and the access controller or account they move into or out of. Only
/// XRD withdrawals from accounts are returned: withdrawing anything else isn't
/// permitted.
fn xrd_movement(
    invocation: &TypedNativeInvocation,
) -> Option<(XrdMovement, &ResolvedManifestAddress<GlobalAddress>)> {
    let ManifestInvocationReceiver::GlobalMethod(receiver) =
        &invocation.receiver
    else {
        return None;
    };

    let movement = match &invocation.invocation {
        TypedManifestNativeInvocation::AccessControllerBlueprintInvocation(
            AccessControllerBlueprintInvocation::Method(method),
        ) => match method {
            AccessControllerBlueprintMethod::ContributeRecoveryFee(..) => {
                XrdMovement::Contribution
            }
            AccessControllerBlueprintMethod::WithdrawRecoveryFee(..) => {
                XrdMovement::Withdrawal
            }
            _ => return None,
        },
        TypedManifestNativeInvocation::AccountBlueprintInvocation(
            AccountBlueprintInvocation::Method(method),
        ) => match method {
            AccountBlueprintMethod::Withdraw(
                AccountWithdrawManifestInput {
                    resource_address: ManifestResourceAddress::Static(XRD),
                    ..
                },
            ) => XrdMovement::AccountWithdrawal,
            AccountBlueprintMethod::Deposit(..)
            | AccountBlueprintMethod::DepositBatch(..)
            | AccountBlueprintMethod::TryDepositOrAbort(..)
            | AccountBlueprintMethod::TryDepositBatchOrAbort(..) => {
                XrdMovement::AccountDeposit
            }
            _ => return None,
        },
        _ => return None,
    };

    Some((movement, receiver))
}

/// Moving XRD into or out of recovery fee vaults and accounts, taking it from
/// the worktop, and paying the fee.
fn is_instruction_permitted(context: InstructionContext<'_>) -> bool {
    let is_xrd_movement = matches!(
        context,
        InstructionContext::InvocationInstruction {
            typed_native_invocation: Some(invocation),
            ..
        } if xrd_movement(invocation).is_some()
    );

    is_xrd_movement
        || is_fee_payment(context)
        || is_permitted_subintent_instruction(context)
        || matches!(
            context.instruction(),
            GroupedInstruction::TakeFromWorktopInstructions(..)
                | GroupedInstruction::ReturnToWorktopInstructions(..)
                | GroupedInstruction::AssertionInstructions(..)
        )
}
