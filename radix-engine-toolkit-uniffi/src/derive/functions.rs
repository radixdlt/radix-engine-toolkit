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

#[uniffi::export]
pub fn derive_virtual_account_address_from_public_key(
    public_key: PublicKey,
    network_id: u8,
) -> Result<Arc<Address>> {
    let public_key = NativePublicKey::try_from(public_key)?;
    let address = NativeComponentAddress::virtual_account_from_public_key(&public_key);
    let node_id = address.into_node_id();
    Ok(Arc::new(Address(node_id, network_id)))
}

#[uniffi::export]
pub fn derive_virtual_identity_address_from_public_key(
    public_key: PublicKey,
    network_id: u8,
) -> Result<Arc<Address>> {
    let public_key = NativePublicKey::try_from(public_key)?;
    let address = NativeComponentAddress::virtual_identity_from_public_key(&public_key);
    let node_id = address.into_node_id();
    Ok(Arc::new(Address(node_id, network_id)))
}

#[uniffi::export]
pub fn derive_virtual_signature_non_fungible_global_id_from_public_key(
    public_key: PublicKey,
    network_id: u8,
) -> Result<Arc<NonFungibleGlobalId>> {
    let public_key = NativePublicKey::try_from(public_key)?;
    let non_fungible_global_id =
        core_virtual_signature_non_fungible_global_id_from_public_key(&public_key);
    Ok(Arc::new(NonFungibleGlobalId(
        non_fungible_global_id,
        network_id,
    )))
}

#[uniffi::export]
pub fn derive_virtual_account_address_from_olympia_account_address(
    olympia_account_address: Arc<OlympiaAddress>,
    network_id: u8,
) -> Result<Arc<Address>> {
    let component_address =
        core_virtual_account_address_from_olympia_account_address(&olympia_account_address.0)?;
    let node_id = component_address.into_node_id();
    Ok(Arc::new(Address(node_id, network_id)))
}

#[uniffi::export]
pub fn derive_resource_address_from_olympia_resource_address(
    olympia_resource_address: Arc<OlympiaAddress>,
    network_id: u8,
) -> Result<Arc<Address>> {
    let resource_address =
        core_resource_address_from_olympia_resource_address(&olympia_resource_address.0)?;
    let node_id = resource_address.into_node_id();
    Ok(Arc::new(Address(node_id, network_id)))
}

#[uniffi::export]
pub fn derive_public_key_from_olympia_account_address(
    olympia_resource_address: Arc<OlympiaAddress>,
) -> Result<PublicKey> {
    core_public_key_from_olympia_account_address(&olympia_resource_address.0)
        .map(
            |NativeEcdsaSecp256k1PublicKey(public_key)| PublicKey::EcdsaSecp256k1 {
                value: public_key.to_vec(),
            },
        )
        .map_err(Into::into)
}

#[uniffi::export]
pub fn derive_olympia_account_address_from_public_key(
    public_key: PublicKey,
    olympia_network: OlympiaNetwork,
) -> Result<Arc<OlympiaAddress>> {
    let public_key = match NativePublicKey::try_from(public_key)? {
        NativePublicKey::EcdsaSecp256k1(pk) => Ok(pk),
        NativePublicKey::EddsaEd25519(..) => Err(RadixEngineToolkitError::InvalidPublicKey),
    }?;
    let address = core_olympia_account_address_from_public_key(&public_key, olympia_network.into());
    Ok(Arc::new(OlympiaAddress(address)))
}
