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
pub struct EncounteredEntitiesVisitor(EncounteredEntitiesOutput);

impl EncounteredEntitiesVisitor {
    pub fn new() -> Self {
        Default::default()
    }
}

impl ManifestAnalysisVisitor for EncounteredEntitiesVisitor {
    type Output = EncounteredEntitiesOutput;
    type ValidityState = ConstManifestAnalysisVisitorValidityState<true>;

    fn output(self) -> Self::Output {
        self.0
    }

    fn validity_state(&self) -> &Self::ValidityState {
        &ConstManifestAnalysisVisitorValidityState::<true>
    }

    fn on_instruction(
        &mut self,
        _: &NamedAddressStore,
        grouped_instruction: &GroupedInstruction,
        _: &InstructionIndex,
        _: Option<&TypedManifestNativeInvocation>,
    ) {
        let indexed_value =
            IndexedManifestValue::from_typed(&grouped_instruction);

        let named_addresses = indexed_value.named_addresses();
        let static_addresses = indexed_value.static_addresses();

        self.0.entities.extend(
            named_addresses
                .iter()
                .copied()
                .map(ManifestAddress::Named)
                .chain(
                    static_addresses
                        .into_iter()
                        .map(|node_id| *node_id.as_node_id())
                        .map(ManifestAddress::Static),
                ),
        );
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct EncounteredEntitiesOutput {
    pub entities: IndexSet<ManifestAddress>,
}
