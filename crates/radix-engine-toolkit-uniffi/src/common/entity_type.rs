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

use crate::prelude::*;

#[derive(Clone, Copy, Debug, Enum, Hash, PartialEq, Eq)]
pub enum EntityType {
    GlobalPackage,
    GlobalFungibleResourceManager,
    GlobalNonFungibleResourceManager,
    GlobalConsensusManager,
    GlobalValidator,
    GlobalAccessController,
    GlobalAccount,
    GlobalIdentity,
    GlobalGenericComponent,
    GlobalPreallocatedSecp256k1Account,
    GlobalPreallocatedEd25519Account,
    GlobalPreallocatedSecp256k1Identity,
    GlobalPreallocatedEd25519Identity,
    GlobalOneResourcePool,
    GlobalTwoResourcePool,
    GlobalMultiResourcePool,
    GlobalAccountLocker,
    GlobalTransactionTracker,
    InternalFungibleVault,
    InternalNonFungibleVault,
    InternalGenericComponent,
    InternalKeyValueStore,
}

impl From<EntityType> for engine::EntityType {
    fn from(value: EntityType) -> Self {
        match value {
            EntityType::GlobalPackage => Self::GlobalPackage,
            EntityType::GlobalFungibleResourceManager => {
                Self::GlobalFungibleResourceManager
            }
            EntityType::GlobalNonFungibleResourceManager => {
                Self::GlobalNonFungibleResourceManager
            }
            EntityType::GlobalConsensusManager => Self::GlobalConsensusManager,
            EntityType::GlobalValidator => Self::GlobalValidator,
            EntityType::GlobalAccessController => Self::GlobalAccessController,
            EntityType::GlobalAccount => Self::GlobalAccount,
            EntityType::GlobalIdentity => Self::GlobalIdentity,
            EntityType::GlobalGenericComponent => Self::GlobalGenericComponent,
            EntityType::GlobalPreallocatedSecp256k1Account => {
                Self::GlobalPreallocatedSecp256k1Account
            }
            EntityType::GlobalPreallocatedEd25519Account => {
                Self::GlobalPreallocatedEd25519Account
            }
            EntityType::GlobalPreallocatedSecp256k1Identity => {
                Self::GlobalPreallocatedSecp256k1Identity
            }
            EntityType::GlobalPreallocatedEd25519Identity => {
                Self::GlobalPreallocatedEd25519Identity
            }
            EntityType::GlobalOneResourcePool => Self::GlobalOneResourcePool,
            EntityType::GlobalTwoResourcePool => Self::GlobalTwoResourcePool,
            EntityType::GlobalMultiResourcePool => {
                Self::GlobalMultiResourcePool
            }
            EntityType::GlobalTransactionTracker => {
                Self::GlobalTransactionTracker
            }
            EntityType::InternalFungibleVault => Self::InternalFungibleVault,
            EntityType::InternalNonFungibleVault => {
                Self::InternalNonFungibleVault
            }
            EntityType::InternalGenericComponent => {
                Self::InternalGenericComponent
            }
            EntityType::InternalKeyValueStore => Self::InternalKeyValueStore,
            EntityType::GlobalAccountLocker => Self::GlobalAccountLocker,
        }
    }
}

impl From<engine::EntityType> for EntityType {
    fn from(value: engine::EntityType) -> Self {
        match value {
            engine::EntityType::GlobalPackage => Self::GlobalPackage,
            engine::EntityType::GlobalFungibleResourceManager => {
                Self::GlobalFungibleResourceManager
            }
            engine::EntityType::GlobalNonFungibleResourceManager => {
                Self::GlobalNonFungibleResourceManager
            }
            engine::EntityType::GlobalConsensusManager => {
                Self::GlobalConsensusManager
            }
            engine::EntityType::GlobalValidator => Self::GlobalValidator,
            engine::EntityType::GlobalAccessController => {
                Self::GlobalAccessController
            }
            engine::EntityType::GlobalAccount => Self::GlobalAccount,
            engine::EntityType::GlobalIdentity => Self::GlobalIdentity,
            engine::EntityType::GlobalGenericComponent => {
                Self::GlobalGenericComponent
            }
            engine::EntityType::GlobalPreallocatedSecp256k1Account => {
                Self::GlobalPreallocatedSecp256k1Account
            }
            engine::EntityType::GlobalPreallocatedEd25519Account => {
                Self::GlobalPreallocatedEd25519Account
            }
            engine::EntityType::GlobalPreallocatedSecp256k1Identity => {
                Self::GlobalPreallocatedSecp256k1Identity
            }
            engine::EntityType::GlobalPreallocatedEd25519Identity => {
                Self::GlobalPreallocatedEd25519Identity
            }
            engine::EntityType::GlobalOneResourcePool => {
                Self::GlobalOneResourcePool
            }
            engine::EntityType::GlobalTwoResourcePool => {
                Self::GlobalTwoResourcePool
            }
            engine::EntityType::GlobalMultiResourcePool => {
                Self::GlobalMultiResourcePool
            }
            engine::EntityType::GlobalTransactionTracker => {
                Self::GlobalTransactionTracker
            }
            engine::EntityType::InternalFungibleVault => {
                Self::InternalFungibleVault
            }
            engine::EntityType::InternalNonFungibleVault => {
                Self::InternalNonFungibleVault
            }
            engine::EntityType::InternalGenericComponent => {
                Self::InternalGenericComponent
            }
            engine::EntityType::InternalKeyValueStore => {
                Self::InternalKeyValueStore
            }
            engine::EntityType::GlobalAccountLocker => Self::GlobalAccountLocker,
        }
    }
}
