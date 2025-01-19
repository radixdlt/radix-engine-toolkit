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
pub enum SignatureWithPublicKeyV1 {
    Secp256k1 {
        signature: Vec<u8>,
    },
    Ed25519 {
        signature: Vec<u8>,
        public_key: Vec<u8>,
    },
}

impl From<engine::SignatureWithPublicKeyV1> for SignatureWithPublicKeyV1 {
    fn from(value: engine::SignatureWithPublicKeyV1) -> Self {
        match value {
            engine::SignatureWithPublicKeyV1::Secp256k1 {
                signature: engine::Secp256k1Signature(signature),
            } => Self::Secp256k1 {
                signature: signature.to_vec(),
            },
            engine::SignatureWithPublicKeyV1::Ed25519 {
                signature: engine::Ed25519Signature(signature),
                public_key: engine::Ed25519PublicKey(public_key),
            } => Self::Ed25519 {
                signature: signature.to_vec(),
                public_key: public_key.to_vec(),
            },
        }
    }
}

impl TryFrom<SignatureWithPublicKeyV1> for engine::SignatureWithPublicKeyV1 {
    type Error = RadixEngineToolkitError;

    fn try_from(value: SignatureWithPublicKeyV1) -> Result<Self> {
        match value {
            SignatureWithPublicKeyV1::Ed25519 {
                signature,
                public_key,
            } => {
                let public_key = public_key
                    .try_into()
                    .map(engine::Ed25519PublicKey)
                    .map_err(|public_key| {
                        RadixEngineToolkitError::InvalidLength {
                            expected: engine::Ed25519PublicKey::LENGTH as u64,
                            actual: public_key.len() as u64,
                            data: public_key,
                        }
                    })?;
                let signature = signature
                    .try_into()
                    .map(engine::Ed25519Signature)
                    .map_err(|signature| {
                        RadixEngineToolkitError::InvalidLength {
                            expected: engine::Ed25519Signature::LENGTH as u64,
                            actual: signature.len() as u64,
                            data: signature,
                        }
                    })?;
                Ok(Self::Ed25519 {
                    public_key,
                    signature,
                })
            }
            SignatureWithPublicKeyV1::Secp256k1 { signature } => signature
                .try_into()
                .map(engine::Secp256k1Signature)
                .map(|signature| Self::Secp256k1 { signature })
                .map_err(|signature| RadixEngineToolkitError::InvalidLength {
                    expected: engine::Secp256k1Signature::LENGTH as u64,
                    actual: signature.len() as u64,
                    data: signature,
                }),
        }
    }
}
