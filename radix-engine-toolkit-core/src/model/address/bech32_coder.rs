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

use scrypto::radix_engine_interface::address::{Bech32Decoder, Bech32Encoder};
use scrypto::radix_engine_interface::core::NetworkDefinition;

use crate::error::Error;
use crate::utils::{
    network_definition_from_network_id, network_id_from_address_string, network_id_from_hrp,
};

pub struct Bech32Coder {
    pub network_definition: NetworkDefinition,
    pub encoder: Bech32Encoder,
    pub decoder: Bech32Decoder,
}

impl Bech32Coder {
    pub fn new(network_id: u8) -> Self {
        let network_definition = network_definition_from_network_id(network_id);
        Self {
            network_definition: network_definition.clone(),
            encoder: Bech32Encoder::new(&network_definition),
            decoder: Bech32Decoder::new(&network_definition),
        }
    }

    pub fn network_id(&self) -> u8 {
        self.network_definition.id
    }

    pub fn new_from_hrp(hrp: &str) -> Result<Self, Error> {
        Ok(Self::new(network_id_from_hrp(hrp)?))
    }

    pub fn new_from_address(address: &str) -> Result<Self, Error> {
        Ok(Self::new(network_id_from_address_string(address)?))
    }
}

impl AsRef<Bech32Coder> for Bech32Coder {
    fn as_ref(&self) -> &Bech32Coder {
        self
    }
}
