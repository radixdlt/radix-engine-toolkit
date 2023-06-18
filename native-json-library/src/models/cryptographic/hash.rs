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

use std::ops::{Deref, DerefMut};

use schemars::JsonSchema;
use scrypto::prelude::*;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(transparent)]
#[schemars(transparent)]
pub struct SerializableHash(
    #[schemars(with = "String")]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    Hash,
);

impl Deref for SerializableHash {
    type Target = Hash;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SerializableHash {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<[u8; 32]> for SerializableHash {
    fn from(value: [u8; 32]) -> Self {
        Self(Hash(value))
    }
}

impl From<SerializableHash> for [u8; 32] {
    fn from(value: SerializableHash) -> Self {
        value.0 .0
    }
}

impl From<Hash> for SerializableHash {
    fn from(value: Hash) -> Self {
        Self(value)
    }
}

impl From<SerializableHash> for Hash {
    fn from(value: SerializableHash) -> Self {
        value.0
    }
}
