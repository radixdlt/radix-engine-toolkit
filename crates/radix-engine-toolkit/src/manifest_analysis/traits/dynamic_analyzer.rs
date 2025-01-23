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

    /// The type that the visitor uses to describe if it's requirements for
    /// instructions is permitted or not.
    type RequirementState: ManifestAnalyzerRequirementState + Sized;

    /// A function used to construct the manifest analysis static visitor as
    /// well as its permission state and requirement state and return them back
    /// to the caller. The function takes in the [`Initializer`] as an argument
    /// which is an associated type on this trait. Through this, we are able to
    /// pass "arbitrary" arguments to these constructors. This function takes in
    /// the same initializer that's used in the static analyzer.
    ///
    /// [`Initializer`]: ManifestStaticAnalyzer::Initializer
    fn new(
        initializer: Self::Initializer,
    ) -> (
        Self,
        <Self as ManifestStaticAnalyzer>::PermissionState,
        <Self as ManifestStaticAnalyzer>::RequirementState,
        <Self as ManifestDynamicAnalyzer>::RequirementState,
    );

    /// A method that consumes the visitor and returns the output of the static
    /// and dynamic analyzers.
    fn output(
        self,
    ) -> CombinedAnalysisOutput<
        <Self as ManifestStaticAnalyzer>::Output,
        <Self as ManifestDynamicAnalyzer>::Output,
    >;

    /// A method used to process instructions and extract information from them.
    fn process_instruction(&mut self, context: AnalysisContext<'_>);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct CombinedAnalysisOutput<A, B> {
    pub static_analyzer_output: A,
    pub dynamic_analyzer_output: B,
}
