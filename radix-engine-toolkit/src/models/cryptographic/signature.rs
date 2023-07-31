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

use radix_engine_common::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use transaction::prelude::*;

use crate::prelude::*;

#[serde_as]
#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
#[serde(tag = "kind", content = "value")]
pub enum SerializableSignature {
    Secp256k1(#[typeshare(serialized_as = "String")] AsHex<[u8; 65]>),
    Ed25519(#[typeshare(serialized_as = "String")] AsHex<[u8; 64]>),
}

impl From<SignatureV1> for SerializableSignature {
    fn from(value: SignatureV1) -> Self {
        match value {
            SignatureV1::Secp256k1(signature) => Self::Secp256k1(signature.0.into()),
            SignatureV1::Ed25519(signature) => Self::Ed25519(signature.0.into()),
        }
    }
}

impl From<SerializableSignature> for SignatureV1 {
    fn from(value: SerializableSignature) -> Self {
        match value {
            SerializableSignature::Secp256k1(value) => Self::Secp256k1(Secp256k1Signature(*value)),
            SerializableSignature::Ed25519(value) => Self::Ed25519(Ed25519Signature(*value)),
        }
    }
}
