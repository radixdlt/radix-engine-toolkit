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

use std::str::FromStr;

use crate::define_kind_enum;
use crate::model::address::NetworkAwareNodeId;

use native_transaction::manifest::KNOWN_ENUM_DISCRIMINATORS;
use scrypto::prelude::{
    Decimal, ManifestBlobRef, ManifestExpression, NonFungibleLocalId, PreciseDecimal,
};
use serde_with::serde_as;
use toolkit_derive::serializable;

use super::ManifestAstValueConversionError;

define_kind_enum! {
    /// A value model used to describe an algebraic sum type which is used to express transaction
    /// manifests as an abstract syntax tree. This is serialized as a discriminated union of types.
    #[serializable]
    #[serde(tag = "kind")]
    #[schemars(example = "crate::example::value::ast_value::value")]
    #[derive(PartialEq, Eq, Hash)]
    pub enum ManifestAstValue {
        /// A boolean value which can either be true or false
        #[schemars(
            example = "crate::example::value::ast_value::bool1",
            example = "crate::example::value::ast_value::bool2"
        )]
        Bool { value: bool },

        /// An 8-bit unsigned integer which is serialized and deserialized as a string.
        #[schemars(example = "crate::example::value::ast_value::u8")]
        U8 {
            #[schemars(regex(pattern = "[0-9]+"))]
            #[schemars(with = "String")]
            #[serde_as(as = "serde_with::DisplayFromStr")]
            value: u8,
        },

        /// A 16-bit unsigned integer which is serialized and deserialized as a string.
        #[schemars(example = "crate::example::value::ast_value::u16")]
        U16 {
            #[schemars(regex(pattern = "[0-9]+"))]
            #[schemars(with = "String")]
            #[serde_as(as = "serde_with::DisplayFromStr")]
            value: u16,
        },

        /// A 32-bit unsigned integer which is serialized and deserialized as a string.
        #[schemars(example = "crate::example::value::ast_value::u32")]
        U32 {
            #[schemars(regex(pattern = "[0-9]+"))]
            #[schemars(with = "String")]
            #[serde_as(as = "serde_with::DisplayFromStr")]
            value: u32,
        },

        /// A 64-bit unsigned integer which is serialized and deserialized as a string.
        #[schemars(example = "crate::example::value::ast_value::u64")]
        U64 {
            #[schemars(regex(pattern = "[0-9]+"))]
            #[schemars(with = "String")]
            #[serde_as(as = "serde_with::DisplayFromStr")]
            value: u64,
        },

        /// A 128-bit unsigned integer which is serialized and deserialized as a string.
        #[schemars(example = "crate::example::value::ast_value::u128")]
        U128 {
            #[schemars(regex(pattern = "[0-9]+"))]
            #[schemars(with = "String")]
            #[serde_as(as = "serde_with::DisplayFromStr")]
            value: u128,
        },

        /// An 8-bit signed integer which is serialized and deserialized as a string.
        #[schemars(example = "crate::example::value::ast_value::i8")]
        I8 {
            #[schemars(regex(pattern = "[0-9]+"))]
            #[schemars(with = "String")]
            #[serde_as(as = "serde_with::DisplayFromStr")]
            value: i8,
        },

        /// A 16-bit signed integer which is serialized and deserialized as a string.
        #[schemars(example = "crate::example::value::ast_value::i16")]
        I16 {
            #[schemars(regex(pattern = "[0-9]+"))]
            #[schemars(with = "String")]
            #[serde_as(as = "serde_with::DisplayFromStr")]
            value: i16,
        },

        /// A 32-bit signed integer which is serialized and deserialized as a string.
        #[schemars(example = "crate::example::value::ast_value::i32")]
        I32 {
            #[schemars(regex(pattern = "[0-9]+"))]
            #[schemars(with = "String")]
            #[serde_as(as = "serde_with::DisplayFromStr")]
            value: i32,
        },

        /// A 64-bit signed integer which is serialized and deserialized as a string.
        #[schemars(example = "crate::example::value::ast_value::i64")]
        I64 {
            #[schemars(regex(pattern = "[0-9]+"))]
            #[schemars(with = "String")]
            #[serde_as(as = "serde_with::DisplayFromStr")]
            value: i64,
        },

        /// A 128-bit signed integer which is serialized and deserialized as a string.
        #[schemars(example = "crate::example::value::ast_value::i128")]
        I128 {
            #[schemars(regex(pattern = "[0-9]+"))]
            #[schemars(with = "String")]
            #[serde_as(as = "serde_with::DisplayFromStr")]
            value: i128,
        },

        /// A type representing a string
        #[schemars(example = "crate::example::value::ast_value::string")]
        String { value: String },

        /// A Rust-style Enum which has a variant and can optionally also have a list of values
        /// (acting in a way similar to discriminated algebraic sum types)
        #[schemars(
            example = "crate::example::value::ast_value::enum1",
            example = "crate::example::value::ast_value::enum2",
            example = "crate::example::value::ast_value::enum3",
            example = "crate::example::value::ast_value::enum4"
        )]
        Enum {
            /// The enum discriminator which is either a string or an unsigned 8-bit integer.
            variant: EnumDiscriminator,

            /// Optional fields that the enum may have
            fields: Vec<Self>,
        },

        /// The `Some` case of Rust Options where the value is some Self
        #[schemars(example = "crate::example::value::ast_value::some")]
        Some { value: Box<Self> },

        /// The `None` case of Rust Options where there is value
        #[schemars(example = "crate::example::value::ast_value::none")]
        None,

        /// The `Ok` case of Rust Results where the value is some Self
        #[schemars(example = "crate::example::value::ast_value::ok")]
        Ok { value: Box<Self> },

        /// The `Err` case of Rust Results where the value is some Self
        #[schemars(example = "crate::example::value::ast_value::err")]
        Err { value: Box<Self> },

        /// An array values of a single value kind
        #[schemars(example = "crate::example::value::ast_value::array")]
        Array {
            /// The kind of elements that the array contains. An array will be validated to ensure
            /// that it contains a single element kind.
            element_kind: ManifestAstValueKind,

            /// The elements of the array which may contain 0 or more elements.
            elements: Vec<Self>,
        },

        /// A key-value map of values where all keys are of a single kind and all values are of a
        /// single kind
        #[schemars(example = "crate::example::value::ast_value::map")]
        Map {
            /// The kind of the keys used for the map. A map will be validated to ensure that its keys
            /// are all of a single kind.
            key_kind: ManifestAstValueKind,

            /// The kind of the values used for the map. A map will be validated to ensure that its
            /// values are all of a single kind.
            value_kind: ManifestAstValueKind,

            /// A vector of tuples representing the entires in the map where each tuple is made up of
            /// two elements: a key and a value.
            entries: Vec<(Self, Self)>,
        },

        /// An array of elements where elements could be of different kinds.
        #[schemars(example = "crate::example::value::ast_value::tuple")]
        Tuple { fields: Vec<Self> },

        /// A Scrypto Decimal which has a precision of 18 decimal places and has a maximum and minimum
        /// of 57896044618658097711785492504343953926634992332820282019728.792003956564819967 and
        /// -57896044618658097711785492504343953926634992332820282019728.792003956564819968
        /// respectively
        #[schemars(example = "crate::example::value::ast_value::decimal")]
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
        #[schemars(example = "crate::example::value::ast_value::precise_decimal")]
        PreciseDecimal {
            #[schemars(regex(pattern = "[+-]?([0-9]*[.])?[0-9]+"))]
            #[schemars(with = "String")]
            #[serde_as(as = "serde_with::DisplayFromStr")]
            value: PreciseDecimal,
        },

        /// Represents a Bech32m encoded human-readable address which may be used to address a package,
        /// component, or resource. This address is serialized as a human-readable bech32m encoded
        /// string.
        #[schemars(
            example = "crate::example::value::ast_value::address1",
            example = "crate::example::value::ast_value::address2",
            example = "crate::example::value::ast_value::address3"
        )]
        Address {
            #[schemars(with = "String")]
            #[serde_as(as = "serde_with::DisplayFromStr")]
            value: NetworkAwareNodeId,
        },

        /// Represents a Scrypto bucket which is identified through a transient identifier which is
        /// either a string or an unsigned 32-bit integer which is serialized as a Integer.
        #[schemars(
            example = "crate::example::value::ast_value::bucket1",
            example = "crate::example::value::ast_value::bucket2",
        )]
        Bucket { value: BucketId },

        /// Represents a Scrypto proof which is identified through a transient identifier which is
        /// either a string or an unsigned 32-bit integer which is serialized as a Integer.
        #[schemars(
            example = "crate::example::value::ast_value::proof1",
            example = "crate::example::value::ast_value::proof2",
        )]
        Proof { value: ProofId },

        /// Represents non-fungible ids which is a discriminated union of the different types that
        /// non-fungible ids may be.
        #[schemars(
            example = "crate::example::value::ast_value::non_fungible_local_id1",
            example = "crate::example::value::ast_value::non_fungible_local_id2",
            example = "crate::example::value::ast_value::non_fungible_local_id3",
            example = "crate::example::value::ast_value::non_fungible_local_id4",
        )]
        NonFungibleLocalId {
            #[schemars(with = "crate::model::address::NonFungibleLocalId")]
            #[serde_as(as = "serde_with::TryFromInto<crate::model::address::NonFungibleLocalId>")]
            value: NonFungibleLocalId,
        },

        /// Represents a non-fungible address which may be considered as the "global" address of a
        /// non-fungible unit as it contains both the resource address and the non-fungible id for that
        /// unit.
        #[schemars(
            example = "crate::example::value::ast_value::non_fungible_global_id1",
            example = "crate::example::value::ast_value::non_fungible_global_id2",
            example = "crate::example::value::ast_value::non_fungible_global_id3",
            example = "crate::example::value::ast_value::non_fungible_global_id4",
        )]
        NonFungibleGlobalId {
            #[schemars(with = "ManifestAstValue")]
            #[serde_as(as = "serde_with::TryFromInto<ManifestAstValue>")]
            resource_address: NetworkAwareNodeId,

            #[schemars(with = "ManifestAstValue")]
            #[serde_as(as = "serde_with::TryFromInto<ManifestAstValue>")]
            non_fungible_local_id: NonFungibleLocalId,
        },

        /// Represents a transaction manifest expression.
        #[schemars(
            example = "crate::example::value::ast_value::expression1",
            example = "crate::example::value::ast_value::expression2",
        )]
        Expression {
            #[schemars(with = "crate::model::runtime::Expression")]
            #[serde_as(as = "serde_with::FromInto<crate::model::runtime::Expression>")]
            value: ManifestExpression,
        },

        /// Represents the hash of a blob provided as part of a transaction manifest. This is
        /// represented as a byte array of 32 bytes which is serialized as a hex string.
        #[schemars(example = "crate::example::value::ast_value::blob")]
        Blob {
            #[schemars(with = "crate::model::runtime::Blob")]
            #[serde_as(as = "serde_with::FromInto<crate::model::runtime::Blob>")]
            value: ManifestBlobRef,
        },

        /// Represents a byte array of an unknown size which is serialized as a hex string
        #[schemars(example = "crate::example::value::ast_value::bytes")]
        Bytes {
            #[serde_as(as = "serde_with::hex::Hex")]
            #[schemars(with = "String")]
            hex: Vec<u8>,
        },
    }
}

/// A union of the types of discriminators that enums may have. This may either be a string or an
/// 8-bit unsigned number.
#[serializable]
#[serde(tag = "type")]
#[derive(Hash, Eq, PartialEq, PartialOrd, Ord)]
#[schemars(
    example = "crate::example::value::ast_value::enum_discriminator1",
    example = "crate::example::value::ast_value::enum_discriminator2"
)]
pub enum EnumDiscriminator {
    String {
        /// A string discriminator of the fully qualified well-known enum name
        discriminator: String,
    },
    U8 {
        /// An 8-bit unsigned integer serialized as a string.
        #[schemars(regex(pattern = "[0-9]+"))]
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        discriminator: u8,
    },
}

impl EnumDiscriminator {
    /// Resolves the enum discriminator to a [`u8`] discriminator.
    pub fn resolve_discriminator(&self) -> Result<u8, ManifestAstValueConversionError> {
        match self {
            Self::U8 { discriminator } => Ok(*discriminator),
            Self::String { discriminator } => KNOWN_ENUM_DISCRIMINATORS
                .get(discriminator.as_str())
                .copied()
                .ok_or(
                    ManifestAstValueConversionError::FailedToResolveEnumDiscriminator {
                        variant_name: discriminator.clone(),
                    },
                ),
        }
    }
}

#[serializable]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(tag = "type")]
/// Represents a tagged transient identifier typically used as an identifiers for Scrypto buckets
/// and proofs. Could either be a string or an unsigned 32-bit number (which is serialized as a
/// number and not a string)
pub enum TransientIdentifier {
    #[schemars(example = "crate::example::engine_identifier::transient_identifier::string")]
    String {
        /// A string identifier
        value: String,
    },

    #[schemars(example = "crate::example::engine_identifier::transient_identifier::u32")]
    U32 {
        /// A 32-bit unsigned integer which is serialized and deserialized as a string.
        #[schemars(regex(pattern = "[0-9]+"))]
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        value: u32,
    },
}

#[serializable]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[schemars(
    example = "crate::example::engine_identifier::transient_identifier::bucket_id1",
    example = "crate::example::engine_identifier::transient_identifier::bucket_id2"
)]
/// Represents a BucketId which uses a transient identifier.
pub struct BucketId(pub TransientIdentifier);

#[serializable]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[schemars(
    example = "crate::example::engine_identifier::transient_identifier::proof_id1",
    example = "crate::example::engine_identifier::transient_identifier::proof_id2"
)]
/// Represents a ProofId which uses a transient identifier.
pub struct ProofId(pub TransientIdentifier);

// ============
// Conversions
// ============

impl FromStr for TransientIdentifier {
    type Err = ManifestAstValueConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::String {
            value: s.to_owned(),
        })
    }
}

impl From<String> for TransientIdentifier {
    fn from(identifier: String) -> Self {
        Self::String { value: identifier }
    }
}

impl From<u32> for TransientIdentifier {
    fn from(identifier: u32) -> Self {
        Self::U32 { value: identifier }
    }
}

impl From<TransientIdentifier> for BucketId {
    fn from(identifier: TransientIdentifier) -> Self {
        Self(identifier)
    }
}

impl From<BucketId> for TransientIdentifier {
    fn from(bucket_id: BucketId) -> Self {
        bucket_id.0
    }
}

impl From<TransientIdentifier> for ProofId {
    fn from(identifier: TransientIdentifier) -> Self {
        Self(identifier)
    }
}

impl From<ProofId> for TransientIdentifier {
    fn from(proof_id: ProofId) -> Self {
        proof_id.0
    }
}
