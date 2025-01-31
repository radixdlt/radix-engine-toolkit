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

/// A resolved manifest address is a type similar to [`ManifestAddress`] in the
/// sense that it can either be static or named. The primary difference is that
/// this type is generic over the type used for static addresses and that the
/// [`Named`] variant of the enum stores the [`BlueprintId`] of the allocated
/// address. This means that we can obtain the [`EntityType`] for both variants
/// of this enum.
///
/// [`Named`]: ResolvedManifestAddress::Named
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ResolvedManifestAddress<T> {
    Static {
        static_address: T,
    },
    Named {
        blueprint_id: BlueprintId,
        named_address: ManifestNamedAddress,
    },
}

impl<T> ResolvedManifestAddress<T>
where
    T: Copy + Into<NodeId>,
{
    pub fn entity_type(&self) -> Option<EntityType> {
        match self {
            Self::Static { static_address } => {
                (*static_address).into().entity_type()
            }
            Self::Named { blueprint_id, .. } => blueprint_id.entity_type(),
        }
    }

    pub fn grouped_entity_type(&self) -> Option<GroupedEntityType> {
        self.entity_type().map(Into::into)
    }

    pub fn is_access_controller(&self) -> bool {
        self.grouped_entity_type().as_ref().is_some_and(
            GroupedEntityType::belongs_to_access_controller_entities,
        )
    }

    pub fn is_account(&self) -> bool {
        self.grouped_entity_type()
            .as_ref()
            .is_some_and(GroupedEntityType::belongs_to_account_entities)
    }

    pub fn is_identity(&self) -> bool {
        self.grouped_entity_type()
            .as_ref()
            .is_some_and(GroupedEntityType::belongs_to_identity_entities)
    }

    pub fn is_fungible_resource_manager(&self) -> bool {
        self.grouped_entity_type()
            .as_ref()
            .is_some_and(GroupedEntityType::is_global_fungible_resource_manager)
    }

    pub fn is_non_fungible_resource_manager(&self) -> bool {
        self.grouped_entity_type().as_ref().is_some_and(
            GroupedEntityType::is_global_non_fungible_resource_manager,
        )
    }
}

macro_rules! impl_conversions {
    (
        $(
            ($scrypto_type: ty $(, $manifest_type: ty)? $(,)?)
        ),* $(,)?
    ) => {
        paste! {
            $(
                impl From<$scrypto_type> for ResolvedManifestAddress<$scrypto_type> {
                    fn from(value: $scrypto_type) -> Self {
                        Self::Static { static_address: value }
                    }
                }

                impl TryFrom<ResolvedManifestAddress<$scrypto_type>> for $scrypto_type {
                    type Error = ();

                    fn try_from(value: ResolvedManifestAddress<$scrypto_type>) -> Result<Self, Self::Error> {
                        if let ResolvedManifestAddress::Static { static_address: value } = value {
                            Ok(value)
                        } else {
                            Err(())
                        }
                    }
                }

                impl From<ResolvedManifestAddress<$scrypto_type>> for ResolvedDynamicAddress<$scrypto_type> {
                    fn from(value: ResolvedManifestAddress<$scrypto_type>) -> Self {
                        match value {
                            ResolvedManifestAddress::Static { static_address } => ResolvedDynamicAddress::StaticAddress(static_address),
                            ResolvedManifestAddress::Named { blueprint_id, .. } => ResolvedDynamicAddress::BlueprintResolvedFromNamedAddress(blueprint_id),
                        }
                    }
                }

                $(
                    impl From<ResolvedManifestAddress<$scrypto_type>> for $manifest_type {
                        fn from(value: ResolvedManifestAddress<$scrypto_type>) -> Self {
                            match value {
                                ResolvedManifestAddress::Static { static_address: value } => Self::Static(value),
                                ResolvedManifestAddress::Named { named_address: value, .. } => Self::Named(value),
                            }
                        }
                    }

                    impl From<&ResolvedManifestAddress<$scrypto_type>> for $manifest_type {
                        fn from(value: &ResolvedManifestAddress<$scrypto_type>) -> Self {
                            match value {
                                ResolvedManifestAddress::Static { static_address: value } => Self::Static(*value),
                                ResolvedManifestAddress::Named { named_address: value, .. } => Self::Named(*value),
                            }
                        }
                    }

                    impl ResolvedManifestAddress<$scrypto_type> {
                        pub fn [< from_ $manifest_type:snake >](
                            address: &$manifest_type,
                            named_address_store: &NamedAddressStore,
                        ) -> Option<Self> {
                            match address {
                                $manifest_type::Static(address) => Some(Self::Static {
                                    static_address: *address,
                                }),
                                $manifest_type::Named(address) => named_address_store
                                    .get(address)
                                    .map(|blueprint_id| Self::Named {
                                        blueprint_id: blueprint_id.clone(),
                                        named_address: *address,
                                    }),
                            }
                        }
                    }
                )?
            )*
        }
    };
}
impl_conversions![
    (InternalAddress),
    (NodeId, ManifestAddress),
    (GlobalAddress, ManifestGlobalAddress),
    (ComponentAddress, ManifestComponentAddress),
    (ResourceAddress, ManifestResourceAddress),
    (PackageAddress, ManifestPackageAddress),
];
