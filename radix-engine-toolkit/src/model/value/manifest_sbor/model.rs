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

use crate::define_kind_enum;
use crate::model::address::EntityAddress;

use native_transaction::data::{ManifestBlobRef, ManifestExpression};
use scrypto::prelude::{Decimal, NonFungibleLocalId, PreciseDecimal};
use serde_with::serde_as;
use toolkit_derive::serializable;

define_kind_enum! {
    /// A value model used to describe an algebraic sum type which is used to express Manifest SBOR
    /// values. This is serialized as a discriminated union of types.
    #[serializable]
    #[serde(tag = "type")]
    #[derive(Hash, Eq, PartialEq)]
    pub enum ManifestSborValue {
        /// A boolean value which can either be true or false
        Bool { value: bool },

        /// An 8-bit unsigned integer which is serialized and deserialized as a string.
        U8 {
            #[schemars(regex(pattern = "[0-9]+"))]
            #[schemars(with = "String")]
            #[serde_as(as = "serde_with::DisplayFromStr")]
            value: u8,
        },

        /// A 16-bit unsigned integer which is serialized and deserialized as a string.
        U16 {
            #[schemars(regex(pattern = "[0-9]+"))]
            #[schemars(with = "String")]
            #[serde_as(as = "serde_with::DisplayFromStr")]
            value: u16,
        },

        /// A 32-bit unsigned integer which is serialized and deserialized as a string.
        U32 {
            #[schemars(regex(pattern = "[0-9]+"))]
            #[schemars(with = "String")]
            #[serde_as(as = "serde_with::DisplayFromStr")]
            value: u32,
        },

        /// A 64-bit unsigned integer which is serialized and deserialized as a string.
        U64 {
            #[schemars(regex(pattern = "[0-9]+"))]
            #[schemars(with = "String")]
            #[serde_as(as = "serde_with::DisplayFromStr")]
            value: u64,
        },

        /// A 128-bit unsigned integer which is serialized and deserialized as a string.
        U128 {
            #[schemars(regex(pattern = "[0-9]+"))]
            #[schemars(with = "String")]
            #[serde_as(as = "serde_with::DisplayFromStr")]
            value: u128,
        },

        /// An 8-bit signed integer which is serialized and deserialized as a string.
        I8 {
            #[schemars(regex(pattern = "[0-9]+"))]
            #[schemars(with = "String")]
            #[serde_as(as = "serde_with::DisplayFromStr")]
            value: i8,
        },

        /// A 16-bit signed integer which is serialized and deserialized as a string.
        I16 {
            #[schemars(regex(pattern = "[0-9]+"))]
            #[schemars(with = "String")]
            #[serde_as(as = "serde_with::DisplayFromStr")]
            value: i16,
        },

        /// A 32-bit signed integer which is serialized and deserialized as a string.
        I32 {
            #[schemars(regex(pattern = "[0-9]+"))]
            #[schemars(with = "String")]
            #[serde_as(as = "serde_with::DisplayFromStr")]
            value: i32,
        },

        /// A 64-bit signed integer which is serialized and deserialized as a string.
        I64 {
            #[schemars(regex(pattern = "[0-9]+"))]
            #[schemars(with = "String")]
            #[serde_as(as = "serde_with::DisplayFromStr")]
            value: i64,
        },

        /// A 128-bit signed integer which is serialized and deserialized as a string.
        I128 {
            #[schemars(regex(pattern = "[0-9]+"))]
            #[schemars(with = "String")]
            #[serde_as(as = "serde_with::DisplayFromStr")]
            value: i128,
        },

        /// A type representing a string
        String { value: String },

        /// A Rust-style Enum which has a variant and can optionally also have a list of values
        /// (acting in a way similar to discriminated algebraic sum types)
        Enum {
            /// The variant of the enum.
            variant: u8,

            /// Optional fields that the enum may have
            #[serde(default, skip_serializing_if = "Option::is_none")]
            fields: Option<Vec<Self>>,
        },

        /// An array values of a single value kind
        Array {
            /// The kind of elements that the array contains. An array will be validated to ensure
            /// that it contains a single element kind.
            element_kind: ManifestSborValueKind,

            /// The elements of the array which may contain 0 or more elements.
            elements: Vec<Self>,
        },

        /// A key-value map of values where all keys are of a single kind and all values are of a
        /// single kind
        Map {
            /// The kind of the keys used for the map. A map will be validated to ensure that its keys
            /// are all of a single kind.
            key_value_kind: ManifestSborValueKind,

            /// The kind of the values used for the map. A map will be validated to ensure that its
            /// values are all of a single kind.
            value_value_kind: ManifestSborValueKind,

            /// A vector of tuples representing the entires in the map where each tuple is made up of
            /// two elements: a key and a value.
            entries: Vec<(Self, Self)>,
        },

        /// An array of elements where elements could be of different kinds.
        Tuple { elements: Vec<Self> },

        /// Represents a Bech32m encoded human-readable address which may be used to address a package,
        /// component, or resource. This address is serialized as a human-readable bech32m encoded
        /// string.
        Address {
            #[schemars(with = "String")]
            #[serde_as(as = "serde_with::DisplayFromStr")]
            address: EntityAddress,
        },

        /// Represents a Scrypto bucket which is identified through a transient identifier which is
        /// a 32-bit integer which is serialized as a Integer.
        Bucket { identifier: u32 },

        /// Represents a Scrypto proof which is identified through a transient identifier which is
        /// a 32-bit integer which is serialized as a Integer.
        Proof { identifier: u32 },

        /// A Scrypto Decimal which has a precision of 18 decimal places and has a maximum and minimum
        /// of 57896044618658097711785492504343953926634992332820282019728.792003956564819967 and
        /// -57896044618658097711785492504343953926634992332820282019728.792003956564819968
        /// respectively
        Decimal {
            #[schemars(regex(pattern = "[+-]?([0-9]*[.])?[0-9]+"))]
            #[schemars(with = "String")]
            #[serde_as(as = "serde_with::DisplayFromStr")]
            value: Decimal,
        },

        /// A Scrypto PreciseDecimal which has a precision of 64 decimal places and has a maximum and
        /// minimum of
        /// 670390396497129854978701249910292306373968291029619668886178072186088201503677348840093714.
        /// 9083451713845015929093243025426876941405973284973216824503042047
        /// and -670390396497129854978701249910292306373968291029619668886178072186088201503677348840093714.9083451713845015929093243025426876941405973284973216824503042048
        /// respectively
        PreciseDecimal {
            #[schemars(regex(pattern = "[+-]?([0-9]*[.])?[0-9]+"))]
            #[schemars(with = "String")]
            #[serde_as(as = "serde_with::DisplayFromStr")]
            value: PreciseDecimal,
        },

        /// Represents non-fungible ids which is a discriminated union of the different types that
        /// non-fungible ids may be.
        NonFungibleLocalId {
            #[schemars(with = "crate::model::address::NonFungibleLocalId")]
            #[serde_as(as = "serde_with::TryFromInto<crate::model::address::NonFungibleLocalId>")]
            value: NonFungibleLocalId,
        },

        /// Represents a transaction manifest expression.
        Expression {
            #[schemars(with = "crate::model::runtime::Expression")]
            #[serde_as(as = "serde_with::FromInto<crate::model::runtime::Expression>")]
            value: ManifestExpression,
        },

        /// Represents the hash of a blob provided as part of a transaction manifest. This is
        /// represented as a byte array of 32 bytes which is serialized as a hex string.
        Blob {
            #[schemars(with = "crate::model::runtime::Blob")]
            #[serde_as(as = "serde_with::FromInto<crate::model::runtime::Blob>")]
            hash: ManifestBlobRef,
        },
    }
}
