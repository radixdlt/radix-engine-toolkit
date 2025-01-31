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
pub struct WorktopChanges(IndexMap<InstructionIndex, Vec<WorktopChange>>);

impl WorktopChanges {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn first_take_of_resource(
        &self,
        instruction_index: &InstructionIndex,
        resource_address: ResourceAddress,
    ) -> Option<&ResourceSpecifier> {
        self.take_iterator(instruction_index)
            .find(|take| take.resource_address() == &resource_address)
    }

    pub fn first_put_of_resource(
        &self,
        instruction_index: &InstructionIndex,
        resource_address: ResourceAddress,
    ) -> Option<&ResourceSpecifier> {
        self.put_iterator(instruction_index)
            .find(|put| put.resource_address() == &resource_address)
    }

    pub fn first_take(
        &self,
        instruction_index: &InstructionIndex,
    ) -> Option<&ResourceSpecifier> {
        self.take_iterator(instruction_index).next()
    }

    pub fn first_put(
        &self,
        instruction_index: &InstructionIndex,
    ) -> Option<&ResourceSpecifier> {
        self.put_iterator(instruction_index).next()
    }

    pub fn take_iterator(
        &self,
        instruction_index: &InstructionIndex,
    ) -> impl Iterator<Item = &ResourceSpecifier> {
        self.resource_specifier_iterator(
            instruction_index,
            WorktopChange::as_take,
        )
    }

    pub fn put_iterator(
        &self,
        instruction_index: &InstructionIndex,
    ) -> impl Iterator<Item = &ResourceSpecifier> {
        self.resource_specifier_iterator(
            instruction_index,
            WorktopChange::as_put,
        )
    }

    fn resource_specifier_iterator(
        &self,
        instruction_index: &InstructionIndex,
        filter_predicate: impl FnMut(&WorktopChange) -> Option<&ResourceSpecifier>,
    ) -> impl Iterator<Item = &ResourceSpecifier> {
        self.worktop_changes_iterator(instruction_index)
            .filter_map(filter_predicate)
    }

    fn worktop_changes_iterator(
        &self,
        instruction_index: &InstructionIndex,
    ) -> impl Iterator<Item = &WorktopChange> {
        self.0
            .get(instruction_index)
            .map(Vec::as_slice)
            .unwrap_or(&[])
            .iter()
    }
}

impl<I, K, V, W> From<I> for WorktopChanges
where
    I: IntoIterator<Item = (K, V)>,
    K: Into<InstructionIndex>,
    V: IntoIterator<Item = W>,
    W: Into<WorktopChange>,
{
    fn from(value: I) -> Self {
        Self(
            value
                .into_iter()
                .map(|(k, v)| {
                    (k.into(), v.into_iter().map(Into::into).collect())
                })
                .collect(),
        )
    }
}
