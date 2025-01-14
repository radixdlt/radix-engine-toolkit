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
pub struct EncounteredEntitiesAnalyzer(EncounteredEntitiesOutput);

impl ManifestStaticAnalyzer for EncounteredEntitiesAnalyzer {
    type Initializer = ();
    type Output = EncounteredEntitiesOutput;
    type PermissionState = ConstState<true>;
    type RequirementState = ConstState<true>;

    fn new(
        _: Self::Initializer,
    ) -> (Self, Self::PermissionState, Self::RequirementState) {
        Default::default()
    }

    fn output(self) -> Self::Output {
        self.0
    }

    fn process_permission(
        &mut self,
        _: &mut Self::PermissionState,
        _: &NamedAddressStore,
        _: &GroupedInstruction,
        _: Option<(
            &ManifestInvocationReceiver,
            &TypedManifestNativeInvocation,
        )>,
    ) {
    }

    fn process_requirement(
        &mut self,
        _: &mut Self::RequirementState,
        _: &NamedAddressStore,
        _: &GroupedInstruction,
        _: Option<(
            &ManifestInvocationReceiver,
            &TypedManifestNativeInvocation,
        )>,
    ) {
    }

    fn process_instruction(
        &mut self,
        _: &NamedAddressStore,
        instruction: &GroupedInstruction,
        _: Option<(
            &ManifestInvocationReceiver,
            &TypedManifestNativeInvocation,
        )>,
    ) {
        let indexed_value = IndexedManifestValue::from_typed(&instruction);

        let named_addresses = indexed_value.named_addresses();
        let static_addresses = indexed_value.static_addresses();

        let addresses = named_addresses
            .iter()
            .copied()
            .map(ManifestAddress::Named)
            .chain(
                static_addresses
                    .iter()
                    .copied()
                    .map(ManifestAddress::Static),
            );
        self.0.entities.extend(addresses);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct EncounteredEntitiesOutput {
    pub entities: IndexSet<ManifestAddress>,
}
