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

#[derive(Clone, Enum, Object)]
pub enum PublicKey {
    Secp256k1 { value: Vec<u8> },
    Ed25519 { value: Vec<u8> },
}

#[uniffi::export]
impl PublicKey {
    pub fn bytes(&self) -> Vec<u8> {
        match self {
            Self::Ed25519 { value } => value.clone(),
            Self::Secp256k1 { value } => value.clone(),
        }
    }

    pub fn hex(&self) -> String {
        let bytes = self.bytes();
        hex::encode(bytes)
    }

    pub fn curve(&self) -> Curve {
        match self {
            Self::Ed25519 { .. } => Curve::Ed25519,
            Self::Secp256k1 { .. } => Curve::Secp256k1,
        }
    }

    pub fn hash(&self) -> Result<PublicKeyHash, CryptographyConversionError> {
        let native = NativePublicKey::try_from(self.clone())?;
        Ok(native.get_hash().into())
    }
}

impl From<NativePublicKey> for PublicKey {
    fn from(value: NativePublicKey) -> Self {
        match value {
            NativePublicKey::Secp256k1(NativeSecp256k1PublicKey(value)) => Self::Secp256k1 {
                value: value.to_vec(),
            },
            NativePublicKey::Ed25519(NativeEd25519PublicKey(value)) => Self::Ed25519 {
                value: value.to_vec(),
            },
        }
    }
}

impl TryFrom<PublicKey> for NativePublicKey {
    type Error = CryptographyConversionError;

    fn try_from(value: PublicKey) -> Result<Self, Self::Error> {
        match value {
            PublicKey::Ed25519 { value } => value
                .try_into()
                .map(NativeEd25519PublicKey)
                .map(Self::Ed25519)
                .map_err(|value| CryptographyConversionError::InvalidLength {
                    expected: NativeEd25519PublicKey::LENGTH as u64,
                    actual: value.len() as u64,
                    data: value,
                }),
            PublicKey::Secp256k1 { value } => value
                .try_into()
                .map(NativeSecp256k1PublicKey)
                .map(Self::Secp256k1)
                .map_err(|value| CryptographyConversionError::InvalidLength {
                    expected: NativeSecp256k1PublicKey::LENGTH as u64,
                    actual: value.len() as u64,
                    data: value,
                }),
        }
    }
}
