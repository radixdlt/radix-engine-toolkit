use crate::internal_prelude::*;

type DefaultInstructionPresentRequirement =
    InstructionPresentRequirement<fn(AnalysisContext<'_>) -> bool>;

struct InstructionPresentRequirement<F: FnMut(AnalysisContext<'_>) -> bool> {
    is_instruction_seen: bool,
    is_instruction_seen_callback: F,
}

impl<F: FnMut(AnalysisContext<'_>) -> bool> InstructionPresentRequirement<F> {
    pub fn new(callback: F) -> Self {
        Self {
            is_instruction_seen: false,
            is_instruction_seen_callback: callback,
        }
    }

    pub fn account_withdraw() -> DefaultInstructionPresentRequirement {
        InstructionPresentRequirement::new(|context| {
            matches!(
                context,
                AnalysisContext::InvocationInstruction {
                    typed_native_invocation: Some(TypedNativeInvocation {
                        invocation: TypedManifestNativeInvocation::AccountBlueprintInvocation(
                            AccountBlueprintInvocation::Method(
                                AccountBlueprintMethod::Withdraw(..)
                                    | AccountBlueprintMethod::WithdrawNonFungibles(..)
                                    | AccountBlueprintMethod::LockFeeAndWithdraw(..)
                                    | AccountBlueprintMethod::LockFeeAndWithdrawNonFungibles(..)
                            )
                        ),
                        ..
                    }),
                    ..
                }
            )
        })
    }

    pub fn account_deposit() -> DefaultInstructionPresentRequirement {
        InstructionPresentRequirement::new(|context| {
            matches!(
                context,
                AnalysisContext::InvocationInstruction {
                    typed_native_invocation: Some(TypedNativeInvocation {
                        invocation:
                            TypedManifestNativeInvocation::AccountBlueprintInvocation(
                                AccountBlueprintInvocation::Method(
                                    AccountBlueprintMethod::Deposit(..)
                                        | AccountBlueprintMethod::DepositBatch(..)
                                        | AccountBlueprintMethod::TryDepositOrAbort(..)
                                        | AccountBlueprintMethod::TryDepositBatchOrAbort(
                                            ..
                                        )
                                )
                            ),
                        ..
                    }),
                    ..
                }
            )
        })
    }

    pub fn account_set_default_deposit_rule(
    ) -> DefaultInstructionPresentRequirement {
        InstructionPresentRequirement::new(|context| {
            matches!(
                context,
                AnalysisContext::InvocationInstruction {
                    typed_native_invocation: Some(TypedNativeInvocation {
                        invocation:
                            TypedManifestNativeInvocation::AccountBlueprintInvocation(
                                AccountBlueprintInvocation::Method(
                                    AccountBlueprintMethod::SetDefaultDepositRule(..)
                                )
                            ),
                        ..
                    }),
                    ..
                }
            )
        })
    }

    pub fn account_set_resource_preference(
    ) -> DefaultInstructionPresentRequirement {
        InstructionPresentRequirement::new(|context| {
            matches!(
                context,
                AnalysisContext::InvocationInstruction {
                    typed_native_invocation: Some(TypedNativeInvocation {
                        invocation:
                            TypedManifestNativeInvocation::AccountBlueprintInvocation(
                                AccountBlueprintInvocation::Method(
                                    AccountBlueprintMethod::SetResourcePreference(..)
                                )
                            ),
                        ..
                    }),
                    ..
                }
            )
        })
    }

    pub fn account_remove_resource_preference(
    ) -> DefaultInstructionPresentRequirement {
        InstructionPresentRequirement::new(|context| {
            matches!(
                context,
                AnalysisContext::InvocationInstruction {
                    typed_native_invocation: Some(TypedNativeInvocation {
                        invocation:
                            TypedManifestNativeInvocation::AccountBlueprintInvocation(
                                AccountBlueprintInvocation::Method(
                                    AccountBlueprintMethod::RemoveResourcePreference(..)
                                )
                            ),
                        ..
                    }),
                    ..
                }
            )
        })
    }

    pub fn account_add_authorized_depositor(
    ) -> DefaultInstructionPresentRequirement {
        InstructionPresentRequirement::new(|context| {
            matches!(
                context,
                AnalysisContext::InvocationInstruction {
                    typed_native_invocation: Some(TypedNativeInvocation {
                        invocation:
                            TypedManifestNativeInvocation::AccountBlueprintInvocation(
                                AccountBlueprintInvocation::Method(
                                    AccountBlueprintMethod::AddAuthorizedDepositor(..)
                                )
                            ),
                        ..
                    }),
                    ..
                }
            )
        })
    }

    pub fn account_remove_authorized_depositor(
    ) -> DefaultInstructionPresentRequirement {
        InstructionPresentRequirement::new(|context| {
            matches!(
                context,
                AnalysisContext::InvocationInstruction {
                    typed_native_invocation: Some(TypedNativeInvocation {
                        invocation:
                            TypedManifestNativeInvocation::AccountBlueprintInvocation(
                                AccountBlueprintInvocation::Method(
                                    AccountBlueprintMethod::RemoveAuthorizedDepositor(..)
                                )
                            ),
                        ..
                    }),
                    ..
                }
            )
        })
    }

    pub fn validator_stake() -> DefaultInstructionPresentRequirement {
        InstructionPresentRequirement::new(|context| {
            matches!(
                context,
                AnalysisContext::InvocationInstruction {
                    typed_native_invocation: Some(TypedNativeInvocation {
                        invocation:
                            TypedManifestNativeInvocation::ValidatorBlueprintInvocation(
                                ValidatorBlueprintInvocation::Method(
                                    ValidatorBlueprintMethod::Stake(..)
                                )
                            ),
                        ..
                    }),
                    ..
                }
            )
        })
    }

    pub fn validator_unstake() -> DefaultInstructionPresentRequirement {
        InstructionPresentRequirement::new(|context| {
            matches!(
                context,
                AnalysisContext::InvocationInstruction {
                    typed_native_invocation: Some(TypedNativeInvocation {
                        invocation:
                            TypedManifestNativeInvocation::ValidatorBlueprintInvocation(
                                ValidatorBlueprintInvocation::Method(
                                    ValidatorBlueprintMethod::Unstake(..)
                                )
                            ),
                        ..
                    }),
                    ..
                }
            )
        })
    }

    pub fn validator_claim_xrd() -> DefaultInstructionPresentRequirement {
        InstructionPresentRequirement::new(|context| {
            matches!(
                context,
                AnalysisContext::InvocationInstruction {
                    typed_native_invocation: Some(TypedNativeInvocation {
                        invocation:
                            TypedManifestNativeInvocation::ValidatorBlueprintInvocation(
                                ValidatorBlueprintInvocation::Method(
                                    ValidatorBlueprintMethod::ClaimXrd(..)
                                )
                            ),
                        ..
                    }),
                    ..
                }
            )
        })
    }

    pub fn pool_contribute() -> DefaultInstructionPresentRequirement {
        InstructionPresentRequirement::new(|context| {
            matches!(
                context,
                AnalysisContext::InvocationInstruction {
                    typed_native_invocation: Some(TypedNativeInvocation {
                        invocation:
                            TypedManifestNativeInvocation::OneResourcePoolBlueprintInvocation(
                                OneResourcePoolBlueprintInvocation::Method(
                                    OneResourcePoolBlueprintMethod::Contribute(..)
                                )
                            ) | TypedManifestNativeInvocation::TwoResourcePoolBlueprintInvocation(
                                TwoResourcePoolBlueprintInvocation::Method(
                                    TwoResourcePoolBlueprintMethod::Contribute(..)
                                )
                            ) | TypedManifestNativeInvocation::MultiResourcePoolBlueprintInvocation(
                                MultiResourcePoolBlueprintInvocation::Method(
                                    MultiResourcePoolBlueprintMethod::Contribute(..)
                                )
                            ),
                        ..
                    }),
                    ..
                }
            )
        })
    }

    pub fn pool_redeem() -> DefaultInstructionPresentRequirement {
        InstructionPresentRequirement::new(|context| {
            matches!(
                context,
                AnalysisContext::InvocationInstruction {
                    typed_native_invocation: Some(TypedNativeInvocation {
                        invocation:
                            TypedManifestNativeInvocation::OneResourcePoolBlueprintInvocation(
                                OneResourcePoolBlueprintInvocation::Method(
                                    OneResourcePoolBlueprintMethod::Redeem(..)
                                )
                            ) | TypedManifestNativeInvocation::TwoResourcePoolBlueprintInvocation(
                                TwoResourcePoolBlueprintInvocation::Method(
                                    TwoResourcePoolBlueprintMethod::Redeem(..)
                                )
                            ) | TypedManifestNativeInvocation::MultiResourcePoolBlueprintInvocation(
                                MultiResourcePoolBlueprintInvocation::Method(
                                    MultiResourcePoolBlueprintMethod::Redeem(..)
                                )
                            ),
                        ..
                    }),
                    ..
                }
            )
        })
    }
}

impl<F: FnMut(AnalysisContext<'_>) -> bool> ManifestAnalyzerRequirementState
    for InstructionPresentRequirement<F>
{
    fn requirement_state(&self) -> RequirementState {
        match self.is_instruction_seen {
            true => RequirementState::Fulfilled,
            false => RequirementState::CurrentlyUnfulfilled,
        }
    }

    fn process_instruction(&mut self, context: AnalysisContext<'_>) {
        self.is_instruction_seen |= (self.is_instruction_seen_callback)(context)
    }
}

// This macro is here since I would like for each of the requirements to have
// their own separate types so that reviewing the requirements used in one of
// the visitors is relatively easy compared to just seeing an "instruction is
// required" requirement and needing to dive into the implementation. So, this
// macro generates these wrappers for us.
macro_rules! define_instruction_present_type {
    (
        $(
            $ty_ident: ident => $func: ident
        ),* $(,)?
    ) => {
        $(
            pub struct $ty_ident (
                DefaultInstructionPresentRequirement
            );

            impl Default for $ty_ident {
                fn default() -> Self {
                    Self(DefaultInstructionPresentRequirement::$func())
                }
            }

            impl ManifestAnalyzerRequirementState for $ty_ident {
                fn requirement_state(&self) -> RequirementState {
                    self.0.requirement_state()
                }

                fn process_instruction(&mut self, context: AnalysisContext<'_>) {
                    self.0.process_instruction(context)
                }
            }
        )*
    };
}
define_instruction_present_type! {
    /* Account */
    AccountWithdrawInstructionPresentRequirement => account_withdraw,
    AccountDepositInstructionPresentRequirement => account_deposit,
    AccountSetDefaultDepositRuleInstructionPresentRequirement
        => account_set_default_deposit_rule,
    AccountSetResourcePreferenceInstructionPresentRequirement
        => account_set_resource_preference,
    AccountRemoveResourcePreferenceInstructionPresentRequirement
        => account_remove_resource_preference,
    AccountAddAuthorizedDepositorInstructionPresentRequirement
        => account_add_authorized_depositor,
    AccountRemoveAuthorizedDepositorInstructionPresentRequirement
        => account_remove_authorized_depositor,
    /* Validator */
    ValidatorStakeInstructionPresentRequirement => validator_stake,
    ValidatorUnstakeInstructionPresentRequirement => validator_unstake,
    ValidatorClaimXrdInstructionPresentRequirement => validator_claim_xrd,
    /* Pools */
    PoolContributeInstructionPresentRequirement => pool_contribute,
    PoolRedeemInstructionPresentRequirement => pool_redeem,
}
