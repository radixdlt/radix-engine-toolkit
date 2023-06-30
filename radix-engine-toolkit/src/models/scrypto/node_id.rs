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

use radix_engine_common::prelude::{Bech32Decoder, Bech32Encoder, DecodeBech32AddressError};
use schemars::JsonSchema;
use scrypto::prelude::*;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(
    Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[serde(transparent)]
#[schemars(transparent)]
pub struct SerializableNodeId(
    #[schemars(with = "String")]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub SerializableNodeIdInternal,
);

impl SerializableNodeId {
    pub fn new(node_id: NodeId, network_id: u8) -> Self {
        Self(SerializableNodeIdInternal {
            node_id,
            network_id,
        })
    }

    pub fn from_global_address<G: Into<GlobalAddress>>(address: G, network_id: u8) -> Self {
        let global_address: GlobalAddress = address.into();
        let node_id = global_address.as_node_id();
        Self::new(*node_id, network_id)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SerializableNodeIdInternal {
    pub node_id: NodeId,
    pub network_id: u8,
}

impl Display for SerializableNodeIdInternal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let network_definition =
            radix_engine_toolkit_core::utils::network_definition_from_network_id(self.network_id);
        let bech32_encoder = Bech32Encoder::new(&network_definition);
        let string = bech32_encoder
            .encode(&self.node_id.0)
            .map_err(|_| fmt::Error)?;
        write!(f, "{}", string)
    }
}

impl FromStr for SerializableNodeIdInternal {
    type Err = SerializableNodeIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let network_id = radix_engine_toolkit_core::utils::network_id_from_address_string(s)
            .map_or(
                Err(SerializableNodeIdError::FailedToParseStringAsAddress(
                    s.to_owned(),
                )),
                Ok,
            )?;

        let network_definition =
            radix_engine_toolkit_core::utils::network_definition_from_network_id(network_id);
        let bech32_decoder = Bech32Decoder::new(&network_definition);
        let (_, data) = bech32_decoder.validate_and_decode(s)?;

        data.try_into()
            .map_err(|_| SerializableNodeIdError::InvalidAddressLength)
            .map(|node_id| Self {
                network_id,
                node_id: NodeId(node_id),
            })
    }
}

impl AsRef<NodeId> for SerializableNodeIdInternal {
    fn as_ref(&self) -> &NodeId {
        &self.node_id
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(tag = "kind", content = "error")]
pub enum SerializableNodeIdError {
    FailedToParseStringAsAddress(String),
    Bech32DecodingError(String),
    InvalidAddressLength,
}

impl From<DecodeBech32AddressError> for SerializableNodeIdError {
    fn from(value: DecodeBech32AddressError) -> Self {
        Self::Bech32DecodingError(format!("{:?}", value))
    }
}

impl Display for SerializableNodeIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self, f)
    }
}
