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

use std::hash::Hash;
use std::ops::Deref;

use ::indexmap::IndexMap;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use transaction::prelude::{
    AesGcmPayload, AesWrapped128BitKey, CurveType, DecryptorsByCurve,
    EncryptedMessageV1, MessageContentsV1, MessageV1, PlaintextMessageV1,
    PublicKeyFingerprint,
};

use crate::prelude::*;

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
#[serde(tag = "kind", content = "value")]
pub enum SerializableMessage {
    None,
    PlainText(SerializablePlainTextMessage),
    Encrypted(SerializableEncryptedMessage),
}

#[typeshare::typeshare]
#[derive(
    Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq, Hash,
)]
pub struct SerializablePlainTextMessage {
    pub mime_type: String,
    pub message: SerializableMessageContent,
}

#[typeshare::typeshare]
#[derive(
    Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq, Hash,
)]
#[serde(tag = "kind", content = "value")]
pub enum SerializableMessageContent {
    String(String),
    Bytes(SerializableBytes),
}

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct SerializableEncryptedMessage {
    pub encrypted: SerializableBytes,
    #[typeshare(
        serialized_as = "HashMap<SerializableCurveType, SerializableDecryptorsByCurve>"
    )]
    pub decryptors_by_curve:
        IndexMap<SerializableCurveType, SerializableDecryptorsByCurve>,
}

#[typeshare::typeshare]
#[derive(
    Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq, Hash,
)]
pub enum SerializableCurveType {
    Ed25519,
    Secp256k1,
}

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
#[serde(tag = "kind", content = "value")]
pub enum SerializableDecryptorsByCurve {
    Ed25519 {
        dh_ephemeral_public_key: SerializableEd25519PublicKey,
        #[typeshare(
            serialized_as = "HashMap<SerializablePublicKeyFingerprint, SerializableAesWrapped128BitKey>"
        )]
        decryptors: IndexMap<
            SerializablePublicKeyFingerprint,
            SerializableAesWrapped128BitKey,
        >,
    },
    Secp256k1 {
        dh_ephemeral_public_key: SerializableSecp256k1PublicKey,
        #[typeshare(
            serialized_as = "HashMap<SerializablePublicKeyFingerprint, SerializableAesWrapped128BitKey>"
        )]
        decryptors: IndexMap<
            SerializablePublicKeyFingerprint,
            SerializableAesWrapped128BitKey,
        >,
    },
}

pub type SerializableAesWrapped128BitKey =
    AsHex<[u8; AesWrapped128BitKey::LENGTH]>;
pub type SerializablePublicKeyFingerprint =
    AsHex<[u8; PublicKeyFingerprint::LENGTH]>;

#[allow(dead_code)]
mod __private {
    #[typeshare::typeshare]
    pub type SerializableAesWrapped128BitKey = String;
    #[typeshare::typeshare]
    pub type SerializablePublicKeyFingerprint = String;
}

//==================
// From Trait Impls
//==================

impl From<SerializableMessage> for MessageV1 {
    fn from(value: SerializableMessage) -> Self {
        match value {
            SerializableMessage::None => Self::None,
            SerializableMessage::Encrypted(value) => {
                Self::Encrypted(value.into())
            }
            SerializableMessage::PlainText(value) => {
                Self::Plaintext(value.into())
            }
        }
    }
}

impl From<MessageV1> for SerializableMessage {
    fn from(value: MessageV1) -> Self {
        match value {
            MessageV1::None => Self::None,
            MessageV1::Encrypted(value) => Self::Encrypted(value.into()),
            MessageV1::Plaintext(value) => Self::PlainText(value.into()),
        }
    }
}

impl From<SerializablePlainTextMessage> for PlaintextMessageV1 {
    fn from(value: SerializablePlainTextMessage) -> Self {
        Self {
            message: value.message.into(),
            mime_type: value.mime_type,
        }
    }
}

impl From<PlaintextMessageV1> for SerializablePlainTextMessage {
    fn from(value: PlaintextMessageV1) -> Self {
        Self {
            message: value.message.into(),
            mime_type: value.mime_type,
        }
    }
}

impl From<SerializableMessageContent> for MessageContentsV1 {
    fn from(value: SerializableMessageContent) -> Self {
        match value {
            SerializableMessageContent::Bytes(value) => {
                MessageContentsV1::Bytes(value.deref().clone())
            }
            SerializableMessageContent::String(value) => {
                MessageContentsV1::String(value)
            }
        }
    }
}

impl From<MessageContentsV1> for SerializableMessageContent {
    fn from(value: MessageContentsV1) -> Self {
        match value {
            MessageContentsV1::Bytes(value) => Self::Bytes(value.into()),
            MessageContentsV1::String(value) => Self::String(value),
        }
    }
}

impl From<SerializableEncryptedMessage> for EncryptedMessageV1 {
    fn from(value: SerializableEncryptedMessage) -> Self {
        Self {
            encrypted: AesGcmPayload(value.encrypted.deref().clone()),
            decryptors_by_curve: map_into!(value.decryptors_by_curve),
        }
    }
}

impl From<EncryptedMessageV1> for SerializableEncryptedMessage {
    fn from(value: EncryptedMessageV1) -> Self {
        Self {
            encrypted: value.encrypted.0.into(),
            decryptors_by_curve: map_into!(value.decryptors_by_curve),
        }
    }
}

impl From<SerializableCurveType> for CurveType {
    fn from(value: SerializableCurveType) -> Self {
        match value {
            SerializableCurveType::Ed25519 => Self::Ed25519,
            SerializableCurveType::Secp256k1 => Self::Secp256k1,
        }
    }
}

impl From<CurveType> for SerializableCurveType {
    fn from(value: CurveType) -> Self {
        match value {
            CurveType::Ed25519 => Self::Ed25519,
            CurveType::Secp256k1 => Self::Secp256k1,
        }
    }
}

impl From<SerializableDecryptorsByCurve> for DecryptorsByCurve {
    fn from(value: SerializableDecryptorsByCurve) -> Self {
        match value {
            SerializableDecryptorsByCurve::Ed25519 {
                dh_ephemeral_public_key,
                decryptors,
            } => Self::Ed25519 {
                dh_ephemeral_public_key: dh_ephemeral_public_key.into(),
                decryptors: map_into!(decryptors),
            },
            SerializableDecryptorsByCurve::Secp256k1 {
                dh_ephemeral_public_key,
                decryptors,
            } => Self::Secp256k1 {
                dh_ephemeral_public_key: dh_ephemeral_public_key.into(),
                decryptors: map_into!(decryptors),
            },
        }
    }
}

impl From<DecryptorsByCurve> for SerializableDecryptorsByCurve {
    fn from(value: DecryptorsByCurve) -> Self {
        match value {
            DecryptorsByCurve::Ed25519 {
                dh_ephemeral_public_key,
                decryptors,
            } => Self::Ed25519 {
                dh_ephemeral_public_key: dh_ephemeral_public_key.into(),
                decryptors: map_into!(decryptors),
            },
            DecryptorsByCurve::Secp256k1 {
                dh_ephemeral_public_key,
                decryptors,
            } => Self::Secp256k1 {
                dh_ephemeral_public_key: dh_ephemeral_public_key.into(),
                decryptors: map_into!(decryptors),
            },
        }
    }
}

impl From<AesWrapped128BitKey> for SerializableAesWrapped128BitKey {
    fn from(value: AesWrapped128BitKey) -> Self {
        value.0.into()
    }
}

impl From<SerializableAesWrapped128BitKey> for AesWrapped128BitKey {
    fn from(value: SerializableAesWrapped128BitKey) -> Self {
        Self(*value)
    }
}

impl From<PublicKeyFingerprint> for SerializablePublicKeyFingerprint {
    fn from(value: PublicKeyFingerprint) -> Self {
        value.0.into()
    }
}

impl From<SerializablePublicKeyFingerprint> for PublicKeyFingerprint {
    fn from(value: SerializablePublicKeyFingerprint) -> Self {
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
