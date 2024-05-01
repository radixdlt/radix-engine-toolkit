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

use std::fmt::Display;

use radix_common::prelude::*;

/// A representation of a serializable NodeId made up of a node id and a
/// network. The network here is:
/// * The network that will be used during serialization to Bech32m encode the
///   node id.
/// * The network that was encountered while deserializing into this type and
///   was used to bech32m
/// decode the address.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SerializableNodeId(pub NodeId, pub u8);

impl Display for SerializableNodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let network_definition =
            crate::utils::network_definition_from_network_id(self.1);
        let bech32_encoder = AddressBech32Encoder::new(&network_definition);
        bech32_encoder
            .encode_to_fmt(f, &self.0 .0)
            .map_err(|_| std::fmt::Error)
    }
}

impl FromStr for SerializableNodeId {
    type Err = ParseSerializableNodeIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let network_id = crate::utils::network_id_from_address_string(s)
            .ok_or(
            ParseSerializableNodeIdError::FailedToExtractNetworkIfFromAddress(
                s.to_owned(),
            ),
        )?;
        let network_definition =
            crate::utils::network_definition_from_network_id(network_id);
        let bech32_decoder = AddressBech32Decoder::new(&network_definition);

        let (_, bytes) = bech32_decoder
            .validate_and_decode(s)
            .map_err(ParseSerializableNodeIdError::FailedToBech32Decode)?;
        let node_id = bytes
            .try_into()
            .map(NodeId)
            .map_err(|_| ParseSerializableNodeIdError::InvalidLength)?;

        Ok(Self(node_id, network_id))
    }
}

#[derive(Debug, Clone)]
pub enum ParseSerializableNodeIdError {
    FailedToExtractNetworkIfFromAddress(String),
    FailedToBech32Decode(AddressBech32DecodeError),
    InvalidLength,
}

impl Display for ParseSerializableNodeIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self, f)
    }
}
