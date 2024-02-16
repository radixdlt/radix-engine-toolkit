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

use crate::utils::{
    network_definition_from_network_id, network_id_from_address_string,
};
use paste::paste;
use radix_engine_common::prelude::*;
use scrypto::prelude::AddressBech32Decoder;
use serde_with::{DeserializeFromStr, SerializeDisplay};

#[derive(Debug)]
pub enum CanonicalAddressError {
    FailedToDecodeBech32,
    FailedToEncodeBech32,
}

pub type NetworkId = u8;

pub trait CanonicalAddress: FromStr + std::fmt::Display {
    fn entity_type(&self) -> EntityType;
    fn network_id(&self) -> NetworkId;
    fn to_bech32(&self) -> Result<String, CanonicalAddressError>;
}
pub trait CanonicalAddressResourceType {
    fn is_fungible(&self) -> bool;
}

impl Display for CanonicalAddressError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{:?}", self)
    }
}

macro_rules! make_canonical_address {

    ($name: ident, $entity_type: pat) => {
        paste! {
            #[derive(
                Clone, Debug, PartialEq, Eq, Hash, SerializeDisplay, DeserializeFromStr,
            )]
            pub struct [<Canonical $name Address>] {
                address: NodeId,
                network_id: NetworkId,
            }

            impl [<Canonical $name Address>] {
                pub fn try_from_global_address(
                    global_address: &GlobalAddress,
                    network_id: NetworkId,
                ) -> Option<Self> {
                    Self::try_from_node_id(&global_address.into_node_id(), network_id)
                }

                pub fn try_from_internal_address(
                    internal_address: &InternalAddress,
                    network_id: NetworkId,
                ) -> Option<Self> {
                    Self::try_from_node_id(&internal_address.into_node_id(), network_id)
                }

                pub fn try_from_node_id(
                    node_id: &NodeId,
                    network_id: NetworkId,
                ) -> Option<Self> {
                    if let Some(entity_type) = node_id.entity_type() {
                        if Self::is_entity_type_valid(entity_type) {
                            Some(Self {
                                address: node_id.clone(),
                                network_id,
                            })
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }

                pub fn try_from_bech32(bech32: &str) -> Option<Self> {
                    let network_id = network_id_from_address_string(bech32)?;

                    let decoder = AddressBech32Decoder::new(
                        &network_definition_from_network_id(network_id),
                    );
                    if let Ok((entity_type, mut full_data)) =
                        decoder.validate_and_decode(bech32)
                    {
                        full_data.remove(0); // skip entity type
                        if let Ok(node_id) = full_data.as_slice().try_into() {
                            Self::try_from_node_id(
                                &NodeId::new(entity_type as u8, node_id),
                                network_id,
                            )
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }

                fn is_entity_type_valid(entity_type: EntityType) -> bool {
                    matches!(
                        entity_type,
                        $entity_type
                    )
                }
            }

            impl CanonicalAddress for [<Canonical $name Address>] {
                fn entity_type(&self) -> EntityType {
                    // Safe to unwrap as entity type is validated during this objcet creation.
                    self.address.entity_type().unwrap()
                }

                fn network_id(&self) -> NetworkId {
                    self.network_id
                }

                fn to_bech32(&self) -> Result<String, CanonicalAddressError> {
                    let encode = AddressBech32Encoder::new(
                        &network_definition_from_network_id(self.network_id),
                    );
                    encode
                        .encode(self.address.as_bytes())
                        .map_err(|_| CanonicalAddressError::FailedToEncodeBech32)
                }
            }

            impl FromStr for [<Canonical $name Address>] {
                type Err = CanonicalAddressError;

                fn from_str(bech32: &str) -> Result<Self, Self::Err> {
                    Self::try_from_bech32(bech32)
                        .ok_or(CanonicalAddressError::FailedToDecodeBech32)
                }
            }

            impl Display for [<Canonical $name Address>] {
                fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str(&self.to_bech32().map_err(|_| fmt::Error)?)
                }
            }
        }
    }

}

make_canonical_address!(
    Account,
    EntityType::GlobalAccount
        | EntityType::GlobalVirtualSecp256k1Account
        | EntityType::GlobalVirtualEd25519Account
);
make_canonical_address!(
    Identity,
    EntityType::GlobalIdentity
        | EntityType::GlobalVirtualSecp256k1Identity
        | EntityType::GlobalVirtualEd25519Identity
);
make_canonical_address!(Package, EntityType::GlobalPackage);
make_canonical_address!(
    Component,
    EntityType::GlobalGenericComponent | EntityType::InternalGenericComponent
);
make_canonical_address!(AccessController, EntityType::GlobalAccessController);
make_canonical_address!(Validator, EntityType::GlobalValidator);
make_canonical_address!(
    ResourcePool,
    EntityType::GlobalOneResourcePool
        | EntityType::GlobalTwoResourcePool
        | EntityType::GlobalMultiResourcePool
);
make_canonical_address!(
    Resource,
    EntityType::GlobalFungibleResourceManager
        | EntityType::GlobalNonFungibleResourceManager
);
make_canonical_address!(
    Vault,
    EntityType::InternalFungibleVault | EntityType::InternalNonFungibleVault
);

impl CanonicalAddressResourceType for CanonicalResourceAddress {
    fn is_fungible(&self) -> bool {
        self.address.is_global_fungible_resource_manager()
    }
}

impl CanonicalAddressResourceType for CanonicalVaultAddress {
    fn is_fungible(&self) -> bool {
        self.address.is_internal_fungible_vault()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate serde_json;

    #[test]
    fn canonical_account_address_test() {
        let input = "account_sim1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cve475w0q";

        let x = CanonicalAccountAddress::from_str(input).unwrap();
        assert_eq!(
            x.address.as_bytes(),
            [
                193, 24, 131, 70, 47, 57, 121, 109, 168, 63, 47, 130, 202, 239,
                166, 121, 170, 241, 241, 137, 37, 126, 189, 60, 140, 39, 125,
                90, 225, 153
            ]
        );
        assert_eq!(x.network_id, 0xf2);
        assert_eq!(x.to_string(), input);
        assert_eq!(x.to_bech32().unwrap(), input);

        let json_string = serde_json::to_string(&x).unwrap();
        assert_eq!(json_string, format!("\"{}\"", input));

        let y = serde_json::from_str::<CanonicalAccountAddress>(&json_string)
            .unwrap();
        assert_eq!(y, x);
    }

    #[test]
    fn canonical_non_fungible_resource_address_test() {
        let input =
            "9a5d92f6bc4a0a8b43abe39c0fba235e357fc89286111b78844c5b57e587";
        let canonical_input = "resource_tdx_a_1nfwe9a4ufg9gksatuwwqlw3rtc6hljyjscg3k7yyf3d40ev85cjrzp";

        let x = CanonicalResourceAddress::try_from_node_id(
            &NodeId::try_from_hex(input).unwrap(),
            0x0a,
        )
        .unwrap();

        let input_vec: Vec<u8> = (0..input.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&input[i..=i + 1], 16).unwrap())
            .collect();

        assert_eq!(x.address.as_bytes(), input_vec);
        assert_eq!(x.network_id, 0x0a);
        assert_eq!(x.to_string(), canonical_input);
        assert_eq!(x.to_bech32().unwrap(), canonical_input);
        assert_eq!(x.is_fungible(), false);

        let json_string = serde_json::to_string(&x).unwrap();
        assert_eq!(json_string, format!("\"{}\"", canonical_input));

        let y = serde_json::from_str::<CanonicalResourceAddress>(&json_string)
            .unwrap();
        assert_eq!(y, x);
    }

    #[test]
    fn canonical_fungible_resource_address_test() {
        let input =
            "5dbd2333630248b3e688c93892cec2d199bd917b8a4e019864a552e1f774";
        let canonical_input = "resource_rdx1tk7jxvmrqfyt8e5geyuf9nkz6xvmmytm3f8qrxry54fwram5cwvcw9";

        let x = CanonicalResourceAddress::try_from_node_id(
            &NodeId::try_from_hex(input).unwrap(),
            0x01,
        )
        .unwrap();
        assert_eq!(x.to_string(), canonical_input);
        assert_eq!(x.is_fungible(), true);
    }

    #[test]
    fn canonical_non_fungible_vault_address_test() {
        let input =
            "9818740463586485efcbd5fcff22cc4fd0f401f44836cb53235fc42f9623";
        let canonical_input = "internal_vault_rdx1nqv8gprrtpjgtm7t6h707gkvflg0gq05fqmvk5ertlzzl93rrmns58";

        let x = CanonicalVaultAddress::try_from_node_id(
            &NodeId::try_from_hex(input).unwrap(),
            0x01,
        )
        .unwrap();
        assert_eq!(x.to_string(), canonical_input);
        assert_eq!(x.is_fungible(), false);
    }

    #[test]
    fn canonical_fungible_vault_address_test() {
        let input =
            "58619833de031de3aad69cad02a22656e083e307fb617b28e1b275bd7ed7";
        let canonical_input = "internal_vault_rdx1tpsesv77qvw782kknjks9g3x2msg8cc8ldshk28pkf6m6lkhzcw3fr";

        let x = CanonicalVaultAddress::try_from_node_id(
            &NodeId::try_from_hex(input).unwrap(),
            0x01,
        )
        .unwrap();
        assert_eq!(x.to_string(), canonical_input);
        assert_eq!(x.is_fungible(), true);
    }
}
