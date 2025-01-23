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

/// A visitor used in manifest analysis that's called by a static analyzer and
/// is provided only with static data.
///
/// All static analysis visitors do a similar set of actions to arrive at a
/// classification or even a detailed classification. These actions are:
///
/// 1. Verifying that an instruction is valid within some instruction rule set.
///    As an example, the validator stake classifier permits calls to the
///    the stake method of validators but doesn't permit calls to pools.
/// 2. Verifying that the requirements of the classifier are set. As an example
///    the validator stake classifier requires that the `stake` method is called
///    in order for the manifest to be a valid validator stake manifest.
/// 3. Extracting data from the invocation - finally, after processing the
///    instruction permission and requirement, the classifier may want to
///    extract information from the instruction, invocation, or other info
///    provided to the visitor.
///
/// This trait enforces the above through the methods that it has. The traverser
/// enforces the order in which things are called.
pub trait ManifestStaticAnalyzer: Sized {
    /// The initializer type to provide to the constructor when instantiating
    /// a new instance of the visitor.
    type Initializer;

    /// All manifest analysis visitors have some form of an output type, this is
    /// most commonly the result of their analysis. There are no trait bounds on
    /// the output type as it can realistically be just about anything.
    type Output;

    /// The type that the visitor uses to describe if the instructions that it
    /// has encountered have so far been permitted or not.
    type PermissionState: ManifestAnalyzerPermissionState + Sized;

    /// The type that the visitor uses to describe if it's requirements for
    /// instructions is permitted or not.
    type RequirementState: ManifestAnalyzerRequirementState + Sized;

    /// A function used to construct the manifest analysis static visitor as
    /// well as its permission state and requirement state and return them back
    /// to the caller. The function takes in the [`Initializer`] as an argument
    /// which is an associated type on this trait. Through this, we are able to
    /// pass "arbitrary" arguments to these constructors.
    ///
    /// [`Initializer`]: ManifestStaticAnalyzer::Initializer
    fn new(
        initializer: Self::Initializer,
    ) -> (Self, Self::PermissionState, Self::RequirementState);

    /// A method that consumes the visitor and returns the output.
    fn output(self) -> Self::Output;

    /// A method that is used to process the [`PermissionState`] for some
    /// instruction.
    ///
    /// We do not provide a default implementation of this method to require all
    /// visitors to provide one. A default implementation to a method like this
    /// could lead to security issues and therefore we require that visitors
    /// always implement this method even if it does nothing.
    ///
    /// [`PermissionState`]: ManifestStaticAnalyzer::PermissionState
    fn process_permission(
        &mut self,
        permission_state: &mut Self::PermissionState,
        named_address_store: &NamedAddressStore,
        instruction: &GroupedInstruction,
        typed_native_invocation: Option<&TypedNativeInvocation>,
    );

    /// A method that is used to process the [`RequirementState`] for some
    /// instruction.
    ///
    /// We do not provide a default implementation of this method to require all
    /// visitors to provide one. A default implementation to a method like this
    /// could lead to security issues and therefore we require that visitors
    /// always implement this method even if it does nothing.
    ///
    /// [`RequirementState`]: ManifestStaticAnalyzer::RequirementState
    fn process_requirement(
        &mut self,
        requirement_state: &mut Self::RequirementState,
        named_address_store: &NamedAddressStore,
        instruction: &GroupedInstruction,
        typed_native_invocation: Option<&TypedNativeInvocation>,
    );

    /// A method used to process instructions and extract information from them.
    fn process_instruction(
        &mut self,
        named_address_store: &NamedAddressStore,
        instruction: &GroupedInstruction,
        typed_native_invocation: Option<&TypedNativeInvocation>,
    );
}
