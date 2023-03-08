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

use radix_engine::types::address::EntityType as NativeEntityType;
use toolkit_derive::serializable;

// =================
// Model Definition
// =================

/// An enum describing the different entity types in the Radix Engine and Scrypto
#[serializable]
pub enum EntityType {
    Resource,
    Package,
    NormalComponent,
    AccountComponent,
    EcdsaSecp256k1VirtualAccountComponent,
    EddsaEd25519VirtualAccountComponent,
    EpochManager,
    Clock,
    Validator,
    IdentityComponent,
    EcdsaSecp256k1VirtualIdentityComponent,
    EddsaEd25519VirtualIdentityComponent,
    AccessControllerComponent,
}

// ============
// Conversions
// ============

impl From<EntityType> for NativeEntityType {
    fn from(value: EntityType) -> Self {
        match value {
            EntityType::Resource => Self::Resource,
            EntityType::Package => Self::Package,
            EntityType::NormalComponent => Self::NormalComponent,
            EntityType::AccountComponent => Self::AccountComponent,
            EntityType::EcdsaSecp256k1VirtualAccountComponent => {
                Self::EcdsaSecp256k1VirtualAccountComponent
            }
            EntityType::EddsaEd25519VirtualAccountComponent => {
                Self::EddsaEd25519VirtualAccountComponent
            }
            EntityType::EpochManager => Self::EpochManager,
            EntityType::Clock => Self::Clock,
            EntityType::Validator => Self::Validator,
            EntityType::IdentityComponent => Self::IdentityComponent,
            EntityType::EcdsaSecp256k1VirtualIdentityComponent => {
                Self::EcdsaSecp256k1VirtualIdentityComponent
            }
            EntityType::EddsaEd25519VirtualIdentityComponent => {
                Self::EddsaEd25519VirtualIdentityComponent
            }
            EntityType::AccessControllerComponent => Self::AccessControllerComponent,
        }
    }
}

impl From<NativeEntityType> for EntityType {
    fn from(value: NativeEntityType) -> Self {
        match value {
            NativeEntityType::Resource => Self::Resource,
            NativeEntityType::Package => Self::Package,
            NativeEntityType::NormalComponent => Self::NormalComponent,
            NativeEntityType::AccountComponent => Self::AccountComponent,
            NativeEntityType::EcdsaSecp256k1VirtualAccountComponent => {
                Self::EcdsaSecp256k1VirtualAccountComponent
            }
            NativeEntityType::EddsaEd25519VirtualAccountComponent => {
                Self::EddsaEd25519VirtualAccountComponent
            }
            NativeEntityType::EpochManager => Self::EpochManager,
            NativeEntityType::Clock => Self::Clock,
            NativeEntityType::Validator => Self::Validator,
            NativeEntityType::IdentityComponent => Self::IdentityComponent,
            NativeEntityType::EcdsaSecp256k1VirtualIdentityComponent => {
                Self::EcdsaSecp256k1VirtualIdentityComponent
            }
            NativeEntityType::EddsaEd25519VirtualIdentityComponent => {
                Self::EddsaEd25519VirtualIdentityComponent
            }
            NativeEntityType::AccessControllerComponent => Self::AccessControllerComponent,
        }
    }
}
