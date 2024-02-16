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
use radix_engine_common::prelude::*;
use scrypto::prelude::AddressBech32Decoder;
use serde_with::{DeserializeFromStr, SerializeDisplay};

#[derive(Debug)]
pub enum CanonicalAddressError {
    FailedToDecodeBech32,
    FailedToEncodeBech32,
}

pub type NetworkId = u8;

trait CanonicalAddress {
    fn entity_type(&self) -> Option<EntityType>;
    fn network_id(&self) -> NetworkId;
    fn to_bech32(&self) -> Result<String, CanonicalAddressError>;
}
trait CanonicalResourceAddress {
    fn is_fungible(&self) -> bool;
}

#[derive(
    Clone, Debug, PartialEq, Eq, Hash, SerializeDisplay, DeserializeFromStr,
)]
pub struct CanonicalAccountAddress {
    address: NodeId,
    network_id: NetworkId,
}

impl CanonicalAccountAddress {
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
            EntityType::GlobalAccount
                | EntityType::GlobalVirtualSecp256k1Account
                | EntityType::GlobalVirtualEd25519Account
        )
    }
}

impl CanonicalAddress for CanonicalAccountAddress {
    fn entity_type(&self) -> Option<EntityType> {
        self.address.entity_type()
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

impl FromStr for CanonicalAccountAddress {
    type Err = CanonicalAddressError;

    fn from_str(bech32: &str) -> Result<Self, Self::Err> {
        Self::try_from_bech32(bech32)
            .ok_or(CanonicalAddressError::FailedToDecodeBech32)
    }
}

impl Display for CanonicalAccountAddress {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(&self.to_bech32().map_err(|_| fmt::Error)?)
    }
}

impl Display for CanonicalAddressError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{:?}", self)
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
}
