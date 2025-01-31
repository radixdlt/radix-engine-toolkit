use crate::internal_prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AllOfRequirement<R>(R);

impl<R> AllOfRequirement<R> {
    pub fn new(value: R) -> Self {
        Self(value)
    }

    pub fn into_inner(self) -> R {
        self.0
    }
}

impl<R> From<R> for AllOfRequirement<R> {
    fn from(value: R) -> Self {
        Self(value)
    }
}

impl ManifestAnalyzerRequirementState for AllOfRequirement<()> {
    fn requirement_state(&self) -> RequirementState {
        RequirementState::PermanentlyUnfulfilled
    }

    fn process_instruction(&mut self, _: InstructionContext<'_>) {}
}

macro_rules! define_all_of_requirement_internal {
    (
        $($generic: ident),* $(,)?
    ) => {
        paste! {
            impl<$($generic: ManifestAnalyzerRequirementState),*> ManifestAnalyzerRequirementState
                for AllOfRequirement<( $($generic,)* )>
            {
                fn requirement_state(&self) -> RequirementState {
                    let (
                        $(ref [<$generic: snake>],)*
                    ) = self.0;

                    let requirements = [$([<$generic: snake>].requirement_state()),*];

                    if requirements.iter().any(|r| matches!(r, RequirementState::PermanentlyUnfulfilled)) {
                        RequirementState::PermanentlyUnfulfilled
                    } else if requirements.iter().any(|r| matches!(r, RequirementState::CurrentlyUnfulfilled)) {
                        RequirementState::CurrentlyUnfulfilled
                    } else if requirements.iter().all(|r| matches!(r, RequirementState::Fulfilled)) {
                        RequirementState::Fulfilled
                    } else {
                        unreachable!()
                    }
                }

                fn process_instruction(&mut self, context: InstructionContext<'_>) {
                    let (
                        $(ref mut [<$generic: snake>],)*
                    ) = self.0;
                    $(
                        [<$generic: snake>].process_instruction(context);
                    )*
                }
            }
        }
    };
}

macro_rules! define_all_of_requirement {
    (
        $generic_ident: ident
        $(,$($generic_idents: ident),* $(,)?)?
    ) => {
        define_all_of_requirement_internal!($generic_ident, $($($generic_idents),*)?);
        $(
            define_all_of_requirement!($($generic_idents),*);
        )?
    };
}

define_all_of_requirement![
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y,
    Z
];
