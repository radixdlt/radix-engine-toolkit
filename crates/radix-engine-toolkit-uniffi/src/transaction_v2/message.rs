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

#[derive(Clone, Debug, Enum, Default)]
pub enum MessageV2 {
    #[default]
    None,
    PlainText {
        value: PlainTextMessageV2,
    },
    Encrypted {
        value: EncryptedMessageV2,
    },
}

#[derive(Clone, Debug, Record)]
pub struct PlainTextMessageV2 {
    pub mime_type: String,
    pub message: MessageContentsV2,
}

#[derive(Clone, Debug, Enum)]
pub enum MessageContentsV2 {
    Str { value: String },
    Bytes { value: Vec<u8> },
}

#[derive(Clone, Debug, Record)]
pub struct EncryptedMessageV2 {
    pub encrypted: Vec<u8>,
    pub decryptors_by_curve: HashMap<CurveTypeV2, DecryptorsByCurveV2>,
}

#[derive(Clone, Debug, Enum, Hash, PartialEq, Eq)]
pub enum CurveTypeV2 {
    Ed25519,
    Secp256k1,
}

#[derive(Clone, Debug, Enum)]
pub enum DecryptorsByCurveV2 {
    Ed25519 {
        dh_ephemeral_public_key: Ed25519PublicKey,
        decryptors:
            HashMap<PublicKeyFingerprint, AesWrappedVariableLengthKeyV2>,
    },
    Secp256k1 {
        dh_ephemeral_public_key: Secp256k1PublicKey,
        decryptors:
            HashMap<PublicKeyFingerprint, AesWrappedVariableLengthKeyV2>,
    },
}

pub type AesWrappedVariableLengthKeyV2 = Vec<u8>;

#[derive(Clone, Debug, Hash, Eq, PartialEq, Record)]
pub struct PublicKeyFingerprint {
    bytes: HashableBytes,
}

// required for conversion tests on bindgen side
#[uniffi::export]
pub fn public_key_fingerprint_v2_from_vec(
    bytes: Vec<u8>,
) -> PublicKeyFingerprint {
    PublicKeyFingerprint {
        bytes: HashableBytes(bytes),
    }
}

// required for conversion tests on bindgen side
#[uniffi::export]
pub fn public_key_fingerprint_v2_to_vec(
    value: PublicKeyFingerprint,
) -> Vec<u8> {
    value.bytes.0
}

//==================
// From Trait Impls
//==================

impl From<engine::DecryptorsByCurveV2> for DecryptorsByCurveV2 {
    fn from(value: engine::DecryptorsByCurveV2) -> Self {
        match value {
            engine::DecryptorsByCurveV2::Secp256k1 {
                dh_ephemeral_public_key,
                decryptors,
            } => Self::Secp256k1 {
                dh_ephemeral_public_key: dh_ephemeral_public_key.into(),
                decryptors: decryptors
                    .into_iter()
                    .map(|(key, value)| {
                        (
                            public_key_fingerprint_v2_from_vec(key.0.into()),
                            value.0.into(),
                        )
                    })
                    .collect(),
            },
            engine::DecryptorsByCurveV2::Ed25519 {
                dh_ephemeral_public_key,
                decryptors,
            } => Self::Ed25519 {
                dh_ephemeral_public_key: dh_ephemeral_public_key.into(),
                decryptors: decryptors
                    .into_iter()
                    .map(|(key, value)| {
                        (
                            public_key_fingerprint_v2_from_vec(key.0.into()),
                            value.0.into(),
                        )
                    })
                    .collect(),
            },
        }
    }
}

impl TryFrom<DecryptorsByCurveV2> for engine::DecryptorsByCurveV2 {
    type Error = RadixEngineToolkitError;

    fn try_from(value: DecryptorsByCurveV2) -> Result<Self> {
        match value {
            DecryptorsByCurveV2::Ed25519 {
                dh_ephemeral_public_key,
                decryptors,
            } => Ok(Self::Ed25519 {
                dh_ephemeral_public_key: dh_ephemeral_public_key.try_into()?,
                decryptors: decryptors
                    .into_iter()
                    .map(|(key, value)| {
                        public_key_fingerprint_v2_to_vec(key)
                            .try_into()
                            .map(engine::PublicKeyFingerprint)
                            .map_err(|value| RadixEngineToolkitError::InvalidLength {
                                expected: engine::PublicKeyFingerprint::LENGTH as u64,
                                actual: value.len() as u64,
                                data: value,
                            })
                            .and_then(|key| {
                                value
                                    .try_into()
                                    .map(engine::AesWrapped256BitKey)
                                    .map_err(|value| {
                                        RadixEngineToolkitError::InvalidLength {
                                            expected: engine::AesWrapped256BitKey::LENGTH
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
            DecryptorsByCurveV2::Secp256k1 {
                dh_ephemeral_public_key,
                decryptors,
            } => Ok(Self::Secp256k1 {
                dh_ephemeral_public_key: dh_ephemeral_public_key.try_into()?,
                decryptors: decryptors
                    .into_iter()
                    .map(|(key, value)| {
                        public_key_fingerprint_v2_to_vec(key)
                            .try_into()
                            .map(engine::PublicKeyFingerprint)
                            .map_err(|value| RadixEngineToolkitError::InvalidLength {
                                expected: engine::PublicKeyFingerprint::LENGTH as u64,
                                actual: value.len() as u64,
                                data: value,
                            })
                            .and_then(|key| {
                                value
                                    .try_into()
                                    .map(engine::AesWrapped256BitKey)
                                    .map_err(|value| {
                                        RadixEngineToolkitError::InvalidLength {
                                            expected: engine::AesWrapped256BitKey::LENGTH
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

impl From<CurveTypeV2> for engine::CurveType {
    fn from(value: CurveTypeV2) -> Self {
        match value {
            CurveTypeV2::Ed25519 => Self::Ed25519,
            CurveTypeV2::Secp256k1 => Self::Secp256k1,
        }
    }
}

impl From<engine::CurveType> for CurveTypeV2 {
    fn from(value: engine::CurveType) -> Self {
        match value {
            engine::CurveType::Ed25519 => Self::Ed25519,
            engine::CurveType::Secp256k1 => Self::Secp256k1,
        }
    }
}

impl From<engine::EncryptedMessageV2> for EncryptedMessageV2 {
    fn from(
        engine::EncryptedMessageV2 {
            encrypted,
            decryptors_by_curve,
        }: engine::EncryptedMessageV2,
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

impl TryFrom<EncryptedMessageV2> for engine::EncryptedMessageV2 {
    type Error = RadixEngineToolkitError;

    fn try_from(
        EncryptedMessageV2 {
            encrypted,
            decryptors_by_curve,
        }: EncryptedMessageV2,
    ) -> Result<Self> {
        let encrypted = engine::AesGcmPayload(encrypted);
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

impl From<MessageContentsV2> for engine::MessageContentsV1 {
    fn from(value: MessageContentsV2) -> Self {
        match value {
            MessageContentsV2::Str { value } => Self::String(value),
            MessageContentsV2::Bytes { value } => Self::Bytes(value),
        }
    }
}

impl From<engine::MessageContentsV1> for MessageContentsV2 {
    fn from(value: engine::MessageContentsV1) -> Self {
        match value {
            engine::MessageContentsV1::String(value) => Self::Str { value },
            engine::MessageContentsV1::Bytes(value) => Self::Bytes { value },
        }
    }
}

impl From<PlainTextMessageV2> for engine::PlaintextMessageV1 {
    fn from(
        PlainTextMessageV2 { message, mime_type }: PlainTextMessageV2,
    ) -> Self {
        Self {
            message: message.into(),
            mime_type,
        }
    }
}

impl From<engine::PlaintextMessageV1> for PlainTextMessageV2 {
    fn from(
        engine::PlaintextMessageV1 { message, mime_type }: engine::PlaintextMessageV1,
    ) -> Self {
        Self {
            message: message.into(),
            mime_type,
        }
    }
}

impl TryFrom<MessageV2> for engine::MessageV2 {
    type Error = RadixEngineToolkitError;

    fn try_from(value: MessageV2) -> Result<Self> {
        match value {
            MessageV2::None => Ok(engine::MessageV2::None),
            MessageV2::Encrypted { value } => {
                value.try_into().map(engine::MessageV2::Encrypted)
            }
            MessageV2::PlainText { value } => {
                Ok(engine::MessageV2::Plaintext(value.into()))
            }
        }
    }
}

impl From<engine::MessageV2> for MessageV2 {
    fn from(value: engine::MessageV2) -> Self {
        match value {
            engine::MessageV2::None => Self::None,
            engine::MessageV2::Encrypted(value) => Self::Encrypted {
                value: value.into(),
            },
            engine::MessageV2::Plaintext(value) => Self::PlainText {
                value: value.into(),
            },
        }
    }
}
