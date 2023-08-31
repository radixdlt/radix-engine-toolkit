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

use radix_engine_common::prelude::*;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use transaction::data::*;
use transaction::prelude::*;

use crate::prelude::*;

#[typeshare::typeshare]
#[derive(
    Clone, Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema, PartialEq, Eq, Hash,
)]
#[serde(tag = "kind", content = "value")]
pub enum SerializableManifestValue {
    Bool {
        value: bool,
    },
    I8 {
        value: SerializableI8,
    },
    I16 {
        value: SerializableI16,
    },
    I32 {
        value: SerializableI32,
    },
    I64 {
        value: SerializableI64,
    },
    I128 {
        value: SerializableI128,
    },
    U8 {
        value: SerializableU8,
    },
    U16 {
        value: SerializableU16,
    },
    U32 {
        value: SerializableU32,
    },
    U64 {
        value: SerializableU64,
    },
    U128 {
        value: SerializableU128,
    },
    String {
        value: String,
    },
    Enum {
        discriminator: SerializableU8,
        fields: Vec<SerializableManifestValue>,
    },
    Array {
        element_value_kind: SerializableManifestValueKind,
        elements: Vec<SerializableManifestValue>,
    },
    Tuple {
        fields: Vec<SerializableManifestValue>,
    },
    Map {
        key_value_kind: SerializableManifestValueKind,
        value_value_kind: SerializableManifestValueKind,
        entries: Vec<SerializableMapEntry>,
    },
    Address {
        value: SerializableManifestAddress,
    },
    Bucket {
        value: SerializableU32,
    },
    Proof {
        value: SerializableU32,
    },
    Expression {
        value: SerializableExpression,
    },
    Blob {
        value: SerializableHash,
    },
    Decimal {
        value: SerializableDecimal,
    },
    PreciseDecimal {
        value: SerializablePreciseDecimal,
    },
    NonFungibleLocalId {
        value: SerializableNonFungibleLocalId,
    },
    AddressReservation {
        value: SerializableU32,
    },
}

#[typeshare::typeshare]
#[derive(
    Clone,
    Copy,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    PartialEq,
    Eq,
    Hash,
)]
pub enum SerializableManifestValueKind {
    Bool,
    I8,
    I16,
    I32,
    I64,
    I128,
    U8,
    U16,
    U32,
    U64,
    U128,
    String,
    Enum,
    Array,
    Tuple,
    Map,
    Address,
    Bucket,
    Proof,
    Expression,
    Blob,
    Decimal,
    PreciseDecimal,
    NonFungibleLocalId,
    AddressReservation,
}

impl From<SerializableManifestValueKind> for ManifestValueKind {
    fn from(value: SerializableManifestValueKind) -> Self {
        match value {
            SerializableManifestValueKind::Bool => Self::Bool,
            SerializableManifestValueKind::I8 => Self::I8,
            SerializableManifestValueKind::I16 => Self::I16,
            SerializableManifestValueKind::I32 => Self::I32,
            SerializableManifestValueKind::I64 => Self::I64,
            SerializableManifestValueKind::I128 => Self::I128,
            SerializableManifestValueKind::U8 => Self::U8,
            SerializableManifestValueKind::U16 => Self::U16,
            SerializableManifestValueKind::U32 => Self::U32,
            SerializableManifestValueKind::U64 => Self::U64,
            SerializableManifestValueKind::U128 => Self::U128,
            SerializableManifestValueKind::String => Self::String,
            SerializableManifestValueKind::Enum => Self::Enum,
            SerializableManifestValueKind::Array => Self::Array,
            SerializableManifestValueKind::Tuple => Self::Tuple,
            SerializableManifestValueKind::Map => Self::Map,
            SerializableManifestValueKind::Address => {
                Self::Custom(ManifestCustomValueKind::Address)
            }
            SerializableManifestValueKind::Bucket => Self::Custom(ManifestCustomValueKind::Bucket),
            SerializableManifestValueKind::Proof => Self::Custom(ManifestCustomValueKind::Proof),
            SerializableManifestValueKind::Expression => {
                Self::Custom(ManifestCustomValueKind::Expression)
            }
            SerializableManifestValueKind::Blob => Self::Custom(ManifestCustomValueKind::Blob),
            SerializableManifestValueKind::Decimal => {
                Self::Custom(ManifestCustomValueKind::Decimal)
            }
            SerializableManifestValueKind::PreciseDecimal => {
                Self::Custom(ManifestCustomValueKind::PreciseDecimal)
            }
            SerializableManifestValueKind::NonFungibleLocalId => {
                Self::Custom(ManifestCustomValueKind::NonFungibleLocalId)
            }
            SerializableManifestValueKind::AddressReservation => {
                Self::Custom(ManifestCustomValueKind::AddressReservation)
            }
        }
    }
}

impl From<ManifestValueKind> for SerializableManifestValueKind {
    fn from(value: ManifestValueKind) -> Self {
        match value {
            ManifestValueKind::Bool => Self::Bool,
            ManifestValueKind::I8 => Self::I8,
            ManifestValueKind::I16 => Self::I16,
            ManifestValueKind::I32 => Self::I32,
            ManifestValueKind::I64 => Self::I64,
            ManifestValueKind::I128 => Self::I128,
            ManifestValueKind::U8 => Self::U8,
            ManifestValueKind::U16 => Self::U16,
            ManifestValueKind::U32 => Self::U32,
            ManifestValueKind::U64 => Self::U64,
            ManifestValueKind::U128 => Self::U128,
            ManifestValueKind::String => Self::String,
            ManifestValueKind::Enum => Self::Enum,
            ManifestValueKind::Array => Self::Array,
            ManifestValueKind::Tuple => Self::Tuple,
            ManifestValueKind::Map => Self::Map,
            ManifestValueKind::Custom(custom) => match custom {
                ManifestCustomValueKind::Address => Self::Address,
                ManifestCustomValueKind::Bucket => Self::Bucket,
                ManifestCustomValueKind::Proof => Self::Proof,
                ManifestCustomValueKind::Expression => Self::Expression,
                ManifestCustomValueKind::Blob => Self::Blob,
                ManifestCustomValueKind::Decimal => Self::Decimal,
                ManifestCustomValueKind::PreciseDecimal => Self::PreciseDecimal,
                ManifestCustomValueKind::NonFungibleLocalId => Self::NonFungibleLocalId,
                ManifestCustomValueKind::AddressReservation => Self::AddressReservation,
            },
        }
    }
}

impl SerializableManifestValue {
    pub fn from_typed<T: ManifestEncode>(
        manifest_value: &T,
        network_id: u8,
    ) -> Result<Self, ValueConversionError> {
        let value =
            manifest_decode::<ManifestValue>(&manifest_encode(manifest_value).unwrap()).unwrap();
        Self::from_manifest_value(&value, network_id)
    }

    pub fn from_manifest_value(
        manifest_value: &ManifestValue,
        network_id: u8,
    ) -> Result<Self, ValueConversionError> {
        let value = match manifest_value {
            ManifestValue::Bool { value } => Self::Bool { value: *value },
            ManifestValue::I8 { value } => Self::I8 {
                value: into!(*value),
            },
            ManifestValue::I16 { value } => Self::I16 {
                value: into!(*value),
            },
            ManifestValue::I32 { value } => Self::I32 {
                value: into!(*value),
            },
            ManifestValue::I64 { value } => Self::I64 {
                value: into!(*value),
            },
            ManifestValue::I128 { value } => Self::I128 {
                value: into!(*value),
            },
            ManifestValue::U8 { value } => Self::U8 {
                value: into!(*value),
            },
            ManifestValue::U16 { value } => Self::U16 {
                value: into!(*value),
            },
            ManifestValue::U32 { value } => Self::U32 {
                value: into!(*value),
            },
            ManifestValue::U64 { value } => Self::U64 {
                value: into!(*value),
            },
            ManifestValue::U128 { value } => Self::U128 {
                value: into!(*value),
            },
            ManifestValue::String { value } => Self::String {
                value: value.to_owned(),
            },
            ManifestValue::Enum {
                discriminator,
                fields,
            } => Self::Enum {
                discriminator: into!(*discriminator),
                fields: fields
                    .iter()
                    .map(|value| Self::from_manifest_value(value, network_id))
                    .collect::<Result<_, _>>()?,
            },
            ManifestValue::Array {
                element_value_kind,
                elements,
            } => Self::Array {
                element_value_kind: into!(*element_value_kind),
                elements: elements
                    .iter()
                    .map(|value| Self::from_manifest_value(value, network_id))
                    .collect::<Result<_, _>>()?,
            },
            ManifestValue::Tuple { fields } => Self::Tuple {
                fields: fields
                    .iter()
                    .map(|value| Self::from_manifest_value(value, network_id))
                    .collect::<Result<_, _>>()?,
            },
            ManifestValue::Map {
                key_value_kind,
                value_value_kind,
                entries,
            } => Self::Map {
                key_value_kind: into!(*key_value_kind),
                value_value_kind: into!(*value_value_kind),
                entries: entries
                    .iter()
                    .map(|(key, value)| {
                        Self::from_manifest_value(key, network_id).and_then(|key| {
                            Self::from_manifest_value(value, network_id)
                                .map(|value| SerializableMapEntry { key, value })
                        })
                    })
                    .collect::<Result<_, _>>()?,
            },
            ManifestValue::Custom {
                value: ManifestCustomValue::Address(value),
            } => match value {
                ManifestAddress::Named(named) => SerializableManifestValue::Address {
                    value: SerializableManifestAddress::Named(into!(*named)),
                },
                ManifestAddress::Static(node_id) => Self::Address {
                    value: SerializableManifestAddress::Static(SerializableNodeId(
                        SerializableNodeIdInternal {
                            node_id: *node_id,
                            network_id,
                        },
                    )),
                },
            },
            ManifestValue::Custom {
                value: ManifestCustomValue::Bucket(value),
            } => SerializableManifestValue::Bucket {
                value: into!(value.0),
            },
            ManifestValue::Custom {
                value: ManifestCustomValue::Proof(value),
            } => SerializableManifestValue::Proof {
                value: into!(value.0),
            },
            ManifestValue::Custom {
                value: ManifestCustomValue::Expression(value),
            } => SerializableManifestValue::Expression {
                value: into!(*value),
            },
            ManifestValue::Custom {
                value: ManifestCustomValue::Blob(value),
            } => SerializableManifestValue::Blob {
                value: into!(Hash(value.0)),
            },
            ManifestValue::Custom {
                value: ManifestCustomValue::Decimal(value),
            } => SerializableManifestValue::Decimal {
                value: into!(to_decimal(value.clone())),
            },
            ManifestValue::Custom {
                value: ManifestCustomValue::PreciseDecimal(value),
            } => SerializableManifestValue::PreciseDecimal {
                value: into!(to_precise_decimal(value.clone())),
            },
            ManifestValue::Custom {
                value: ManifestCustomValue::NonFungibleLocalId(value),
            } => SerializableManifestValue::NonFungibleLocalId {
                value: into!(to_non_fungible_local_id(value.clone())),
            },
            ManifestValue::Custom {
                value: ManifestCustomValue::AddressReservation(value),
            } => SerializableManifestValue::AddressReservation {
                value: into!(value.0),
            },
        };
        Ok(value)
    }

    pub fn to_manifest_value(&self) -> Result<ManifestValue, ValueConversionError> {
        let value = match self {
            Self::Bool { value } => ManifestValue::Bool { value: *value },
            Self::I8 { value } => ManifestValue::I8 { value: **value },
            Self::I16 { value } => ManifestValue::I16 { value: **value },
            Self::I32 { value } => ManifestValue::I32 { value: **value },
            Self::I64 { value } => ManifestValue::I64 { value: **value },
            Self::I128 { value } => ManifestValue::I128 { value: **value },
            Self::U8 { value } => ManifestValue::U8 { value: **value },
            Self::U16 { value } => ManifestValue::U16 { value: **value },
            Self::U32 { value } => ManifestValue::U32 { value: **value },
            Self::U64 { value } => ManifestValue::U64 { value: **value },
            Self::U128 { value } => ManifestValue::U128 { value: **value },
            Self::String { value } => ManifestValue::String {
                value: value.to_owned(),
            },
            Self::Enum {
                discriminator,
                fields,
            } => ManifestValue::Enum {
                discriminator: **discriminator,
                fields: fields
                    .iter()
                    .map(|value| value.to_manifest_value())
                    .collect::<Result<_, _>>()?,
            },
            Self::Array {
                element_value_kind,
                elements,
            } => ManifestValue::Array {
                element_value_kind: into!(*element_value_kind),
                elements: elements
                    .iter()
                    .map(|value| value.to_manifest_value())
                    .collect::<Result<_, _>>()?,
            },
            Self::Tuple { fields } => ManifestValue::Tuple {
                fields: fields
                    .iter()
                    .map(|value| value.to_manifest_value())
                    .collect::<Result<_, _>>()?,
            },
            Self::Map {
                key_value_kind,
                value_value_kind,
                entries,
            } => ManifestValue::Map {
                key_value_kind: into!(*key_value_kind),
                value_value_kind: into!(*value_value_kind),
                entries: entries
                    .iter()
                    .map(|SerializableMapEntry { key, value }| {
                        key.to_manifest_value()
                            .and_then(|key| value.to_manifest_value().map(|value| (key, value)))
                    })
                    .collect::<Result<_, _>>()?,
            },
            Self::Bucket { value } => ManifestValue::Custom {
                value: ManifestCustomValue::Bucket(ManifestBucket(**value)),
            },
            Self::Proof { value } => ManifestValue::Custom {
                value: ManifestCustomValue::Proof(ManifestProof(**value)),
            },
            Self::AddressReservation { value } => ManifestValue::Custom {
                value: ManifestCustomValue::AddressReservation(ManifestAddressReservation(**value)),
            },
            Self::Expression { value } => ManifestValue::Custom {
                value: ManifestCustomValue::Expression(into!(*value)),
            },
            Self::Blob { value } => ManifestValue::Custom {
                value: ManifestCustomValue::Blob(ManifestBlobRef((**value).0)),
            },
            Self::Decimal { value } => ManifestValue::Custom {
                value: ManifestCustomValue::Decimal(from_decimal(**value)),
            },
            Self::PreciseDecimal { value } => ManifestValue::Custom {
                value: ManifestCustomValue::PreciseDecimal(from_precise_decimal(**value)),
            },
            Self::NonFungibleLocalId { value } => ManifestValue::Custom {
                value: ManifestCustomValue::NonFungibleLocalId(from_non_fungible_local_id(
                    (**value).clone(),
                )),
            },
            Self::Address { value } => match value {
                SerializableManifestAddress::Static(value) => ManifestValue::Custom {
                    value: ManifestCustomValue::Address(ManifestAddress::Static(value.0.node_id)),
                },
                SerializableManifestAddress::Named(value) => ManifestValue::Custom {
                    value: ManifestCustomValue::Address(ManifestAddress::Named(**value)),
                },
            },
        };
        Ok(value)
    }

    pub fn to_typed<T: ManifestDecode>(&self) -> Result<T, ValueConversionError> {
        let value = self.to_manifest_value()?;
        manifest_decode(&manifest_encode(&value).unwrap())
            .map_err(|error| ValueConversionError::DecodeError(format!("{:?}", error)))
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
#[serde(tag = "kind", content = "error")]
pub enum ValueConversionError {
    DecodeError(String),
}

macro_rules! into {
    ($expr: expr) => {
        Into::into($expr)
    };
}
use into;

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SerializableMapEntry {
    pub key: SerializableManifestValue,
    pub value: SerializableManifestValue,
}
