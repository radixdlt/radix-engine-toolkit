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

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use transaction::prelude::*;

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
#[serde(tag = "kind", content = "value")]
pub enum SerializableSignatureWithPublicKey {
    Secp256k1 {
        #[typeshare(serialized_as = "String")]
        signature: AsHex<[u8; 65]>,
    },
    Ed25519 {
        #[typeshare(serialized_as = "String")]
        signature: AsHex<[u8; 64]>,
        #[typeshare(serialized_as = "String")]
        public_key: AsHex<[u8; 32]>,
    },
}

impl From<SignatureWithPublicKeyV1> for SerializableSignatureWithPublicKey {
    fn from(value: SignatureWithPublicKeyV1) -> Self {
        match value {
            SignatureWithPublicKeyV1::Secp256k1 { signature } => {
                Self::Secp256k1 {
                    signature: signature.0.into(),
                }
            }
            SignatureWithPublicKeyV1::Ed25519 {
                signature,
                public_key,
            } => Self::Ed25519 {
                signature: signature.0.into(),
                public_key: public_key.0.into(),
            },
        }
    }
}

impl From<SerializableSignatureWithPublicKey> for SignatureWithPublicKeyV1 {
    fn from(value: SerializableSignatureWithPublicKey) -> Self {
        match value {
            SerializableSignatureWithPublicKey::Secp256k1 { signature } => {
                Self::Secp256k1 {
                    signature: Secp256k1Signature(*signature),
                }
            }
            SerializableSignatureWithPublicKey::Ed25519 {
                signature,
                public_key,
            } => Self::Ed25519 {
                signature: Ed25519Signature(*signature),
                public_key: Ed25519PublicKey(*public_key),
            },
        }
    }
}
