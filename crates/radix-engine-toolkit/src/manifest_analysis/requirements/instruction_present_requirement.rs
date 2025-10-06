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

type DefaultInstructionPresentRequirement =
    InstructionPresentRequirement<fn(InstructionContext<'_>) -> bool>;

struct InstructionPresentRequirement<F: FnMut(InstructionContext<'_>) -> bool> {
    is_instruction_seen: bool,
    is_instruction_seen_callback: F,
}

impl<F: FnMut(InstructionContext<'_>) -> bool>
    InstructionPresentRequirement<F>
{
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
                InstructionContext::InvocationInstruction {
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
                InstructionContext::InvocationInstruction {
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
                InstructionContext::InvocationInstruction {
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
                InstructionContext::InvocationInstruction {
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
                InstructionContext::InvocationInstruction {
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
                InstructionContext::InvocationInstruction {
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
                InstructionContext::InvocationInstruction {
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
                InstructionContext::InvocationInstruction {
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
                InstructionContext::InvocationInstruction {
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
                InstructionContext::InvocationInstruction {
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
                InstructionContext::InvocationInstruction {
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
                InstructionContext::InvocationInstruction {
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

    pub fn entity_securify() -> DefaultInstructionPresentRequirement {
        InstructionPresentRequirement::new(|context| {
            matches!(
                context,
                InstructionContext::InvocationInstruction {
                    typed_native_invocation: Some(TypedNativeInvocation {
                        invocation:
                        TypedManifestNativeInvocation::AccountBlueprintInvocation(
                            AccountBlueprintInvocation::Method(
                                AccountBlueprintMethod::Securify(..)
                            )
                        ) | TypedManifestNativeInvocation::IdentityBlueprintInvocation(
                            IdentityBlueprintInvocation::Method(
                                IdentityBlueprintMethod::Securify(..)
                            )
                        ),
                        ..
                    }),
                    ..
                }
            )
        })
    }

    pub fn create_access_controller() -> DefaultInstructionPresentRequirement {
        InstructionPresentRequirement::new(|context| {
            matches!(
                context,
                InstructionContext::InvocationInstruction {
                    typed_native_invocation: Some(TypedNativeInvocation {
                        invocation:
                        TypedManifestNativeInvocation::AccessControllerBlueprintInvocation(
                            AccessControllerBlueprintInvocation::Function(
                                AccessControllerBlueprintFunction::Create(..)
                            )
                        ),
                        ..
                    }),
                    ..
                }
            )
        })
    }

    pub fn access_controller_iniate_recovery_as_primary(
    ) -> DefaultInstructionPresentRequirement {
        InstructionPresentRequirement::new(|context| {
            matches!(
                context,
                InstructionContext::InvocationInstruction {
                    typed_native_invocation: Some(TypedNativeInvocation {
                        invocation:
                        TypedManifestNativeInvocation::AccessControllerBlueprintInvocation(
                            AccessControllerBlueprintInvocation::Method(
                                AccessControllerBlueprintMethod::InitiateRecoveryAsPrimary(..)
                            )
                        ),
                        ..
                    }),
                    ..
                }
            )
        })
    }

    pub fn access_controller_iniate_recovery_as_recovery(
    ) -> DefaultInstructionPresentRequirement {
        InstructionPresentRequirement::new(|context| {
            matches!(
                context,
                InstructionContext::InvocationInstruction {
                    typed_native_invocation: Some(TypedNativeInvocation {
                        invocation:
                        TypedManifestNativeInvocation::AccessControllerBlueprintInvocation(
                            AccessControllerBlueprintInvocation::Method(
                                AccessControllerBlueprintMethod::InitiateRecoveryAsRecovery(..)
                            )
                        ),
                        ..
                    }),
                    ..
                }
            )
        })
    }

    pub fn access_controller_stop_timed_recovery(
    ) -> DefaultInstructionPresentRequirement {
        InstructionPresentRequirement::new(|context| {
            matches!(
                context,
                InstructionContext::InvocationInstruction {
                    typed_native_invocation: Some(TypedNativeInvocation {
                        invocation:
                        TypedManifestNativeInvocation::AccessControllerBlueprintInvocation(
                            AccessControllerBlueprintInvocation::Method(
                                AccessControllerBlueprintMethod::StopTimedRecovery(..)
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

impl<F: FnMut(InstructionContext<'_>) -> bool> ManifestAnalyzerRequirementState
    for InstructionPresentRequirement<F>
{
    fn requirement_state(&self) -> RequirementState {
        match self.is_instruction_seen {
            true => RequirementState::Fulfilled,
            false => RequirementState::CurrentlyUnfulfilled,
        }
    }

    fn process_instruction(&mut self, context: InstructionContext<'_>) {
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

                fn process_instruction(&mut self, context: InstructionContext<'_>) {
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
    EntitySecurify => entity_securify,
    CreateAccessController => create_access_controller,
    AccessControllerInitiateRecoveryAsPrimary => access_controller_iniate_recovery_as_primary,
    AccessControllerInitiateRecoveryAsRecovery => access_controller_iniate_recovery_as_recovery,
    AccessControllerStopTimeRecovery => access_controller_stop_timed_recovery,
}
