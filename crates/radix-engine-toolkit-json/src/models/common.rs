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

use schemars::JsonSchema;
use scrypto::prelude::*;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(
    Serialize,
    Deserialize,
    JsonSchema,
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
#[serde(transparent)]
#[schemars(transparent)]
#[schemars(bound = "")]
pub struct AsHex<T>(
    #[schemars(with = "String")]
    #[serde_as(as = "serde_with::hex::Hex")]
    T,
)
where
    T: AsRef<[u8]> + TryFrom<Vec<u8>>;

impl<T> std::ops::Deref for AsHex<T>
where
    T: AsRef<[u8]> + TryFrom<Vec<u8>>,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for AsHex<T>
where
    T: AsRef<[u8]> + TryFrom<Vec<u8>>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<T> for AsHex<T>
where
    T: AsRef<[u8]> + TryFrom<Vec<u8>>,
{
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T> Copy for AsHex<T> where T: AsRef<[u8]> + TryFrom<Vec<u8>> + Copy {}

#[serde_as]
#[derive(
    Serialize,
    Deserialize,
    JsonSchema,
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
#[serde(transparent)]
#[schemars(transparent)]
#[schemars(bound = "")]
pub struct AsStr<T>(
    #[schemars(with = "String")]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    T,
)
where
    T: Display + FromStr,
    <T as FromStr>::Err: Display;

impl<T> std::ops::Deref for AsStr<T>
where
    T: Display + FromStr,
    <T as FromStr>::Err: Display,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for AsStr<T>
where
    T: Display + FromStr,
    <T as FromStr>::Err: Display,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<T> for AsStr<T>
where
    T: Display + FromStr,
    <T as FromStr>::Err: Display,
{
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T> Copy for AsStr<T>
where
    T: Display + FromStr + Copy,
    <T as FromStr>::Err: Display,
{
}

pub type SerializableHash = AsStr<Hash>;
pub type SerializableNonFungibleLocalId = AsStr<NonFungibleLocalId>;
pub type SerializableU8 = AsStr<u8>;
pub type SerializableU16 = AsStr<u16>;
pub type SerializableU32 = AsStr<u32>;
pub type SerializableU64 = AsStr<u64>;
pub type SerializableU128 = AsStr<u128>;
pub type SerializableI8 = AsStr<i8>;
pub type SerializableI16 = AsStr<i16>;
pub type SerializableI32 = AsStr<i32>;
pub type SerializableI64 = AsStr<i64>;
pub type SerializableI128 = AsStr<i128>;
pub type SerializableDecimal = AsStr<Decimal>;
pub type SerializablePreciseDecimal = AsStr<PreciseDecimal>;
pub type SerializableBytes = AsHex<Vec<u8>>;

/// A private module containing type definitions to be exposed through typeshare
#[allow(dead_code)]
mod __private {
    #[typeshare::typeshare]
    pub type SerializableHash = String;
    #[typeshare::typeshare]
    pub type SerializableNonFungibleLocalId = String;
    #[typeshare::typeshare]
    pub type SerializableU8 = String;
    #[typeshare::typeshare]
    pub type SerializableU16 = String;
    #[typeshare::typeshare]
    pub type SerializableU32 = String;
    #[typeshare::typeshare]
    pub type SerializableU64 = String;
    #[typeshare::typeshare]
    pub type SerializableU128 = String;
    #[typeshare::typeshare]
    pub type SerializableI8 = String;
    #[typeshare::typeshare]
    pub type SerializableI16 = String;
    #[typeshare::typeshare]
    pub type SerializableI32 = String;
    #[typeshare::typeshare]
    pub type SerializableI64 = String;
    #[typeshare::typeshare]
    pub type SerializableI128 = String;
    #[typeshare::typeshare]
    pub type SerializableDecimal = String;
    #[typeshare::typeshare]
    pub type SerializablePreciseDecimal = String;
    #[typeshare::typeshare]
    pub type SerializableBytes = String;
}
