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
    pub decryptors_by_curve: HashMap<CurveTypeV1, DecryptorsByCurveV1>,
}

#[derive(Clone, Debug, Enum, Hash, PartialEq, Eq)]
pub enum CurveTypeV1 {
    Ed25519,
    Secp256k1,
}

#[derive(Clone, Debug, Enum)]
pub enum DecryptorsByCurveV1 {
    Ed25519 {
        dh_ephemeral_public_key: Ed25519PublicKey,
        decryptors:
            HashMap<PublicKeyFingerprintV1, AesWrappedVariableLengthKeyV1>,
    },
    Secp256k1 {
        dh_ephemeral_public_key: Secp256k1PublicKey,
        decryptors:
            HashMap<PublicKeyFingerprintV1, AesWrappedVariableLengthKeyV1>,
    },
}

pub type AesWrappedVariableLengthKeyV1 = Vec<u8>;

#[derive(Clone, Debug, Hash, Eq, PartialEq, Record)]
pub struct PublicKeyFingerprintV1 {
    bytes: HashableBytes,
}

// required for conversion tests on bindgen side
#[uniffi::export]
pub fn public_key_fingerprint_v1_from_vec(
    bytes: Vec<u8>,
) -> PublicKeyFingerprintV1 {
    PublicKeyFingerprintV1 {
        bytes: HashableBytes(bytes),
    }
}

// required for conversion tests on bindgen side
#[uniffi::export]
pub fn public_key_fingerprint_v1_to_vec(
    value: PublicKeyFingerprintV1,
) -> Vec<u8> {
    value.bytes.0
}

//==================
// From Trait Impls
//==================

impl From<NativeDecryptorsByCurveV1> for DecryptorsByCurveV1 {
    fn from(value: NativeDecryptorsByCurveV1) -> Self {
        match value {
            NativeDecryptorsByCurveV1::Secp256k1 {
                dh_ephemeral_public_key,
                decryptors,
            } => Self::Secp256k1 {
                dh_ephemeral_public_key: dh_ephemeral_public_key.into(),
                decryptors: decryptors
                    .into_iter()
                    .map(|(key, value)| {
                        (
                            public_key_fingerprint_v1_from_vec(key.0.into()),
                            value.0.into(),
                        )
                    })
                    .collect(),
            },
            NativeDecryptorsByCurveV1::Ed25519 {
                dh_ephemeral_public_key,
                decryptors,
            } => Self::Ed25519 {
                dh_ephemeral_public_key: dh_ephemeral_public_key.into(),
                decryptors: decryptors
                    .into_iter()
                    .map(|(key, value)| {
                        (
                            public_key_fingerprint_v1_from_vec(key.0.into()),
                            value.0.into(),
                        )
                    })
                    .collect(),
            },
        }
    }
}

impl TryFrom<DecryptorsByCurveV1> for NativeDecryptorsByCurveV1 {
    type Error = RadixEngineToolkitError;

    fn try_from(value: DecryptorsByCurveV1) -> Result<Self> {
        match value {
            DecryptorsByCurveV1::Ed25519 {
                dh_ephemeral_public_key,
                decryptors,
            } => Ok(Self::Ed25519 {
                dh_ephemeral_public_key: dh_ephemeral_public_key.try_into()?,
                decryptors: decryptors
                    .into_iter()
                    .map(|(key, value)| {
                        public_key_fingerprint_v1_to_vec(key)
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
            DecryptorsByCurveV1::Secp256k1 {
                dh_ephemeral_public_key,
                decryptors,
            } => Ok(Self::Secp256k1 {
                dh_ephemeral_public_key: dh_ephemeral_public_key.try_into()?,
                decryptors: decryptors
                    .into_iter()
                    .map(|(key, value)| {
                        public_key_fingerprint_v1_to_vec(key)
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

impl From<CurveTypeV1> for NativeCurveType {
    fn from(value: CurveTypeV1) -> Self {
        match value {
            CurveTypeV1::Ed25519 => Self::Ed25519,
            CurveTypeV1::Secp256k1 => Self::Secp256k1,
        }
    }
}

impl From<NativeCurveType> for CurveTypeV1 {
    fn from(value: NativeCurveType) -> Self {
        match value {
            NativeCurveType::Ed25519 => Self::Ed25519,
            NativeCurveType::Secp256k1 => Self::Secp256k1,
        }
    }
}

impl From<NativeEncryptedMessageV1> for EncryptedMessageV1 {
    fn from(
        NativeEncryptedMessageV1 {
            encrypted,
            decryptors_by_curve,
        }: NativeEncryptedMessageV1,
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

impl TryFrom<EncryptedMessageV1> for NativeEncryptedMessageV1 {
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

impl From<MessageContentV1> for NativeMessageContentsV1 {
    fn from(value: MessageContentV1) -> Self {
        match value {
            MessageContentV1::Str { value } => Self::String(value),
            MessageContentV1::Bytes { value } => Self::Bytes(value),
        }
    }
}

impl From<NativeMessageContentsV1> for MessageContentV1 {
    fn from(value: NativeMessageContentsV1) -> Self {
        match value {
            NativeMessageContentsV1::String(value) => Self::Str { value },
            NativeMessageContentsV1::Bytes(value) => Self::Bytes { value },
        }
    }
}

impl From<PlainTextMessageV1> for NativePlaintextMessageV1 {
    fn from(
        PlainTextMessageV1 { message, mime_type }: PlainTextMessageV1,
    ) -> Self {
        Self {
            message: message.into(),
            mime_type,
        }
    }
}

impl From<NativePlaintextMessageV1> for PlainTextMessageV1 {
    fn from(
        NativePlaintextMessageV1 { message, mime_type }: NativePlaintextMessageV1,
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
