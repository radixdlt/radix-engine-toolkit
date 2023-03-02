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

use native_transaction::{
    ecdsa_secp256k1::EcdsaSecp256k1Signature, eddsa_ed25519::EddsaEd25519Signature,
};
use toolkit_derive::serializable;

// =================
// Model Definition
// =================

/// A discriminated union of the possible cryptographic signatures used by Scrypto and the Radix
/// Engine.
#[serializable]
#[serde(tag = "curve")]
pub enum Signature {
    /// A byte array of 65 bytes which are serialized as a 130 character long hex-encoded string
    /// representing a signature from the ECDSA Secp256k1 elliptic curve. An important note on
    /// ECDSA Secp256k1 signatures is that the format used and accepted by Scrypto is [v, r, s]
    /// where `v` is the recovery id and is a single byte and `r` and `s` are the signature results
    /// and are 32 bytes each.
    #[schemars(example = "crate::example::crypto::signature1")]
    EcdsaSecp256k1 {
        #[schemars(length(equal = 130))]
        #[schemars(regex(pattern = "[0-9a-fA-F]+"))]
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        signature: EcdsaSecp256k1Signature,
    },

    /// A byte array of 64 bytes which are serialized as a 128 character long hex-encoded string
    /// representing a signature from the EDDSA Ed25519 edwards curve.
    #[schemars(example = "crate::example::crypto::signature2")]
    EddsaEd25519 {
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

impl From<Signature> for native_transaction::model::Signature {
    fn from(value: Signature) -> Self {
        match value {
            Signature::EcdsaSecp256k1 { signature } => Self::EcdsaSecp256k1(signature),
            Signature::EddsaEd25519 { signature } => Self::EddsaEd25519(signature),
        }
    }
}

impl From<native_transaction::model::Signature> for Signature {
    fn from(value: native_transaction::model::Signature) -> Self {
        match value {
            native_transaction::model::Signature::EcdsaSecp256k1(signature) => {
                Self::EcdsaSecp256k1 { signature }
            }
            native_transaction::model::Signature::EddsaEd25519(signature) => {
                Self::EddsaEd25519 { signature }
            }
        }
    }
}
