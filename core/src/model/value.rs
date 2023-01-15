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

use crate::address::*;
use crate::engine_identifier::{BucketId, ProofId};
use crate::error::{Error, Result};
use crate::TransientIdentifier;

use scrypto::prelude::ScryptoCustomValue;
use scrypto::prelude::{
    scrypto_decode, scrypto_encode, Decimal, EcdsaSecp256k1PublicKey, EcdsaSecp256k1Signature,
    EddsaEd25519PublicKey, EddsaEd25519Signature, Hash, NonFungibleId, PreciseDecimal,
    ScryptoCustomValueKind, ScryptoValue, ScryptoValueKind,
};
use scrypto::runtime::{ManifestBlobRef, ManifestExpression, Own};
use serde_with::serde_as;
use serializable::serializable;

/// The Value model used to describe all of the types that the Radix Engine Toolkit accepts and
/// returns.
#[serializable]
#[serde(tag = "type")]
#[derive(Clone)]
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

    /// Represents a tagged enum of Radix Engine Nodes which may be owned in the point of view of
    /// the transaction manifest.
    Own {
        #[serde(flatten)]
        #[schemars(with = "crate::Own")]
        #[serde_as(as = "serde_with::FromInto<crate::Own>")]
        value: Own,
    },

    /// Represents a Bech32m encoded human-readable component address. This address is serialized
    /// as a human-readable bech32m encoded string.
    ComponentAddress {
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        address: NetworkAwareComponentAddress,
    },

    /// Represents a Bech32m encoded human-readable resource address. This address is serialized
    /// as a human-readable bech32m encoded string.
    ResourceAddress {
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        address: NetworkAwareResourceAddress,
    },

    /// Represents a Bech32m encoded human-readable system address. This address is serialized
    /// as a human-readable bech32m encoded string.
    SystemAddress {
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        address: NetworkAwareSystemAddress,
    },

    /// Represents a Bech32m encoded human-readable package address. This address is serialized
    /// as a human-readable bech32m encoded string.
    PackageAddress {
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        address: NetworkAwarePackageAddress,
    },

    /// Represents a hash coming from Scrypto's and the Radix Engine's common hash function. The
    /// hashing function that they use is SHA256 which produces 32 byte long hashes which are
    /// serialized as a 64 character long hex string (since hex encoding doubles the number of
    /// bytes needed)
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
    /// and are 32 bytes each.
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

    /// Represents the hash of a blob provided as part of a transaction manifest. This is
    /// represented as a byte array of 32 bytes which is serialized as a hex string.
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

/// An Enum of all of the supported kinds of values by the Radix Engine Toolkit. This enum is
/// essentially the `type` tags used for the value model.
#[serializable]
#[derive(Clone, Copy)]
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
    /// SBOR Encodes a [`Value`].
    pub fn encode(&self) -> Result<Vec<u8>> {
        // Convert the value first to a Scrypto value
        let scrypto_value = self.to_scrypto_value()?;

        // SBOR encode the Scrypto Value and return the result
        scrypto_encode(&scrypto_value).map_err(Error::from)
    }

    /// Decodes an SBOR payload to a [`Value`] given the network context.
    pub fn decode<T: AsRef<[u8]>>(bytes: T, network_id: u8) -> Result<Self> {
        scrypto_decode::<ScryptoValue>(bytes.as_ref())
            .map(|scrypto_value| Self::from_scrypto_value(&scrypto_value, network_id))
            .map_err(Error::from)
    }

    /// Gets the [`ValueKind`] for the given value
    pub fn kind(&self) -> ValueKind {
        match self {
            Self::Bool { .. } => ValueKind::Bool,

            Self::I8 { .. } => ValueKind::I8,
            Self::I16 { .. } => ValueKind::I16,
            Self::I32 { .. } => ValueKind::I32,
            Self::I64 { .. } => ValueKind::I64,
            Self::I128 { .. } => ValueKind::I128,

            Self::U8 { .. } => ValueKind::U8,
            Self::U16 { .. } => ValueKind::U16,
            Self::U32 { .. } => ValueKind::U32,
            Self::U64 { .. } => ValueKind::U64,
            Self::U128 { .. } => ValueKind::U128,

            Self::String { .. } => ValueKind::String,

            Self::Enum { .. } => ValueKind::Enum,

            Self::Some { .. } => ValueKind::Some,
            Self::None => ValueKind::None,
            Self::Ok { .. } => ValueKind::Ok,
            Self::Err { .. } => ValueKind::Err,

            Self::Array { .. } => ValueKind::Array,
            Self::Tuple { .. } => ValueKind::Tuple,

            Self::Decimal { .. } => ValueKind::Decimal,
            Self::PreciseDecimal { .. } => ValueKind::PreciseDecimal,

            Self::PackageAddress { .. } => ValueKind::PackageAddress,
            Self::ComponentAddress { .. } => ValueKind::ComponentAddress,
            Self::ResourceAddress { .. } => ValueKind::ResourceAddress,
            Self::SystemAddress { .. } => ValueKind::SystemAddress,

            Self::Hash { .. } => ValueKind::Hash,

            Self::Bucket { .. } => ValueKind::Bucket,
            Self::Proof { .. } => ValueKind::Proof,

            Self::NonFungibleId { .. } => ValueKind::NonFungibleId,
            Self::NonFungibleAddress { .. } => ValueKind::NonFungibleAddress,

            Self::EcdsaSecp256k1PublicKey { .. } => ValueKind::EcdsaSecp256k1PublicKey,
            Self::EcdsaSecp256k1Signature { .. } => ValueKind::EcdsaSecp256k1Signature,
            Self::EddsaEd25519PublicKey { .. } => ValueKind::EddsaEd25519PublicKey,
            Self::EddsaEd25519Signature { .. } => ValueKind::EddsaEd25519Signature,

            Self::Blob { .. } => ValueKind::Blob,
            Self::Expression { .. } => ValueKind::Expression,
            Self::Bytes { .. } => ValueKind::Bytes,
            Self::Own { .. } => ValueKind::Own,
        }
    }

    /// Converts a [`Value`] to a [`ScryptoValue`].
    pub fn to_scrypto_value(&self) -> Result<ScryptoValue> {
        let value = match self {
            Self::Bool { value } => ScryptoValue::Bool { value: *value },

            Self::U8 { value } => ScryptoValue::U8 { value: *value },
            Self::U16 { value } => ScryptoValue::U16 { value: *value },
            Self::U32 { value } => ScryptoValue::U32 { value: *value },
            Self::U64 { value } => ScryptoValue::U64 { value: *value },
            Self::U128 { value } => ScryptoValue::U128 { value: *value },

            Self::I8 { value } => ScryptoValue::I8 { value: *value },
            Self::I16 { value } => ScryptoValue::I16 { value: *value },
            Self::I32 { value } => ScryptoValue::I32 { value: *value },
            Self::I64 { value } => ScryptoValue::I64 { value: *value },
            Self::I128 { value } => ScryptoValue::I128 { value: *value },

            Self::String { value } => ScryptoValue::String {
                value: value.clone(),
            },
            Self::Enum { variant, fields } => ScryptoValue::Enum {
                discriminator: variant.clone(),
                fields: fields
                    .clone()
                    .unwrap_or_default()
                    .into_iter()
                    .map(|value| value.to_scrypto_value())
                    .collect::<Result<Vec<_>>>()?,
            },
            Self::Some { value } => ScryptoValue::Enum {
                discriminator: "Some".into(),
                fields: vec![value.to_scrypto_value()?],
            },
            Self::None => ScryptoValue::Enum {
                discriminator: "None".into(),
                fields: Vec::new(),
            },
            Self::Ok { value } => ScryptoValue::Enum {
                discriminator: "Ok".into(),
                fields: vec![value.to_scrypto_value()?],
            },
            Self::Err { value } => ScryptoValue::Enum {
                discriminator: "Err".into(),
                fields: vec![value.to_scrypto_value()?],
            },
            Self::Array {
                element_kind,
                elements,
            } => ScryptoValue::Array {
                element_value_kind: (*element_kind).into(),
                elements: elements
                    .clone()
                    .into_iter()
                    .map(|value| value.to_scrypto_value())
                    .collect::<Result<Vec<_>>>()?,
            },
            Self::Tuple { elements } => ScryptoValue::Tuple {
                fields: elements
                    .clone()
                    .into_iter()
                    .map(|value| value.to_scrypto_value())
                    .collect::<Result<Vec<_>>>()?,
            },

            Self::Decimal { value } => ScryptoValue::Custom {
                value: ScryptoCustomValue::Decimal(*value),
            },
            Self::PreciseDecimal { value } => ScryptoValue::Custom {
                value: ScryptoCustomValue::PreciseDecimal(*value),
            },
            Self::ComponentAddress { address } => ScryptoValue::Custom {
                value: ScryptoCustomValue::ComponentAddress(address.address),
            },
            Self::PackageAddress { address } => ScryptoValue::Custom {
                value: ScryptoCustomValue::PackageAddress(address.address),
            },
            Self::ResourceAddress { address } => ScryptoValue::Custom {
                value: ScryptoCustomValue::ResourceAddress(address.address),
            },
            Self::SystemAddress { address } => ScryptoValue::Custom {
                value: ScryptoCustomValue::SystemAddress(address.address),
            },

            Self::Hash { value } => ScryptoValue::Custom {
                value: ScryptoCustomValue::Hash(*value),
            },

            Self::EcdsaSecp256k1PublicKey { public_key } => ScryptoValue::Custom {
                value: ScryptoCustomValue::EcdsaSecp256k1PublicKey(*public_key),
            },
            Self::EddsaEd25519PublicKey { public_key } => ScryptoValue::Custom {
                value: ScryptoCustomValue::EddsaEd25519PublicKey(*public_key),
            },

            Self::EcdsaSecp256k1Signature { signature } => ScryptoValue::Custom {
                value: ScryptoCustomValue::EcdsaSecp256k1Signature(*signature),
            },
            Self::EddsaEd25519Signature { signature } => ScryptoValue::Custom {
                value: ScryptoCustomValue::EddsaEd25519Signature(*signature),
            },

            Self::Bucket { identifier } => ScryptoValue::Custom {
                value: identifier.try_into()?,
            },
            Self::Proof { identifier } => ScryptoValue::Custom {
                value: identifier.try_into()?,
            },

            Self::NonFungibleId { value } => ScryptoValue::Custom {
                value: ScryptoCustomValue::NonFungibleId(value.clone()),
            },
            Self::NonFungibleAddress { address } => ScryptoValue::Tuple {
                fields: vec![
                    Self::ResourceAddress {
                        address: address.resource_address,
                    }
                    .to_scrypto_value()?,
                    Self::NonFungibleId {
                        value: address.non_fungible_id.clone(),
                    }
                    .to_scrypto_value()?,
                ],
            },

            Self::Blob { hash } => ScryptoValue::Custom {
                value: ScryptoCustomValue::Blob(hash.clone()),
            },
            Self::Expression { value } => ScryptoValue::Custom {
                value: ScryptoCustomValue::Expression(value.clone()),
            },
            Self::Bytes { value } => ScryptoValue::Array {
                element_value_kind: ScryptoValueKind::U8,
                elements: value
                    .clone()
                    .into_iter()
                    .map(|value| ScryptoValue::U8 { value })
                    .collect(),
            },

            Self::Own { value } => ScryptoValue::Custom {
                value: ScryptoCustomValue::Own(value.clone()),
            },
        };
        Ok(value)
    }

    /// Converts a [`ScryptoValue`] to a [`Value`] given the network id as context.
    pub fn from_scrypto_value(scrypto_value: &ScryptoValue, network_id: u8) -> Self {
        match scrypto_value {
            ScryptoValue::Bool { value } => Self::Bool { value: *value },

            ScryptoValue::U8 { value } => Self::U8 { value: *value },
            ScryptoValue::U16 { value } => Self::U16 { value: *value },
            ScryptoValue::U32 { value } => Self::U32 { value: *value },
            ScryptoValue::U64 { value } => Self::U64 { value: *value },
            ScryptoValue::U128 { value } => Self::U128 { value: *value },

            ScryptoValue::I8 { value } => Self::I8 { value: *value },
            ScryptoValue::I16 { value } => Self::I16 { value: *value },
            ScryptoValue::I32 { value } => Self::I32 { value: *value },
            ScryptoValue::I64 { value } => Self::I64 { value: *value },
            ScryptoValue::I128 { value } => Self::I128 { value: *value },

            ScryptoValue::String { value } => Self::String {
                value: value.clone(),
            },

            ScryptoValue::Enum {
                discriminator,
                fields,
            } => Self::Enum {
                variant: discriminator.clone(),
                fields: if fields.is_empty() {
                    None
                } else {
                    Some(
                        fields
                            .clone()
                            .into_iter()
                            .map(|value| Self::from_scrypto_value(&value, network_id))
                            .collect(),
                    )
                },
            },
            ScryptoValue::Array {
                element_value_kind,
                elements,
            } => Self::Array {
                element_kind: (*element_value_kind).into(),
                elements: elements
                    .clone()
                    .into_iter()
                    .map(|value| Self::from_scrypto_value(&value, network_id))
                    .collect(),
            },
            ScryptoValue::Tuple { fields } => Self::Tuple {
                elements: fields
                    .clone()
                    .into_iter()
                    .map(|value| Self::from_scrypto_value(&value, network_id))
                    .collect(),
            },

            ScryptoValue::Custom {
                value: ScryptoCustomValue::PackageAddress(address),
            } => Self::PackageAddress {
                address: NetworkAwarePackageAddress {
                    network_id,
                    address: *address,
                },
            },
            ScryptoValue::Custom {
                value: ScryptoCustomValue::ResourceAddress(address),
            } => Self::ResourceAddress {
                address: NetworkAwareResourceAddress {
                    network_id,
                    address: *address,
                },
            },
            ScryptoValue::Custom {
                value: ScryptoCustomValue::ComponentAddress(address),
            } => Self::ComponentAddress {
                address: NetworkAwareComponentAddress {
                    network_id,
                    address: *address,
                },
            },
            ScryptoValue::Custom {
                value: ScryptoCustomValue::SystemAddress(address),
            } => Self::SystemAddress {
                address: NetworkAwareSystemAddress {
                    network_id,
                    address: *address,
                },
            },

            ScryptoValue::Custom {
                value: ScryptoCustomValue::Bucket(identifier),
            } => Self::Bucket {
                identifier: TransientIdentifier::U32(identifier.0).into(),
            },
            ScryptoValue::Custom {
                value: ScryptoCustomValue::Proof(identifier),
            } => Self::Proof {
                identifier: TransientIdentifier::U32(identifier.0).into(),
            },

            ScryptoValue::Custom {
                value: ScryptoCustomValue::Expression(value),
            } => Self::Expression {
                value: value.clone(),
            },
            ScryptoValue::Custom {
                value: ScryptoCustomValue::Blob(value),
            } => Self::Blob {
                hash: value.clone(),
            },
            ScryptoValue::Custom {
                value: ScryptoCustomValue::Hash(value),
            } => Self::Hash { value: *value },

            ScryptoValue::Custom {
                value: ScryptoCustomValue::EcdsaSecp256k1PublicKey(value),
            } => Self::EcdsaSecp256k1PublicKey { public_key: *value },
            ScryptoValue::Custom {
                value: ScryptoCustomValue::EddsaEd25519PublicKey(value),
            } => Self::EddsaEd25519PublicKey { public_key: *value },
            ScryptoValue::Custom {
                value: ScryptoCustomValue::EcdsaSecp256k1Signature(value),
            } => Self::EcdsaSecp256k1Signature { signature: *value },
            ScryptoValue::Custom {
                value: ScryptoCustomValue::EddsaEd25519Signature(value),
            } => Self::EddsaEd25519Signature { signature: *value },

            ScryptoValue::Custom {
                value: ScryptoCustomValue::Decimal(value),
            } => Self::Decimal { value: *value },
            ScryptoValue::Custom {
                value: ScryptoCustomValue::PreciseDecimal(value),
            } => Self::PreciseDecimal { value: *value },

            ScryptoValue::Custom {
                value: ScryptoCustomValue::NonFungibleId(value),
            } => Self::NonFungibleId {
                value: value.clone(),
            },

            ScryptoValue::Custom {
                value: ScryptoCustomValue::Own(value),
            } => Self::Own {
                value: value.clone(),
            },
        }
    }

    /// Handles the aliasing of certain [`Value`] kinds such as [`Value::Enum`] and
    /// [`Value::NonFungibleAddress`]
    pub fn alias(self) -> Self {
        match self {
            // Case: Some - An enum with a discriminator of "Some" which has a single field.
            Self::Enum { variant, fields }
                if variant == "Some" && fields.as_ref().map_or(0, |fields| fields.len()) == 1 =>
            {
                Self::Some {
                    value: Box::new(
                        fields
                            .unwrap_or_default()
                            .get(0)
                            .expect("Illegal State!")
                            .clone(),
                    ),
                }
            }
            // Case: None - An enum with a discriminator of "None" which has no fields.
            Self::Enum { variant, fields }
                if variant == "None" && fields.as_ref().map_or(0, |fields| fields.len()) == 0 =>
            {
                Self::None
            }
            // Case: Ok - An enum with a discriminator of "Ok" which has a single field.
            Self::Enum { variant, fields }
                if variant == "Ok" && fields.as_ref().map_or(0, |fields| fields.len()) == 1 =>
            {
                Self::Ok {
                    value: Box::new(
                        fields
                            .unwrap_or_default()
                            .get(0)
                            .expect("Illegal State!")
                            .clone(),
                    ),
                }
            }
            // Case: Err - An enum with a discriminator of "Err" which has a single field.
            Self::Enum { variant, fields }
                if variant == "Err" && fields.as_ref().map_or(0, |fields| fields.len()) == 1 =>
            {
                Self::Err {
                    value: Box::new(
                        fields
                            .unwrap_or_default()
                            .get(0)
                            .expect("Illegal State!")
                            .clone(),
                    ),
                }
            }
            Self::Tuple { ref elements } => {
                // Case: NonFungibleAddress - A tuple of ResourceAddress and NonFungibleId
                match (elements.get(0), elements.get(1)) {
                    (
                        Some(Value::ResourceAddress {
                            address: resource_address,
                        }),
                        Some(Value::NonFungibleId {
                            value: non_fungible_id,
                        }),
                    ) if elements.len() == 2 => Value::NonFungibleAddress {
                        address: NonFungibleAddress {
                            resource_address: *resource_address,
                            non_fungible_id: non_fungible_id.clone(),
                        },
                    },
                    _ => self,
                }
            }
            v => v,
        }
    }
}

impl From<ScryptoValueKind> for ValueKind {
    fn from(value: ScryptoValueKind) -> Self {
        match value {
            ScryptoValueKind::Bool => ValueKind::Bool,

            ScryptoValueKind::U8 => ValueKind::U8,
            ScryptoValueKind::U16 => ValueKind::U16,
            ScryptoValueKind::U32 => ValueKind::U32,
            ScryptoValueKind::U64 => ValueKind::U64,
            ScryptoValueKind::U128 => ValueKind::U128,

            ScryptoValueKind::I8 => ValueKind::I8,
            ScryptoValueKind::I16 => ValueKind::I16,
            ScryptoValueKind::I32 => ValueKind::I32,
            ScryptoValueKind::I64 => ValueKind::I64,
            ScryptoValueKind::I128 => ValueKind::I128,

            ScryptoValueKind::String => ValueKind::String,

            ScryptoValueKind::Enum => ValueKind::Enum,
            ScryptoValueKind::Array => ValueKind::Array,
            ScryptoValueKind::Tuple => ValueKind::Tuple,

            ScryptoValueKind::Custom(custom_type_id) => match custom_type_id {
                ScryptoCustomValueKind::PackageAddress => ValueKind::PackageAddress,
                ScryptoCustomValueKind::ComponentAddress => ValueKind::ComponentAddress,
                ScryptoCustomValueKind::ResourceAddress => ValueKind::ResourceAddress,
                ScryptoCustomValueKind::SystemAddress => ValueKind::SystemAddress,

                ScryptoCustomValueKind::Bucket => ValueKind::Bucket,
                ScryptoCustomValueKind::Proof => ValueKind::Proof,

                ScryptoCustomValueKind::Expression => ValueKind::Expression,
                ScryptoCustomValueKind::Blob => ValueKind::Blob,

                ScryptoCustomValueKind::Hash => ValueKind::Hash,
                ScryptoCustomValueKind::EcdsaSecp256k1PublicKey => {
                    ValueKind::EcdsaSecp256k1PublicKey
                }
                ScryptoCustomValueKind::EcdsaSecp256k1Signature => {
                    ValueKind::EcdsaSecp256k1Signature
                }
                ScryptoCustomValueKind::EddsaEd25519PublicKey => ValueKind::EddsaEd25519PublicKey,
                ScryptoCustomValueKind::EddsaEd25519Signature => ValueKind::EddsaEd25519Signature,

                ScryptoCustomValueKind::Decimal => ValueKind::Decimal,
                ScryptoCustomValueKind::PreciseDecimal => ValueKind::PreciseDecimal,

                ScryptoCustomValueKind::NonFungibleId => ValueKind::NonFungibleId,
                ScryptoCustomValueKind::Own => ValueKind::Own,
            },
        }
    }
}

impl From<ValueKind> for ScryptoValueKind {
    fn from(value: ValueKind) -> Self {
        match value {
            ValueKind::Bool => ScryptoValueKind::Bool,

            ValueKind::U8 => ScryptoValueKind::U8,
            ValueKind::U16 => ScryptoValueKind::U16,
            ValueKind::U32 => ScryptoValueKind::U32,
            ValueKind::U64 => ScryptoValueKind::U64,
            ValueKind::U128 => ScryptoValueKind::U128,

            ValueKind::I8 => ScryptoValueKind::I8,
            ValueKind::I16 => ScryptoValueKind::I16,
            ValueKind::I32 => ScryptoValueKind::I32,
            ValueKind::I64 => ScryptoValueKind::I64,
            ValueKind::I128 => ScryptoValueKind::I128,

            ValueKind::String => ScryptoValueKind::String,

            ValueKind::Enum => ScryptoValueKind::Enum,

            ValueKind::Some => ScryptoValueKind::Enum,
            ValueKind::None => ScryptoValueKind::Enum,
            ValueKind::Ok => ScryptoValueKind::Enum,
            ValueKind::Err => ScryptoValueKind::Enum,

            ValueKind::Array => ScryptoValueKind::Array,
            ValueKind::Bytes => ScryptoValueKind::Array,
            ValueKind::Tuple => ScryptoValueKind::Tuple,

            ValueKind::SystemAddress => {
                ScryptoValueKind::Custom(ScryptoCustomValueKind::SystemAddress)
            }
            ValueKind::PackageAddress => {
                ScryptoValueKind::Custom(ScryptoCustomValueKind::PackageAddress)
            }
            ValueKind::ResourceAddress => {
                ScryptoValueKind::Custom(ScryptoCustomValueKind::ResourceAddress)
            }
            ValueKind::ComponentAddress => {
                ScryptoValueKind::Custom(ScryptoCustomValueKind::ComponentAddress)
            }

            ValueKind::Proof => ScryptoValueKind::Custom(ScryptoCustomValueKind::Proof),
            ValueKind::Bucket => ScryptoValueKind::Custom(ScryptoCustomValueKind::Bucket),

            ValueKind::Expression => ScryptoValueKind::Custom(ScryptoCustomValueKind::Expression),
            ValueKind::Blob => ScryptoValueKind::Custom(ScryptoCustomValueKind::Blob),
            ValueKind::NonFungibleAddress => ScryptoValueKind::Tuple,

            ValueKind::Hash => ScryptoValueKind::Custom(ScryptoCustomValueKind::Hash),
            ValueKind::EcdsaSecp256k1PublicKey => {
                ScryptoValueKind::Custom(ScryptoCustomValueKind::EcdsaSecp256k1PublicKey)
            }
            ValueKind::EcdsaSecp256k1Signature => {
                ScryptoValueKind::Custom(ScryptoCustomValueKind::EcdsaSecp256k1Signature)
            }
            ValueKind::EddsaEd25519PublicKey => {
                ScryptoValueKind::Custom(ScryptoCustomValueKind::EddsaEd25519PublicKey)
            }
            ValueKind::EddsaEd25519Signature => {
                ScryptoValueKind::Custom(ScryptoCustomValueKind::EddsaEd25519Signature)
            }
            ValueKind::Decimal => ScryptoValueKind::Custom(ScryptoCustomValueKind::Decimal),
            ValueKind::PreciseDecimal => {
                ScryptoValueKind::Custom(ScryptoCustomValueKind::PreciseDecimal)
            }
            ValueKind::NonFungibleId => {
                ScryptoValueKind::Custom(ScryptoCustomValueKind::NonFungibleId)
            }
            ValueKind::Own => ScryptoValueKind::Custom(ScryptoCustomValueKind::Own),
        }
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

value_invertible! {U8, u8, value}
value_invertible! {U32, u32, value}
value_invertible! {Own, Own, value}
value_invertible! {String, String, value}
value_invertible! {Decimal, Decimal, value}
value_invertible! {Proof, ProofId, identifier}
value_invertible! {Blob, ManifestBlobRef, hash}
value_invertible! {Bucket, BucketId, identifier}
value_invertible! {NonFungibleId, NonFungibleId, value}
value_invertible! {NonFungibleAddress, NonFungibleAddress, address}
value_invertible! {SystemAddress, NetworkAwareSystemAddress, address}
value_invertible! {PackageAddress, NetworkAwarePackageAddress, address}
value_invertible! {ResourceAddress, NetworkAwareResourceAddress, address}
value_invertible! {ComponentAddress, NetworkAwareComponentAddress, address}
value_invertible! {EcdsaSecp256k1PublicKey, EcdsaSecp256k1PublicKey, public_key}
