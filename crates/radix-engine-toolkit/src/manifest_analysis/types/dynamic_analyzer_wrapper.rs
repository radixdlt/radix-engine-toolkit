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

/// A wrapper type that provides a blanket implementation of the dynamic
/// analyzer for types that implement the [`ManifestStaticAnalyzer`] trait.
///
/// Currently, not all the analyzers implement the [`ManifestDynamicAnalyzer`]
/// trait. The ones that only require static analysis do not implement it. But,
/// the primary traversal logic we have in the code base assumes that we have
/// analyzers that all implement [`ManifestDynamicAnalyzer`].
///
/// Therefore, we add this type as a wrapper to any [`ManifestStaticAnalyzer`]
/// and this type itself implements the [`ManifestStaticAnalyzer`] in a way
/// where all of the calls are passed to the underlying type. This type also
/// implements the [`ManifestDynamicAnalyzer`] trait in a way where the no
/// actions are done at all.
pub struct DynamicAnalyzerWrapper<A: ManifestStaticAnalyzer>(A);

impl<A: ManifestStaticAnalyzer> ManifestStaticAnalyzer
    for DynamicAnalyzerWrapper<A>
{
    type Initializer = A::Initializer;
    type Output = A::Output;
    type PermissionState = A::PermissionState;
    type RequirementState = A::RequirementState;

    fn new(
        initializer: Self::Initializer,
    ) -> (Self, Self::PermissionState, Self::RequirementState) {
        let (analyzer, permission_state, requirement_state) =
            A::new(initializer);
        (Self(analyzer), permission_state, requirement_state)
    }

    fn output(self) -> Self::Output {
        self.0.output()
    }

    fn process_instruction(&mut self, context: AnalysisContext<'_>) {
        self.0.process_instruction(context);
    }
}

impl<A: ManifestStaticAnalyzer> ManifestDynamicAnalyzer
    for DynamicAnalyzerWrapper<A>
{
    type Output = ();

    fn output(
        self,
    ) -> CombinedAnalysisOutput<
        <Self as ManifestStaticAnalyzer>::Output,
        <Self as ManifestDynamicAnalyzer>::Output,
    > {
        CombinedAnalysisOutput {
            static_analyzer_output: self.0.output(),
            dynamic_analyzer_output: (),
        }
    }

    fn process_instruction(&mut self, _: AnalysisContext<'_>) {}
}
