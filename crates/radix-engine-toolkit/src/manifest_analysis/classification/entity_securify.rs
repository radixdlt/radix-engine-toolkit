
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

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct EntitySecurifyAnalyzer(EntitySecurifyOutput);

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct EntitySecurifyOutput {
    pub securified_accounts: Vec<ManifestGlobalAddress>,
    pub securified_personas: Vec<ManifestGlobalAddress>
}

impl ManifestStaticAnalyzer for EntitySecurifyAnalyzer {
    type Initializer = ();
    type Output = EntitySecurifyOutput;
    type PermissionState = 
        CallbackPermissionState<PermissionStateStaticCallback>;

    type RequirementState = AllOfRequirement<(
        EntitySecurify, 
        CreateAccessController,
    )>;

    fn new(
        initializer: Self::Initializer,
    ) -> (Self, Self::PermissionState, Self::RequirementState) {
        todo!()
    }

    fn output(self) -> Self::Output {
        todo!()
    }

    fn process_instruction(&mut self, context: InstructionContext<'_>) {
        todo!()
    }
}