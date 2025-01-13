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

pub trait ManifestAnalysisVisitor {
    /// All manifest analysis visitors have some form of an output type, this is
    /// most commonly the result of their analysis. There are no trait bounds on
    /// the output type as it can realistically be just about anything.
    type Output;

    /// A type representing the visitor's validity state.
    ///
    /// A [`ManifestAnalysisVisitor`] can either be valid and accepting more
    /// instructions or could have reached a state in its analysis where it is
    /// invalid and is no longer accepting any additional instructions.
    ///
    /// The [`ManifestAnalysisVisitor::validity_state`] method is called by the
    /// traverse to get the current validity state object of the visitor to
    /// decide if it should be called or not. In the event that the visitor is
    /// determined to be in an invalid state where it's no longer accepting any
    /// more instructions then traversing halts.
    type ValidityState: ManifestAnalysisVisitorValidityState;

    /// A method that consumes [`Self`] and returns the output of the visitor.
    /// Since this method is consuming it means that it will only be called when
    /// the traversal is done and completed.
    ///
    /// All visitors are expected to produce some form of output regardless of
    /// whether they're in an acceptable state or not. Therefore, visitors may
    /// choose to return an [`Option`] to signal whether their output is valid
    /// or not or perhaps even use custom defined enum types to signal this info
    /// to the client.
    fn output(self) -> Self::Output;

    /// A method that returns an immutable reference to the visitor's validity
    /// status.
    fn validity_state(&self) -> &Self::ValidityState;

    /// A method called by the traverser when an instruction is encountered
    /// supplying the visitor with information on the instruction and other
    /// information too needed by most visitors.
    ///
    /// While this is not enforced by the visitor trait to allow visitors to
    /// have freedom over what they do, a visitor will roughly almost always
    /// do the following in order:
    ///
    /// 1. Determine if they permit this instruction.
    /// 2. Determine if their requirements are satisfied or not by this
    ///    instruction.
    /// 3. Extract information from the instruction.
    #[allow(unused_variables)]
    fn on_instruction(
        &mut self,
        named_address_store: &NamedAddressStore,
        grouped_instruction: &GroupedInstruction,
        instruction_index: &InstructionIndex,
        invocation_io: Option<&InvocationIo<InvocationIoItems>>,
        maybe_typed_invocation: Option<&TypedManifestNativeInvocation>,
    );
}
