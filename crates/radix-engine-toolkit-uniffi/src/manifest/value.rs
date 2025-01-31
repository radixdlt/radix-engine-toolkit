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
        value: ManifestAddress,
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
    AddressReservationValue {
        value: ManifestAddressReservation,
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
    AddressReservationValue,
}

impl From<ManifestValueKind> for engine::ManifestValueKind {
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
            ManifestValueKind::AddressValue => {
                Self::Custom(engine::ManifestCustomValueKind::Address)
            }
            ManifestValueKind::BucketValue => {
                Self::Custom(engine::ManifestCustomValueKind::Bucket)
            }
            ManifestValueKind::ProofValue => {
                Self::Custom(engine::ManifestCustomValueKind::Proof)
            }
            ManifestValueKind::ExpressionValue => {
                Self::Custom(engine::ManifestCustomValueKind::Expression)
            }
            ManifestValueKind::BlobValue => {
                Self::Custom(engine::ManifestCustomValueKind::Blob)
            }
            ManifestValueKind::DecimalValue => {
                Self::Custom(engine::ManifestCustomValueKind::Decimal)
            }
            ManifestValueKind::PreciseDecimalValue => {
                Self::Custom(engine::ManifestCustomValueKind::PreciseDecimal)
            }
            ManifestValueKind::NonFungibleLocalIdValue => Self::Custom(
                engine::ManifestCustomValueKind::NonFungibleLocalId,
            ),
            ManifestValueKind::AddressReservationValue => Self::Custom(
                engine::ManifestCustomValueKind::AddressReservation,
            ),
        }
    }
}

impl From<engine::ManifestValueKind> for ManifestValueKind {
    fn from(value: engine::ManifestValueKind) -> Self {
        match value {
            /* Primitive */
            engine::ManifestValueKind::Bool => Self::BoolValue,
            engine::ManifestValueKind::I8 => Self::I8Value,
            engine::ManifestValueKind::I16 => Self::I16Value,
            engine::ManifestValueKind::I32 => Self::I32Value,
            engine::ManifestValueKind::I64 => Self::I64Value,
            engine::ManifestValueKind::I128 => Self::I128Value,
            engine::ManifestValueKind::U8 => Self::U8Value,
            engine::ManifestValueKind::U16 => Self::U16Value,
            engine::ManifestValueKind::U32 => Self::U32Value,
            engine::ManifestValueKind::U64 => Self::U64Value,
            engine::ManifestValueKind::U128 => Self::U128Value,
            engine::ManifestValueKind::String => Self::StringValue,
            engine::ManifestValueKind::Enum => Self::EnumValue,
            engine::ManifestValueKind::Array => Self::ArrayValue,
            engine::ManifestValueKind::Tuple => Self::TupleValue,
            engine::ManifestValueKind::Map => Self::MapValue,
            /* Custom */
            engine::ManifestValueKind::Custom(
                engine::ManifestCustomValueKind::Address,
            ) => Self::AddressValue,
            engine::ManifestValueKind::Custom(
                engine::ManifestCustomValueKind::Bucket,
            ) => Self::BucketValue,
            engine::ManifestValueKind::Custom(
                engine::ManifestCustomValueKind::Proof,
            ) => Self::ProofValue,
            engine::ManifestValueKind::Custom(
                engine::ManifestCustomValueKind::Expression,
            ) => Self::ExpressionValue,
            engine::ManifestValueKind::Custom(
                engine::ManifestCustomValueKind::Blob,
            ) => Self::BlobValue,
            engine::ManifestValueKind::Custom(
                engine::ManifestCustomValueKind::Decimal,
            ) => Self::DecimalValue,
            engine::ManifestValueKind::Custom(
                engine::ManifestCustomValueKind::PreciseDecimal,
            ) => Self::PreciseDecimalValue,
            engine::ManifestValueKind::Custom(
                engine::ManifestCustomValueKind::NonFungibleLocalId,
            ) => Self::NonFungibleLocalIdValue,
            engine::ManifestValueKind::Custom(
                engine::ManifestCustomValueKind::AddressReservation,
            ) => Self::AddressReservationValue,
        }
    }
}

impl ManifestValue {
    pub fn to_native(&self) -> Result<engine::ManifestValue> {
        let value = match self {
            Self::BoolValue { value } => {
                engine::ManifestValue::Bool { value: *value }
            }

            Self::U8Value { value } => {
                engine::ManifestValue::U8 { value: *value }
            }
            Self::U16Value { value } => {
                engine::ManifestValue::U16 { value: *value }
            }
            Self::U32Value { value } => {
                engine::ManifestValue::U32 { value: *value }
            }
            Self::U64Value { value } => {
                engine::ManifestValue::U64 { value: *value }
            }
            Self::U128Value { value } => engine::ManifestValue::U128 {
                value: value.parse()?,
            },

            Self::I8Value { value } => {
                engine::ManifestValue::I8 { value: *value }
            }
            Self::I16Value { value } => {
                engine::ManifestValue::I16 { value: *value }
            }
            Self::I32Value { value } => {
                engine::ManifestValue::I32 { value: *value }
            }
            Self::I64Value { value } => {
                engine::ManifestValue::I64 { value: *value }
            }
            Self::I128Value { value } => engine::ManifestValue::I128 {
                value: value.parse()?,
            },

            Self::StringValue { value } => engine::ManifestValue::String {
                value: value.clone(),
            },
            Self::EnumValue {
                discriminator,
                fields,
            } => engine::ManifestValue::Enum {
                discriminator: *discriminator,
                fields: fields
                    .iter()
                    .map(|item| item.to_native())
                    .collect::<Result<_>>()?,
            },
            Self::ArrayValue {
                element_value_kind,
                elements,
            } => engine::ManifestValue::Array {
                element_value_kind: (*element_value_kind).into(),
                elements: elements
                    .iter()
                    .map(|item| item.to_native())
                    .collect::<Result<_>>()?,
            },
            Self::TupleValue { fields } => engine::ManifestValue::Tuple {
                fields: fields
                    .iter()
                    .map(|item| item.to_native())
                    .collect::<Result<_>>()?,
            },
            Self::MapValue {
                key_value_kind,
                value_value_kind,
                entries,
            } => engine::ManifestValue::Map {
                key_value_kind: (*key_value_kind).into(),
                value_value_kind: (*value_value_kind).into(),
                entries: entries
                    .iter()
                    .map(|MapEntry { key, value }| {
                        key.to_native().and_then(|key| {
                            value.to_native().map(|value| (key, value))
                        })
                    })
                    .collect::<Result<_>>()?,
            },
            Self::AddressValue { value } => engine::ManifestValue::Custom {
                value: engine::ManifestCustomValue::Address(
                    value.clone().into(),
                ),
            },
            Self::BucketValue { value } => engine::ManifestValue::Custom {
                value: engine::ManifestCustomValue::Bucket((*value).into()),
            },
            Self::ProofValue { value } => engine::ManifestValue::Custom {
                value: engine::ManifestCustomValue::Proof((*value).into()),
            },
            Self::AddressReservationValue { value } => {
                engine::ManifestValue::Custom {
                    value: engine::ManifestCustomValue::AddressReservation(
                        (*value).into(),
                    ),
                }
            }
            Self::ExpressionValue { value } => engine::ManifestValue::Custom {
                value: engine::ManifestCustomValue::Expression((*value).into()),
            },
            Self::BlobValue { value } => engine::ManifestValue::Custom {
                value: engine::ManifestCustomValue::Blob(value.clone().into()),
            },
            Self::DecimalValue { value } => engine::ManifestValue::Custom {
                value: engine::ManifestCustomValue::Decimal(
                    engine::from_decimal(value.0),
                ),
            },
            Self::PreciseDecimalValue { value } => {
                engine::ManifestValue::Custom {
                    value: engine::ManifestCustomValue::PreciseDecimal(
                        engine::from_precise_decimal(value.0),
                    ),
                }
            }
            Self::NonFungibleLocalIdValue { value } => {
                engine::ManifestValue::Custom {
                    value: engine::ManifestCustomValue::NonFungibleLocalId(
                        engine::from_non_fungible_local_id(
                            value.clone().try_into()?,
                        ),
                    ),
                }
            }
        };
        Ok(value)
    }

    pub fn from_native(native: &engine::ManifestValue, network_id: u8) -> Self {
        match native {
            engine::ManifestValue::Bool { value } => {
                Self::BoolValue { value: *value }
            }

            engine::ManifestValue::U8 { value } => {
                Self::U8Value { value: *value }
            }
            engine::ManifestValue::U16 { value } => {
                Self::U16Value { value: *value }
            }
            engine::ManifestValue::U32 { value } => {
                Self::U32Value { value: *value }
            }
            engine::ManifestValue::U64 { value } => {
                Self::U64Value { value: *value }
            }
            engine::ManifestValue::U128 { value } => Self::U128Value {
                value: value.to_string(),
            },

            engine::ManifestValue::I8 { value } => {
                Self::I8Value { value: *value }
            }
            engine::ManifestValue::I16 { value } => {
                Self::I16Value { value: *value }
            }
            engine::ManifestValue::I32 { value } => {
                Self::I32Value { value: *value }
            }
            engine::ManifestValue::I64 { value } => {
                Self::I64Value { value: *value }
            }
            engine::ManifestValue::I128 { value } => Self::I128Value {
                value: value.to_string(),
            },

            engine::ManifestValue::String { value } => Self::StringValue {
                value: value.clone(),
            },
            engine::ManifestValue::Enum {
                discriminator,
                fields,
            } => Self::EnumValue {
                discriminator: *discriminator,
                fields: fields
                    .iter()
                    .map(|value| Self::from_native(value, network_id))
                    .collect(),
            },
            engine::ManifestValue::Array {
                element_value_kind,
                elements,
            } => Self::ArrayValue {
                element_value_kind: (*element_value_kind).into(),
                elements: elements
                    .iter()
                    .map(|value| Self::from_native(value, network_id))
                    .collect(),
            },
            engine::ManifestValue::Tuple { fields } => Self::TupleValue {
                fields: fields
                    .iter()
                    .map(|value| Self::from_native(value, network_id))
                    .collect(),
            },
            engine::ManifestValue::Map {
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
            engine::ManifestValue::Custom {
                value: engine::ManifestCustomValue::Address(value),
            } => Self::AddressValue {
                value: ManifestAddress::new(value, network_id),
            },
            engine::ManifestValue::Custom {
                value: engine::ManifestCustomValue::Bucket(value),
            } => Self::BucketValue {
                value: ManifestBucket { value: value.0 },
            },
            engine::ManifestValue::Custom {
                value: engine::ManifestCustomValue::Proof(value),
            } => Self::ProofValue {
                value: ManifestProof { value: value.0 },
            },
            engine::ManifestValue::Custom {
                value: engine::ManifestCustomValue::Expression(value),
            } => Self::ExpressionValue {
                value: (*value).into(),
            },
            engine::ManifestValue::Custom {
                value: engine::ManifestCustomValue::Decimal(value),
            } => Self::DecimalValue {
                value: Arc::new(Decimal(engine::to_decimal(
                    engine::ManifestDecimal(value.0),
                ))),
            },
            engine::ManifestValue::Custom {
                value: engine::ManifestCustomValue::PreciseDecimal(value),
            } => Self::PreciseDecimalValue {
                value: Arc::new(PreciseDecimal(engine::to_precise_decimal(
                    engine::ManifestPreciseDecimal(value.0),
                ))),
            },
            engine::ManifestValue::Custom {
                value: engine::ManifestCustomValue::NonFungibleLocalId(value),
            } => Self::NonFungibleLocalIdValue {
                value: engine::to_non_fungible_local_id(value.clone()).into(),
            },
            engine::ManifestValue::Custom {
                value: engine::ManifestCustomValue::AddressReservation(value),
            } => Self::AddressReservationValue {
                value: ManifestAddressReservation { value: value.0 },
            },
            engine::ManifestValue::Custom {
                value: engine::ManifestCustomValue::Blob(value),
            } => Self::BlobValue {
                value: ManifestBlobRef {
                    value: Arc::new(Hash(engine::Hash(value.0))),
                },
            },
        }
    }
}
