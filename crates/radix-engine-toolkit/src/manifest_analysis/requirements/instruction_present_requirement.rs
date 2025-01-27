use crate::internal_prelude::*;

pub type DefaultInstructionPresentRequirement =
    InstructionPresentRequirement<fn(AnalysisContext<'_>) -> bool>;

pub struct InstructionPresentRequirement<F: FnMut(AnalysisContext<'_>) -> bool>
{
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

    pub fn account_withdraw(
    ) -> InstructionPresentRequirement<fn(AnalysisContext<'_>) -> bool> {
        InstructionPresentRequirement::new(|context| {
            matches!(
                context,
                AnalysisContext::InvocationInstruction {
                    typed_native_invocation: Some(TypedNativeInvocation {
                        invocation:
                            TypedManifestNativeInvocation::AccountBlueprintInvocation(
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
