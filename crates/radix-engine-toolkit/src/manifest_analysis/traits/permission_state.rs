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

/// A trait that's implemented to mark some type as being the permission state
/// of a visitor. This trait provides a method for computing the current state
/// of whether the visitor is accepting additional instructions or not and also
/// a method for handling possible updates to the state.
pub trait ManifestAnalyzerPermissionState {
    /// A method that computes if all of the instructions that were encountered
    /// in a manifest were permitted for some visitor or not.
    fn all_instructions_permitted(&self) -> bool;

    /// A method that's called when an instruction is encountered to be
    /// processed by the permission state.
    fn process_instruction(&mut self, context: InstructionContext<'_>);
}
