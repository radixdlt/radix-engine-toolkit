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

use crate::address::network_aware_address::*;
use crate::error::Result;
use crate::utils::{
    network_definition_from_network_id, network_id_from_address_string, network_id_from_hrp,
};
use crate::Error;
use scrypto::prelude::{ComponentAddress, PackageAddress, ResourceAddress, SystemAddress};
use scrypto::radix_engine_interface::address::{Bech32Decoder, Bech32Encoder};
use scrypto::radix_engine_interface::node::NetworkDefinition;

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

    pub fn new_from_hrp<S: AsRef<str>>(hrp: S) -> Result<Self> {
        network_id_from_hrp(hrp).map(Self::new)
    }

    pub fn new_from_address<S: AsRef<str>>(address: S) -> Result<Self> {
        network_id_from_address_string(address).map(Self::new)
    }

    pub fn encode_component_address(&self, component_address: &ComponentAddress) -> String {
        self.encoder
            .encode_component_address_to_string(component_address)
    }

    pub fn encode_resource_address(&self, resource_address: &ResourceAddress) -> String {
        self.encoder
            .encode_resource_address_to_string(resource_address)
    }

    pub fn encode_system_address(&self, system_address: &SystemAddress) -> String {
        self.encoder.encode_system_address_to_string(system_address)
    }

    pub fn encode_package_address(&self, package_address: &PackageAddress) -> String {
        self.encoder
            .encode_package_address_to_string(package_address)
    }

    pub fn decode_component_address<S: AsRef<str>>(
        &self,
        component_address: S,
    ) -> Result<ComponentAddress> {
        self.decoder
            .validate_and_decode_component_address(component_address.as_ref())
            .map_err(Error::from)
    }

    pub fn decode_resource_address<S: AsRef<str>>(
        &self,
        resource_address: S,
    ) -> Result<ResourceAddress> {
        self.decoder
            .validate_and_decode_resource_address(resource_address.as_ref())
            .map_err(Error::from)
    }

    pub fn decode_system_address<S: AsRef<str>>(&self, system_address: S) -> Result<SystemAddress> {
        self.decoder
            .validate_and_decode_system_address(system_address.as_ref())
            .map_err(Error::from)
    }

    pub fn decode_package_address<S: AsRef<str>>(
        &self,
        package_address: S,
    ) -> Result<PackageAddress> {
        self.decoder
            .validate_and_decode_package_address(package_address.as_ref())
            .map_err(Error::from)
    }

    pub fn decode_to_network_aware_component_address<S: AsRef<str>>(
        &self,
        component_address: S,
    ) -> Result<NetworkAwareComponentAddress> {
        self.decode_component_address(component_address)
            .map(|component_address| NetworkAwareComponentAddress {
                network_id: self.network_id(),
                address: component_address,
            })
    }

    pub fn decode_to_network_aware_resource_address<S: AsRef<str>>(
        &self,
        resource_address: S,
    ) -> Result<NetworkAwareResourceAddress> {
        self.decode_resource_address(resource_address)
            .map(|resource_address| NetworkAwareResourceAddress {
                network_id: self.network_id(),
                address: resource_address,
            })
    }

    pub fn decode_to_network_aware_system_address<S: AsRef<str>>(
        &self,
        system_address: S,
    ) -> Result<NetworkAwareSystemAddress> {
        self.decode_system_address(system_address)
            .map(|system_address| NetworkAwareSystemAddress {
                network_id: self.network_id(),
                address: system_address,
            })
    }

    pub fn decode_to_network_aware_package_address<S: AsRef<str>>(
        &self,
        package_address: S,
    ) -> Result<NetworkAwarePackageAddress> {
        self.decode_package_address(package_address)
            .map(|package_address| NetworkAwarePackageAddress {
                network_id: self.network_id(),
                address: package_address,
            })
    }
}