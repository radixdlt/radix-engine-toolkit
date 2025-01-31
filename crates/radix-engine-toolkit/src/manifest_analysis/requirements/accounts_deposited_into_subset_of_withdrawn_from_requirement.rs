use crate::internal_prelude::*;

#[derive(Clone, Debug, Default)]
pub struct AccountsDepositedIntoSubsetOfWithdrawnFromRequirement {
    accounts_withdrawn_from: IndexSet<ResolvedManifestAddress<GlobalAddress>>,
    accounts_deposited_into: IndexSet<ResolvedManifestAddress<GlobalAddress>>,
}

impl ManifestAnalyzerRequirementState
    for AccountsDepositedIntoSubsetOfWithdrawnFromRequirement
{
    fn requirement_state(&self) -> RequirementState {
        let is_subset = self
            .accounts_deposited_into
            .is_subset(&self.accounts_withdrawn_from);
        match is_subset {
            true => RequirementState::Fulfilled,
            false => RequirementState::CurrentlyUnfulfilled,
        }
    }

    fn process_instruction(&mut self, context: InstructionContext<'_>) {
        let InstructionContext::InvocationInstruction {
            typed_native_invocation:
                Some(TypedNativeInvocation {
                    receiver: ManifestInvocationReceiver::GlobalMethod(account),
                    invocation:
                        TypedManifestNativeInvocation::AccountBlueprintInvocation(
                            AccountBlueprintInvocation::Method(method),
                        ),
                }),
            ..
        } = context
        else {
            return;
        };

        match method {
            AccountBlueprintMethod::Withdraw(..)
            | AccountBlueprintMethod::WithdrawNonFungibles(..)
            | AccountBlueprintMethod::LockFeeAndWithdraw(..)
            | AccountBlueprintMethod::LockFeeAndWithdrawNonFungibles(..) => {
                self.accounts_withdrawn_from.insert(account.clone());
            }
            AccountBlueprintMethod::Deposit(..)
            | AccountBlueprintMethod::DepositBatch(..)
            | AccountBlueprintMethod::SetDefaultDepositRule(..)
            | AccountBlueprintMethod::TryDepositOrAbort(..)
            | AccountBlueprintMethod::TryDepositBatchOrAbort(..) => {
                self.accounts_deposited_into.insert(account.clone());
            }
            AccountBlueprintMethod::TryDepositOrRefund(..)
            | AccountBlueprintMethod::TryDepositBatchOrRefund(..)
            | AccountBlueprintMethod::Securify(..)
            | AccountBlueprintMethod::LockFee(..)
            | AccountBlueprintMethod::LockContingentFee(..)
            | AccountBlueprintMethod::Burn(..)
            | AccountBlueprintMethod::BurnNonFungibles(..)
            | AccountBlueprintMethod::CreateProofOfAmount(..)
            | AccountBlueprintMethod::CreateProofOfNonFungibles(..)
            | AccountBlueprintMethod::SetResourcePreference(..)
            | AccountBlueprintMethod::RemoveResourcePreference(..)
            | AccountBlueprintMethod::AddAuthorizedDepositor(..)
            | AccountBlueprintMethod::RemoveAuthorizedDepositor(..)
            | AccountBlueprintMethod::Balance(..)
            | AccountBlueprintMethod::NonFungibleLocalIds(..)
            | AccountBlueprintMethod::HasNonFungible(..) => {}
        }
    }
}
