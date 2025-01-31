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

                    [$([<$generic: snake>].requirement_state()),*]
                        .into_iter()
                        .reduce(RequirementState::and)
                        .unwrap_or(RequirementState::CurrentlyUnfulfilled)
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
