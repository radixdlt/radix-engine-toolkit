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

use radix_engine_common::prelude::{Ed25519PublicKey, PublicKey, Secp256k1PublicKey};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Serialize, Deserialize, JsonSchema, Clone)]
#[serde(tag = "kind")]
pub enum SerializablePublicKey {
    Secp256k1 {
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::hex::Hex")]
        value: [u8; Secp256k1PublicKey::LENGTH],
    },
    Ed25519 {
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::hex::Hex")]
        value: [u8; Ed25519PublicKey::LENGTH],
    },
}

impl From<PublicKey> for SerializablePublicKey {
    fn from(value: PublicKey) -> Self {
        match value {
            PublicKey::Secp256k1(public_key) => Self::Secp256k1 {
                value: public_key.0,
            },
            PublicKey::Ed25519(public_key) => Self::Ed25519 {
                value: public_key.0,
            },
        }
    }
}

impl From<SerializablePublicKey> for PublicKey {
    fn from(value: SerializablePublicKey) -> Self {
        match value {
            SerializablePublicKey::Secp256k1 { value } => {
                Self::Secp256k1(Secp256k1PublicKey(value))
            }
            SerializablePublicKey::Ed25519 { value } => Self::Ed25519(Ed25519PublicKey(value)),
        }
    }
}

#[serde_as]
#[derive(Serialize, Deserialize, JsonSchema, Clone)]
#[schemars(transparent)]
#[serde(transparent)]
pub struct SerializableSecp256k1PublicKey(
    #[schemars(with = "String")]
    #[serde_as(as = "serde_with::hex::Hex")]
    [u8; Secp256k1PublicKey::LENGTH],
);

impl From<SerializableSecp256k1PublicKey> for Secp256k1PublicKey {
    fn from(value: SerializableSecp256k1PublicKey) -> Self {
        Self(value.0)
    }
}

impl From<Secp256k1PublicKey> for SerializableSecp256k1PublicKey {
    fn from(value: Secp256k1PublicKey) -> Self {
        Self(value.0)
    }
}

#[serde_as]
#[derive(Serialize, Deserialize, JsonSchema)]
#[schemars(transparent)]
#[serde(transparent)]
pub struct SerializableEd25519PublicKey(
    #[schemars(with = "String")]
    #[serde_as(as = "serde_with::hex::Hex")]
    [u8; Ed25519PublicKey::LENGTH],
);

impl From<SerializableEd25519PublicKey> for Ed25519PublicKey {
    fn from(value: SerializableEd25519PublicKey) -> Self {
        Self(value.0)
    }
}

impl From<Ed25519PublicKey> for SerializableEd25519PublicKey {
    fn from(value: Ed25519PublicKey) -> Self {
        Self(value.0)
    }
}
