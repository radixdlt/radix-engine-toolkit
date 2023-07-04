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
    EcdsaSecp256k1 { value: Vec<u8> },
    EddsaEd25519 { value: Vec<u8> },
}

impl From<NativePublicKeyHash> for PublicKeyHash {
    fn from(value: NativePublicKeyHash) -> Self {
        match value {
            NativePublicKeyHash::EcdsaSecp256k1(NativeEcdsaSecp256k1PublicKeyHash(value)) => {
                Self::EcdsaSecp256k1 {
                    value: value.to_vec(),
                }
            }
            NativePublicKeyHash::EddsaEd25519(NativeEddsaEd25519PublicKeyHash(value)) => {
                Self::EddsaEd25519 {
                    value: value.to_vec(),
                }
            }
        }
    }
}

impl TryFrom<PublicKeyHash> for NativePublicKeyHash {
    type Error = RadixEngineToolkitError;

    fn try_from(value: PublicKeyHash) -> Result<Self> {
        match value {
            PublicKeyHash::EddsaEd25519 { value } => value
                .try_into()
                .map(NativeEddsaEd25519PublicKeyHash)
                .map(Self::EddsaEd25519)
                .map_err(|value| RadixEngineToolkitError::InvalidLength {
                    expected: NativeNodeId::UUID_LENGTH as u64,
                    actual: value.len() as u64,
                    data: value,
                }),
            PublicKeyHash::EcdsaSecp256k1 { value } => value
                .try_into()
                .map(NativeEcdsaSecp256k1PublicKeyHash)
                .map(Self::EcdsaSecp256k1)
                .map_err(|value| RadixEngineToolkitError::InvalidLength {
                    expected: NativeNodeId::UUID_LENGTH as u64,
                    actual: value.len() as u64,
                    data: value,
                }),
        }
    }
}
