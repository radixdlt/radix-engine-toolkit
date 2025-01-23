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

/// A trait that's implemented to mark some type as being the requirement state
/// of a visitor. This trait provides a single method which computes the current
/// requirement state of the visitor.
pub trait ManifestAnalyzerRequirementState {
    /// A method that computes the visitor's requirement state.
    fn requirement_state(&self) -> RequirementState;
}

/// An enum that captures the various states that the visitor's requirements
/// could be in.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RequirementState {
    /// In this state the requirements of the visitor are fulfilled and if the
    /// manifest traverser were to stop at this instruction then the visitor's
    /// requirements would be fulfilled and an output would be produced assuming
    /// that the permission state also resolves to [`true`].
    Fulfilled,
    /// In this state the requirements of the visitor are unfulfilled but not
    /// violated and could certainly be fulfilled if more instructions are seen
    /// by the visitor. If the manifest were to end with a visitor in this state
    /// then the visitor would not produce output since it's requirements were't
    /// fulfilled.
    CurrentlyUnfulfilled,
    /// In this case the requirements of the visitor are unfulfilled and the
    /// visitor is certain that they will never be fulfilled in the future. In
    /// this case, the traverser may stop going through instructions and just
    /// report that there's no output for the visitor.
    PermanentlyUnfulfilled,
}

impl RequirementState {
    pub fn is_fulfilled(&self) -> bool {
        matches!(self, Self::Fulfilled)
    }
}
