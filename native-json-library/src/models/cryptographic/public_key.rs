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

use radix_engine_common::prelude::{EcdsaSecp256k1PublicKey, EddsaEd25519PublicKey, PublicKey};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Serialize, Deserialize, JsonSchema, Clone)]
#[serde(tag = "kind")]
pub enum SerializablePublicKey {
    EcdsaSecp256k1 {
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::hex::Hex")]
        value: [u8; EcdsaSecp256k1PublicKey::LENGTH],
    },
    EddsaEd25519 {
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::hex::Hex")]
        value: [u8; EddsaEd25519PublicKey::LENGTH],
    },
}

impl From<PublicKey> for SerializablePublicKey {
    fn from(value: PublicKey) -> Self {
        match value {
            PublicKey::EcdsaSecp256k1(public_key) => Self::EcdsaSecp256k1 {
                value: public_key.0,
            },
            PublicKey::EddsaEd25519(public_key) => Self::EddsaEd25519 {
                value: public_key.0,
            },
        }
    }
}

impl From<SerializablePublicKey> for PublicKey {
    fn from(value: SerializablePublicKey) -> Self {
        match value {
            SerializablePublicKey::EcdsaSecp256k1 { value } => {
                Self::EcdsaSecp256k1(EcdsaSecp256k1PublicKey(value))
            }
            SerializablePublicKey::EddsaEd25519 { value } => {
                Self::EddsaEd25519(EddsaEd25519PublicKey(value))
            }
        }
    }
}

#[serde_as]
#[derive(Serialize, Deserialize, JsonSchema, Clone)]
#[schemars(transparent)]
#[serde(transparent)]
pub struct SerializableEcdsaSecp256k1PublicKey(
    #[schemars(with = "String")]
    #[serde_as(as = "serde_with::hex::Hex")]
    [u8; EcdsaSecp256k1PublicKey::LENGTH],
);

impl From<SerializableEcdsaSecp256k1PublicKey> for EcdsaSecp256k1PublicKey {
    fn from(value: SerializableEcdsaSecp256k1PublicKey) -> Self {
        Self(value.0)
    }
}

impl From<EcdsaSecp256k1PublicKey> for SerializableEcdsaSecp256k1PublicKey {
    fn from(value: EcdsaSecp256k1PublicKey) -> Self {
        Self(value.0)
    }
}

#[serde_as]
#[derive(Serialize, Deserialize, JsonSchema)]
#[schemars(transparent)]
#[serde(transparent)]
pub struct SerializableEddsaEd25519PublicKey(
    #[schemars(with = "String")]
    #[serde_as(as = "serde_with::hex::Hex")]
    [u8; EddsaEd25519PublicKey::LENGTH],
);

impl From<SerializableEddsaEd25519PublicKey> for EddsaEd25519PublicKey {
    fn from(value: SerializableEddsaEd25519PublicKey) -> Self {
        Self(value.0)
    }
}

impl From<EddsaEd25519PublicKey> for SerializableEddsaEd25519PublicKey {
    fn from(value: EddsaEd25519PublicKey) -> Self {
        Self(value.0)
    }
}
