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
use crate::utils::hashable_bytes::HashableBytes;

#[derive(Clone, Debug, PartialEq, Eq, Enum)]
pub enum Message {
    None,
    PlainText { value: PlainTextMessage },
    Encrypted { value: EncryptedMessage },
}

#[derive(Clone, Debug, PartialEq, Eq, Record)]
pub struct PlainTextMessage {
    pub mime_type: String,
    pub message: MessageContent,
}

#[derive(Clone, Debug, PartialEq, Eq, Enum)]
pub enum MessageContent {
    Str { value: String },
    Bytes { value: Vec<u8> },
}

#[derive(Clone, Debug, PartialEq, Eq, Record)]
pub struct EncryptedMessage {
    pub encrypted: Vec<u8>,
    pub decryptors_by_curve: HashMap<CurveType, DecryptorsByCurve>,
}

#[derive(Clone, Debug, Enum, Hash, PartialEq, Eq)]
pub enum CurveType {
    Ed25519,
    Secp256k1,
}

#[derive(Clone, Debug, PartialEq, Eq, Enum)]
pub enum DecryptorsByCurve {
    Ed25519 {
        dh_ephemeral_public_key: Ed25519PublicKey,
        decryptors: HashMap<PublicKeyFingerprint, AesWrapped128BitKey>,
    },
    Secp256k1 {
        dh_ephemeral_public_key: Secp256k1PublicKey,
        decryptors: HashMap<PublicKeyFingerprint, AesWrapped128BitKey>,
    },
}

pub type AesWrapped128BitKey = Vec<u8>;

#[derive(Clone, Debug, Hash, Eq, PartialEq, Record)]
pub struct PublicKeyFingerprint {
    bytes: HashableBytes,
}

// required for conversion tests on bindgen side
#[uniffi::export]
pub fn public_key_fingerprint_from_vec(bytes: Vec<u8>) -> PublicKeyFingerprint {
    PublicKeyFingerprint {
        bytes: HashableBytes(bytes),
    }
}

// required for conversion tests on bindgen side
#[uniffi::export]
pub fn public_key_fingerprint_to_vec(value: PublicKeyFingerprint) -> Vec<u8> {
    value.bytes.0
}

//==================
// From Trait Impls
//==================

impl From<NativeDecryptorsByCurve> for DecryptorsByCurve {
    fn from(value: NativeDecryptorsByCurve) -> Self {
        match value {
            NativeDecryptorsByCurve::Secp256k1 {
                dh_ephemeral_public_key,
                decryptors,
            } => Self::Secp256k1 {
                dh_ephemeral_public_key: dh_ephemeral_public_key.into(),
                decryptors: decryptors
                    .into_iter()
                    .map(|(key, value)| {
                        (
                            public_key_fingerprint_from_vec(key.0.into()),
                            value.0.into(),
                        )
                    })
                    .collect(),
            },
            NativeDecryptorsByCurve::Ed25519 {
                dh_ephemeral_public_key,
                decryptors,
            } => Self::Ed25519 {
                dh_ephemeral_public_key: dh_ephemeral_public_key.into(),
                decryptors: decryptors
                    .into_iter()
                    .map(|(key, value)| {
                        (
                            public_key_fingerprint_from_vec(key.0.into()),
                            value.0.into(),
                        )
                    })
                    .collect(),
            },
        }
    }
}

impl TryFrom<DecryptorsByCurve> for NativeDecryptorsByCurve {
    type Error = RadixEngineToolkitError;

    fn try_from(value: DecryptorsByCurve) -> Result<Self> {
        match value {
            DecryptorsByCurve::Ed25519 {
                dh_ephemeral_public_key,
                decryptors,
            } => Ok(Self::Ed25519 {
                dh_ephemeral_public_key: dh_ephemeral_public_key.try_into()?,
                decryptors: decryptors
                    .into_iter()
                    .map(|(key, value)| {
                        public_key_fingerprint_to_vec(key)
                            .try_into()
                            .map(NativePublicKeyFingerprint)
                            .map_err(|value| {
                                RadixEngineToolkitError::InvalidLength {
                                    expected: NativePublicKeyFingerprint::LENGTH
                                        as u64,
                                    actual: value.len() as u64,
                                    data: value,
                                }
                            })
                            .and_then(|key| {
                                value
                                    .try_into()
                                    .map(NativeAesWrapped128BitKey)
                                    .map_err(|value| {
                                        RadixEngineToolkitError::InvalidLength {
                                            expected:
                                            NativeAesWrapped128BitKey::LENGTH
                                                    as u64,
                                            actual: value.len() as u64,
                                            data: value,
                                        }
                                    })
                                    .map(|value| (key, value))
                            })
                    })
                    .collect::<Result<_>>()?,
            }),
            DecryptorsByCurve::Secp256k1 {
                dh_ephemeral_public_key,
                decryptors,
            } => Ok(Self::Secp256k1 {
                dh_ephemeral_public_key: dh_ephemeral_public_key.try_into()?,
                decryptors: decryptors
                    .into_iter()
                    .map(|(key, value)| {
                        public_key_fingerprint_to_vec(key)
                            .try_into()
                            .map(NativePublicKeyFingerprint)
                            .map_err(|value| {
                                RadixEngineToolkitError::InvalidLength {
                                    expected: NativePublicKeyFingerprint::LENGTH
                                        as u64,
                                    actual: value.len() as u64,
                                    data: value,
                                }
                            })
                            .and_then(|key| {
                                value
                                    .try_into()
                                    .map(NativeAesWrapped128BitKey)
                                    .map_err(|value| {
                                        RadixEngineToolkitError::InvalidLength {
                                            expected:
                                            NativeAesWrapped128BitKey::LENGTH
                                                    as u64,
                                            actual: value.len() as u64,
                                            data: value,
                                        }
                                    })
                                    .map(|value| (key, value))
                            })
                    })
                    .collect::<Result<_>>()?,
            }),
        }
    }
}

impl From<CurveType> for NativeCurveType {
    fn from(value: CurveType) -> Self {
        match value {
            CurveType::Ed25519 => Self::Ed25519,
            CurveType::Secp256k1 => Self::Secp256k1,
        }
    }
}

impl From<NativeCurveType> for CurveType {
    fn from(value: NativeCurveType) -> Self {
        match value {
            NativeCurveType::Ed25519 => Self::Ed25519,
            NativeCurveType::Secp256k1 => Self::Secp256k1,
        }
    }
}

impl From<NativeEncryptedMessage> for EncryptedMessage {
    fn from(
        NativeEncryptedMessage {
            encrypted,
            decryptors_by_curve,
        }: NativeEncryptedMessage,
    ) -> Self {
        let encrypted = encrypted.0;
        let decryptors_by_curve = decryptors_by_curve
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();

        Self {
            encrypted,
            decryptors_by_curve,
        }
    }
}

impl TryFrom<EncryptedMessage> for NativeEncryptedMessage {
    type Error = RadixEngineToolkitError;

    fn try_from(
        EncryptedMessage {
            encrypted,
            decryptors_by_curve,
        }: EncryptedMessage,
    ) -> Result<Self> {
        let encrypted = NativeAesGcmPayload(encrypted);
        let decryptors_by_curve = decryptors_by_curve
            .into_iter()
            .map(|(k, v)| v.try_into().map(|v| (k.into(), v)))
            .collect::<Result<_>>()?;

        Ok(Self {
            encrypted,
            decryptors_by_curve,
        })
    }
}

impl From<MessageContent> for NativeMessageContents {
    fn from(value: MessageContent) -> Self {
        match value {
            MessageContent::Str { value } => Self::String(value),
            MessageContent::Bytes { value } => Self::Bytes(value),
        }
    }
}

impl From<NativeMessageContents> for MessageContent {
    fn from(value: NativeMessageContents) -> Self {
        match value {
            NativeMessageContents::String(value) => Self::Str { value },
            NativeMessageContents::Bytes(value) => Self::Bytes { value },
        }
    }
}

impl From<PlainTextMessage> for NativePlaintextMessage {
    fn from(PlainTextMessage { message, mime_type }: PlainTextMessage) -> Self {
        Self {
            message: message.into(),
            mime_type,
        }
    }
}

impl From<NativePlaintextMessage> for PlainTextMessage {
    fn from(
        NativePlaintextMessage { message, mime_type }: NativePlaintextMessage,
    ) -> Self {
        Self {
            message: message.into(),
            mime_type,
        }
    }
}

impl TryFrom<Message> for NativeMessage {
    type Error = RadixEngineToolkitError;

    fn try_from(value: Message) -> Result<Self> {
        match value {
            Message::None => Ok(NativeMessage::None),
            Message::Encrypted { value } => {
                value.try_into().map(NativeMessage::Encrypted)
            }
            Message::PlainText { value } => {
                Ok(NativeMessage::Plaintext(value.into()))
            }
        }
    }
}

impl From<NativeMessage> for Message {
    fn from(value: NativeMessage) -> Self {
        match value {
            NativeMessage::None => Self::None,
            NativeMessage::Encrypted(value) => Self::Encrypted {
                value: value.into(),
            },
            NativeMessage::Plaintext(value) => Self::PlainText {
                value: value.into(),
            },
        }
    }
}
