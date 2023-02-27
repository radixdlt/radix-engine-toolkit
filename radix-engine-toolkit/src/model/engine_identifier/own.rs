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

use crate::NodeIdentifier;
use scrypto::runtime::Own as ScryptoOwn;
use toolkit_derive::serializable;

// =================
// Model Definition
// =================

#[serializable]
#[serde(tag = "type", content = "value")]
/// Represents a tagged enum of Radix Engine Nodes which may be owned in the point of view of the
/// transaction manifest.
pub enum Own {
    /// Represents an owned KeyValueStore
    KeyValueStore(NodeIdentifier),

    /// Represents an owned Component
    Component(NodeIdentifier),

    /// Represents an owned Vault
    Vault(NodeIdentifier),

    /// Represents an owned Bucket
    Bucket(NodeIdentifier),

    /// Represents an owned Proof
    Proof(NodeIdentifier),

    /// Represents an owned Account
    Account(NodeIdentifier),
}

impl From<ScryptoOwn> for Own {
    fn from(value: ScryptoOwn) -> Self {
        match value {
            ScryptoOwn::Bucket(v) => Self::Bucket(NodeIdentifier(v)),
            ScryptoOwn::Proof(v) => Self::Proof(NodeIdentifier(v)),
            ScryptoOwn::KeyValueStore(v) => Self::KeyValueStore(NodeIdentifier(v)),
            ScryptoOwn::Component(v) => Self::Component(NodeIdentifier(v)),
            ScryptoOwn::Vault(v) => Self::Vault(NodeIdentifier(v)),
            ScryptoOwn::Account(v) => Self::Account(NodeIdentifier(v)),
        }
    }
}

impl From<Own> for ScryptoOwn {
    fn from(value: Own) -> Self {
        match value {
            Own::Bucket(v) => Self::Bucket(v.0),
            Own::Proof(v) => Self::Proof(v.0),
            Own::KeyValueStore(v) => Self::KeyValueStore(v.0),
            Own::Component(v) => Self::Component(v.0),
            Own::Vault(v) => Self::Vault(v.0),
            Own::Account(v) => Self::Account(v.0),
        }
    }
}
