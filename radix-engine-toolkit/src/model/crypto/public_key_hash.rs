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

use scrypto::prelude::{EcdsaSecp256k1PublicKeyHash, EddsaEd25519PublicKeyHash, NodeId};
use toolkit_derive::serializable;

// =================
// Model Definition
// =================

/// A discriminated union of the possible public keys used by Scrypto and the Radix Engine.
#[serializable]
#[serde(tag = "curve")]
pub enum PublicKeyHash {
    /// A byte array of 29 bytes which are serialized as a 58 character long hex-encoded string
    /// representing the hash of an ECDSA Secp256k1 public key.
    EcdsaSecp256k1 {
        #[schemars(length(equal = 58))]
        #[schemars(regex(pattern = "[0-9a-fA-F]+"))]
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::hex::Hex")]
        public_key_hash: [u8; NodeId::UUID_LENGTH],
    },

    /// A byte array of 29 bytes which are serialized as a 58 character long hex-encoded string
    /// representing the hash of an EdDSA Ed25519 public key.
    EddsaEd25519 {
        #[schemars(length(equal = 58))]
        #[schemars(regex(pattern = "[0-9a-fA-F]+"))]
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::hex::Hex")]
        public_key_hash: [u8; NodeId::UUID_LENGTH],
    },
}

// ============
// Conversions
// ============

impl From<PublicKeyHash> for scrypto::prelude::PublicKeyHash {
    fn from(value: PublicKeyHash) -> Self {
        match value {
            PublicKeyHash::EcdsaSecp256k1 { public_key_hash } => {
                Self::EcdsaSecp256k1(EcdsaSecp256k1PublicKeyHash(public_key_hash))
            }
            PublicKeyHash::EddsaEd25519 { public_key_hash } => {
                Self::EddsaEd25519(EddsaEd25519PublicKeyHash(public_key_hash))
            }
        }
    }
}

impl From<scrypto::prelude::PublicKeyHash> for PublicKeyHash {
    fn from(value: scrypto::prelude::PublicKeyHash) -> Self {
        match value {
            scrypto::prelude::PublicKeyHash::EcdsaSecp256k1(EcdsaSecp256k1PublicKeyHash(
                public_key_hash,
            )) => Self::EcdsaSecp256k1 { public_key_hash },
            scrypto::prelude::PublicKeyHash::EddsaEd25519(EddsaEd25519PublicKeyHash(
                public_key_hash,
            )) => Self::EddsaEd25519 { public_key_hash },
        }
    }
}
