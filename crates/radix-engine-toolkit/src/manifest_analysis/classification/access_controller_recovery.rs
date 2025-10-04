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
pub struct AccessControllerRecoveryAnalyzer(AccessControllerRecoveryOutput);

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct AccessControllerRecoveryOutput {
    pub access_controllers: Vec<ComponentAddress>,
}

impl ManifestStaticAnalyzer for AccessControllerRecoveryAnalyzer {
    type Initializer = ();
    type Output = AccessControllerRecoveryOutput;

    type PermissionState =
        CallbackPermissionState<PermissionStateStaticCallback>;

    type RequirementState = AnyOfRequirement<(
        AccessControllerInitiateRecoveryAsPrimary,
        AccessControllerInitiateRecoveryAsRecovery,
    )>;

    fn new(
        _: Self::Initializer,
    ) -> (Self, Self::PermissionState, Self::RequirementState) {
        (
            Default::default(),
            CallbackPermissionState::new(is_instruction_permitted),
            Default::default(),
        )
    }

    fn output(self) -> Self::Output {
        self.0
    }

    fn process_instruction(&mut self, context: InstructionContext<'_>) {
        todo!()
    }
}

fn is_instruction_permitted(context: InstructionContext<'_>) -> bool {
    todo!()
}
