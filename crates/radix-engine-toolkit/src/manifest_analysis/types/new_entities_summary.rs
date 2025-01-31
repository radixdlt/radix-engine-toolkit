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

/// A summary of the newly created entities both global and local (internal).
pub struct NewEntitiesOutput {
    /// The set of newly created global entities.
    pub new_global_entities: IndexSet<GlobalAddress>,
    /// The set of newly created internal entities.
    pub new_internal_entities: IndexSet<InternalAddress>,

    /// The set of newly created component entities.
    pub new_component_entities: IndexSet<ComponentAddress>,
    /// The set of newly created resource entities.
    pub new_resource_entities: IndexSet<ResourceAddress>,
    /// The set of newly created package entities.
    pub new_package_entities: IndexSet<PackageAddress>,

    /// The set of newly minted non-fungibles.
    pub new_non_fungibles: IndexSet<NonFungibleGlobalId>,

    /// A map of the metadata of the newly created global entities. Note that
    /// not all of the global entities in the [`new_global_entities`] field
    /// have an entry here since some of the global entities might have no
    /// metadata.
    ///
    /// [`new_global_entities`]: NewEntitiesSummary::new_global_entities
    pub global_entities_metadata:
        IndexMap<GlobalAddress, IndexMap<String, Option<MetadataValue>>>,
}
