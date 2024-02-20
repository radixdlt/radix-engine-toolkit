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

use crate::utils::*;
use paste::*;
use radix_engine_common::prelude::*;
use scrypto::prelude::*;
use serde_with::*;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CanonicalAddressError {
    Bech32mEncodeError(AddressBech32EncodeError),
    Bech32mDecodeError(AddressBech32DecodeError),
    InvalidEntityType {
        expected: &'static [EntityType],
        actual: EntityType,
        node_id: NodeId,
    },
    NoEntityType {
        node_id: NodeId,
    },
    FailedToFindNetworkIdFromBech32mString {
        bech32m_encoded_address: String,
    },
    InvalidNodeIdLength {
        expected: usize,
        actual: usize,
    },
}

impl From<AddressBech32EncodeError> for CanonicalAddressError {
    fn from(value: AddressBech32EncodeError) -> Self {
        Self::Bech32mEncodeError(value)
    }
}

impl From<AddressBech32DecodeError> for CanonicalAddressError {
    fn from(value: AddressBech32DecodeError) -> Self {
        Self::Bech32mDecodeError(value)
    }
}

// Required for serde Serializer
impl Display for CanonicalAddressError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{:?}", self)
    }
}

pub type NetworkId = u8;

pub trait CanonicalAddress: FromStr + Display {
    /// A constant of the entity types that we are allowed to have on this
    /// canonical address. This is a static array to support addresses which
    /// can have multiple entity types such as the account address with virtual
    /// and non-virtual addresses.
    const ALLOWED_ENTITY_TYPES: &'static [EntityType];

    fn node_id(&self) -> NodeId;
    fn entity_type(&self) -> EntityType;
    fn network_id(&self) -> NetworkId;
}

// Macro which declares dedicated Canonical Address types.
// Arguments:
//  - $name: used for composition of the new type name Canonical_NAME_Address
//  - $entity_type: pattern of valid entity types for this new type
macro_rules! define_canonical_addresses {
    (
        $(
            $name: ident => [$($entity_type: expr),* $(,)?]
        ),* $(,)?
    ) => {
        paste! {
            $(
                #[derive(
                    Clone,
                    Copy,
                    Debug,
                    PartialEq,
                    Eq,
                    Hash,
                    SerializeDisplay,
                    DeserializeFromStr,
                )]
                pub struct [<Canonical $name Address>] {
                    /// The NodeId of the address.
                    node_id: NodeId,
                    /// The network that the address is to be used for. This is
                    /// used in the Bech32m encoding and decoding of the address
                    /// essentially providing us with the network context.
                    network_id: NetworkId,
                    /// The entity type of the address. This is checked in the
                    /// constructor that it is one of the allowed entity types
                    /// for this particular address type and then cached here
                    /// to avoid any additional unwraps when being retrieved.
                    entity_type: EntityType
                }

                impl [<Canonical $name Address>] {
                    pub fn new(
                        node_id: impl Into<NodeId>,
                        network_id: NetworkId
                    ) -> Result<Self, CanonicalAddressError> {
                        let node_id = node_id.into();
                        if let Some(entity_type) = node_id.entity_type() {
                            if matches!(
                                entity_type,
                                $($entity_type)|*
                            ) {
                                Ok(Self {
                                    node_id,
                                    network_id,
                                    entity_type
                                })
                            } else {
                                Err(CanonicalAddressError::InvalidEntityType {
                                    expected: Self::ALLOWED_ENTITY_TYPES,
                                    actual: entity_type,
                                    node_id
                                })
                            }
                        } else {
                            Err(CanonicalAddressError::NoEntityType { node_id })
                        }
                    }

                    /// Attempts to construct this type from a Bech32m string.
                    /// The network id does not need to be passed as it will be
                    /// determined based on the HRP of the address
                    pub fn try_from_bech32(
                        address_string: &str,
                    ) -> Result<Self, CanonicalAddressError> {
                        // Find the network definition based on the network of
                        // the passed address.
                        let unrecognized_network_err = CanonicalAddressError::FailedToFindNetworkIdFromBech32mString {
                            bech32m_encoded_address: address_string.to_owned(),
                        };
                        let network_definition = network_id_from_address_string(address_string)
                            .ok_or(unrecognized_network_err.clone())
                            .map(network_definition_from_network_id_strict)
                            .and_then(|o: Option<NetworkDefinition>| o.ok_or(unrecognized_network_err))?;

                        // Construct the decoder and decode the address
                        let decoder = AddressBech32Decoder::new(&network_definition);
                        let (_, data) = decoder.validate_and_decode(address_string)?;

                        // Construct a NodeId from the returned data.
                        let node_id = data.try_into().map(NodeId).map_err(|vec| {
                            CanonicalAddressError::InvalidNodeIdLength {
                                expected: NodeId::LENGTH,
                                actual: vec.len(),
                            }
                        })?;

                        Self::new(node_id, network_definition.id)
                    }
                }

                impl From<[<Canonical $name Address>]> for NodeId {
                    fn from(value: [<Canonical $name Address>]) -> Self {
                        value.node_id
                    }
                }

                impl CanonicalAddress for [<Canonical $name Address>] {
                    const ALLOWED_ENTITY_TYPES: &'static [EntityType] = &[
                        $($entity_type),*
                    ];

                    fn node_id(&self) -> NodeId {
                        self.node_id
                    }

                    fn entity_type(&self) -> EntityType {
                        self.entity_type
                    }

                    fn network_id(&self) -> NetworkId {
                        self.network_id
                    }
                }

                impl FromStr for [<Canonical $name Address>] {
                    type Err = CanonicalAddressError;

                    fn from_str(bech32: &str) -> Result<Self, Self::Err> {
                        Self::try_from_bech32(bech32)
                    }
                }

                impl Display for [<Canonical $name Address>] {
                    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        let encoder = AddressBech32Encoder::new(
                            &network_definition_from_network_id(self.network_id),
                        );
                        encoder
                            .encode_to_fmt(formatter, &self.node_id.0)
                            .map_err(|_| fmt::Error)
                    }
                }
            )*
        }
    };
}

define_canonical_addresses!(
    // CanonicalAccountAddress type definition
    Account => [
        EntityType::GlobalAccount,
        EntityType::GlobalVirtualSecp256k1Account,
        EntityType::GlobalVirtualEd25519Account
    ],
    // CanonicalIdentityAddress type definition
    Identity => [
        EntityType::GlobalIdentity,
        EntityType::GlobalVirtualSecp256k1Identity,
        EntityType::GlobalVirtualEd25519Identity
    ],
    // CanonicalResourceAddress type definition
    Resource => [
        EntityType::GlobalFungibleResourceManager,
        EntityType::GlobalNonFungibleResourceManager
    ],
    // CanonicalPackageAddress type definition
    Package => [EntityType::GlobalPackage],
    // CanonicalComponentAddress type definition
    Component => [
        EntityType::GlobalGenericComponent,
        EntityType::InternalGenericComponent
    ],
    // CanonicalAccessControllerAddress type definition
    AccessController => [EntityType::GlobalAccessController],
    // CanonicalValidatorAddress type definition
    Validator => [EntityType::GlobalValidator],
    // CanonicalVaultAddress type definition
    Vault => [
        EntityType::InternalFungibleVault,
        EntityType::InternalNonFungibleVault
    ],
    // CanonicalPoolAddress type definition
    Pool =>  [
        EntityType::GlobalOneResourcePool,
        EntityType::GlobalTwoResourcePool,
        EntityType::GlobalMultiResourcePool
    ]
);

// Additional implementations
impl CanonicalResourceAddress {
    pub fn is_fungible(&self) -> bool {
        self.node_id.is_global_fungible_resource_manager()
    }

    pub fn is_non_fungible(&self) -> bool {
        !self.is_fungible()
    }
}

impl CanonicalVaultAddress {
    pub fn is_fungible(&self) -> bool {
        self.node_id.is_internal_fungible_vault()
    }

    pub fn is_non_fungible(&self) -> bool {
        !self.is_fungible()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_id_results_in() {
        // Arrange
        let address_unknown_id = CanonicalAccountAddress::new(
            NodeId::new(
                EntityType::GlobalVirtualEd25519Account as u8,
                &[0xff; 29],
            ),
            222, // unknown network id
        )
        .unwrap();

        let bech32 = address_unknown_id.to_string();

        assert_eq!(
            CanonicalAccountAddress::try_from_bech32(&bech32),
            Err(
                CanonicalAddressError::FailedToFindNetworkIdFromBech32mString {
                    bech32m_encoded_address: bech32
                }
            )
        );
    }

    #[test]
    fn test_network_id_from_zabanet_address() {
        let s = "account_tdx_e_128vkt2fur65p4hqhulfv3h0cknrppwtjsstlttkfamj4jnnpm82gsw";
        assert_eq!(
            CanonicalAccountAddress::from_str(s).unwrap().network_id(),
            0xe // zabanet
        );
    }

    #[test]
    fn canonical_account_address_test() {
        let input = "account_sim1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cve475w0q";

        let x = CanonicalAccountAddress::from_str(input).unwrap();
        assert_eq!(
            x.node_id.as_bytes(),
            [
                193, 24, 131, 70, 47, 57, 121, 109, 168, 63, 47, 130, 202, 239,
                166, 121, 170, 241, 241, 137, 37, 126, 189, 60, 140, 39, 125,
                90, 225, 153
            ]
        );
        assert_eq!(x.network_id, 0xf2);
        assert_eq!(x.to_string(), input);

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

        let x = CanonicalResourceAddress::new(
            NodeId::try_from_hex(input).unwrap(),
            0x0a,
        )
        .unwrap();

        let input_vec: Vec<u8> = (0..input.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&input[i..=i + 1], 16).unwrap())
            .collect();

        assert_eq!(x.node_id.as_bytes(), input_vec);
        assert_eq!(x.network_id, 0x0a);
        assert_eq!(x.to_string(), canonical_input);
        assert!(!x.is_fungible());

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

        let x = CanonicalResourceAddress::new(
            NodeId::try_from_hex(input).unwrap(),
            0x01,
        )
        .unwrap();
        assert_eq!(x.to_string(), canonical_input);
        assert!(x.is_fungible());
    }

    #[test]
    fn canonical_non_fungible_vault_address_test() {
        let input =
            "9818740463586485efcbd5fcff22cc4fd0f401f44836cb53235fc42f9623";
        let canonical_input = "internal_vault_rdx1nqv8gprrtpjgtm7t6h707gkvflg0gq05fqmvk5ertlzzl93rrmns58";

        let x = CanonicalVaultAddress::new(
            NodeId::try_from_hex(input).unwrap(),
            0x01,
        )
        .unwrap();
        assert_eq!(x.to_string(), canonical_input);
        assert!(!x.is_fungible());
    }

    #[test]
    fn canonical_fungible_vault_address_test() {
        let input =
            "58619833de031de3aad69cad02a22656e083e307fb617b28e1b275bd7ed7";
        let canonical_input = "internal_vault_rdx1tpsesv77qvw782kknjks9g3x2msg8cc8ldshk28pkf6m6lkhzcw3fr";

        let x = CanonicalVaultAddress::new(
            NodeId::try_from_hex(input).unwrap(),
            0x01,
        )
        .unwrap();
        assert_eq!(x.to_string(), canonical_input);
        assert!(x.is_fungible());
    }
}
