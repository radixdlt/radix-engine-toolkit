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

use crate::prelude::*;

#[derive(Debug, Clone, Object)]
pub struct NonFungibleGlobalId(pub(crate) NativeNonFungibleGlobalId, pub(crate) u8);

#[uniffi::export]
impl NonFungibleGlobalId {
    #[uniffi::constructor]
    pub fn new(non_fungible_global_id: String) -> Result<Arc<Self>> {
        let network_id = non_fungible_global_id
            .split(':')
            .next()
            .and_then(core_network_id_from_address_string)
            .ok_or(RadixEngineToolkitError::ParseError {
                type_name: "scrypto::prelude::NonFungibleGlobalId".to_owned(),
                error: "Failed to obtain network id from non-fungible global id".to_owned(),
            })?;
        let network_definition = core_network_definition_from_network_id(network_id);
        let bech32_decoder = NativeBech32Decoder::new(&network_definition);

        let non_fungible_global_id = NativeNonFungibleGlobalId::try_from_canonical_string(
            &bech32_decoder,
            &non_fungible_global_id,
        )?;
        Ok(Arc::new(Self(non_fungible_global_id, network_id)))
    }

    #[uniffi::constructor]
    pub fn from_parts(
        resource_address: Arc<Address>,
        non_fungible_local_id: NonFungibleLocalId,
    ) -> Result<Arc<Self>> {
        match resource_address.entity_type() {
            EntityType::GlobalNonFungibleResourceManager => Ok(()),
            actual => Err(RadixEngineToolkitError::EntityTypeMismatchError {
                expected: vec![EntityType::GlobalNonFungibleResourceManager],
                actual,
            }),
        }?;
        let network_id = resource_address.1;
        let resource_address = NativeResourceAddress::new_or_panic(resource_address.0 .0);
        let non_fungible_local_id = NativeNonFungibleLocalId::try_from(non_fungible_local_id)?;

        Ok(Arc::new(Self(
            NativeNonFungibleGlobalId::new(resource_address, non_fungible_local_id),
            network_id,
        )))
    }

    #[uniffi::constructor]
    pub fn virtual_signature_badge(public_key: PublicKey, network_id: u8) -> Result<Arc<Self>> {
        derive_virtual_signature_non_fungible_global_id_from_public_key(public_key, network_id)
    }

    pub fn resource_address(&self) -> Arc<Address> {
        let address = self.0.resource_address();
        let node_id = address.as_node_id();
        Arc::new(Address(*node_id, self.1))
    }

    pub fn local_id(&self) -> NonFungibleLocalId {
        self.0.local_id().clone().into()
    }
}

#[derive(Clone, Debug, Enum)]
pub enum NonFungibleLocalId {
    Integer { value: u64 },
    Str { value: String },
    Bytes { value: Vec<u8> },
    Ruid { value: Vec<u8> },
}

impl From<NativeNonFungibleLocalId> for NonFungibleLocalId {
    fn from(value: NativeNonFungibleLocalId) -> Self {
        match value {
            NativeNonFungibleLocalId::String(value) => Self::Str {
                value: value.value().to_owned(),
            },
            NativeNonFungibleLocalId::Integer(value) => Self::Integer {
                value: value.value(),
            },
            NativeNonFungibleLocalId::Bytes(value) => Self::Bytes {
                value: value.value().to_vec(),
            },
            NativeNonFungibleLocalId::RUID(value) => Self::Ruid {
                value: value.value().to_vec(),
            },
        }
    }
}

impl TryFrom<NonFungibleLocalId> for NativeNonFungibleLocalId {
    type Error = RadixEngineToolkitError;

    fn try_from(value: NonFungibleLocalId) -> Result<Self> {
        match value {
            NonFungibleLocalId::Str { value } => Self::string(value).map_err(Into::into),
            NonFungibleLocalId::Bytes { value } => Self::bytes(value).map_err(Into::into),
            NonFungibleLocalId::Ruid { value } => {
                value.try_into().map(Self::ruid).map_err(|value| {
                    RadixEngineToolkitError::InvalidLength {
                        expected: 32,
                        actual: value.len() as u64,
                        data: value,
                    }
                })
            }
            NonFungibleLocalId::Integer { value } => Ok(Self::integer(value)),
        }
    }
}

impl std::str::FromStr for NonFungibleLocalId {
    type Err = <NativeNonFungibleLocalId as std::str::FromStr>::Err;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        NativeNonFungibleLocalId::from_str(s).map(Into::into)
    }
}
