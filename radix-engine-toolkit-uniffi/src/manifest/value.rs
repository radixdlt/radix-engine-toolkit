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

use crate::prelude::*;

#[derive(Clone, Debug, Enum)]
pub enum ManifestValue {
    /* Primitive */
    BoolValue {
        value: bool,
    },
    I8Value {
        value: i8,
    },
    I16Value {
        value: i16,
    },
    I32Value {
        value: i32,
    },
    I64Value {
        value: i64,
    },
    I128Value {
        value: String,
    },
    U8Value {
        value: u8,
    },
    U16Value {
        value: u16,
    },
    U32Value {
        value: u32,
    },
    U64Value {
        value: u64,
    },
    U128Value {
        value: String,
    },
    StringValue {
        value: String,
    },
    EnumValue {
        discriminator: u8,
        fields: Vec<ManifestValue>,
    },
    ArrayValue {
        element_value_kind: ManifestValueKind,
        elements: Vec<ManifestValue>,
    },
    TupleValue {
        fields: Vec<ManifestValue>,
    },
    MapValue {
        key_value_kind: ManifestValueKind,
        value_value_kind: ManifestValueKind,
        entries: Vec<MapEntry>,
    },
    /* Custom */
    AddressValue {
        value: Arc<Address>,
    },
    BucketValue {
        value: ManifestBucket,
    },
    ProofValue {
        value: ManifestProof,
    },
    ExpressionValue {
        value: ManifestExpression,
    },
    BlobValue {
        value: ManifestBlobRef,
    },
    DecimalValue {
        value: Arc<Decimal>,
    },
    PreciseDecimalValue {
        value: Arc<PreciseDecimal>,
    },
    NonFungibleLocalIdValue {
        value: NonFungibleLocalId,
    },
}

#[derive(Clone, Debug, Record)]
pub struct MapEntry {
    pub key: ManifestValue,
    pub value: ManifestValue,
}

#[derive(Clone, Debug, Enum, Copy)]
pub enum ManifestValueKind {
    /* Primitive */
    BoolValue,
    I8Value,
    I16Value,
    I32Value,
    I64Value,
    I128Value,
    U8Value,
    U16Value,
    U32Value,
    U64Value,
    U128Value,
    StringValue,
    EnumValue,
    ArrayValue,
    TupleValue,
    MapValue,
    /* Custom */
    AddressValue,
    BucketValue,
    ProofValue,
    ExpressionValue,
    BlobValue,
    DecimalValue,
    PreciseDecimalValue,
    NonFungibleLocalIdValue,
}

impl From<ManifestValueKind> for NativeManifestValueKind {
    fn from(value: ManifestValueKind) -> Self {
        match value {
            /* Primitive */
            ManifestValueKind::BoolValue => Self::Bool,
            ManifestValueKind::I8Value => Self::I8,
            ManifestValueKind::I16Value => Self::I16,
            ManifestValueKind::I32Value => Self::I32,
            ManifestValueKind::I64Value => Self::I64,
            ManifestValueKind::I128Value => Self::I128,
            ManifestValueKind::U8Value => Self::U8,
            ManifestValueKind::U16Value => Self::U16,
            ManifestValueKind::U32Value => Self::U32,
            ManifestValueKind::U64Value => Self::U64,
            ManifestValueKind::U128Value => Self::U128,
            ManifestValueKind::StringValue => Self::String,
            ManifestValueKind::EnumValue => Self::Enum,
            ManifestValueKind::ArrayValue => Self::Array,
            ManifestValueKind::TupleValue => Self::Tuple,
            ManifestValueKind::MapValue => Self::Map,
            /* Custom */
            ManifestValueKind::AddressValue => Self::Custom(NativeManifestCustomValueKind::Address),
            ManifestValueKind::BucketValue => Self::Custom(NativeManifestCustomValueKind::Bucket),
            ManifestValueKind::ProofValue => Self::Custom(NativeManifestCustomValueKind::Proof),
            ManifestValueKind::ExpressionValue => {
                Self::Custom(NativeManifestCustomValueKind::Expression)
            }
            ManifestValueKind::BlobValue => Self::Custom(NativeManifestCustomValueKind::Blob),
            ManifestValueKind::DecimalValue => Self::Custom(NativeManifestCustomValueKind::Decimal),
            ManifestValueKind::PreciseDecimalValue => {
                Self::Custom(NativeManifestCustomValueKind::PreciseDecimal)
            }
            ManifestValueKind::NonFungibleLocalIdValue => {
                Self::Custom(NativeManifestCustomValueKind::NonFungibleLocalId)
            }
        }
    }
}

impl From<NativeManifestValueKind> for ManifestValueKind {
    fn from(value: NativeManifestValueKind) -> Self {
        match value {
            /* Primitive */
            NativeManifestValueKind::Bool => Self::BoolValue,
            NativeManifestValueKind::I8 => Self::I8Value,
            NativeManifestValueKind::I16 => Self::I16Value,
            NativeManifestValueKind::I32 => Self::I32Value,
            NativeManifestValueKind::I64 => Self::I64Value,
            NativeManifestValueKind::I128 => Self::I128Value,
            NativeManifestValueKind::U8 => Self::U8Value,
            NativeManifestValueKind::U16 => Self::U16Value,
            NativeManifestValueKind::U32 => Self::U32Value,
            NativeManifestValueKind::U64 => Self::U64Value,
            NativeManifestValueKind::U128 => Self::U128Value,
            NativeManifestValueKind::String => Self::StringValue,
            NativeManifestValueKind::Enum => Self::EnumValue,
            NativeManifestValueKind::Array => Self::ArrayValue,
            NativeManifestValueKind::Tuple => Self::TupleValue,
            NativeManifestValueKind::Map => Self::MapValue,
            /* Custom */
            NativeManifestValueKind::Custom(NativeManifestCustomValueKind::Address) => {
                Self::AddressValue
            }
            NativeManifestValueKind::Custom(NativeManifestCustomValueKind::Bucket) => {
                Self::BucketValue
            }
            NativeManifestValueKind::Custom(NativeManifestCustomValueKind::Proof) => {
                Self::ProofValue
            }
            NativeManifestValueKind::Custom(NativeManifestCustomValueKind::Expression) => {
                Self::ExpressionValue
            }
            NativeManifestValueKind::Custom(NativeManifestCustomValueKind::Blob) => Self::BlobValue,
            NativeManifestValueKind::Custom(NativeManifestCustomValueKind::Decimal) => {
                Self::DecimalValue
            }
            NativeManifestValueKind::Custom(NativeManifestCustomValueKind::PreciseDecimal) => {
                Self::PreciseDecimalValue
            }
            NativeManifestValueKind::Custom(NativeManifestCustomValueKind::NonFungibleLocalId) => {
                Self::NonFungibleLocalIdValue
            }
        }
    }
}

impl ManifestValue {
    pub fn to_native(&self) -> Result<NativeManifestValue> {
        let value = match self {
            Self::BoolValue { value } => NativeManifestValue::Bool { value: *value },

            Self::U8Value { value } => NativeManifestValue::U8 { value: *value },
            Self::U16Value { value } => NativeManifestValue::U16 { value: *value },
            Self::U32Value { value } => NativeManifestValue::U32 { value: *value },
            Self::U64Value { value } => NativeManifestValue::U64 { value: *value },
            Self::U128Value { value } => NativeManifestValue::U128 {
                value: value.parse()?,
            },

            Self::I8Value { value } => NativeManifestValue::I8 { value: *value },
            Self::I16Value { value } => NativeManifestValue::I16 { value: *value },
            Self::I32Value { value } => NativeManifestValue::I32 { value: *value },
            Self::I64Value { value } => NativeManifestValue::I64 { value: *value },
            Self::I128Value { value } => NativeManifestValue::I128 {
                value: value.parse()?,
            },

            Self::StringValue { value } => NativeManifestValue::String {
                value: value.clone(),
            },
            Self::EnumValue {
                discriminator,
                fields,
            } => NativeManifestValue::Enum {
                discriminator: *discriminator,
                fields: fields
                    .iter()
                    .map(|item| item.to_native())
                    .collect::<Result<_>>()?,
            },
            Self::ArrayValue {
                element_value_kind,
                elements,
            } => NativeManifestValue::Array {
                element_value_kind: (*element_value_kind).into(),
                elements: elements
                    .iter()
                    .map(|item| item.to_native())
                    .collect::<Result<_>>()?,
            },
            Self::TupleValue { fields } => NativeManifestValue::Tuple {
                fields: fields
                    .iter()
                    .map(|item| item.to_native())
                    .collect::<Result<_>>()?,
            },
            Self::MapValue {
                key_value_kind,
                value_value_kind,
                entries,
            } => NativeManifestValue::Map {
                key_value_kind: (*key_value_kind).into(),
                value_value_kind: (*value_value_kind).into(),
                entries: entries
                    .iter()
                    .map(|MapEntry { key, value }| {
                        key.to_native()
                            .and_then(|key| value.to_native().map(|value| (key, value)))
                    })
                    .collect::<Result<_>>()?,
            },
            Self::AddressValue { value } => NativeManifestValue::Custom {
                value: NativeManifestCustomValue::Address(NativeManifestAddress(value.0)),
            },
            Self::BucketValue { value } => NativeManifestValue::Custom {
                value: NativeManifestCustomValue::Bucket((*value).into()),
            },
            Self::ProofValue { value } => NativeManifestValue::Custom {
                value: NativeManifestCustomValue::Proof((*value).into()),
            },
            Self::ExpressionValue { value } => NativeManifestValue::Custom {
                value: NativeManifestCustomValue::Expression((*value).into()),
            },
            Self::BlobValue { value } => NativeManifestValue::Custom {
                value: NativeManifestCustomValue::Blob(value.clone().into()),
            },
            Self::DecimalValue { value } => NativeManifestValue::Custom {
                value: NativeManifestCustomValue::Decimal(native_from_decimal(&value.0)),
            },
            Self::PreciseDecimalValue { value } => NativeManifestValue::Custom {
                value: NativeManifestCustomValue::PreciseDecimal(native_from_precise_decimal(
                    &value.0,
                )),
            },
            Self::NonFungibleLocalIdValue { value } => NativeManifestValue::Custom {
                value: NativeManifestCustomValue::NonFungibleLocalId(
                    native_from_non_fungible_local_id(value.clone().try_into()?),
                ),
            },
        };
        Ok(value)
    }

    pub fn from_native(native: &NativeManifestValue, network_id: u8) -> Self {
        match native {
            NativeManifestValue::Bool { value } => Self::BoolValue { value: *value },

            NativeManifestValue::U8 { value } => Self::U8Value { value: *value },
            NativeManifestValue::U16 { value } => Self::U16Value { value: *value },
            NativeManifestValue::U32 { value } => Self::U32Value { value: *value },
            NativeManifestValue::U64 { value } => Self::U64Value { value: *value },
            NativeManifestValue::U128 { value } => Self::U128Value {
                value: value.to_string(),
            },

            NativeManifestValue::I8 { value } => Self::I8Value { value: *value },
            NativeManifestValue::I16 { value } => Self::I16Value { value: *value },
            NativeManifestValue::I32 { value } => Self::I32Value { value: *value },
            NativeManifestValue::I64 { value } => Self::I64Value { value: *value },
            NativeManifestValue::I128 { value } => Self::I128Value {
                value: value.to_string(),
            },

            NativeManifestValue::String { value } => Self::StringValue {
                value: value.clone(),
            },
            NativeManifestValue::Enum {
                discriminator,
                fields,
            } => Self::EnumValue {
                discriminator: *discriminator,
                fields: fields
                    .iter()
                    .map(|value| Self::from_native(value, network_id))
                    .collect(),
            },
            NativeManifestValue::Array {
                element_value_kind,
                elements,
            } => Self::ArrayValue {
                element_value_kind: (*element_value_kind).into(),
                elements: elements
                    .iter()
                    .map(|value| Self::from_native(value, network_id))
                    .collect(),
            },
            NativeManifestValue::Tuple { fields } => Self::TupleValue {
                fields: fields
                    .iter()
                    .map(|value| Self::from_native(value, network_id))
                    .collect(),
            },
            NativeManifestValue::Map {
                key_value_kind,
                value_value_kind,
                entries,
            } => Self::MapValue {
                key_value_kind: (*key_value_kind).into(),
                value_value_kind: (*value_value_kind).into(),
                entries: entries
                    .iter()
                    .map(|(key, value)| MapEntry {
                        key: Self::from_native(key, network_id),
                        value: Self::from_native(value, network_id),
                    })
                    .collect(),
            },
            NativeManifestValue::Custom {
                value: NativeManifestCustomValue::Address(value),
            } => Self::AddressValue {
                value: Arc::new(Address(value.0, network_id)),
            },
            NativeManifestValue::Custom {
                value: NativeManifestCustomValue::Bucket(value),
            } => Self::BucketValue {
                value: ManifestBucket { value: value.0 },
            },
            NativeManifestValue::Custom {
                value: NativeManifestCustomValue::Proof(value),
            } => Self::ProofValue {
                value: ManifestProof { value: value.0 },
            },
            NativeManifestValue::Custom {
                value: NativeManifestCustomValue::Expression(value),
            } => Self::ExpressionValue {
                value: (*value).into(),
            },
            NativeManifestValue::Custom {
                value: NativeManifestCustomValue::Decimal(value),
            } => Self::DecimalValue {
                value: Arc::new(Decimal(native_to_decimal(value))),
            },
            NativeManifestValue::Custom {
                value: NativeManifestCustomValue::PreciseDecimal(value),
            } => Self::PreciseDecimalValue {
                value: Arc::new(PreciseDecimal(native_to_precise_decimal(value))),
            },
            NativeManifestValue::Custom {
                value: NativeManifestCustomValue::NonFungibleLocalId(value),
            } => Self::NonFungibleLocalIdValue {
                value: native_to_non_fungible_local_id(value.clone()).into(),
            },
            NativeManifestValue::Custom {
                value: NativeManifestCustomValue::Blob(value),
            } => Self::BlobValue {
                value: ManifestBlobRef {
                    value: Arc::new(Hash(NativeHash(value.0))),
                },
            },
        }
    }
}
