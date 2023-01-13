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
use serializable::serializable;

// =================
// Model Definition
// =================

#[serializable]
#[serde(tag = "variant", content = "value")]
/// Represents a tagged enum of Radix Engine Nodes which may be owned in the point of view of the
/// transaction manifest.
pub enum Own {
    /// Represents an owned KeyValueStore
    KeyValueStore(NodeIdentifier),

    /// Represents an owned Component
    Component(NodeIdentifier),

    /// Represents an owned Vault
    Vault(NodeIdentifier),

    /// Represents an owned Bucket identified through an unsigned 32-bit integer which is serialized
    /// as a string
    Bucket(
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        u32,
    ),

    /// Represents an owned Proof identified through an unsigned 32-bit integer which is serialized
    /// as a string
    Proof(
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        u32,
    ),
}

impl From<ScryptoOwn> for Own {
    fn from(value: ScryptoOwn) -> Self {
        match value {
            ScryptoOwn::Bucket(v) => Self::Bucket(v),
            ScryptoOwn::Proof(v) => Self::Proof(v),
            ScryptoOwn::KeyValueStore(v) => Self::KeyValueStore(NodeIdentifier(v)),
            ScryptoOwn::Component(v) => Self::Component(NodeIdentifier(v)),
            ScryptoOwn::Vault(v) => Self::Vault(NodeIdentifier(v)),
        }
    }
}

impl From<Own> for ScryptoOwn {
    fn from(value: Own) -> Self {
        match value {
            Own::Bucket(v) => Self::Bucket(v),
            Own::Proof(v) => Self::Proof(v),
            Own::KeyValueStore(v) => Self::KeyValueStore(v.0),
            Own::Component(v) => Self::Component(v.0),
            Own::Vault(v) => Self::Vault(v.0),
        }
    }
}
