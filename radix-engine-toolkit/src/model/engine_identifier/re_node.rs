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
    /// Represents the Radix Engine Node Identifier of an AuthZoneStack which stores proof during
    /// transaction runtime.
    AuthZoneStack,

    /// Represents the Radix Engine Node Identifier of a Worktop which is the root store of
    /// resources returned during transaction runtime.
    Worktop,

    /// Represents the Radix Engine Node Identifier of a Transaction Runtime.
    TransactionRuntime,

    // =======
    // Object
    // =======
    /// Represents an owned object
    Object { identifier: NodeIdentifier },

    /// Represents the Radix Engine Node Identifier of a NonFungibleStore.
    NonFungibleStore { identifier: NodeIdentifier },

    /// Represents the Radix Engine Node Identifier of a KeyValueStore.
    KeyValueStore { identifier: NodeIdentifier },
}

// ============
// Conversions
// ============

impl From<RENodeId> for NativeRENodeId {
    fn from(value: RENodeId) -> Self {
        match value {
            RENodeId::Object { identifier } => Self::Object(identifier.0),
            RENodeId::KeyValueStore { identifier } => Self::KeyValueStore(identifier.0),
            RENodeId::NonFungibleStore { identifier } => Self::NonFungibleStore(identifier.0),
            RENodeId::GlobalComponent { address } => Self::GlobalComponent(address.address),
            RENodeId::GlobalResourceManager { address } => {
                Self::GlobalResourceManager(address.address)
            }
            RENodeId::GlobalPackage { address } => Self::GlobalPackage(address.address),
            RENodeId::AuthZoneStack => Self::AuthZoneStack,
            RENodeId::Worktop => Self::Worktop,
            RENodeId::TransactionRuntime => Self::TransactionRuntime,
        }
    }
}
