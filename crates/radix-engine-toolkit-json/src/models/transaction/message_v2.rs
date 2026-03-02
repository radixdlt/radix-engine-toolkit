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

use std::ops::Deref;

use ::indexmap::IndexMap;
use radix_transactions::prelude::{
    AesGcmPayload, AesWrapped256BitKey, DecryptorsByCurveV2,
    EncryptedMessageV2, MessageV2,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
#[serde(tag = "kind", content = "value")]
pub enum SerializableMessageV2 {
    None,
    PlainText(SerializablePlainTextMessage),
    Encrypted(SerializableEncryptedMessageV2),
}

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct SerializableEncryptedMessageV2 {
    pub encrypted: SerializableBytes,
    #[typeshare(
        serialized_as = "HashMap<SerializableCurveType, SerializableDecryptorsByCurveV2>"
    )]
    pub decryptors_by_curve:
        IndexMap<SerializableCurveType, SerializableDecryptorsByCurveV2>,
}

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
#[serde(tag = "kind", content = "value")]
pub enum SerializableDecryptorsByCurveV2 {
    Ed25519 {
        dh_ephemeral_public_key: SerializableEd25519PublicKey,
        #[typeshare(
            serialized_as = "HashMap<SerializablePublicKeyFingerprint, SerializableAesWrapped256BitKey>"
        )]
        decryptors: IndexMap<
            SerializablePublicKeyFingerprint,
            SerializableAesWrapped256BitKey,
        >,
    },
    Secp256k1 {
        dh_ephemeral_public_key: SerializableSecp256k1PublicKey,
        #[typeshare(
            serialized_as = "HashMap<SerializablePublicKeyFingerprint, SerializableAesWrapped256BitKey>"
        )]
        decryptors: IndexMap<
            SerializablePublicKeyFingerprint,
            SerializableAesWrapped256BitKey,
        >,
    },
}

pub type SerializableAesWrapped256BitKey =
    AsHex<[u8; AesWrapped256BitKey::LENGTH]>;

#[allow(dead_code)]
mod __private {
    #[typeshare::typeshare]
    pub type SerializableAesWrapped256BitKey = String;
}

//==================
// From Trait Impls
//==================

impl From<SerializableMessageV2> for MessageV2 {
    fn from(value: SerializableMessageV2) -> Self {
        match value {
            SerializableMessageV2::None => Self::None,
            SerializableMessageV2::Encrypted(value) => {
                Self::Encrypted(value.into())
            }
            SerializableMessageV2::PlainText(value) => {
                Self::Plaintext(value.into())
            }
        }
    }
}

impl From<MessageV2> for SerializableMessageV2 {
    fn from(value: MessageV2) -> Self {
        match value {
            MessageV2::None => Self::None,
            MessageV2::Encrypted(value) => Self::Encrypted(value.into()),
            MessageV2::Plaintext(value) => Self::PlainText(value.into()),
        }
    }
}

impl From<SerializableEncryptedMessageV2> for EncryptedMessageV2 {
    fn from(value: SerializableEncryptedMessageV2) -> Self {
        Self {
            encrypted: AesGcmPayload(value.encrypted.deref().clone()),
            decryptors_by_curve: map_into!(value.decryptors_by_curve),
        }
    }
}

impl From<EncryptedMessageV2> for SerializableEncryptedMessageV2 {
    fn from(value: EncryptedMessageV2) -> Self {
        Self {
            encrypted: value.encrypted.0.into(),
            decryptors_by_curve: map_into!(value.decryptors_by_curve),
        }
    }
}

impl From<SerializableDecryptorsByCurveV2> for DecryptorsByCurveV2 {
    fn from(value: SerializableDecryptorsByCurveV2) -> Self {
        match value {
            SerializableDecryptorsByCurveV2::Ed25519 {
                dh_ephemeral_public_key,
                decryptors,
            } => Self::Ed25519 {
                dh_ephemeral_public_key: dh_ephemeral_public_key.into(),
                decryptors: map_into!(decryptors),
            },
            SerializableDecryptorsByCurveV2::Secp256k1 {
                dh_ephemeral_public_key,
                decryptors,
            } => Self::Secp256k1 {
                dh_ephemeral_public_key: dh_ephemeral_public_key.into(),
                decryptors: map_into!(decryptors),
            },
        }
    }
}

impl From<DecryptorsByCurveV2> for SerializableDecryptorsByCurveV2 {
    fn from(value: DecryptorsByCurveV2) -> Self {
        match value {
            DecryptorsByCurveV2::Ed25519 {
                dh_ephemeral_public_key,
                decryptors,
            } => Self::Ed25519 {
                dh_ephemeral_public_key: dh_ephemeral_public_key.into(),
                decryptors: map_into!(decryptors),
            },
            DecryptorsByCurveV2::Secp256k1 {
                dh_ephemeral_public_key,
                decryptors,
            } => Self::Secp256k1 {
                dh_ephemeral_public_key: dh_ephemeral_public_key.into(),
                decryptors: map_into!(decryptors),
            },
        }
    }
}

impl From<AesWrapped256BitKey> for SerializableAesWrapped256BitKey {
    fn from(value: AesWrapped256BitKey) -> Self {
        value.0.into()
    }
}

impl From<SerializableAesWrapped256BitKey> for AesWrapped256BitKey {
    fn from(value: SerializableAesWrapped256BitKey) -> Self {
        Self(*value)
    }
}

macro_rules! map_into {
    ($map: expr) => {
        $map.into_iter()
            .map(|(key, value)| (key.into(), value.into()))
            .collect()
    };
}
use map_into;
