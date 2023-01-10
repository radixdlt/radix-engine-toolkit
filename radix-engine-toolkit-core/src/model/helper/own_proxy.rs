// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at

//   http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use crate::model::NodeIdentifier;
use scrypto::runtime::Own;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Serialize, Deserialize)]
#[serde(tag = "variant", content = "value")]
pub enum OwnProxy {
    KeyValueStore(NodeIdentifier),
    Component(NodeIdentifier),
    Vault(NodeIdentifier),
    Bucket(#[serde_as(as = "DisplayFromStr")] u32),
    Proof(#[serde_as(as = "DisplayFromStr")] u32),
}

impl From<Own> for OwnProxy {
    fn from(value: Own) -> Self {
        match value {
            Own::Bucket(v) => Self::Bucket(v),
            Own::Proof(v) => Self::Proof(v),
            Own::KeyValueStore(v) => Self::KeyValueStore(NodeIdentifier::from_bytes(v)),
            Own::Component(v) => Self::Component(NodeIdentifier::from_bytes(v)),
            Own::Vault(v) => Self::Vault(NodeIdentifier::from_bytes(v)),
        }
    }
}

impl From<OwnProxy> for Own {
    fn from(value: OwnProxy) -> Self {
        match value {
            OwnProxy::Bucket(v) => Self::Bucket(v),
            OwnProxy::Proof(v) => Self::Proof(v),
            OwnProxy::KeyValueStore(v) => Self::KeyValueStore(v.to_bytes()),
            OwnProxy::Component(v) => Self::Component(v.to_bytes()),
            OwnProxy::Vault(v) => Self::Vault(v.to_bytes()),
        }
    }
}
