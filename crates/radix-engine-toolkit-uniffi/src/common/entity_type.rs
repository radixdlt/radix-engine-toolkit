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

impl From<EntityType> for NativeEntityType {
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

impl From<NativeEntityType> for EntityType {
    fn from(value: NativeEntityType) -> Self {
        match value {
            NativeEntityType::GlobalPackage => Self::GlobalPackage,
            NativeEntityType::GlobalFungibleResourceManager => {
                Self::GlobalFungibleResourceManager
            }
            NativeEntityType::GlobalNonFungibleResourceManager => {
                Self::GlobalNonFungibleResourceManager
            }
            NativeEntityType::GlobalConsensusManager => {
                Self::GlobalConsensusManager
            }
            NativeEntityType::GlobalValidator => Self::GlobalValidator,
            NativeEntityType::GlobalAccessController => {
                Self::GlobalAccessController
            }
            NativeEntityType::GlobalAccount => Self::GlobalAccount,
            NativeEntityType::GlobalIdentity => Self::GlobalIdentity,
            NativeEntityType::GlobalGenericComponent => {
                Self::GlobalGenericComponent
            }
            NativeEntityType::GlobalPreallocatedSecp256k1Account => {
                Self::GlobalPreallocatedSecp256k1Account
            }
            NativeEntityType::GlobalPreallocatedEd25519Account => {
                Self::GlobalPreallocatedEd25519Account
            }
            NativeEntityType::GlobalPreallocatedSecp256k1Identity => {
                Self::GlobalPreallocatedSecp256k1Identity
            }
            NativeEntityType::GlobalPreallocatedEd25519Identity => {
                Self::GlobalPreallocatedEd25519Identity
            }
            NativeEntityType::GlobalOneResourcePool => {
                Self::GlobalOneResourcePool
            }
            NativeEntityType::GlobalTwoResourcePool => {
                Self::GlobalTwoResourcePool
            }
            NativeEntityType::GlobalMultiResourcePool => {
                Self::GlobalMultiResourcePool
            }
            NativeEntityType::GlobalTransactionTracker => {
                Self::GlobalTransactionTracker
            }
            NativeEntityType::InternalFungibleVault => {
                Self::InternalFungibleVault
            }
            NativeEntityType::InternalNonFungibleVault => {
                Self::InternalNonFungibleVault
            }
            NativeEntityType::InternalGenericComponent => {
                Self::InternalGenericComponent
            }
            NativeEntityType::InternalKeyValueStore => {
                Self::InternalKeyValueStore
            }
            NativeEntityType::GlobalAccountLocker => Self::GlobalAccountLocker,
        }
    }
}
