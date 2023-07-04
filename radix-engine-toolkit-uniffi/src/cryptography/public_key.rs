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

#[derive(Clone, Enum, Debug)]
pub enum PublicKey {
    EcdsaSecp256k1 { value: Vec<u8> },
    EddsaEd25519 { value: Vec<u8> },
}

#[derive(Clone, Record, Debug)]
pub struct EddsaEd25519PublicKey {
    value: Vec<u8>,
}

#[derive(Clone, Record, Debug)]
pub struct EcdsaSecp256k1PublicKey {
    value: Vec<u8>,
}

impl From<NativePublicKey> for PublicKey {
    fn from(value: NativePublicKey) -> Self {
        match value {
            NativePublicKey::EcdsaSecp256k1(NativeEcdsaSecp256k1PublicKey(value)) => {
                Self::EcdsaSecp256k1 {
                    value: value.to_vec(),
                }
            }
            NativePublicKey::EddsaEd25519(NativeEddsaEd25519PublicKey(value)) => {
                Self::EddsaEd25519 {
                    value: value.to_vec(),
                }
            }
        }
    }
}

impl TryFrom<PublicKey> for NativePublicKey {
    type Error = RadixEngineToolkitError;

    fn try_from(value: PublicKey) -> Result<Self> {
        match value {
            PublicKey::EddsaEd25519 { value } => value
                .try_into()
                .map(NativeEddsaEd25519PublicKey)
                .map(Self::EddsaEd25519)
                .map_err(|value| RadixEngineToolkitError::InvalidLength {
                    expected: NativeEddsaEd25519PublicKey::LENGTH as u64,
                    actual: value.len() as u64,
                    data: value,
                }),
            PublicKey::EcdsaSecp256k1 { value } => value
                .try_into()
                .map(NativeEcdsaSecp256k1PublicKey)
                .map(Self::EcdsaSecp256k1)
                .map_err(|value| RadixEngineToolkitError::InvalidLength {
                    expected: NativeEcdsaSecp256k1PublicKey::LENGTH as u64,
                    actual: value.len() as u64,
                    data: value,
                }),
        }
    }
}

impl From<NativeEddsaEd25519PublicKey> for EddsaEd25519PublicKey {
    fn from(value: NativeEddsaEd25519PublicKey) -> Self {
        Self {
            value: value.0.to_vec(),
        }
    }
}

impl TryFrom<EddsaEd25519PublicKey> for NativeEddsaEd25519PublicKey {
    type Error = RadixEngineToolkitError;

    fn try_from(value: EddsaEd25519PublicKey) -> Result<Self> {
        value
            .value
            .try_into()
            .map(NativeEddsaEd25519PublicKey)
            .map_err(|value| RadixEngineToolkitError::InvalidLength {
                expected: NativeEcdsaSecp256k1PublicKey::LENGTH as u64,
                actual: value.len() as u64,
                data: value,
            })
    }
}

impl From<NativeEcdsaSecp256k1PublicKey> for EcdsaSecp256k1PublicKey {
    fn from(value: NativeEcdsaSecp256k1PublicKey) -> Self {
        Self {
            value: value.0.to_vec(),
        }
    }
}

impl TryFrom<EcdsaSecp256k1PublicKey> for NativeEcdsaSecp256k1PublicKey {
    type Error = RadixEngineToolkitError;

    fn try_from(value: EcdsaSecp256k1PublicKey) -> Result<Self> {
        value
            .value
            .try_into()
            .map(NativeEcdsaSecp256k1PublicKey)
            .map_err(|value| RadixEngineToolkitError::InvalidLength {
                expected: NativeEcdsaSecp256k1PublicKey::LENGTH as u64,
                actual: value.len() as u64,
                data: value,
            })
    }
}
