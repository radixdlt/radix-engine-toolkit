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
use radix_engine_common::prelude::PublicKey;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

//================================================
// Derive Virtual Account Address from Public Key
//================================================

#[derive(Serialize, Deserialize, JsonSchema, Clone)]
pub struct DeriveVirtualAccountAddressFromPublicKeyInput {
    pub public_key: SerializablePublicKey,
    pub network_id: SerializableU8,
}
pub type DeriveVirtualAccountAddressFromPublicKeyOutput = SerializableNodeId;

pub struct DeriveVirtualAccountAddressFromPublicKey;
impl<'a> Function<'a> for DeriveVirtualAccountAddressFromPublicKey {
    type Input = DeriveVirtualAccountAddressFromPublicKeyInput;
    type Output = DeriveVirtualAccountAddressFromPublicKeyOutput;

    fn handle(input: Self::Input) -> Result<Self::Output, crate::error::InvocationHandlingError> {
        let DeriveVirtualAccountAddressFromPublicKeyInput {
            public_key,
            network_id,
        } = input;

        let virtual_account_address =
            radix_engine_toolkit::functions::derive::virtual_account_address_from_public_key(
                &public_key,
            );

        Ok(SerializableNodeId(SerializableNodeIdInternal {
            network_id: *network_id,
            node_id: virtual_account_address.into_node_id(),
        }))
    }
}

export_function!(
    DeriveVirtualAccountAddressFromPublicKey as derive_virtual_account_address_from_public_key
);
export_jni_function!(
    DeriveVirtualAccountAddressFromPublicKey as deriveVirtualAccountAddressFromPublicKey
);

//=================================================
// Derive Virtual Identity Address from Public Key
//=================================================

#[derive(Serialize, Deserialize, JsonSchema, Clone)]
pub struct DeriveVirtualIdentityAddressFromPublicKeyInput {
    pub public_key: SerializablePublicKey,
    pub network_id: SerializableU8,
}
pub type DeriveVirtualIdentityAddressFromPublicKeyOutput = SerializableNodeId;

pub struct DeriveVirtualIdentityAddressFromPublicKey;
impl<'a> Function<'a> for DeriveVirtualIdentityAddressFromPublicKey {
    type Input = DeriveVirtualIdentityAddressFromPublicKeyInput;
    type Output = DeriveVirtualIdentityAddressFromPublicKeyOutput;

    fn handle(input: Self::Input) -> Result<Self::Output, crate::error::InvocationHandlingError> {
        let DeriveVirtualIdentityAddressFromPublicKeyInput {
            public_key,
            network_id,
        } = input;

        let virtual_identity_address =
            radix_engine_toolkit::functions::derive::virtual_identity_address_from_public_key(
                &public_key,
            );

        Ok(SerializableNodeId(SerializableNodeIdInternal {
            network_id: *network_id,
            node_id: virtual_identity_address.into_node_id(),
        }))
    }
}

export_function!(
    DeriveVirtualIdentityAddressFromPublicKey as derive_virtual_identity_address_from_public_key
);
export_jni_function!(
    DeriveVirtualIdentityAddressFromPublicKey as deriveVirtualIdentityAddressFromPublicKey
);

//=================================================================
// Derive Virtual Signature Non-Fungible Global Id from Public Key
//=================================================================

#[derive(Serialize, Deserialize, JsonSchema, Clone)]
pub struct DeriveVirtualSignatureNonFungibleGlobalIdFromPublicKeyInput {
    pub public_key: SerializablePublicKey,
    pub network_id: SerializableU8,
}
pub type DeriveVirtualSignatureNonFungibleGlobalIdFromPublicKeyOutput =
    SerializableNonFungibleGlobalId;

pub struct DeriveVirtualSignatureNonFungibleGlobalIdFromPublicKey;
impl<'a> Function<'a> for DeriveVirtualSignatureNonFungibleGlobalIdFromPublicKey {
    type Input = DeriveVirtualSignatureNonFungibleGlobalIdFromPublicKeyInput;
    type Output = DeriveVirtualSignatureNonFungibleGlobalIdFromPublicKeyOutput;

    fn handle(input: Self::Input) -> Result<Self::Output, crate::error::InvocationHandlingError> {
        let DeriveVirtualSignatureNonFungibleGlobalIdFromPublicKeyInput {
            public_key,
            network_id,
        } = input;

        let non_fungible_global_id =
            radix_engine_toolkit::functions::derive::virtual_signature_non_fungible_global_id_from_public_key(
                &PublicKey::from(public_key),
            );

        Ok(SerializableNonFungibleGlobalId(
            SerializableNonFungibleGlobalIdInternal {
                network_id: *network_id,
                non_fungible_global_id,
            },
        ))
    }
}

export_function!(
    DeriveVirtualSignatureNonFungibleGlobalIdFromPublicKey
        as derive_virtual_signature_non_fungible_global_id_from_public_key
);
export_jni_function!(
    DeriveVirtualSignatureNonFungibleGlobalIdFromPublicKey
        as deriveVirtualSignatureNonFungibleGlobalIdFromPublicKey
);

//=============================================================
// Derive Virtual Account Address from Olympia Account Address
//=============================================================

#[derive(Serialize, Deserialize, JsonSchema, Clone)]
pub struct DeriveVirtualAccountAddressFromOlympiaAccountAddressInput {
    pub olympia_account_address: String,
    pub network_id: SerializableU8,
}
pub type DeriveVirtualAccountAddressFromOlympiaAccountAddressOutput = SerializableNodeId;

pub struct DeriveVirtualAccountAddressFromOlympiaAccountAddress;
impl<'a> Function<'a> for DeriveVirtualAccountAddressFromOlympiaAccountAddress {
    type Input = DeriveVirtualAccountAddressFromOlympiaAccountAddressInput;
    type Output = DeriveVirtualAccountAddressFromOlympiaAccountAddressOutput;

    fn handle(input: Self::Input) -> Result<Self::Output, crate::error::InvocationHandlingError> {
        let DeriveVirtualAccountAddressFromOlympiaAccountAddressInput {
            olympia_account_address,
            network_id,
        } = input;

        let component_address =
            radix_engine_toolkit::functions::derive::virtual_account_address_from_olympia_account_address(
                olympia_account_address,
            ).map_err(|error| InvocationHandlingError::DerivationError(debug_string(error)))?;

        Ok(SerializableNodeId(SerializableNodeIdInternal {
            network_id: *network_id,
            node_id: component_address.into_node_id(),
        }))
    }
}

export_function!(
    DeriveVirtualAccountAddressFromOlympiaAccountAddress
        as derive_virtual_account_address_from_olympia_account_address
);
export_jni_function!(
    DeriveVirtualAccountAddressFromOlympiaAccountAddress
        as deriveVirtualAccountAddressFromOlympiaAccountAddress
);

//=======================================================
// Derive Resource Address from Olympia Resource Address
//=======================================================

#[derive(Serialize, Deserialize, JsonSchema, Clone)]
pub struct DeriveResourceAddressFromOlympiaResourceAddressInput {
    pub olympia_resource_address: String,
    pub network_id: SerializableU8,
}
pub type DeriveResourceAddressFromOlympiaResourceAddressOutput = SerializableNodeId;

pub struct DeriveResourceAddressFromOlympiaResourceAddress;
impl<'a> Function<'a> for DeriveResourceAddressFromOlympiaResourceAddress {
    type Input = DeriveResourceAddressFromOlympiaResourceAddressInput;
    type Output = DeriveResourceAddressFromOlympiaResourceAddressOutput;

    fn handle(input: Self::Input) -> Result<Self::Output, crate::error::InvocationHandlingError> {
        let DeriveResourceAddressFromOlympiaResourceAddressInput {
            olympia_resource_address,
            network_id,
        } = input;

        let component_address =
            radix_engine_toolkit::functions::derive::resource_address_from_olympia_resource_address(
                olympia_resource_address,
            ).map_err(|error| InvocationHandlingError::DerivationError(debug_string(error)))?;

        Ok(SerializableNodeId(SerializableNodeIdInternal {
            network_id: *network_id,
            node_id: component_address.into_node_id(),
        }))
    }
}

export_function!(
    DeriveResourceAddressFromOlympiaResourceAddress
        as derive_resource_address_from_olympia_resource_address
);
export_jni_function!(
    DeriveResourceAddressFromOlympiaResourceAddress
        as deriveResourceAddressFromOlympiaResourceAddress
);

//================================================
// Derive Public Key from Olympia Account Address
//================================================

pub type DerivePublicKeyFromOlympiaAccountAddressInput = String;
pub type DerivePublicKeyFromOlympiaAccountAddressOutput = SerializableSecp256k1PublicKey;

pub struct DerivePublicKeyFromOlympiaAccountAddress;
impl<'a> Function<'a> for DerivePublicKeyFromOlympiaAccountAddress {
    type Input = DerivePublicKeyFromOlympiaAccountAddressInput;
    type Output = DerivePublicKeyFromOlympiaAccountAddressOutput;

    fn handle(input: Self::Input) -> Result<Self::Output, crate::error::InvocationHandlingError> {
        let public_key =
            radix_engine_toolkit::functions::derive::public_key_from_olympia_account_address(input)
                .map_err(|error| InvocationHandlingError::DerivationError(debug_string(error)))?;

        Ok(public_key.into())
    }
}

export_function!(
    DerivePublicKeyFromOlympiaAccountAddress as derive_public_key_from_olympia_account_address
);
export_jni_function!(
    DerivePublicKeyFromOlympiaAccountAddress as derivePublicKeyFromOlympiaAccountAddress
);

//================================================
// Derive Olympia Account from Public Key Address
//================================================

#[derive(Serialize, Deserialize, JsonSchema, Clone)]
pub struct DeriveOlympiaAccountAddressFromPublicKeyInput {
    pub olympia_network: SerializableOlympiaNetwork,
    pub public_key: SerializableSecp256k1PublicKey,
}
pub type DeriveOlympiaAccountAddressFromPublicKeyOutput = String;

pub struct DeriveOlympiaAccountAddressFromPublicKey;
impl<'a> Function<'a> for DeriveOlympiaAccountAddressFromPublicKey {
    type Input = DeriveOlympiaAccountAddressFromPublicKeyInput;
    type Output = DeriveOlympiaAccountAddressFromPublicKeyOutput;

    fn handle(input: Self::Input) -> Result<Self::Output, crate::error::InvocationHandlingError> {
        let DeriveOlympiaAccountAddressFromPublicKeyInput {
            olympia_network,
            public_key,
        } = input;

        let olympia_account_address =
            radix_engine_toolkit::functions::derive::olympia_account_address_from_public_key(
                &public_key.into(),
                olympia_network.into(),
            );

        Ok(olympia_account_address)
    }
}

export_function!(
    DeriveOlympiaAccountAddressFromPublicKey as derive_olympia_account_address_from_public_key
);
export_jni_function!(
    DeriveOlympiaAccountAddressFromPublicKey as deriveOlympiaAccountAddressFromPublicKey
);

//=============================================
// Derive Node Address from Public Key Address
//=============================================

#[derive(Serialize, Deserialize, JsonSchema, Clone)]
pub struct DeriveNodeAddressFromPublicKeyInput {
    pub network_id: SerializableU8,
    pub public_key: SerializableSecp256k1PublicKey,
}
pub type DeriveNodeAddressFromPublicKeyOutput = String;

pub struct DeriveNodeAddressFromPublicKey;
impl<'a> Function<'a> for DeriveNodeAddressFromPublicKey {
    type Input = DeriveNodeAddressFromPublicKeyInput;
    type Output = DeriveNodeAddressFromPublicKeyOutput;

    fn handle(input: Self::Input) -> Result<Self::Output, crate::error::InvocationHandlingError> {
        let DeriveNodeAddressFromPublicKeyInput {
            network_id,
            public_key,
        } = input;

        let node_address = radix_engine_toolkit::functions::derive::node_address_from_public_key(
            &public_key.into(),
            *network_id,
        );

        Ok(node_address)
    }
}

export_function!(DeriveNodeAddressFromPublicKey as derive_node_address_from_public_key);
export_jni_function!(DeriveNodeAddressFromPublicKey as deriveNodeAddressFromPublicKey);
