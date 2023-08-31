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

use radix_engine_toolkit_core::functions::derive::OlympiaNetwork;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone)]
pub enum SerializableOlympiaNetwork {
    Mainnet,
    Stokenet,
    Releasenet,
    RCNet,
    Milestonenet,
    Devopsnet,
    Sandpitnet,
    Localnet,
}

impl From<OlympiaNetwork> for SerializableOlympiaNetwork {
    fn from(value: OlympiaNetwork) -> Self {
        match value {
            OlympiaNetwork::Mainnet => Self::Mainnet,
            OlympiaNetwork::Stokenet => Self::Stokenet,
            OlympiaNetwork::Releasenet => Self::Releasenet,
            OlympiaNetwork::RCNet => Self::RCNet,
            OlympiaNetwork::Milestonenet => Self::Milestonenet,
            OlympiaNetwork::Devopsnet => Self::Devopsnet,
            OlympiaNetwork::Sandpitnet => Self::Sandpitnet,
            OlympiaNetwork::Localnet => Self::Localnet,
        }
    }
}

impl From<SerializableOlympiaNetwork> for OlympiaNetwork {
    fn from(value: SerializableOlympiaNetwork) -> Self {
        match value {
            SerializableOlympiaNetwork::Mainnet => Self::Mainnet,
            SerializableOlympiaNetwork::Stokenet => Self::Stokenet,
            SerializableOlympiaNetwork::Releasenet => Self::Releasenet,
            SerializableOlympiaNetwork::RCNet => Self::RCNet,
            SerializableOlympiaNetwork::Milestonenet => Self::Milestonenet,
            SerializableOlympiaNetwork::Devopsnet => Self::Devopsnet,
            SerializableOlympiaNetwork::Sandpitnet => Self::Sandpitnet,
            SerializableOlympiaNetwork::Localnet => Self::Localnet,
        }
    }
}
