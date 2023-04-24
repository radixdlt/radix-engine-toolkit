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

use bech32::ToBase32;
use scrypto::prelude::PublicKey;
use toolkit_derive::serializable;

use crate::utils::debug_string;

use super::traits::Handler;

// =================
// Model Definition
// =================

#[serializable]
pub enum OlympiaNetwork {
    Mainnet,
    Stokenet,
    Releasenet,
    Rcnet,
    Milestonenet,
    Devopsnet,
    Sandpitnet,
    Localnet,
}

impl OlympiaNetwork {
    pub fn hrp(&self) -> &str {
        match self {
            Self::Mainnet => "rdx",
            Self::Stokenet => "tdx",
            Self::Releasenet => "tdx3",
            Self::Rcnet => "tdx4",
            Self::Milestonenet => "tdx5",
            Self::Devopsnet => "tdx6",
            Self::Sandpitnet => "tdx7",
            Self::Localnet => "ddx",
        }
    }
}

/// Given an ECDSA Secp256k1 Public Key and Olympia network, this function derives the Olympia
/// account address associated with the public key on that network.
#[serializable]
pub struct DeriveOlympiaAddressFromPublicKeyRequest {
    /// The Olympia network to derive the account address for.
    pub network: OlympiaNetwork,

    /// The public key to derive the non-fungible global id for.
    #[schemars(with = "crate::model::crypto::PublicKey")]
    #[serde_as(as = "serde_with::FromInto<crate::model::crypto::PublicKey>")]
    pub public_key: PublicKey,
}

/// The response form [`DeriveOlympiaAddressFromPublicKeyRequest`] requests
#[serializable]
pub struct DeriveOlympiaAddressFromPublicKeyResponse {
    /// The Olympia account address associated with the given public key on the desired network
    pub olympia_account_address: String,
}

// ===============
// Implementation
// ===============

pub struct DeriveOlympiaAddressFromPublicKeyHandler;

impl Handler<DeriveOlympiaAddressFromPublicKeyRequest, DeriveOlympiaAddressFromPublicKeyResponse>
    for DeriveOlympiaAddressFromPublicKeyHandler
{
    type Error = DeriveOlympiaAddressFromPublicKeyError;

    fn pre_process(
        request: DeriveOlympiaAddressFromPublicKeyRequest,
    ) -> Result<DeriveOlympiaAddressFromPublicKeyRequest, DeriveOlympiaAddressFromPublicKeyError>
    {
        Ok(request)
    }

    fn handle(
        request: &DeriveOlympiaAddressFromPublicKeyRequest,
    ) -> Result<DeriveOlympiaAddressFromPublicKeyResponse, DeriveOlympiaAddressFromPublicKeyError>
    {
        // Ensure that the passed public key is an Ecdsa Secp256k1 since this is the only public
        // key supported by Olympia.
        let mut public_key_bytes = match request.public_key {
            PublicKey::EcdsaSecp256k1(public_key) => Ok(public_key.to_vec()),
            PublicKey::EddsaEd25519(_) => {
                Err(DeriveOlympiaAddressFromPublicKeyError::InvalidPublicKeyType)
            }
        }?;

        // In Olympia, before a public key is Bech32 encoded into an Olympia account address, it has
        // a 0x04 prefixed to it.
        public_key_bytes.insert(0, 0x04);

        bech32::encode(
            request.network.hrp(),
            public_key_bytes.to_base32(),
            bech32::Variant::Bech32,
        )
        .map_err(|error| {
            DeriveOlympiaAddressFromPublicKeyError::Bech32EncodingOfOlympiaAddressFailed {
                message: debug_string(error),
            }
        })
        .map(|address| DeriveOlympiaAddressFromPublicKeyResponse {
            olympia_account_address: address,
        })
    }

    fn post_process(
        _: &DeriveOlympiaAddressFromPublicKeyRequest,
        response: DeriveOlympiaAddressFromPublicKeyResponse,
    ) -> Result<DeriveOlympiaAddressFromPublicKeyResponse, DeriveOlympiaAddressFromPublicKeyError>
    {
        Ok(response)
    }
}

#[serializable]
#[serde(tag = "type")]
pub enum DeriveOlympiaAddressFromPublicKeyError {
    /// Emitted when an invalid public key is passed. This function only accepts Ecdsa Secp256k1
    /// public keys.
    InvalidPublicKeyType,

    Bech32EncodingOfOlympiaAddressFailed {
        message: String,
    },
}
