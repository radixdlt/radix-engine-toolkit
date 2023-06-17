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

use native_json_library::models::cryptographic::public_key::*;
use transaction::prelude::{Ed25519PrivateKey, Secp256k1PrivateKey};

use super::traits::HasExamples;

impl<'f> HasExamples<'f> for SerializablePublicKey {
    fn examples() -> Vec<Self> {
        vec![
            Self::Secp256k1 {
                value: Secp256k1PrivateKey::from_u64(1)
                    .unwrap()
                    .public_key()
                    .0
                    .into(),
            },
            Self::Ed25519 {
                value: Ed25519PrivateKey::from_u64(1)
                    .unwrap()
                    .public_key()
                    .0
                    .into(),
            },
        ]
    }
}

impl<'f> HasExamples<'f> for SerializableSecp256k1PublicKey {
    fn examples() -> Vec<Self> {
        vec![Secp256k1PrivateKey::from_u64(1)
            .unwrap()
            .public_key()
            .into()]
    }
}

impl<'f> HasExamples<'f> for SerializableEd25519PublicKey {
    fn examples() -> Vec<Self> {
        vec![Ed25519PrivateKey::from_u64(1).unwrap().public_key().into()]
    }
}
