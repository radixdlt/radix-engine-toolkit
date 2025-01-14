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

//! This module introduces the concept of entity types grouping used in the RET
//! transaction classification.

use crate::internal_prelude::*;

define_entity_type_groups! {
    AccountEntities => [
        GlobalAccount,
        GlobalPreallocatedSecp256k1Account,
        GlobalPreallocatedEd25519Account,
    ],
    IdentityEntities => [
        GlobalIdentity,
        GlobalPreallocatedSecp256k1Identity,
        GlobalPreallocatedEd25519Identity,
    ],
    PoolEntities => [
        GlobalOneResourcePool,
        GlobalTwoResourcePool,
        GlobalMultiResourcePool,
    ],
    InternalEntities => [
        InternalGenericComponent,
        InternalFungibleVault,
        InternalNonFungibleVault,
        InternalKeyValueStore,
    ],
    SystemEntities => [
        GlobalConsensusManager,
        GlobalTransactionTracker,
    ],
    ResourceManagerEntities => [
        GlobalFungibleResourceManager,
        GlobalNonFungibleResourceManager,
    ],
    AccessControllerEntities => [GlobalAccessController],
    GenericComponentEntities => [GlobalGenericComponent],
    AccountLockerEntities => [GlobalAccountLocker],
    PackageEntities => [GlobalPackage],
    ValidatorEntities => [GlobalValidator],
}

macro_rules! define_entity_type_groups {
    (
        $(
            $group_ident: ident => [
                $($entity_type_ident: ident),* $(,)?
            ]
        ),* $(,)?
    ) => {
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Ord, Hash, PartialOrd, Sbor,
        )]
        pub enum GroupedEntityType {
            $(
                $group_ident($group_ident)
            ),*
        }

        paste! {
            impl GroupedEntityType {
                $(
                    pub fn [< belongs_to_ $group_ident:snake >](&self) -> bool {
                        matches!(self, Self::$group_ident(..))
                    }
                )*

                $(
                    $(
                        pub fn [< is_ $entity_type_ident:snake >](&self) -> bool {
                            matches!(
                                self,
                                Self::$group_ident($group_ident::$entity_type_ident)
                            )
                        }
                    )*
                )*
            }
        }

        $(
            impl From<$group_ident> for GroupedEntityType {
                fn from(value: $group_ident) -> Self {
                    GroupedEntityType::$group_ident(value)
                }
            }

            impl TryFrom<GroupedEntityType> for $group_ident {
                type Error = ();

                fn try_from(value: GroupedEntityType) -> Result<Self, Self::Error> {
                    if let GroupedEntityType::$group_ident(value) = value {
                        Ok(value)
                    } else {
                        Err(())
                    }
                }
            }
        )*

        impl From<EntityType> for GroupedEntityType {
            #[inline]
            fn from(value: EntityType) -> Self {
                match value {
                    $(
                        $(
                            EntityType::$entity_type_ident
                                => GroupedEntityType::$group_ident(
                                    $group_ident::$entity_type_ident
                                ),
                        )*
                    )*
                }
            }
        }

        impl From<GroupedEntityType> for EntityType {
            #[inline]
            fn from(value: GroupedEntityType) -> Self {
                match value {
                    $(
                        GroupedEntityType::$group_ident(value) => value.into(),
                    )*
                }
            }
        }

        $(
            #[derive(
                Clone, Copy, Debug, PartialEq, Eq, Ord, Hash, PartialOrd, Sbor,
            )]
            pub enum $group_ident {
                $(
                    $entity_type_ident
                ),*
            }

            impl From<$group_ident> for EntityType {
                #[inline]
                fn from(value: $group_ident) -> Self {
                    match value {
                        $(
                            $group_ident::$entity_type_ident => Self::$entity_type_ident,
                        )*
                    }
                }
            }

            impl TryFrom<EntityType> for $group_ident {
                type Error = ();

                #[inline]
                fn try_from(value: EntityType) -> Result<Self, Self::Error> {
                    match value {
                        $(
                            EntityType::$entity_type_ident => Ok(
                                Self::$entity_type_ident
                            ),
                        )*
                        _ => Err(())
                    }
                }
            }
        )*

        paste::paste! {
            $(
                pub fn [< node_belongs_to_ $group_ident:snake >](
                    node_id: impl TryInto<NodeId>
                ) -> bool {
                    let Ok(node_id) = node_id.try_into()
                    else {
                        return false;
                    };

                    let Some(entity_type) = node_id.entity_type()
                    else {
                        return false;
                    };

                    $group_ident::try_from(entity_type).is_ok()
                }
            )*
        }
    };
}
use define_entity_type_groups;
