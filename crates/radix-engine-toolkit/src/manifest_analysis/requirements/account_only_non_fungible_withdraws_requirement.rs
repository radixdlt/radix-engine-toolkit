use crate::internal_prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AccountOnlyNonFungibleWithdrawsRequirement {
    is_withdraws_just_of_non_fungible: bool,
}

impl Default for AccountOnlyNonFungibleWithdrawsRequirement {
    fn default() -> Self {
        Self {
            is_withdraws_just_of_non_fungible: true,
        }
    }
}

impl ManifestAnalyzerRequirementState
    for AccountOnlyNonFungibleWithdrawsRequirement
{
    fn requirement_state(&self) -> RequirementState {
        match self.is_withdraws_just_of_non_fungible {
            true => RequirementState::Fulfilled,
            false => RequirementState::PermanentlyUnfulfilled,
        }
    }

    fn process_instruction(&mut self, context: AnalysisContext<'_>) {
        if let AnalysisContext::InvocationInstruction {
            typed_native_invocation:
                Some(TypedNativeInvocation {
                    invocation:
                        TypedManifestNativeInvocation::AccountBlueprintInvocation(
                            AccountBlueprintInvocation::Method(
                                AccountBlueprintMethod::Withdraw(
                                    AccountWithdrawManifestInput {
                                        resource_address, ..
                                    },
                                )
                                | AccountBlueprintMethod::WithdrawNonFungibles(
                                    AccountWithdrawNonFungiblesManifestInput {
                                        resource_address,
                                        ..
                                    },
                                )
                                | AccountBlueprintMethod::LockFeeAndWithdraw(
                                    AccountLockFeeAndWithdrawManifestInput {
                                        resource_address,
                                        ..
                                    },
                                )
                                | AccountBlueprintMethod::LockFeeAndWithdrawNonFungibles(
                                    AccountLockFeeAndWithdrawNonFungiblesManifestInput {
                                        resource_address,
                                        ..
                                    },
                                ),
                            ),
                        ),
                    ..
                }),
            ..
        } = context
        {
            let is_non_fungible = match resource_address {
                ManifestResourceAddress::Static(resource_address) => {
                    !resource_address.is_fungible()
                }
                ManifestResourceAddress::Named(named_address) => context
                    .named_address_store()
                    .get(named_address)
                    .and_then(BlueprintId::entity_type)
                    .is_some_and(|e| {
                        matches!(e, EntityType::GlobalNonFungibleResourceManager)
                    }),
            };
            self.is_withdraws_just_of_non_fungible &= is_non_fungible;
        }
    }
}
