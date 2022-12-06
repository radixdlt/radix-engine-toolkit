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

use radix_engine_toolkit_core::{model::Bech32Coder, utils::network_id_from_address_string};
use scrypto::prelude::{
    ComponentAddress, PackageAddress, ResourceAddress, SystemAddress, ACCOUNT_PACKAGE, CLOCK,
    ECDSA_SECP256K1_TOKEN, EDDSA_ED25519_TOKEN, EPOCH_MANAGER, FAUCET_COMPONENT, FAUCET_PACKAGE,
    RADIX_TOKEN, SYSTEM_TOKEN,
};

#[test]
pub fn network_id_can_be_derived_from_address_string_for_all_networks() {
    // Arrange
    let addresses = vec![
        EntityAddress::ComponentAddress(FAUCET_COMPONENT),
        EntityAddress::SystemAddress(EPOCH_MANAGER),
        EntityAddress::SystemAddress(CLOCK),
        EntityAddress::PackageAddress(ACCOUNT_PACKAGE),
        EntityAddress::PackageAddress(FAUCET_PACKAGE),
        EntityAddress::ResourceAddress(RADIX_TOKEN),
        EntityAddress::ResourceAddress(SYSTEM_TOKEN),
        EntityAddress::ResourceAddress(EDDSA_ED25519_TOKEN),
        EntityAddress::ResourceAddress(ECDSA_SECP256K1_TOKEN),
    ];
    let networks = 0u8..0xFFu8;

    for network in networks {
        for address in addresses.clone() {
            let network = network + 1;
            let address_string = address.to_string(network);

            // Act
            let derived_network_id = network_id_from_address_string(&address_string).unwrap();

            // Assert
            assert_eq!(
                network, derived_network_id,
                "Derivation of network id from address {} produced network {} but we expected network {}",
                address_string, derived_network_id, network
            );
        }
    }
}

#[derive(Debug, Clone)]
pub enum EntityAddress {
    ComponentAddress(ComponentAddress),
    ResourceAddress(ResourceAddress),
    PackageAddress(PackageAddress),
    SystemAddress(SystemAddress),
}

impl EntityAddress {
    fn to_string(&self, network: u8) -> String {
        let bech32_coder = Bech32Coder::new(network);
        match self {
            Self::ComponentAddress(address) => bech32_coder
                .encoder
                .encode_component_address_to_string(address),
            Self::ResourceAddress(address) => bech32_coder
                .encoder
                .encode_resource_address_to_string(address),
            Self::PackageAddress(address) => bech32_coder
                .encoder
                .encode_package_address_to_string(address),
            Self::SystemAddress(address) => bech32_coder
                .encoder
                .encode_system_address_to_string(address),
        }
    }
}

impl From<ComponentAddress> for EntityAddress {
    fn from(address: ComponentAddress) -> Self {
        Self::ComponentAddress(address)
    }
}

impl From<ResourceAddress> for EntityAddress {
    fn from(address: ResourceAddress) -> Self {
        Self::ResourceAddress(address)
    }
}

impl From<PackageAddress> for EntityAddress {
    fn from(address: PackageAddress) -> Self {
        Self::PackageAddress(address)
    }
}

impl From<SystemAddress> for EntityAddress {
    fn from(address: SystemAddress) -> Self {
        Self::SystemAddress(address)
    }
}
