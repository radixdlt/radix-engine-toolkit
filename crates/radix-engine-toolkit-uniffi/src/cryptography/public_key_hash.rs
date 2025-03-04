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

#[derive(Clone, Debug, Enum)]
pub enum PublicKeyHash {
    Secp256k1 { value: Vec<u8> },
    Ed25519 { value: Vec<u8> },
}

impl From<engine::PublicKeyHash> for PublicKeyHash {
    fn from(value: engine::PublicKeyHash) -> Self {
        match value {
            engine::PublicKeyHash::Secp256k1(
                engine::Secp256k1PublicKeyHash(value),
            ) => Self::Secp256k1 {
                value: value.to_vec(),
            },
            engine::PublicKeyHash::Ed25519(engine::Ed25519PublicKeyHash(
                value,
            )) => Self::Ed25519 {
                value: value.to_vec(),
            },
        }
    }
}

impl TryFrom<PublicKeyHash> for engine::PublicKeyHash {
    type Error = RadixEngineToolkitError;

    fn try_from(value: PublicKeyHash) -> Result<Self> {
        match value {
            PublicKeyHash::Ed25519 { value } => value
                .try_into()
                .map(engine::Ed25519PublicKeyHash)
                .map(Self::Ed25519)
                .map_err(|value| RadixEngineToolkitError::InvalidLength {
                    expected: engine::NodeId::RID_LENGTH as u64,
                    actual: value.len() as u64,
                    data: value,
                }),
            PublicKeyHash::Secp256k1 { value } => value
                .try_into()
                .map(engine::Secp256k1PublicKeyHash)
                .map(Self::Secp256k1)
                .map_err(|value| RadixEngineToolkitError::InvalidLength {
                    expected: engine::NodeId::RID_LENGTH as u64,
                    actual: value.len() as u64,
                    data: value,
                }),
        }
    }
}
