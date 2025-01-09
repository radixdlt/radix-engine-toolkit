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

#[derive(Debug, Clone, Copy, Object, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Address(CoreTypedNodeId, u8);

#[uniffi::export]
impl Address {
    #[uniffi::constructor]
    pub fn new(address: String) -> Result<Arc<Self>> {
        let network_definition = NativeNetworkDefinition::from_address_string(
            &address,
        )
        .ok_or(RadixEngineToolkitError::FailedToExtractNetwork {
            address: address.clone(),
        })?;
        let bech32_decoder =
            NativeAddressBech32Decoder::new(&network_definition);

        let (_, bytes) =
            bech32_decoder
                .validate_and_decode(&address)
                .map_err(|error| {
                    RadixEngineToolkitError::Bech32DecodeError {
                        error: format!("{error:?}"),
                    }
                })?;

        Self::from_raw(bytes, network_definition.id)
    }

    #[uniffi::constructor]
    pub fn from_raw(
        node_id_bytes: Vec<u8>,
        network_id: u8,
    ) -> Result<Arc<Self>> {
        let node_id =
            node_id_bytes
                .try_into()
                .map(NativeNodeId)
                .map_err(|bytes| RadixEngineToolkitError::InvalidLength {
                    expected: NativeNodeId::LENGTH as u64,
                    actual: bytes.len() as u64,
                    data: bytes,
                })?;
        let typed_node_id = CoreTypedNodeId::new(node_id)?;

        Ok(Arc::new(Self(typed_node_id, network_id)))
    }

    #[uniffi::constructor]
    pub fn preallocated_account_address_from_public_key(
        public_key: PublicKey,
        network_id: u8,
    ) -> Result<Arc<Self>> {
        derive_preallocated_account_address_from_public_key(
            public_key, network_id,
        )
    }

    #[uniffi::constructor]
    pub fn preallocated_identity_address_from_public_key(
        public_key: PublicKey,
        network_id: u8,
    ) -> Result<Arc<Self>> {
        derive_preallocated_identity_address_from_public_key(
            public_key, network_id,
        )
    }

    #[uniffi::constructor]
    pub fn preallocated_account_address_from_olympia_address(
        olympia_account_address: Arc<OlympiaAddress>,
        network_id: u8,
    ) -> Result<Arc<Self>> {
        derive_preallocated_account_address_from_olympia_account_address(
            olympia_account_address,
            network_id,
        )
    }

    #[uniffi::constructor]
    pub fn resource_address_from_olympia_resource_address(
        olympia_resource_address: Arc<OlympiaAddress>,
        network_id: u8,
    ) -> Result<Arc<Self>> {
        derive_resource_address_from_olympia_resource_address(
            olympia_resource_address,
            network_id,
        )
    }

    pub fn network_id(&self) -> u8 {
        self.1
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub fn address_string(&self) -> String {
        let network_definition =
            NativeNetworkDefinition::from_network_id(self.1);
        let bech32_encoder =
            NativeAddressBech32Encoder::new(&network_definition);
        bech32_encoder.encode(self.0.as_bytes()).expect(
            "Safe to unwrap here. Node id has a valid entity type byte.",
        )
    }

    pub fn as_str(&self) -> String {
        self.address_string()
    }

    pub fn entity_type(&self) -> EntityType {
        self.0.entity_type().into()
    }

    pub fn is_global(&self) -> bool {
        self.0.entity_type().is_global()
    }
    pub fn is_internal(&self) -> bool {
        self.0.entity_type().is_internal()
    }
    pub fn is_global_component(&self) -> bool {
        self.0.entity_type().is_global_component()
    }
    pub fn is_global_package(&self) -> bool {
        self.0.entity_type().is_global_package()
    }
    pub fn is_global_consensus_manager(&self) -> bool {
        self.0.entity_type().is_global_consensus_manager()
    }
    pub fn is_global_resource_manager(&self) -> bool {
        self.0.entity_type().is_global_resource_manager()
    }
    pub fn is_global_fungible_resource_manager(&self) -> bool {
        self.0.entity_type().is_global_fungible_resource_manager()
    }
    pub fn is_global_non_fungible_resource_manager(&self) -> bool {
        self.0
            .entity_type()
            .is_global_non_fungible_resource_manager()
    }
    pub fn is_global_preallocated(&self) -> bool {
        self.0.entity_type().is_global_preallocated()
    }
    pub fn is_internal_kv_store(&self) -> bool {
        self.0.entity_type().is_internal_kv_store()
    }
    pub fn is_internal_fungible_vault(&self) -> bool {
        self.0.entity_type().is_internal_fungible_vault()
    }
    pub fn is_internal_non_fungible_vault(&self) -> bool {
        self.0.entity_type().is_internal_non_fungible_vault()
    }
    pub fn is_internal_vault(&self) -> bool {
        self.0.entity_type().is_internal_vault()
    }
}

impl Address {
    pub fn from_typed_node_id<T>(node_id: T, network_id: u8) -> Self
    where
        T: Into<CoreTypedNodeId>,
    {
        Self(node_id.into(), network_id)
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    pub(crate) fn unsafe_from_raw(
        node_id: NativeNodeId,
        network_id: u8,
    ) -> Self {
        Self(CoreTypedNodeId::new(node_id).unwrap(), network_id)
    }
}

impl From<Address> for NativeNodeId {
    fn from(value: Address) -> Self {
        *value.0.as_node_id()
    }
}

impl TryFrom<Address> for NativeResourceAddress {
    type Error = RadixEngineToolkitError;

    fn try_from(value: Address) -> Result<Self> {
        value.0.try_into().map_err(Into::into)
    }
}

impl TryFrom<Address> for NativeComponentAddress {
    type Error = RadixEngineToolkitError;

    fn try_from(value: Address) -> Result<Self> {
        value.0.try_into().map_err(Into::into)
    }
}

impl TryFrom<Address> for NativePackageAddress {
    type Error = RadixEngineToolkitError;

    fn try_from(value: Address) -> Result<Self> {
        value.0.try_into().map_err(Into::into)
    }
}

impl TryFrom<Address> for NativeInternalAddress {
    type Error = RadixEngineToolkitError;

    fn try_from(value: Address) -> Result<Self> {
        value.0.try_into().map_err(Into::into)
    }
}

impl TryFrom<Address> for NativeGlobalAddress {
    type Error = RadixEngineToolkitError;

    fn try_from(value: Address) -> Result<Self> {
        value.0.try_into().map_err(Into::into)
    }
}

impl TryFrom<Address> for NativeDynamicResourceAddress {
    type Error = RadixEngineToolkitError;

    fn try_from(value: Address) -> Result<Self> {
        value.0.try_into().map_err(Into::into).map(Self::Static)
    }
}

impl TryFrom<Address> for NativeDynamicComponentAddress {
    type Error = RadixEngineToolkitError;

    fn try_from(value: Address) -> Result<Self> {
        value.0.try_into().map_err(Into::into).map(Self::Static)
    }
}

impl TryFrom<Address> for NativeDynamicPackageAddress {
    type Error = RadixEngineToolkitError;

    fn try_from(value: Address) -> Result<Self> {
        value.0.try_into().map_err(Into::into).map(Self::Static)
    }
}

impl TryFrom<Address> for NativeDynamicGlobalAddress {
    type Error = RadixEngineToolkitError;

    fn try_from(value: Address) -> Result<Self> {
        value.0.try_into().map_err(Into::into).map(Self::Static)
    }
}
