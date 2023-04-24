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

use super::utils::*;
use super::AddressError;
use crate::utils::checked_copy_u8_slice;

use scrypto::address::{Bech32Decoder, Bech32Encoder};
use scrypto::network::NetworkDefinition;
use scrypto::prelude::NodeId;

/// A Bech32m encoder and decoder used in the Radix Engine Toolkit for all of it's address encoding
/// and decoding needs
pub struct Bech32Coder {
    network_definition: NetworkDefinition,
    encoder: Bech32Encoder,
    decoder: Bech32Decoder,
}

impl Bech32Coder {
    pub fn new(network_id: u8) -> Self {
        let network_definition = network_definition_from_network_id(network_id);
        Self::new_with_network_definition(network_definition)
    }

    pub fn new_with_network_definition(network_definition: NetworkDefinition) -> Self {
        Self {
            encoder: Bech32Encoder::new(&network_definition),
            decoder: Bech32Decoder::new(&network_definition),
            network_definition,
        }
    }

    pub fn encoder(&self) -> &Bech32Encoder {
        &self.encoder
    }

    pub fn decoder(&self) -> &Bech32Decoder {
        &self.decoder
    }

    pub fn network_definition(&self) -> &NetworkDefinition {
        &self.network_definition
    }

    pub fn network_id(&self) -> u8 {
        self.network_definition.id
    }

    pub fn new_from_hrp<S: AsRef<str>>(hrp: S) -> Result<Self, AddressError> {
        network_id_from_hrp(hrp).map(Self::new)
    }

    pub fn new_from_address<S: AsRef<str>>(address: S) -> Result<Self, AddressError> {
        network_id_from_address_string(address).map(Self::new)
    }

    pub fn encode<T: Into<NodeId>>(&self, node_id: T) -> Result<String, AddressError> {
        self.encoder
            .encode(node_id.into().0.as_ref())
            .map_err(AddressError::from)
    }

    pub fn decode<S: AsRef<str>>(&self, string: S) -> Result<NodeId, AddressError> {
        let (_, full_data) = self.decoder.validate_and_decode(string.as_ref())?;
        checked_copy_u8_slice(&full_data).map(NodeId).map_or(
            Err(AddressError::InvalidDataLength {
                expected: NodeId::LENGTH,
                actual: full_data.len(),
            }),
            Ok,
        )
    }
}
