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

use crate::address::network_aware_address::*;
use crate::engine_identifier::{BucketId, ProofId};
use crate::NonFungibleAddress;

use scrypto::prelude::{
    Decimal, EcdsaSecp256k1PublicKey, EcdsaSecp256k1Signature, EddsaEd25519PublicKey,
    EddsaEd25519Signature, Hash, NonFungibleId, PreciseDecimal,
};
use scrypto::runtime::{ManifestBlobRef, ManifestExpression, Own};
use serde_with::serde_as;
use serializable::serializable;

#[serializable]
#[serde(tag = "type")]
/// The Value model used to describe all of the types that the Radix Engine Toolkit accepts and
/// returns.
pub enum Value {
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

    /// A Rust-style Enum which has a variant and can optionally also have a list of values (acting
    /// in a way similar to discriminated algebraic sum types)
    Enum {
        /// The name of the variant of the enum
        variant: String,

        /// Optional fields that the enum may have
        #[serde(default, skip_serializing_if = "Option::is_none")]
        fields: Option<Vec<Value>>,
    },

    /// The `Some` case of Rust Options where the value is some Value
    Some { value: Box<Value> },

    /// The `None` case of Rust Options where there is value
    None,

    /// The `Ok` case of Rust Results where the value is some Value
    Ok { value: Box<Value> },

    /// The `Err` case of Rust Results where the value is some Value
    Err { value: Box<Value> },

    /// An array values of a single value kind
    Array {
        /// The kind of elements that the array contains. An array will be validated to ensure that
        /// it contains a single element kind.
        element_kind: ValueKind,

        /// The elements of the array which may contain 0 or more elements.
        elements: Vec<Value>,
    },

    /// An array of elements where elements could be of different kinds.
    Tuple { elements: Vec<Value> },

    /// A Scrypto Decimal which has a precision of 18 decimal places and has a maximum and minimum
    /// of 57896044618658097711785492504343953926634992332820282019728.792003956564819967 and
    /// -57896044618658097711785492504343953926634992332820282019728.792003956564819968 respectively
    Decimal {
        #[schemars(regex(pattern = "[+-]?([0-9]*[.])?[0-9]+"))]
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        value: Decimal,
    },

    /// A Scrypto PreciseDecimal which has a precision of 64 decimal places and has a maximum and
    /// minimum of 670390396497129854978701249910292306373968291029619668886178072186088201503677348840093714.9083451713845015929093243025426876941405973284973216824503042047
    /// and -670390396497129854978701249910292306373968291029619668886178072186088201503677348840093714.9083451713845015929093243025426876941405973284973216824503042048
    /// respectively
    PreciseDecimal {
        #[schemars(regex(pattern = "[+-]?([0-9]*[.])?[0-9]+"))]
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        value: PreciseDecimal,
    },

    /// Represents a tagged enum of Radix Engine Nodes which may be owned in the point of view of
    /// the transaction manifest.
    Own {
        #[serde(flatten)]
        #[schemars(with = "crate::Own")]
        #[serde_as(as = "serde_with::FromInto<crate::Own>")]
        value: Own,
    },

    /// Represents a Bech32m encoded human-readable component address
    ComponentAddress {
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        address: NetworkAwareComponentAddress,
    },

    /// Represents a Bech32m encoded human-readable resource address
    ResourceAddress {
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        address: NetworkAwareResourceAddress,
    },

    /// Represents a Bech32m encoded human-readable system address
    SystemAddress {
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        address: NetworkAwareSystemAddress,
    },

    /// Represents a Bech32m encoded human-readable package address
    PackageAddress {
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        address: NetworkAwarePackageAddress,
    },

    /// Represents a hash coming from Scrypto's and the Radix Engine's common hash function. The
    /// hashing function that they use is SHA256 which produces 32 byte long hashes which are
    /// serialized as a 64 character long hex string (since hex encoding doubles the number of bytes
    /// needed)
    Hash {
        #[schemars(length(equal = 64))]
        #[schemars(regex(pattern = "[0-9a-fA-F]+"))]
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        value: Hash,
    },

    /// A byte array of 33 bytes which are serialized as a 66 character long hex-encoded string
    /// representing a public key from the ECDSA Secp256k1 elliptic curve.
    EcdsaSecp256k1PublicKey {
        #[schemars(length(equal = 66))]
        #[schemars(regex(pattern = "[0-9a-fA-F]+"))]
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        public_key: EcdsaSecp256k1PublicKey,
    },

    /// A byte array of 65 bytes which are serialized as a 130 character long hex-encoded string
    /// representing a signature from the ECDSA Secp256k1 elliptic curve. An important note on
    /// ECDSA Secp256k1 signatures is that the format used and accepted by Scrypto is [v, r, s]
    /// where `v` is the recovery id and is a single byte and `r` and `s` are the signature results
    /// and are 32 bytes each. The use of other signature format
    EcdsaSecp256k1Signature {
        #[schemars(length(equal = 130))]
        #[schemars(regex(pattern = "[0-9a-fA-F]+"))]
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        signature: EcdsaSecp256k1Signature,
    },

    /// A byte array of 32 bytes which are serialized as a 64 character long hex-encoded string
    /// representing a public key from the EDDSA Ed25519 edwards curve.
    EddsaEd25519PublicKey {
        #[schemars(length(equal = 64))]
        #[schemars(regex(pattern = "[0-9a-fA-F]+"))]
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        public_key: EddsaEd25519PublicKey,
    },

    /// A byte array of 64 bytes which are serialized as a 128 character long hex-encoded string
    /// representing a signature from the EDDSA Ed25519 edwards curve.
    EddsaEd25519Signature {
        #[schemars(length(equal = 128))]
        #[schemars(regex(pattern = "[0-9a-fA-F]+"))]
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        signature: EddsaEd25519Signature,
    },

    /// Represents a Scrypto bucket which is identified through a transient identifier which is
    /// either a string or an unsigned 32-bit integer which is serialized as a number.
    Bucket { identifier: BucketId },

    /// Represents a Scrypto proof which is identified through a transient identifier which is
    /// either a string or an unsigned 32-bit integer which is serialized as a number.
    Proof { identifier: ProofId },

    /// Represents non-fungible ids which is a discriminated union of the different types that
    /// non-fungible ids may be.
    NonFungibleId {
        #[serde(flatten)]
        #[schemars(with = "crate::NonFungibleId")]
        #[serde_as(as = "serde_with::FromInto<crate::NonFungibleId>")]
        value: NonFungibleId,
    },

    /// Represents a non-fungible address which may be considered as the "global" address of a
    /// non-fungible unit as it contains both the resource address and the non-fungible id for that
    /// unit.
    NonFungibleAddress {
        #[serde(flatten)]
        address: NonFungibleAddress,
    },

    /// Represents a transaction manifest expression.
    Expression {
        #[schemars(with = "crate::Expression")]
        #[serde_as(as = "serde_with::FromInto<crate::Expression>")]
        value: ManifestExpression,
    },

    /// Represents the hash of a blob provided as part of a transaction manifest. This is represented as
    /// a byte array of 32 bytes which is serialized as a hex string.
    Blob {
        #[schemars(with = "crate::Blob")]
        #[serde_as(as = "serde_with::FromInto<crate::Blob>")]
        hash: ManifestBlobRef,
    },

    /// Represents a byte array of an unknown size which is serialized as a hex string
    Bytes {
        #[serde_as(as = "serde_with::hex::Hex")]
        #[schemars(with = "String")]
        value: Vec<u8>,
    },
}

#[serializable]
/// An Enum of all of the supported kinds of values by the Radix Engine Toolkit. This enum is
/// essentially the `type` tags used for the value model.
pub enum ValueKind {
    Bool,

    U8,
    U16,
    U32,
    U64,
    U128,

    I8,
    I16,
    I32,
    I64,
    I128,

    String,

    Enum,

    Some,
    None,
    Ok,
    Err,

    Array,
    Tuple,

    Decimal,
    PreciseDecimal,

    Own,

    ComponentAddress,
    ResourceAddress,
    SystemAddress,
    PackageAddress,

    Hash,

    EcdsaSecp256k1PublicKey,
    EcdsaSecp256k1Signature,
    EddsaEd25519PublicKey,
    EddsaEd25519Signature,

    Bucket,
    Proof,

    NonFungibleId,
    NonFungibleAddress,

    Expression,
    Blob,
    Bytes,
}

impl Value {
    pub fn kind(&self) -> ValueKind {
        todo!()
    }
}

// ============
// Conversions
// ============

macro_rules! value_invertible {
    ($variant_name: ident, $underlying_type: ident, $field: ident) => {
        impl TryFrom<$underlying_type> for Value {
            type Error = $crate::error::Error;

            fn try_from($field: $underlying_type) -> $crate::error::Result<Self> {
                Ok(Value::$variant_name { $field })
            }
        }

        impl TryFrom<Value> for $underlying_type {
            type Error = $crate::error::Error;

            fn try_from(val: Value) -> $crate::error::Result<Self> {
                match val {
                    Value::$variant_name { $field } => Ok($field),
                    _ => Err($crate::error::Error::InvalidKind {
                        expected: vec![ValueKind::$variant_name],
                        found: val.kind(),
                    }),
                }
            }
        }
    };
}

value_invertible! {NonFungibleId, NonFungibleId, value}
value_invertible! {ResourceAddress, NetworkAwareResourceAddress, address}
