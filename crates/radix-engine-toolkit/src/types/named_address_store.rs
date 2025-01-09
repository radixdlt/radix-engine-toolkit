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
pub struct NamedAddressStore(IndexMap<ManifestNamedAddress, BlueprintId>);

impl NamedAddressStore {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn into_inner(self) -> IndexMap<ManifestNamedAddress, BlueprintId> {
        self.0
    }

    pub fn insert(
        &mut self,
        named_address: ManifestNamedAddress,
        blueprint_id: BlueprintId,
    ) {
        self.0.insert(named_address, blueprint_id);
    }

    pub fn get(
        &self,
        named_address: &ManifestNamedAddress,
    ) -> Option<&BlueprintId> {
        self.0.get(named_address)
    }

    pub fn grouped_entity_type(
        &self,
        address: impl Into<ManifestAddress>,
    ) -> Option<GroupedEntityType> {
        self.entity_type(address).map(Into::into)
    }

    pub fn entity_type(
        &self,
        address: impl Into<ManifestAddress>,
    ) -> Option<EntityType> {
        match address.into() {
            ManifestAddress::Static(static_address) => {
                static_address.entity_type()
            }
            ManifestAddress::Named(named_address) => {
                self.get(&named_address).and_then(BlueprintId::entity_type)
            }
        }
    }
}
