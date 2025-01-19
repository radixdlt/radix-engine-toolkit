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

#[derive(Clone, Enum, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PublicKey {
    Secp256k1 { value: Vec<u8> },
    Ed25519 { value: Vec<u8> },
}

#[derive(Clone, Record, Debug)]
pub struct Ed25519PublicKey {
    value: Vec<u8>,
}

#[derive(Clone, Record, Debug)]
pub struct Secp256k1PublicKey {
    value: Vec<u8>,
}

impl From<engine::PublicKey> for PublicKey {
    fn from(value: engine::PublicKey) -> Self {
        match value {
            engine::PublicKey::Secp256k1(engine::Secp256k1PublicKey(value)) => {
                Self::Secp256k1 {
                    value: value.to_vec(),
                }
            }
            engine::PublicKey::Ed25519(engine::Ed25519PublicKey(value)) => {
                Self::Ed25519 {
                    value: value.to_vec(),
                }
            }
        }
    }
}

impl TryFrom<PublicKey> for engine::PublicKey {
    type Error = RadixEngineToolkitError;

    fn try_from(value: PublicKey) -> Result<Self> {
        match value {
            PublicKey::Ed25519 { value } => value
                .try_into()
                .map(engine::Ed25519PublicKey)
                .map(Self::Ed25519)
                .map_err(|value| RadixEngineToolkitError::InvalidLength {
                    expected: engine::Ed25519PublicKey::LENGTH as u64,
                    actual: value.len() as u64,
                    data: value,
                }),
            PublicKey::Secp256k1 { value } => value
                .try_into()
                .map(engine::Secp256k1PublicKey)
                .map(Self::Secp256k1)
                .map_err(|value| RadixEngineToolkitError::InvalidLength {
                    expected: engine::Secp256k1PublicKey::LENGTH as u64,
                    actual: value.len() as u64,
                    data: value,
                }),
        }
    }
}

impl From<engine::Ed25519PublicKey> for Ed25519PublicKey {
    fn from(value: engine::Ed25519PublicKey) -> Self {
        Self {
            value: value.0.to_vec(),
        }
    }
}

impl TryFrom<Ed25519PublicKey> for engine::Ed25519PublicKey {
    type Error = RadixEngineToolkitError;

    fn try_from(value: Ed25519PublicKey) -> Result<Self> {
        value
            .value
            .try_into()
            .map(engine::Ed25519PublicKey)
            .map_err(|value| RadixEngineToolkitError::InvalidLength {
                expected: engine::Secp256k1PublicKey::LENGTH as u64,
                actual: value.len() as u64,
                data: value,
            })
    }
}

impl From<engine::Secp256k1PublicKey> for Secp256k1PublicKey {
    fn from(value: engine::Secp256k1PublicKey) -> Self {
        Self {
            value: value.0.to_vec(),
        }
    }
}

impl TryFrom<Secp256k1PublicKey> for engine::Secp256k1PublicKey {
    type Error = RadixEngineToolkitError;

    fn try_from(value: Secp256k1PublicKey) -> Result<Self> {
        value
            .value
            .try_into()
            .map(engine::Secp256k1PublicKey)
            .map_err(|value| RadixEngineToolkitError::InvalidLength {
                expected: engine::Secp256k1PublicKey::LENGTH as u64,
                actual: value.len() as u64,
                data: value,
            })
    }
}
