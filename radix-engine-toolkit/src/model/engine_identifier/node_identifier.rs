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

use radix_engine_common::types::NodeId;
use std::borrow::Borrow;
use std::fmt::Display;
use std::str::FromStr;

use crate::error::{Error, Result};
use crate::model::address::Bech32Coder;

// =================
// Model Definition
// =================

/// Represents a Radix Engine persistent node identifier which is 27 bytes long and serialized as a
/// Bech32m encoded string.
#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Copy, Clone, Debug)]
pub struct NetworkAwareNodeId(pub [u8; NodeId::LENGTH], pub u8);

impl NetworkAwareNodeId {
    pub fn node_id(&self) -> NodeId {
        NodeId(self.0)
    }

    pub fn network_id(&self) -> u8 {
        self.1
    }
}

impl From<NetworkAwareNodeId> for NodeId {
    fn from(value: NetworkAwareNodeId) -> Self {
        Self(value.0)
    }
}

impl From<&NetworkAwareNodeId> for NodeId {
    fn from(value: &NetworkAwareNodeId) -> Self {
        Self(value.0)
    }
}

impl From<&mut NetworkAwareNodeId> for NodeId {
    fn from(value: &mut NetworkAwareNodeId) -> Self {
        Self(value.0)
    }
}

// =====
// Text
// =====

impl Display for NetworkAwareNodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let coder = Bech32Coder::new(self.1);
        let node_id = NodeId(self.0);

        coder
            .encode(node_id)
            .map_err(|_| std::fmt::Error)
            .and_then(|node_id| write!(f, "{}", node_id))
    }
}

impl FromStr for NetworkAwareNodeId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let bech32_coder = Bech32Coder::new_from_address(s)?;
        let node_id = bech32_coder.decode(s)?;
        Ok(Self(node_id.0, bech32_coder.network_id()))
    }
}
