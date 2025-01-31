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

/// A trait that defines a dynamic analyzer visitor. This visitor requires its
/// implementations to also implement [`ManifestStaticAnalyzer`] making
/// it a super trait.
pub trait ManifestDynamicAnalyzer: ManifestStaticAnalyzer {
    /// The output type of the dynamic analyzer.
    type Output;

    /// A method that consumes the visitor and returns the output of the static
    /// and dynamic analyzers.
    fn output(
        self,
    ) -> CombinedAnalysisOutput<
        <Self as ManifestStaticAnalyzer>::Output,
        <Self as ManifestDynamicAnalyzer>::Output,
    >;

    /// A method used to process instructions and extract information from them.
    fn process_instruction(&mut self, context: InstructionContext<'_>);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct CombinedAnalysisOutput<A, B> {
    pub static_analyzer_output: A,
    pub dynamic_analyzer_output: B,
}

impl<A, B> CombinedAnalysisOutput<A, B> {
    pub fn into_static_analyzer_output(self) -> A {
        self.static_analyzer_output
    }

    pub fn into_dynamic_analyzer_output(self) -> B {
        self.dynamic_analyzer_output
    }
}
