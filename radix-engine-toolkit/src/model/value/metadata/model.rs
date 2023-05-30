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

use crate::model::address::NetworkAwareNodeId;
use scrypto::prelude::*;
use toolkit_derive::serializable;

#[serializable]
#[serde(tag = "type")]
#[derive(PartialEq, Eq)]
pub enum MetadataValue {
    String {
        value: String,
    },

    Bool {
        value: bool,
    },

    U8 {
        #[schemars(regex(pattern = "[0-9]+"))]
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        value: u8,
    },

    U32 {
        #[schemars(regex(pattern = "[0-9]+"))]
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        value: u32,
    },

    U64 {
        #[schemars(regex(pattern = "[0-9]+"))]
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        value: u64,
    },

    I32 {
        #[schemars(regex(pattern = "[0-9]+"))]
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        value: i32,
    },

    I64 {
        #[schemars(regex(pattern = "[0-9]+"))]
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        value: i64,
    },

    Decimal {
        #[schemars(regex(pattern = "[0-9]+"))]
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        value: Decimal,
    },

    GlobalAddress {
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        value: NetworkAwareNodeId,
    },

    PublicKey {
        #[schemars(with = "crate::model::crypto::PublicKey")]
        #[serde_as(as = "serde_with::FromInto<crate::model::crypto::PublicKey>")]
        value: PublicKey,
    },

    NonFungibleGlobalId {
        value: String,
    },

    NonFungibleLocalId {
        value: String,
    },

    Instant {
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        value: i64,
    },

    Url {
        value: String,
    },

    Origin {
        value: String,
    },

    PublicKeyHash {
        #[schemars(with = "crate::model::crypto::PublicKeyHash")]
        #[serde_as(as = "serde_with::FromInto<crate::model::crypto::PublicKeyHash>")]
        value: PublicKeyHash,
    },

    StringArray {
        value: Vec<String>,
    },

    BoolArray {
        value: Vec<bool>,
    },

    U8Array {
        #[schemars(regex(pattern = "[0-9]+"))]
        #[schemars(with = "Vec<String>")]
        #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
        value: Vec<u8>,
    },

    U32Array {
        #[schemars(regex(pattern = "[0-9]+"))]
        #[schemars(with = "Vec<String>")]
        #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
        value: Vec<u32>,
    },

    U64Array {
        #[schemars(regex(pattern = "[0-9]+"))]
        #[schemars(with = "Vec<String>")]
        #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
        value: Vec<u64>,
    },

    I32Array {
        #[schemars(regex(pattern = "[0-9]+"))]
        #[schemars(with = "Vec<String>")]
        #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
        value: Vec<i32>,
    },

    I64Array {
        #[schemars(regex(pattern = "[0-9]+"))]
        #[schemars(with = "Vec<String>")]
        #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
        value: Vec<i64>,
    },

    DecimalArray {
        #[schemars(regex(pattern = "[0-9]+"))]
        #[schemars(with = "Vec<String>")]
        #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
        value: Vec<Decimal>,
    },

    GlobalAddressArray {
        #[schemars(with = "Vec<String>")]
        #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
        value: Vec<NetworkAwareNodeId>,
    },

    PublicKeyArray {
        #[schemars(with = "Vec<crate::model::crypto::PublicKey>")]
        #[serde_as(as = "Vec<serde_with::FromInto<crate::model::crypto::PublicKey>>")]
        value: Vec<PublicKey>,
    },

    NonFungibleGlobalIdArray {
        value: Vec<String>,
    },

    NonFungibleLocalIdArray {
        value: Vec<String>,
    },

    InstantArray {
        #[schemars(with = "Vec<String>")]
        #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
        value: Vec<i64>,
    },

    UrlArray {
        value: Vec<String>,
    },

    OriginArray {
        value: Vec<String>,
    },

    PublicKeyHashArray {
        #[schemars(with = "Vec<crate::model::crypto::PublicKeyHash>")]
        #[serde_as(as = "Vec<serde_with::FromInto<crate::model::crypto::PublicKeyHash>>")]
        value: Vec<PublicKeyHash>,
    },
}
