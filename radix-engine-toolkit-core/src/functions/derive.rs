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

use bech32::{FromBase32, ToBase32};
use scrypto::prelude::*;

use crate::utils;

pub fn virtual_account_address_from_public_key<P>(public_key: &P) -> ComponentAddress
where
    P: Into<PublicKey> + Clone,
{
    ComponentAddress::virtual_account_from_public_key(public_key)
}

pub fn virtual_identity_address_from_public_key<P>(public_key: &P) -> ComponentAddress
where
    P: Into<PublicKey> + Clone,
{
    ComponentAddress::virtual_identity_from_public_key(public_key)
}

pub fn virtual_signature_non_fungible_global_id_from_public_key<P>(
    public_key: &P,
) -> NonFungibleGlobalId
where
    P: HasPublicKeyHash,
{
    NonFungibleGlobalId::from_public_key(public_key)
}

pub fn virtual_account_address_from_olympia_account_address<S>(
    olympia_account_address: S,
) -> Result<ComponentAddress, DerivationError>
where
    S: AsRef<str>,
{
    public_key_from_olympia_account_address(olympia_account_address)
        .map(|public_key| virtual_account_address_from_public_key(&public_key))
}

pub fn resource_address_from_olympia_resource_address<S>(
    olympia_resource_address: S,
) -> Result<ResourceAddress, DerivationError>
where
    S: AsRef<str>,
{
    let olympia_resource_address = olympia_resource_address.as_ref();
    let (_, data, variant) =
        bech32::decode(olympia_resource_address).map_err(DerivationError::Bech32DecodeError)?;
    if let bech32::Variant::Bech32 = variant {
        Ok(())
    } else {
        Err(DerivationError::InvalidOlympiaBech32Variant {
            expected: bech32::Variant::Bech32,
            actual: bech32::Variant::Bech32m,
        })
    }?;

    // Convert from 5 bits to 8 bits.
    let data = Vec::<u8>::from_base32(&data).map_err(DerivationError::Bech32BaseConversionError)?;

    // Check the length of the data to ensure that it's valid.
    let prefix = data.first();
    let length = data.len();

    match (prefix, length) {
        (Some(0x01), 1) => Ok(scrypto::prelude::XRD),
        (Some(0x03), 27) => {
            let hash = scrypto::prelude::hash(&data);

            let mut bytes = [0u8; 30];
            bytes[0] = EntityType::GlobalFungibleResourceManager as u8;
            bytes[1..].copy_from_slice(&hash.0[..29]);

            Ok(ResourceAddress::new_or_panic(bytes))
        }
        _ => Err(DerivationError::InvalidOlympiaAddressLength {
            expected: 27,
            actual: length,
        }),
    }
}

pub fn public_key_from_olympia_account_address<S>(
    olympia_account_address: S,
) -> Result<Secp256k1PublicKey, DerivationError>
where
    S: AsRef<str>,
{
    let olympia_account_address = olympia_account_address.as_ref();

    // Ensure that the second and third characters in the string are d and x which are present in
    // all account HRPs in Olympia regardless of the network.
    match (
        olympia_account_address.chars().nth(1),
        olympia_account_address.chars().nth(2),
    ) {
        (Some('d'), Some('x')) => Ok(()),
        (Some(char1), Some(char2)) => Err(
            DerivationError::InvalidCharsInOlympiaAddressEntitySpecifier {
                expected: ('d', 'x'),
                actual: (char1, char2),
            },
        ),
        _ => Err(DerivationError::InvalidOlympiaAddressLength {
            expected: 65,
            actual: olympia_account_address.len(),
        }),
    }?;

    let (_, data, variant) =
        bech32::decode(olympia_account_address).map_err(DerivationError::Bech32DecodeError)?;
    if let bech32::Variant::Bech32 = variant {
        Ok(())
    } else {
        Err(DerivationError::InvalidOlympiaBech32Variant {
            expected: bech32::Variant::Bech32,
            actual: bech32::Variant::Bech32m,
        })
    }?;

    let mut data =
        Vec::<u8>::from_base32(&data).map_err(DerivationError::Bech32BaseConversionError)?;

    // Check the length of the data to ensure that it's a public key. Length should be 1 + 33
    // where the added 1 byte is because of the 0x04 prefix that public keys have.
    const EXPECTED_LENGTH: usize = 34;
    match data.first() {
        Some(0x04) => {
            data.remove(0);
            data.try_into().map(Secp256k1PublicKey).map_err(|data| {
                DerivationError::InvalidOlympiaAddressLength {
                    expected: EXPECTED_LENGTH,
                    actual: data.len(),
                }
            })
        }
        Some(prefix) => Err(DerivationError::InvalidOlympiaAddressPrefix {
            expected: 0x04,
            actual: *prefix,
        }),
        None => Err(DerivationError::InvalidOlympiaAddressLength {
            expected: EXPECTED_LENGTH,
            actual: data.len(),
        }),
    }
}

pub fn olympia_account_address_from_public_key(
    public_key: &Secp256k1PublicKey,
    olympia_network: OlympiaNetwork,
) -> String {
    let public_key = {
        let mut vector = vec![0x04];
        vector.extend(public_key.0);
        vector
    };
    bech32::encode(
        olympia_network.hrp(),
        public_key.to_base32(),
        bech32::Variant::Bech32,
    )
    .expect("Should not panic since all data is trusted.")
}

pub fn node_address_from_public_key(public_key: &Secp256k1PublicKey, network_id: u8) -> String {
    let hrp = {
        let network_identifier = utils::network_definition_from_network_id(network_id).hrp_suffix;
        format!("node_{network_identifier}")
    };

    bech32::encode(&hrp, public_key.0.to_base32(), bech32::Variant::Bech32m)
        .expect("Should not panic since all data is trusted.")
}

pub enum OlympiaNetwork {
    Mainnet,
    Stokenet,
    Releasenet,
    RCNet,
    Milestonenet,
    Devopsnet,
    Sandpitnet,
    Localnet,
}

impl OlympiaNetwork {
    pub const fn hrp(&self) -> &str {
        match self {
            Self::Mainnet => "rdx",
            Self::Stokenet => "tdx",
            Self::Releasenet => "tdx3",
            Self::RCNet => "tdx4",
            Self::Milestonenet => "tdx5",
            Self::Devopsnet => "tdx6",
            Self::Sandpitnet => "tdx7",
            Self::Localnet => "ddx",
        }
    }
}

#[derive(Debug)]
pub enum DerivationError {
    InvalidCharsInOlympiaAddressEntitySpecifier {
        expected: (char, char),
        actual: (char, char),
    },
    InvalidOlympiaAddressLength {
        expected: usize,
        actual: usize,
    },
    InvalidOlympiaBech32Variant {
        expected: bech32::Variant,
        actual: bech32::Variant,
    },
    InvalidOlympiaAddressPrefix {
        expected: u8,
        actual: u8,
    },
    Bech32DecodeError(bech32::Error),
    Bech32BaseConversionError(bech32::Error),
}
