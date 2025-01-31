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

macro_rules! impl_conversions {
    (
        $(
            ($scrypto_type: ty, $manifest_type: ty $(,)?)
        ),* $(,)?
    ) => {
        $(
            #[ext_sized]
            pub impl $manifest_type {
                fn as_static(&self) -> Option<&$scrypto_type> {
                    if let Self::Static(v) = self {
                        Some(v)
                    } else {
                        None
                    }
                }

                fn into_static(self) -> Option<$scrypto_type> {
                    if let Self::Static(v) = self {
                        Some(v)
                    } else {
                        None
                    }
                }

                fn resolve_entity_type(
                    &self,
                    named_address_store: &NamedAddressStore
                ) -> Option<EntityType> {
                    match self {
                        Self::Static(static_address) => {
                            let node_id = NodeId::from(*static_address);
                            node_id.entity_type()
                        },
                        Self::Named(named_address) => {
                            named_address_store
                                .get(&named_address)
                                .and_then(BlueprintId::entity_type)
                        }
                    }
                }

                fn resolve_grouped_entity_type(
                    &self,
                    named_address_store: &NamedAddressStore
                ) -> Option<GroupedEntityType> {
                    self.resolve_entity_type(named_address_store)
                        .map(Into::into)
                }
            }
        )*
    };
}
impl_conversions![
    (NodeId, ManifestAddress),
    (GlobalAddress, ManifestGlobalAddress),
    (ComponentAddress, ManifestComponentAddress),
    (ResourceAddress, ManifestResourceAddress),
    (PackageAddress, ManifestPackageAddress),
];
