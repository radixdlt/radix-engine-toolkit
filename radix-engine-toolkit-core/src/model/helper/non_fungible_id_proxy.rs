// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at

//   http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

use scrypto::prelude::NonFungibleId;

#[serde_as]
#[derive(Serialize, Deserialize)]
#[serde(tag = "variant", content = "value")]
pub enum NonFungibleIdProxy {
    String(String),
    U32(#[serde_as(as = "DisplayFromStr")] u32),
    U64(#[serde_as(as = "DisplayFromStr")] u64),
    Bytes(#[serde_as(as = "serde_with::hex::Hex")] Vec<u8>),
    UUID(#[serde_as(as = "DisplayFromStr")] u128),
}

impl From<NonFungibleId> for NonFungibleIdProxy {
    fn from(value: NonFungibleId) -> Self {
        match value {
            NonFungibleId::U32(value) => Self::U32(value),
            NonFungibleId::U64(value) => Self::U64(value),
            NonFungibleId::UUID(value) => Self::UUID(value),
            NonFungibleId::String(value) => Self::String(value),
            NonFungibleId::Bytes(value) => Self::Bytes(value),
        }
    }
}

impl From<NonFungibleIdProxy> for NonFungibleId {
    fn from(value: NonFungibleIdProxy) -> Self {
        match value {
            NonFungibleIdProxy::U32(value) => Self::U32(value),
            NonFungibleIdProxy::U64(value) => Self::U64(value),
            NonFungibleIdProxy::UUID(value) => Self::UUID(value),
            NonFungibleIdProxy::String(value) => Self::String(value),
            NonFungibleIdProxy::Bytes(value) => Self::Bytes(value),
        }
    }
}
