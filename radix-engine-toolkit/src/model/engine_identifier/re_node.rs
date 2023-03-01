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

use scrypto::prelude::RENodeId as NativeRENodeId;
use toolkit_derive::serializable;

use crate::model::address::{
    NetworkAwareComponentAddress, NetworkAwarePackageAddress, NetworkAwareResourceAddress,
};

use super::NodeIdentifier;

// =================
// Model Definition
// =================

#[serializable]
#[serde(tag = "type")]
#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub enum RENodeId {
    // =============
    // Global Nodes
    // =============
    /// Represents the Radix Engine Node Identifier of a global package.
    GlobalPackage {
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        address: NetworkAwarePackageAddress,
    },

    /// Represents the Radix Engine Node Identifier of a global component.
    GlobalComponent {
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        address: NetworkAwareComponentAddress,
    },

    /// Represents the Radix Engine Node Identifier of a global resource manager.
    GlobalResourceManager {
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        address: NetworkAwareResourceAddress,
    },

    // ================
    // Transient Nodes
    // ================
    /// Represents the Radix Engine Node Identifier of a Bucket.
    Bucket { identifier: NodeIdentifier },

    /// Represents the Radix Engine Node Identifier of a Proof.
    Proof { identifier: NodeIdentifier },

    /// Represents the Radix Engine Node Identifier of an AuthZoneStack which stores proof during
    /// transaction runtime.
    AuthZoneStack,

    /// Represents the Radix Engine Node Identifier of a Worktop which is the root store of
    /// resources returned during transaction runtime.
    Worktop,

    /// Represents the Radix Engine Node Identifier of a Logger which is used to emit logs.
    Logger,

    /// Represents the Radix Engine Node Identifier of a Transaction Runtime.
    TransactionRuntime,

    // ========================
    // Owned Native Components
    // ========================
    /// Represents the Radix Engine Node Identifier of a Key Value Store
    KeyValueStore { identifier: NodeIdentifier },

    /// Represents the Radix Engine Node Identifier of a Non Fungible Store
    NonFungibleStore { identifier: NodeIdentifier },

    /// Represents the Radix Engine Node Identifier of a Component
    Component { identifier: NodeIdentifier },

    /// Represents the Radix Engine Node Identifier of a Vault
    Vault { identifier: NodeIdentifier },

    /// Represents the Radix Engine Node Identifier of a Epoch Manager
    EpochManager { identifier: NodeIdentifier },

    /// Represents the Radix Engine Node Identifier of a Identity
    Identity { identifier: NodeIdentifier },

    /// Represents the Radix Engine Node Identifier of a Clock
    Clock { identifier: NodeIdentifier },

    /// Represents the Radix Engine Node Identifier of a Validator
    Validator { identifier: NodeIdentifier },

    /// Represents the Radix Engine Node Identifier of an Account
    Account { identifier: NodeIdentifier },

    /// Represents the Radix Engine Node Identifier of an Access Controller
    AccessController { identifier: NodeIdentifier },
}

// ============
// Conversions
// ============

impl From<RENodeId> for NativeRENodeId {
    fn from(value: RENodeId) -> Self {
        match value {
            RENodeId::Bucket { identifier } => Self::Bucket(identifier.0),
            RENodeId::Proof { identifier } => Self::Proof(identifier.0),
            RENodeId::GlobalComponent { address } => Self::GlobalComponent(address.address),
            RENodeId::KeyValueStore { identifier } => Self::KeyValueStore(identifier.0),
            RENodeId::NonFungibleStore { identifier } => Self::NonFungibleStore(identifier.0),
            RENodeId::Component { identifier } => Self::Component(identifier.0),
            RENodeId::Vault { identifier } => Self::Vault(identifier.0),
            RENodeId::GlobalResourceManager { address } => {
                Self::GlobalResourceManager(address.address)
            }
            RENodeId::GlobalPackage { address } => Self::GlobalPackage(address.address),
            RENodeId::EpochManager { identifier } => Self::EpochManager(identifier.0),
            RENodeId::Identity { identifier } => Self::Identity(identifier.0),
            RENodeId::Clock { identifier } => Self::Clock(identifier.0),
            RENodeId::Validator { identifier } => Self::Validator(identifier.0),
            RENodeId::Account { identifier } => Self::Account(identifier.0),
            RENodeId::AccessController { identifier } => Self::AccessController(identifier.0),
            RENodeId::AuthZoneStack => Self::AuthZoneStack,
            RENodeId::Worktop => Self::Worktop,
            RENodeId::Logger => Self::Logger,
            RENodeId::TransactionRuntime => Self::TransactionRuntime,
        }
    }
}
