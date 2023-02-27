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

use native_transaction::ecdsa_secp256k1::EcdsaSecp256k1Signature;
use native_transaction::eddsa_ed25519::EddsaEd25519Signature;
use scrypto::prelude::EddsaEd25519PublicKey;
use toolkit_derive::serializable;

// =================
// Model Definition
// =================

/// A discriminated union of the possible pairs of signatures and public keys used by Scrypto and
/// the Radix Engine.
#[serializable]
#[serde(tag = "curve")]
pub enum SignatureWithPublicKey {
    /// Cryptographic signature and public key for Ecdsa Secp256k1
    EcdsaSecp256k1 {
        /// A byte array of 65 bytes which are serialized as a 130 character long hex-encoded
        /// string representing a signature from the ECDSA Secp256k1 elliptic curve. An
        /// important note on ECDSA Secp256k1 signatures is that the format used and
        /// accepted by Scrypto is [v, r, s] where `v` is the recovery id and is a single
        /// byte and `r` and `s` are the signature results and are 32 bytes each. In this
        /// case, only a signature is needed since the public key can be derived from the
        /// signature if the message is available.
        #[schemars(length(equal = 130))]
        #[schemars(regex(pattern = "[0-9a-fA-F]+"))]
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        signature: EcdsaSecp256k1Signature,
    },

    /// Cryptographic signature and public key for EdDSA Ed25519
    EddsaEd25519 {
        /// A byte array of 32 bytes which are serialized as a 64 character long hex-encoded string
        /// representing a public key from the EDDSA Ed25519 edwards curve.
        #[schemars(length(equal = 66))]
        #[schemars(regex(pattern = "[0-9a-fA-F]+"))]
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        public_key: EddsaEd25519PublicKey,

        /// A byte array of 64 bytes which are serialized as a 128 character long hex-encoded
        /// string representing a signature from the EDDSA Ed25519 edwards curve.
        #[schemars(length(equal = 128))]
        #[schemars(regex(pattern = "[0-9a-fA-F]+"))]
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        signature: EddsaEd25519Signature,
    },
}

// ============
// Conversions
// ============

impl From<SignatureWithPublicKey> for native_transaction::model::SignatureWithPublicKey {
    fn from(value: SignatureWithPublicKey) -> Self {
        match value {
            SignatureWithPublicKey::EcdsaSecp256k1 { signature } => {
                Self::EcdsaSecp256k1 { signature }
            }
            SignatureWithPublicKey::EddsaEd25519 {
                signature,
                public_key,
            } => Self::EddsaEd25519 {
                signature,
                public_key,
            },
        }
    }
}

impl From<native_transaction::model::SignatureWithPublicKey> for SignatureWithPublicKey {
    fn from(value: native_transaction::model::SignatureWithPublicKey) -> Self {
        match value {
            native_transaction::model::SignatureWithPublicKey::EcdsaSecp256k1 { signature } => {
                Self::EcdsaSecp256k1 { signature }
            }
            native_transaction::model::SignatureWithPublicKey::EddsaEd25519 {
                signature,
                public_key,
            } => Self::EddsaEd25519 {
                signature,
                public_key,
            },
        }
    }
}
