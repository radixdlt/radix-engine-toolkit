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
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
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

#[serde_as]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
#[serde(transparent)]
#[schemars(transparent)]
#[schemars(bound = "")]
pub struct AsStr<T, E>(
    #[schemars(with = "String")]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    T,
)
where
    T: Display + FromStr<Err = E>,
    E: Display;

impl<T, E> std::ops::Deref for AsStr<T, E>
where
    T: Display + FromStr<Err = E>,
    E: Display,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, E> std::ops::DerefMut for AsStr<T, E>
where
    T: Display + FromStr<Err = E>,
    E: Display,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T, E> From<T> for AsStr<T, E>
where
    T: Display + FromStr<Err = E>,
    E: Display,
{
    fn from(value: T) -> Self {
        Self(value)
    }
}

pub type AsStrType<T> = AsStr<T, <T as FromStr>::Err>;

pub type SerializableHash = AsStrType<Hash>;
pub type SerializableNonFungibleLocalId = AsStrType<NonFungibleLocalId>;
pub type SerializableU8 = AsStrType<u8>;
pub type SerializableU16 = AsStrType<u16>;
pub type SerializableU32 = AsStrType<u32>;
pub type SerializableU64 = AsStrType<u64>;
pub type SerializableU128 = AsStrType<u128>;
pub type SerializableI8 = AsStrType<i8>;
pub type SerializableI16 = AsStrType<i16>;
pub type SerializableI32 = AsStrType<i32>;
pub type SerializableI64 = AsStrType<i64>;
pub type SerializableI128 = AsStrType<i128>;
pub type SerializableDecimal = AsStrType<Decimal>;
pub type SerializablePreciseDecimal = AsStrType<PreciseDecimal>;
