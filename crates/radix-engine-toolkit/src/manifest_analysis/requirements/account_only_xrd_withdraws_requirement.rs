use crate::internal_prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AccountOnlyXrdWithdrawsRequirement {
    is_withdraws_just_of_xrd: bool,
}

impl Default for AccountOnlyXrdWithdrawsRequirement {
    fn default() -> Self {
        Self {
            is_withdraws_just_of_xrd: true,
        }
    }
}

impl ManifestAnalyzerRequirementState for AccountOnlyXrdWithdrawsRequirement {
    fn requirement_state(&self) -> RequirementState {
        match self.is_withdraws_just_of_xrd {
            true => RequirementState::Fulfilled,
            false => RequirementState::PermanentlyUnfulfilled,
        }
    }

    fn process_instruction(&mut self, context: InstructionContext<'_>) {
        if let InstructionContext::InvocationInstruction {
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
            self.is_withdraws_just_of_xrd &=
                *resource_address == ManifestResourceAddress::Static(XRD);
        }
    }
}
