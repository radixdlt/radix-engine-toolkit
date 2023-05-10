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

use scrypto::prelude::EntityType as NativeEntityType;
use toolkit_derive::serializable;

// =================
// Model Definition
// =================

/// An enum describing the different entity types in the Radix Engine and Scrypto
#[serializable]
pub enum EntityType {
    GlobalPackage,
    GlobalFungibleResource,
    GlobalNonFungibleResource,
    GlobalEpochManager,
    GlobalValidator,
    GlobalClock,
    GlobalAccessController,
    GlobalAccount,
    GlobalIdentity,
    GlobalGenericComponent,
    GlobalVirtualSecp256k1Account,
    GlobalVirtualEd25519Account,
    GlobalVirtualSecp256k1Identity,
    GlobalVirtualEd25519Identity,
    InternalFungibleVault,
    InternalNonFungibleVault,
    InternalAccount,
    InternalGenericComponent,
    InternalKeyValueStore,
}

// ============
// Conversions
// ============

impl From<EntityType> for NativeEntityType {
    fn from(value: EntityType) -> Self {
        match value {
            EntityType::GlobalPackage => Self::GlobalPackage,
            EntityType::GlobalFungibleResource => Self::GlobalFungibleResource,
            EntityType::GlobalNonFungibleResource => Self::GlobalNonFungibleResource,
            EntityType::GlobalEpochManager => Self::GlobalEpochManager,
            EntityType::GlobalValidator => Self::GlobalValidator,
            EntityType::GlobalClock => Self::GlobalClock,
            EntityType::GlobalAccessController => Self::GlobalAccessController,
            EntityType::GlobalAccount => Self::GlobalAccount,
            EntityType::GlobalIdentity => Self::GlobalIdentity,
            EntityType::GlobalGenericComponent => Self::GlobalGenericComponent,
            EntityType::GlobalVirtualSecp256k1Account => Self::GlobalVirtualSecp256k1Account,
            EntityType::GlobalVirtualEd25519Account => Self::GlobalVirtualEd25519Account,
            EntityType::GlobalVirtualSecp256k1Identity => Self::GlobalVirtualSecp256k1Identity,
            EntityType::GlobalVirtualEd25519Identity => Self::GlobalVirtualEd25519Identity,
            EntityType::InternalFungibleVault => Self::InternalFungibleVault,
            EntityType::InternalNonFungibleVault => Self::InternalNonFungibleVault,
            EntityType::InternalAccount => Self::InternalAccount,
            EntityType::InternalKeyValueStore => Self::InternalKeyValueStore,
            EntityType::InternalGenericComponent => Self::InternalGenericComponent,
        }
    }
}

impl From<NativeEntityType> for EntityType {
    fn from(value: NativeEntityType) -> Self {
        match value {
            NativeEntityType::GlobalPackage => Self::GlobalPackage,
            NativeEntityType::GlobalFungibleResource => Self::GlobalFungibleResource,
            NativeEntityType::GlobalNonFungibleResource => Self::GlobalNonFungibleResource,
            NativeEntityType::GlobalEpochManager => Self::GlobalEpochManager,
            NativeEntityType::GlobalValidator => Self::GlobalValidator,
            NativeEntityType::GlobalClock => Self::GlobalClock,
            NativeEntityType::GlobalAccessController => Self::GlobalAccessController,
            NativeEntityType::GlobalAccount => Self::GlobalAccount,
            NativeEntityType::GlobalIdentity => Self::GlobalIdentity,
            NativeEntityType::GlobalGenericComponent => Self::GlobalGenericComponent,
            NativeEntityType::GlobalVirtualSecp256k1Account => Self::GlobalVirtualSecp256k1Account,
            NativeEntityType::GlobalVirtualEd25519Account => Self::GlobalVirtualEd25519Account,
            NativeEntityType::GlobalVirtualSecp256k1Identity => {
                Self::GlobalVirtualSecp256k1Identity
            }
            NativeEntityType::GlobalVirtualEd25519Identity => Self::GlobalVirtualEd25519Identity,
            NativeEntityType::InternalFungibleVault => Self::InternalFungibleVault,
            NativeEntityType::InternalNonFungibleVault => Self::InternalNonFungibleVault,
            NativeEntityType::InternalAccount => Self::InternalAccount,
            NativeEntityType::InternalKeyValueStore => Self::InternalKeyValueStore,
            NativeEntityType::InternalGenericComponent => Self::InternalGenericComponent,
        }
    }
}
