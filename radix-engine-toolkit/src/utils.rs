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

use std::borrow::Borrow;

use crate::error::{Error, Result};
use bech32;
use scrypto::address::AddressError;
use scrypto::network::NetworkDefinition;
use scrypto::prelude::ComponentAddress;

/// A deterministic function that generates a network definition given a network ID. Implemented
/// with reference to https://github.com/radixdlt/babylon-node/tree/main/common/src/main/java/com/radixdlt/networks/Network.java#L72-L99
pub fn network_definition_from_network_id(network_id: u8) -> NetworkDefinition {
    match network_id {
        0x01 => NetworkDefinition::mainnet(),
        0x02 => NetworkDefinition {
            id: network_id,
            logical_name: "stokenet".into(),
            hrp_suffix: format!("tdx_{:x}_", network_id),
        },

        0x0A => NetworkDefinition {
            id: network_id,
            logical_name: "adapanet".into(),
            hrp_suffix: format!("tdx_{:x}_", network_id),
        },
        0x0B => NetworkDefinition {
            id: network_id,
            logical_name: "nebunet".into(),
            hrp_suffix: format!("tdx_{:x}_", network_id),
        },

        0x20 => NetworkDefinition {
            id: network_id,
            logical_name: "gilganet".into(),
            hrp_suffix: format!("tdx_{:x}_", network_id),
        },
        0x21 => NetworkDefinition {
            id: network_id,
            logical_name: "enkinet".into(),
            hrp_suffix: format!("tdx_{:x}_", network_id),
        },
        0x22 => NetworkDefinition {
            id: network_id,
            logical_name: "hammunet".into(),
            hrp_suffix: format!("tdx_{:x}_", network_id),
        },
        0x23 => NetworkDefinition {
            id: network_id,
            logical_name: "nergalnet".into(),
            hrp_suffix: format!("tdx_{:x}_", network_id),
        },
        0x24 => NetworkDefinition {
            id: network_id,
            logical_name: "mardunet".into(),
            hrp_suffix: format!("tdx_{:x}_", network_id),
        },

        0xF0 => NetworkDefinition {
            id: network_id,
            logical_name: "localnet".into(),
            hrp_suffix: "loc".into(),
        },
        0xF1 => NetworkDefinition {
            id: network_id,
            logical_name: "inttestnet".into(),
            hrp_suffix: format!("tdx_{:x}_", network_id),
        },
        0xF2 => NetworkDefinition::simulator(),

        _ => NetworkDefinition {
            id: network_id,
            logical_name: "unnamed".into(),
            hrp_suffix: format!("tdx_{:x}_", network_id),
        },
    }
}

pub fn network_id_from_hrp<S: AsRef<str>>(hrp: S) -> Result<u8> {
    // Getting the network specifier from the given HRP. Bech32 HRPs used in Babylon are structured
    // as follows:
    let splitted_hrp = hrp.as_ref().split('_').collect::<Vec<&str>>();
    let network_specifier = {
        match splitted_hrp.get(1) {
            Some(_) => Ok(splitted_hrp
                .into_iter()
                .skip(1)
                .collect::<Vec<&str>>()
                .join("_")),
            None => Err(AddressError::InvalidHrp),
        }
    }?;

    // Matching the network specifier to obtain the network id from it
    let network_id = match network_specifier.as_str() {
        "rdx" => NetworkDefinition::mainnet().id,
        "sim" => NetworkDefinition::simulator().id,
        "loc" => 0xF0,
        numeric_network_specifier => {
            match numeric_network_specifier.split('_').nth(1) {
                Some(network_id_string) => Ok(u8::from_str_radix(network_id_string, 16)
                    .map_err(|_| AddressError::InvalidHrp)?),
                None => Err(AddressError::InvalidHrp),
            }
        }?,
    };
    Ok(network_id)
}

pub fn network_id_from_address_string<S: AsRef<str>>(address: S) -> Result<u8> {
    // Attempt to Bech32m decode this address to get the hrp and the data type (will not be used).
    // The decoding process also yields a variant. We will not be verifying that this is bech32m
    // since this method is not meant to be a validation method.
    let (hrp, _, _) =
        bech32::decode(address.as_ref()).map_err(AddressError::Bech32mDecodingError)?;
    network_id_from_hrp(hrp)
}

pub fn is_account<A: Borrow<ComponentAddress>>(address: A) -> bool {
    matches!(
        address.borrow(),
        ComponentAddress::Account(..)
            | ComponentAddress::EcdsaSecp256k1VirtualAccount(..)
            | ComponentAddress::EddsaEd25519VirtualAccount(..)
    )
}

pub fn is_identity<A: Borrow<ComponentAddress>>(address: A) -> bool {
    matches!(
        address.borrow(),
        ComponentAddress::Identity(..)
            | ComponentAddress::EcdsaSecp256k1VirtualIdentity(..)
            | ComponentAddress::EddsaEd25519VirtualIdentity(..)
    )
}

pub fn checked_copy_u8_slice<T: AsRef<[u8]>, const N: usize>(slice: T) -> Result<[u8; N]> {
    let slice = slice.as_ref();
    if slice.len() != N {
        Err(Error::InvalidLength {
            expected: N,
            found: slice.len(),
        })
    } else {
        let mut bytes = [0u8; N];
        bytes.copy_from_slice(&slice[0..N]);
        Ok(bytes)
    }
}
