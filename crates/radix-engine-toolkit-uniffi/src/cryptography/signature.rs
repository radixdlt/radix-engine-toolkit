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
pub enum Signature {
    Secp256k1 { value: Vec<u8> },
    Ed25519 { value: Vec<u8> },
}

impl From<NativeSignature> for Signature {
    fn from(value: NativeSignature) -> Self {
        match value {
            NativeSignature::Secp256k1(NativeSecp256k1Signature(value)) => {
                Self::Secp256k1 {
                    value: value.to_vec(),
                }
            }
            NativeSignature::Ed25519(NativeEd25519Signature(value)) => {
                Self::Ed25519 {
                    value: value.to_vec(),
                }
            }
        }
    }
}

impl TryFrom<Signature> for NativeSignature {
    type Error = RadixEngineToolkitError;

    fn try_from(value: Signature) -> Result<Self> {
        match value {
            Signature::Ed25519 { value } => value
                .try_into()
                .map(NativeEd25519Signature)
                .map(Self::Ed25519)
                .map_err(|value| RadixEngineToolkitError::InvalidLength {
                    expected: NativeEd25519Signature::LENGTH as u64,
                    actual: value.len() as u64,
                    data: value,
                }),
            Signature::Secp256k1 { value } => value
                .try_into()
                .map(NativeSecp256k1Signature)
                .map(Self::Secp256k1)
                .map_err(|value| RadixEngineToolkitError::InvalidLength {
                    expected: NativeSecp256k1Signature::LENGTH as u64,
                    actual: value.len() as u64,
                    data: value,
                }),
        }
    }
}
