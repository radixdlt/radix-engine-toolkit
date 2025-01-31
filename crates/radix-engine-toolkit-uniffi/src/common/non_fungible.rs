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

use std::str::FromStr;

use crate::prelude::*;

#[derive(Debug, Clone, Object, Hash, PartialEq, Eq)]
pub struct NonFungibleGlobalId(
    pub(crate) engine::NonFungibleGlobalId,
    pub(crate) u8,
);

#[uniffi::export]
impl NonFungibleGlobalId {
    #[uniffi::constructor]
    pub fn new(non_fungible_global_id: String) -> Result<Arc<Self>> {
        let network_definition = non_fungible_global_id
            .split(':')
            .next()
            .and_then(engine::NetworkDefinition::from_address_string)
            .ok_or(RadixEngineToolkitError::ParseError {
                type_name: "scrypto::prelude::NonFungibleGlobalId".to_owned(),
                error:
                    "Failed to obtain network id from non-fungible global id"
                        .to_owned(),
            })?;
        let bech32_decoder =
            engine::AddressBech32Decoder::new(&network_definition);

        let non_fungible_global_id =
            engine::NonFungibleGlobalId::try_from_canonical_string(
                &bech32_decoder,
                &non_fungible_global_id,
            )?;
        Ok(Arc::new(Self(
            non_fungible_global_id,
            network_definition.id,
        )))
    }

    #[uniffi::constructor]
    pub fn from_parts(
        resource_address: Arc<Address>,
        non_fungible_local_id: NonFungibleLocalId,
    ) -> Result<Arc<Self>> {
        let network_id = resource_address.network_id();
        let resource_address =
            match engine::ResourceAddress::try_from(*resource_address) {
                Ok(resource_address) if !resource_address.is_fungible() => {
                    Ok(resource_address)
                }
                _ => Err(RadixEngineToolkitError::EntityTypeMismatchError {
                    expected: vec![
                        EntityType::GlobalNonFungibleResourceManager,
                    ],
                    actual: resource_address.entity_type(),
                }),
            }?;
        let non_fungible_local_id =
            engine::NonFungibleLocalId::try_from(non_fungible_local_id)?;

        Ok(Arc::new(Self(
            engine::NonFungibleGlobalId::new(
                resource_address,
                non_fungible_local_id,
            ),
            network_id,
        )))
    }

    #[uniffi::constructor]
    pub fn signature_badge(
        public_key: PublicKey,
        network_id: u8,
    ) -> Result<Arc<Self>> {
        derive_signature_badge_non_fungible_global_id_from_public_key(
            public_key, network_id,
        )
    }

    #[uniffi::constructor]
    pub fn global_caller_badge_from_global_address(
        component_address: Arc<Address>,
        network_id: u8,
    ) -> Result<Arc<Self>> {
        derive_global_caller_non_fungible_global_id_from_global_address(
            component_address,
            network_id,
        )
    }

    #[uniffi::constructor]
    pub fn global_caller_badge_from_blueprint_id(
        package_address: Arc<Address>,
        blueprint_name: String,
        network_id: u8,
    ) -> Result<Arc<Self>> {
        derive_global_caller_non_fungible_global_id_from_blueprint_id(
            package_address,
            blueprint_name,
            network_id,
        )
    }

    #[uniffi::constructor]
    pub fn package_of_direct_caller_badge(
        package_address: Arc<Address>,
        network_id: u8,
    ) -> Result<Arc<Self>> {
        derive_package_of_direct_caller_non_fungible_global_id_from_package_address(
            package_address,
            network_id,
        )
    }

    pub fn resource_address(&self) -> Arc<Address> {
        let address = self.0.resource_address();
        Arc::new(Address::from_node_id(address, self.1))
    }

    pub fn local_id(&self) -> NonFungibleLocalId {
        self.0.local_id().clone().into()
    }

    pub fn as_str(&self) -> String {
        let network_definition =
            engine::NetworkDefinition::from_network_id(self.1);
        let bech32_encoder =
            engine::AddressBech32Encoder::new(&network_definition);
        self.0.to_canonical_string(&bech32_encoder)
    }

    pub fn is_global_caller_badge(&self) -> bool {
        self.0.resource_address() == engine::GLOBAL_CALLER_RESOURCE
    }

    pub fn is_package_of_direct_caller_badge(&self) -> bool {
        self.0.resource_address() == engine::PACKAGE_OF_DIRECT_CALLER_RESOURCE
    }
}

impl FromNativeWithNetworkContext for NonFungibleGlobalId {
    type Native = engine::NonFungibleGlobalId;

    fn from_native(native: Self::Native, network_id: u8) -> Self {
        Self(native, network_id)
    }
}

#[derive(Clone, Debug, Enum, Hash, PartialEq, Eq)]
pub enum NonFungibleLocalId {
    Integer { value: u64 },
    Str { value: String },
    Bytes { value: Vec<u8> },
    Ruid { value: Vec<u8> },
}

impl FromNative for NonFungibleLocalId {
    type Native = engine::NonFungibleLocalId;

    fn from_native(native: Self::Native) -> Self {
        native.into()
    }
}

impl From<engine::NonFungibleLocalId> for NonFungibleLocalId {
    fn from(value: engine::NonFungibleLocalId) -> Self {
        match value {
            engine::NonFungibleLocalId::String(value) => Self::Str {
                value: value.value().to_owned(),
            },
            engine::NonFungibleLocalId::Integer(value) => Self::Integer {
                value: value.value(),
            },
            engine::NonFungibleLocalId::Bytes(value) => Self::Bytes {
                value: value.value().to_vec(),
            },
            engine::NonFungibleLocalId::RUID(value) => Self::Ruid {
                value: value.value().to_vec(),
            },
        }
    }
}

impl TryFrom<NonFungibleLocalId> for engine::NonFungibleLocalId {
    type Error = RadixEngineToolkitError;

    fn try_from(value: NonFungibleLocalId) -> Result<Self> {
        match value {
            NonFungibleLocalId::Str { value } => {
                Self::string(value).map_err(Into::into)
            }
            NonFungibleLocalId::Bytes { value } => {
                Self::bytes(value).map_err(Into::into)
            }
            NonFungibleLocalId::Ruid { value } => value
                .try_into()
                .map(Self::ruid)
                .map_err(|value| RadixEngineToolkitError::InvalidLength {
                    expected: 32,
                    actual: value.len() as u64,
                    data: value,
                }),
            NonFungibleLocalId::Integer { value } => Ok(Self::integer(value)),
        }
    }
}

impl std::str::FromStr for NonFungibleLocalId {
    type Err = <engine::NonFungibleLocalId as std::str::FromStr>::Err;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        engine::NonFungibleLocalId::from_str(s).map(Into::into)
    }
}

// ==================
// Exposed "Methods"
// ==================

#[uniffi::export]
pub fn non_fungible_local_id_sbor_decode(
    bytes: Vec<u8>,
) -> Result<NonFungibleLocalId> {
    let native = match bytes.first().copied() {
        Some(engine::SCRYPTO_SBOR_V1_PAYLOAD_PREFIX) => {
            engine::scrypto_decode::<engine::NonFungibleLocalId>(&bytes)
                .map_err(Into::into)
        }
        Some(engine::MANIFEST_SBOR_V1_PAYLOAD_PREFIX) => {
            engine::manifest_decode::<engine::NonFungibleLocalId>(&bytes)
                .map_err(Into::into)
        }
        v => Err(RadixEngineToolkitError::DecodeError {
            error: format!("Invalid index byte: {v:?}"),
        }),
    }?;
    Ok(NonFungibleLocalId::from(native))
}

#[uniffi::export]
pub fn non_fungible_local_id_sbor_encode(
    value: NonFungibleLocalId,
) -> Result<Vec<u8>> {
    let native = engine::NonFungibleLocalId::try_from(value)?;
    Ok(engine::scrypto_encode(&native).expect("Can't fail"))
}

#[uniffi::export]
pub fn non_fungible_local_id_as_str(
    value: NonFungibleLocalId,
) -> Result<String> {
    engine::NonFungibleLocalId::try_from(value).map(|value| value.to_string())
}

#[uniffi::export]
pub fn non_fungible_local_id_from_str(
    string: String,
) -> Result<NonFungibleLocalId> {
    engine::NonFungibleLocalId::from_str(&string)
        .map_err(Into::into)
        .map(Into::into)
}
