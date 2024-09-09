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

#[derive(Clone, Debug, Enum)]
pub enum MessageV1 {
    None,
    PlainText { value: PlainTextMessageV1 },
    Encrypted { value: EncryptedMessageV1 },
}

#[derive(Clone, Debug, Record)]
pub struct PlainTextMessageV1 {
    pub mime_type: String,
    pub message: MessageContentV1,
}

#[derive(Clone, Debug, Enum)]
pub enum MessageContentV1 {
    Str { value: String },
    Bytes { value: Vec<u8> },
}

#[derive(Clone, Debug, Record)]
pub struct EncryptedMessageV1 {
    pub encrypted: Vec<u8>,
    pub decryptors_by_curve: HashMap<CurveType, DecryptorsByCurve>,
}

#[derive(Clone, Debug, Enum, Hash, PartialEq, Eq)]
pub enum CurveType {
    Ed25519,
    Secp256k1,
}

#[derive(Clone, Debug, Enum)]
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
                            .map_err(|value| RadixEngineToolkitError::InvalidLength {
                                expected: NativePublicKeyFingerprint::LENGTH as u64,
                                actual: value.len() as u64,
                                data: value,
                            })
                            .and_then(|key| {
                                value
                                    .try_into()
                                    .map(NativeAesWrapped128BitKey)
                                    .map_err(|value| {
                                        RadixEngineToolkitError::InvalidLength {
                                            expected: NativeAesWrapped128BitKey::LENGTH
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
                            .map_err(|value| RadixEngineToolkitError::InvalidLength {
                                expected: NativePublicKeyFingerprint::LENGTH as u64,
                                actual: value.len() as u64,
                                data: value,
                            })
                            .and_then(|key| {
                                value
                                    .try_into()
                                    .map(NativeAesWrapped128BitKey)
                                    .map_err(|value| {
                                        RadixEngineToolkitError::InvalidLength {
                                            expected: NativeAesWrapped128BitKey::LENGTH
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

impl From<NativeEncryptedMessage> for EncryptedMessageV1 {
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

impl TryFrom<EncryptedMessageV1> for NativeEncryptedMessage {
    type Error = RadixEngineToolkitError;

    fn try_from(
        EncryptedMessageV1 {
            encrypted,
            decryptors_by_curve,
        }: EncryptedMessageV1,
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

impl From<MessageContentV1> for NativeMessageContents {
    fn from(value: MessageContentV1) -> Self {
        match value {
            MessageContentV1::Str { value } => Self::String(value),
            MessageContentV1::Bytes { value } => Self::Bytes(value),
        }
    }
}

impl From<NativeMessageContents> for MessageContentV1 {
    fn from(value: NativeMessageContents) -> Self {
        match value {
            NativeMessageContents::String(value) => Self::Str { value },
            NativeMessageContents::Bytes(value) => Self::Bytes { value },
        }
    }
}

impl From<PlainTextMessageV1> for NativePlaintextMessage {
    fn from(
        PlainTextMessageV1 { message, mime_type }: PlainTextMessageV1,
    ) -> Self {
        Self {
            message: message.into(),
            mime_type,
        }
    }
}

impl From<NativePlaintextMessage> for PlainTextMessageV1 {
    fn from(
        NativePlaintextMessage { message, mime_type }: NativePlaintextMessage,
    ) -> Self {
        Self {
            message: message.into(),
            mime_type,
        }
    }
}

impl TryFrom<MessageV1> for NativeMessageV1 {
    type Error = RadixEngineToolkitError;

    fn try_from(value: MessageV1) -> Result<Self> {
        match value {
            MessageV1::None => Ok(NativeMessageV1::None),
            MessageV1::Encrypted { value } => {
                value.try_into().map(NativeMessageV1::Encrypted)
            }
            MessageV1::PlainText { value } => {
                Ok(NativeMessageV1::Plaintext(value.into()))
            }
        }
    }
}

impl From<NativeMessageV1> for MessageV1 {
    fn from(value: NativeMessageV1) -> Self {
        match value {
            NativeMessageV1::None => Self::None,
            NativeMessageV1::Encrypted(value) => Self::Encrypted {
                value: value.into(),
            },
            NativeMessageV1::Plaintext(value) => Self::PlainText {
                value: value.into(),
            },
        }
    }
}
