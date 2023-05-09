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

use scrypto::prelude::{EcdsaSecp256k1PublicKey, EddsaEd25519PublicKey};
use toolkit_derive::serializable;

// =================
// Model Definition
// =================

/// A discriminated union of the possible public keys used by Scrypto and the Radix Engine.
#[serializable]
#[serde(tag = "curve")]
pub enum PublicKey {
    /// A byte array of 33 bytes which are serialized as a 66 character long hex-encoded string
    /// representing a public key from the ECDSA Secp256k1 elliptic curve.
    #[schemars(example = "crate::example::crypto::public_key1")]
    EcdsaSecp256k1 {
        #[schemars(length(equal = 66))]
        #[schemars(regex(pattern = "[0-9a-fA-F]+"))]
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        public_key: EcdsaSecp256k1PublicKey,
    },

    /// A byte array of 32 bytes which are serialized as a 64 character long hex-encoded string
    /// representing a public key from the EDDSA Ed25519 edwards curve.
    #[schemars(example = "crate::example::crypto::public_key2")]
    EddsaEd25519 {
        #[schemars(length(equal = 64))]
        #[schemars(regex(pattern = "[0-9a-fA-F]+"))]
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        public_key: EddsaEd25519PublicKey,
    },
}

// ============
// Conversions
// ============

impl From<PublicKey> for scrypto::prelude::PublicKey {
    fn from(value: PublicKey) -> Self {
        match value {
            PublicKey::EcdsaSecp256k1 { public_key } => Self::EcdsaSecp256k1(public_key),
            PublicKey::EddsaEd25519 { public_key } => Self::EddsaEd25519(public_key),
        }
    }
}

impl From<scrypto::prelude::PublicKey> for PublicKey {
    fn from(value: scrypto::prelude::PublicKey) -> Self {
        match value {
            scrypto::prelude::PublicKey::EcdsaSecp256k1(public_key) => {
                Self::EcdsaSecp256k1 { public_key }
            }
            scrypto::prelude::PublicKey::EddsaEd25519(public_key) => {
                Self::EddsaEd25519 { public_key }
            }
        }
    }
}
