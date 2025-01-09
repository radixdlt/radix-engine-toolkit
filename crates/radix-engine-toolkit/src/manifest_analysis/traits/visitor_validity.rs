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

/// A trait used to define the state of the visitor validity.
///
/// All [`ManifestAnalysisVisitor`]s define a type that they use to capture
/// whether they're currently accepting additional instructions or not. This
/// information is then used by the traverser to determine if it should or
/// should not dispatch future instructions to the visitor.
///
/// Therefore, this is used to short circuit any additional computation from
/// happening in the visitors if they've determined that they're no longer valid
/// and no longer wish to accept instructions.
///
/// [`ManifestAnalysisVisitor`]: super::ManifestAnalysisVisitor
pub trait VisitorValidityState {
    fn is_visitor_accepting_instructions(&self) -> bool;
}

/// A type that allows for the use of a constant [`bool`] for the validity
/// status of a visitor.
pub struct ConstVisitorValidityState<const ACCEPTING_INSTRUCTIONS: bool>;
impl<const ACCEPTING_INSTRUCTIONS: bool> VisitorValidityState
    for ConstVisitorValidityState<ACCEPTING_INSTRUCTIONS>
{
    fn is_visitor_accepting_instructions(&self) -> bool {
        ACCEPTING_INSTRUCTIONS
    }
}
