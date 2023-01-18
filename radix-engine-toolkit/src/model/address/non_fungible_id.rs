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

use scrypto::prelude::NonFungibleId as ScryptoNonFungibleId;
use serializable::serializable;

#[serializable]
#[serde(tag = "variant", content = "value")]
/// Represents non-fungible ids which is a discriminated union of the different types that
/// non-fungible ids may be.
pub enum NonFungibleId {
    /// A 64 bit unsigned integer non-fungible id type which is serialized as a string
    Number(
        #[schemars(regex(pattern = "[0-9]+"))]
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        u64,
    ),

    // TODO: Should this be serialized as a GUID?
    /// A 128 bit unsigned integer UUID non-fungible id type which is serialized as a string
    UUID(
        #[schemars(regex(pattern = "[0-9]+"))]
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        u128,
    ),

    /// An byte array non-fungible id type which is serialized as a hex string
    Bytes(
        #[schemars(regex(pattern = "[0-9a-fA-F]+"))]
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::hex::Hex")]
        Vec<u8>,
    ),

    /// A string non-fungible id
    String(String),
}

impl From<ScryptoNonFungibleId> for NonFungibleId {
    fn from(value: ScryptoNonFungibleId) -> Self {
        match value {
            ScryptoNonFungibleId::Number(value) => Self::Number(value),
            ScryptoNonFungibleId::UUID(value) => Self::UUID(value),
            ScryptoNonFungibleId::String(value) => Self::String(value),
            ScryptoNonFungibleId::Bytes(value) => Self::Bytes(value),
        }
    }
}

impl From<NonFungibleId> for ScryptoNonFungibleId {
    fn from(value: NonFungibleId) -> Self {
        match value {
            NonFungibleId::Number(value) => Self::Number(value),
            NonFungibleId::UUID(value) => Self::UUID(value),
            NonFungibleId::String(value) => Self::String(value),
            NonFungibleId::Bytes(value) => Self::Bytes(value),
        }
    }
}