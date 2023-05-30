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
    GlobalConsensusManager,
    GlobalValidator,
    GlobalGenericComponent,
    GlobalAccount,
    GlobalIdentity,
    GlobalAccessController,
    GlobalVirtualSecp256k1Account,
    GlobalVirtualSecp256k1Identity,
    GlobalVirtualEd25519Account,
    GlobalVirtualEd25519Identity,
    GlobalFungibleResourceManager,
    InternalFungibleVault,
    GlobalNonFungibleResourceManager,
    InternalNonFungibleVault,
    InternalGenericComponent,
    InternalAccount,
    InternalKeyValueStore,
}

// ============
// Conversions
// ============

impl From<EntityType> for NativeEntityType {
    fn from(value: EntityType) -> Self {
        match value {
            EntityType::GlobalPackage => Self::GlobalPackage,
            EntityType::GlobalConsensusManager => Self::GlobalConsensusManager,
            EntityType::GlobalValidator => Self::GlobalValidator,
            EntityType::GlobalGenericComponent => Self::GlobalGenericComponent,
            EntityType::GlobalAccount => Self::GlobalAccount,
            EntityType::GlobalIdentity => Self::GlobalIdentity,
            EntityType::GlobalAccessController => Self::GlobalAccessController,
            EntityType::GlobalVirtualSecp256k1Account => Self::GlobalVirtualSecp256k1Account,
            EntityType::GlobalVirtualSecp256k1Identity => Self::GlobalVirtualSecp256k1Identity,
            EntityType::GlobalVirtualEd25519Account => Self::GlobalVirtualEd25519Account,
            EntityType::GlobalVirtualEd25519Identity => Self::GlobalVirtualEd25519Identity,
            EntityType::GlobalFungibleResourceManager => Self::GlobalFungibleResourceManager,
            EntityType::InternalFungibleVault => Self::InternalFungibleVault,
            EntityType::GlobalNonFungibleResourceManager => Self::GlobalNonFungibleResourceManager,
            EntityType::InternalNonFungibleVault => Self::InternalNonFungibleVault,
            EntityType::InternalGenericComponent => Self::InternalGenericComponent,
            EntityType::InternalAccount => Self::InternalAccount,
            EntityType::InternalKeyValueStore => Self::InternalKeyValueStore,
        }
    }
}

impl From<NativeEntityType> for EntityType {
    fn from(value: NativeEntityType) -> Self {
        match value {
            NativeEntityType::GlobalPackage => Self::GlobalPackage,
            NativeEntityType::GlobalConsensusManager => Self::GlobalConsensusManager,
            NativeEntityType::GlobalValidator => Self::GlobalValidator,
            NativeEntityType::GlobalGenericComponent => Self::GlobalGenericComponent,
            NativeEntityType::GlobalAccount => Self::GlobalAccount,
            NativeEntityType::GlobalIdentity => Self::GlobalIdentity,
            NativeEntityType::GlobalAccessController => Self::GlobalAccessController,
            NativeEntityType::GlobalVirtualSecp256k1Account => Self::GlobalVirtualSecp256k1Account,
            NativeEntityType::GlobalVirtualSecp256k1Identity => {
                Self::GlobalVirtualSecp256k1Identity
            }
            NativeEntityType::GlobalVirtualEd25519Account => Self::GlobalVirtualEd25519Account,
            NativeEntityType::GlobalVirtualEd25519Identity => Self::GlobalVirtualEd25519Identity,
            NativeEntityType::GlobalFungibleResourceManager => Self::GlobalFungibleResourceManager,
            NativeEntityType::InternalFungibleVault => Self::InternalFungibleVault,
            NativeEntityType::GlobalNonFungibleResourceManager => {
                Self::GlobalNonFungibleResourceManager
            }
            NativeEntityType::InternalNonFungibleVault => Self::InternalNonFungibleVault,
            NativeEntityType::InternalGenericComponent => Self::InternalGenericComponent,
            NativeEntityType::InternalAccount => Self::InternalAccount,
            NativeEntityType::InternalKeyValueStore => Self::InternalKeyValueStore,
        }
    }
}
