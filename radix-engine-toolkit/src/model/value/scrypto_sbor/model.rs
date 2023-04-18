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
use crate::model::address::NetworkAwareNodeId;

use scrypto::prelude::{Decimal, NonFungibleLocalId, PreciseDecimal};

use serde_with::serde_as;
use toolkit_derive::serializable;

define_kind_enum! {
    /// A value model used to describe an algebraic sum type which is used to express Scrypto SBOR
    /// values. This is serialized as a discriminated union of types.
    #[serializable]
    #[serde(tag = "type")]
    #[derive(Hash, Eq, PartialEq)]
    pub enum ScryptoSborValue {
        /// A boolean value which can either be true or false
        #[schemars(
            example = "crate::example::value::scrypto_sbor_value::bool1",
            example = "crate::example::value::scrypto_sbor_value::bool2"
        )]
        Bool { value: bool },

        /// An 8-bit unsigned integer which is serialized and deserialized as a string.
        #[schemars(example = "crate::example::value::scrypto_sbor_value::u8")]
        U8 {
            #[schemars(regex(pattern = "[0-9]+"))]
            #[schemars(with = "String")]
            #[serde_as(as = "serde_with::DisplayFromStr")]
            value: u8,
        },

        /// A 16-bit unsigned integer which is serialized and deserialized as a string.
        #[schemars(example = "crate::example::value::scrypto_sbor_value::u16")]
        U16 {
            #[schemars(regex(pattern = "[0-9]+"))]
            #[schemars(with = "String")]
            #[serde_as(as = "serde_with::DisplayFromStr")]
            value: u16,
        },

        /// A 32-bit unsigned integer which is serialized and deserialized as a string.
        #[schemars(example = "crate::example::value::scrypto_sbor_value::u32")]
        U32 {
            #[schemars(regex(pattern = "[0-9]+"))]
            #[schemars(with = "String")]
            #[serde_as(as = "serde_with::DisplayFromStr")]
            value: u32,
        },

        /// A 64-bit unsigned integer which is serialized and deserialized as a string.
        #[schemars(example = "crate::example::value::scrypto_sbor_value::u64")]
        U64 {
            #[schemars(regex(pattern = "[0-9]+"))]
            #[schemars(with = "String")]
            #[serde_as(as = "serde_with::DisplayFromStr")]
            value: u64,
        },

        /// A 128-bit unsigned integer which is serialized and deserialized as a string.
        #[schemars(example = "crate::example::value::scrypto_sbor_value::u128")]
        U128 {
            #[schemars(regex(pattern = "[0-9]+"))]
            #[schemars(with = "String")]
            #[serde_as(as = "serde_with::DisplayFromStr")]
            value: u128,
        },

        /// An 8-bit signed integer which is serialized and deserialized as a string.
        #[schemars(example = "crate::example::value::scrypto_sbor_value::i8")]
        I8 {
            #[schemars(regex(pattern = "[0-9]+"))]
            #[schemars(with = "String")]
            #[serde_as(as = "serde_with::DisplayFromStr")]
            value: i8,
        },

        /// A 16-bit signed integer which is serialized and deserialized as a string.
        #[schemars(example = "crate::example::value::scrypto_sbor_value::i16")]
        I16 {
            #[schemars(regex(pattern = "[0-9]+"))]
            #[schemars(with = "String")]
            #[serde_as(as = "serde_with::DisplayFromStr")]
            value: i16,
        },

        /// A 32-bit signed integer which is serialized and deserialized as a string.
        #[schemars(example = "crate::example::value::scrypto_sbor_value::i32")]
        I32 {
            #[schemars(regex(pattern = "[0-9]+"))]
            #[schemars(with = "String")]
            #[serde_as(as = "serde_with::DisplayFromStr")]
            value: i32,
        },

        /// A 64-bit signed integer which is serialized and deserialized as a string.
        #[schemars(example = "crate::example::value::scrypto_sbor_value::i32")]
        I64 {
            #[schemars(regex(pattern = "[0-9]+"))]
            #[schemars(with = "String")]
            #[serde_as(as = "serde_with::DisplayFromStr")]
            value: i64,
        },

        /// A 128-bit signed integer which is serialized and deserialized as a string.
        #[schemars(example = "crate::example::value::scrypto_sbor_value::i128")]
        I128 {
            #[schemars(regex(pattern = "[0-9]+"))]
            #[schemars(with = "String")]
            #[serde_as(as = "serde_with::DisplayFromStr")]
            value: i128,
        },

        /// A type representing a string
        #[schemars(example = "crate::example::value::scrypto_sbor_value::string")]
        String { value: String },

        /// A Rust-style Enum which has a variant and can optionally also have a list of values
        /// (acting in a way similar to discriminated algebraic sum types)
        #[schemars(
            example = "crate::example::value::scrypto_sbor_value::enum1",
            example = "crate::example::value::scrypto_sbor_value::enum2"
        )]
        Enum {
            /// The variant of the enum.
            #[schemars(regex(pattern = "[0-9]+"))]
            #[schemars(with = "String")]
            #[serde_as(as = "serde_with::DisplayFromStr")]
            variant: u8,

            /// Optional fields that the enum may have
            #[serde(default, skip_serializing_if = "Option::is_none")]
            fields: Option<Vec<Self>>,
        },

        /// An array values of a single value kind
        #[schemars(example = "crate::example::value::scrypto_sbor_value::array")]
        Array {
            /// The kind of elements that the array contains. An array will be validated to ensure
            /// that it contains a single element kind.
            element_kind: ScryptoSborValueKind,

            /// The elements of the array which may contain 0 or more elements.
            elements: Vec<Self>,
        },

        /// A key-value map of values where all keys are of a single kind and all values are of a
        /// single kind
        #[schemars(example = "crate::example::value::scrypto_sbor_value::map")]
        Map {
            /// The kind of the keys used for the map. A map will be validated to ensure that its keys
            /// are all of a single kind.
            key_value_kind: ScryptoSborValueKind,

            /// The kind of the values used for the map. A map will be validated to ensure that its
            /// values are all of a single kind.
            value_value_kind: ScryptoSborValueKind,

            /// A vector of tuples representing the entires in the map where each tuple is made up of
            /// two elements: a key and a value.
            entries: Vec<(Self, Self)>,
        },

        /// An array of elements where elements could be of different kinds.
        #[schemars(example = "crate::example::value::scrypto_sbor_value::tuple")]
        Tuple { elements: Vec<Self> },

        /// Represents a tagged enum of owned Radix Engine Nodes.
        #[schemars(example = "crate::example::value::scrypto_sbor_value::own")]
        Own {
            #[schemars(with = "String")]
            #[serde_as(as = "serde_with::DisplayFromStr")]
            value: NetworkAwareNodeId,
        },

        /// A Scrypto Decimal which has a precision of 18 decimal places and has a maximum and minimum
        /// of 57896044618658097711785492504343953926634992332820282019728.792003956564819967 and
        /// -57896044618658097711785492504343953926634992332820282019728.792003956564819968
        /// respectively
        #[schemars(example = "crate::example::value::scrypto_sbor_value::decimal")]
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
        #[schemars(example = "crate::example::value::scrypto_sbor_value::precise_decimal")]
        PreciseDecimal {
            #[schemars(regex(pattern = "[+-]?([0-9]*[.])?[0-9]+"))]
            #[schemars(with = "String")]
            #[serde_as(as = "serde_with::DisplayFromStr")]
            value: PreciseDecimal,
        },

        /// Represents non-fungible ids which is a discriminated union of the different types that
        /// non-fungible ids may be.
        #[schemars(
            example = "crate::example::value::scrypto_sbor_value::non_fungible_local_id1",
            example = "crate::example::value::scrypto_sbor_value::non_fungible_local_id2",
            example = "crate::example::value::scrypto_sbor_value::non_fungible_local_id3",
            example = "crate::example::value::scrypto_sbor_value::non_fungible_local_id4",
        )]
        NonFungibleLocalId {
            #[schemars(with = "crate::model::address::NonFungibleLocalId")]
            #[serde_as(as = "serde_with::TryFromInto<crate::model::address::NonFungibleLocalId>")]
            value: NonFungibleLocalId,
        },

        /// Represents a reference to some RENode.
        #[schemars(example = "crate::example::value::scrypto_sbor_value::reference")]
        Reference {
            #[schemars(with = "String")]
            #[serde_as(as = "serde_with::DisplayFromStr")]
            value: NetworkAwareNodeId,
        }
    }
}
