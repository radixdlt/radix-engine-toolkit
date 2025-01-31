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
pub fn derive_preallocated_account_address_from_public_key(
    public_key: PublicKey,
    network_id: u8,
) -> Result<Arc<Address>> {
    let public_key = engine::PublicKey::try_from(public_key)?;
    let address =
        engine::ComponentAddress::preallocated_account_from_public_key(
            &public_key,
        );
    Ok(Arc::new(Address::from_node_id(address, network_id)))
}

#[uniffi::export]
pub fn derive_preallocated_identity_address_from_public_key(
    public_key: PublicKey,
    network_id: u8,
) -> Result<Arc<Address>> {
    let public_key = engine::PublicKey::try_from(public_key)?;
    let address =
        engine::ComponentAddress::preallocated_identity_from_public_key(
            &public_key,
        );
    Ok(Arc::new(Address::from_node_id(address, network_id)))
}

#[uniffi::export]
pub fn derive_signature_badge_non_fungible_global_id_from_public_key(
    public_key: PublicKey,
    network_id: u8,
) -> Result<Arc<NonFungibleGlobalId>> {
    let public_key = engine::PublicKey::try_from(public_key)?;
    let non_fungible_global_id =
        toolkit::functions::derive::preallocated_signature_non_fungible_global_id_from_public_key(
            &public_key,
        );
    Ok(Arc::new(NonFungibleGlobalId(
        non_fungible_global_id,
        network_id,
    )))
}

#[uniffi::export]
pub fn derive_global_caller_non_fungible_global_id_from_global_address(
    global_address: Arc<Address>,
    network_id: u8,
) -> Result<Arc<NonFungibleGlobalId>> {
    let global_address = engine::GlobalAddress::try_from(*global_address)?;
    let non_fungible_global_id =
        toolkit::functions::derive::global_caller_non_fungible_global_id_from_global_address(
            global_address,
        );
    Ok(Arc::new(NonFungibleGlobalId(
        non_fungible_global_id,
        network_id,
    )))
}

#[uniffi::export]
pub fn derive_global_caller_non_fungible_global_id_from_blueprint_id(
    package_address: Arc<Address>,
    blueprint_name: String,
    network_id: u8,
) -> Result<Arc<NonFungibleGlobalId>> {
    let package_address = engine::PackageAddress::try_from(*package_address)?;
    let blueprint_id = engine::BlueprintId {
        package_address,
        blueprint_name,
    };
    let non_fungible_global_id =
        toolkit::functions::derive::global_caller_non_fungible_global_id_from_blueprint_id(
            blueprint_id,
        );
    Ok(Arc::new(NonFungibleGlobalId(
        non_fungible_global_id,
        network_id,
    )))
}

#[uniffi::export]
pub fn derive_package_of_direct_caller_non_fungible_global_id_from_package_address(
    package_address: Arc<Address>,
    network_id: u8,
) -> Result<Arc<NonFungibleGlobalId>> {
    let package_address = engine::PackageAddress::try_from(*package_address)?;
    let non_fungible_global_id =
        toolkit::functions::derive::package_of_direct_caller_non_fungible_global_id_from_package_address(
            package_address,
        );
    Ok(Arc::new(NonFungibleGlobalId(
        non_fungible_global_id,
        network_id,
    )))
}

#[uniffi::export]
pub fn derive_preallocated_account_address_from_olympia_account_address(
    olympia_account_address: Arc<OlympiaAddress>,
    network_id: u8,
) -> Result<Arc<Address>> {
    let component_address =
        toolkit::functions::derive::preallocated_account_address_from_olympia_account_address(
            &olympia_account_address.0,
        )?;
    Ok(Arc::new(Address::from_node_id(
        component_address,
        network_id,
    )))
}

#[uniffi::export]
pub fn derive_resource_address_from_olympia_resource_address(
    olympia_resource_address: Arc<OlympiaAddress>,
    network_id: u8,
) -> Result<Arc<Address>> {
    let resource_address =
        toolkit::functions::derive::resource_address_from_olympia_resource_address(
            &olympia_resource_address.0,
        )?;
    Ok(Arc::new(Address::from_node_id(
        resource_address,
        network_id,
    )))
}

#[uniffi::export]
pub fn derive_public_key_from_olympia_account_address(
    olympia_resource_address: Arc<OlympiaAddress>,
) -> Result<PublicKey> {
    toolkit::functions::derive::public_key_from_olympia_account_address(
        &olympia_resource_address.0,
    )
    .map(
        |engine::Secp256k1PublicKey(public_key)| PublicKey::Secp256k1 {
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
    let public_key = match engine::PublicKey::try_from(public_key)? {
        engine::PublicKey::Secp256k1(pk) => Ok(pk),
        engine::PublicKey::Ed25519(..) => {
            Err(RadixEngineToolkitError::InvalidPublicKey)
        }
    }?;
    let address =
        toolkit::functions::derive::olympia_account_address_from_public_key(
            &public_key,
            olympia_network.into(),
        );
    Ok(Arc::new(OlympiaAddress(address)))
}

#[uniffi::export]
pub fn public_key_hash_from_public_key(
    public_key: PublicKey,
) -> Result<PublicKeyHash> {
    let public_key = engine::PublicKey::try_from(public_key)?;
    let public_key_hash =
        toolkit::functions::derive::public_key_hash_from_public_key(
            &public_key,
        );
    Ok(public_key_hash.into())
}
