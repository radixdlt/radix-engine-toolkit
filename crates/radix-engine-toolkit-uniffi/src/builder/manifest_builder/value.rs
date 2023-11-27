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
pub enum ManifestBuilderValue {
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
        fields: Vec<ManifestBuilderValue>,
    },
    ArrayValue {
        element_value_kind: ManifestBuilderValueKind,
        elements: Vec<ManifestBuilderValue>,
    },
    TupleValue {
        fields: Vec<ManifestBuilderValue>,
    },
    MapValue {
        key_value_kind: ManifestBuilderValueKind,
        value_value_kind: ManifestBuilderValueKind,
        entries: Vec<ManifestBuilderMapEntry>,
    },
    /* Custom */
    AddressValue {
        value: ManifestBuilderAddress,
    },
    BucketValue {
        value: ManifestBuilderBucket,
    },
    ProofValue {
        value: ManifestBuilderProof,
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
        value: ManifestBuilderAddressReservation,
    },
}

#[derive(Clone, Debug, Record)]
pub struct ManifestBuilderMapEntry {
    pub key: ManifestBuilderValue,
    pub value: ManifestBuilderValue,
}

#[derive(Clone, Debug, Enum, Copy)]
pub enum ManifestBuilderValueKind {
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

impl From<ManifestBuilderValueKind> for NativeManifestValueKind {
    fn from(value: ManifestBuilderValueKind) -> Self {
        match value {
            /* Primitive */
            ManifestBuilderValueKind::BoolValue => Self::Bool,
            ManifestBuilderValueKind::I8Value => Self::I8,
            ManifestBuilderValueKind::I16Value => Self::I16,
            ManifestBuilderValueKind::I32Value => Self::I32,
            ManifestBuilderValueKind::I64Value => Self::I64,
            ManifestBuilderValueKind::I128Value => Self::I128,
            ManifestBuilderValueKind::U8Value => Self::U8,
            ManifestBuilderValueKind::U16Value => Self::U16,
            ManifestBuilderValueKind::U32Value => Self::U32,
            ManifestBuilderValueKind::U64Value => Self::U64,
            ManifestBuilderValueKind::U128Value => Self::U128,
            ManifestBuilderValueKind::StringValue => Self::String,
            ManifestBuilderValueKind::EnumValue => Self::Enum,
            ManifestBuilderValueKind::ArrayValue => Self::Array,
            ManifestBuilderValueKind::TupleValue => Self::Tuple,
            ManifestBuilderValueKind::MapValue => Self::Map,
            /* Custom */
            ManifestBuilderValueKind::AddressValue => {
                Self::Custom(NativeManifestCustomValueKind::Address)
            }
            ManifestBuilderValueKind::BucketValue => {
                Self::Custom(NativeManifestCustomValueKind::Bucket)
            }
            ManifestBuilderValueKind::ProofValue => {
                Self::Custom(NativeManifestCustomValueKind::Proof)
            }
            ManifestBuilderValueKind::ExpressionValue => {
                Self::Custom(NativeManifestCustomValueKind::Expression)
            }
            ManifestBuilderValueKind::BlobValue => {
                Self::Custom(NativeManifestCustomValueKind::Blob)
            }
            ManifestBuilderValueKind::DecimalValue => {
                Self::Custom(NativeManifestCustomValueKind::Decimal)
            }
            ManifestBuilderValueKind::PreciseDecimalValue => {
                Self::Custom(NativeManifestCustomValueKind::PreciseDecimal)
            }
            ManifestBuilderValueKind::NonFungibleLocalIdValue => {
                Self::Custom(NativeManifestCustomValueKind::NonFungibleLocalId)
            }
            ManifestBuilderValueKind::AddressReservationValue => {
                Self::Custom(NativeManifestCustomValueKind::AddressReservation)
            }
        }
    }
}

impl From<NativeManifestValueKind> for ManifestBuilderValueKind {
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
            NativeManifestValueKind::Custom(
                NativeManifestCustomValueKind::Address,
            ) => Self::AddressValue,
            NativeManifestValueKind::Custom(
                NativeManifestCustomValueKind::Bucket,
            ) => Self::BucketValue,
            NativeManifestValueKind::Custom(
                NativeManifestCustomValueKind::Proof,
            ) => Self::ProofValue,
            NativeManifestValueKind::Custom(
                NativeManifestCustomValueKind::Expression,
            ) => Self::ExpressionValue,
            NativeManifestValueKind::Custom(
                NativeManifestCustomValueKind::Blob,
            ) => Self::BlobValue,
            NativeManifestValueKind::Custom(
                NativeManifestCustomValueKind::Decimal,
            ) => Self::DecimalValue,
            NativeManifestValueKind::Custom(
                NativeManifestCustomValueKind::PreciseDecimal,
            ) => Self::PreciseDecimalValue,
            NativeManifestValueKind::Custom(
                NativeManifestCustomValueKind::NonFungibleLocalId,
            ) => Self::NonFungibleLocalIdValue,
            NativeManifestValueKind::Custom(
                NativeManifestCustomValueKind::AddressReservation,
            ) => Self::AddressReservationValue,
        }
    }
}

impl ManifestBuilderValue {
    pub fn to_native(
        &self,
        name_record: &NameRecord,
    ) -> Result<NativeManifestValue> {
        let value = match self {
            Self::BoolValue { value } => {
                NativeManifestValue::Bool { value: *value }
            }

            Self::U8Value { value } => {
                NativeManifestValue::U8 { value: *value }
            }
            Self::U16Value { value } => {
                NativeManifestValue::U16 { value: *value }
            }
            Self::U32Value { value } => {
                NativeManifestValue::U32 { value: *value }
            }
            Self::U64Value { value } => {
                NativeManifestValue::U64 { value: *value }
            }
            Self::U128Value { value } => NativeManifestValue::U128 {
                value: value.parse()?,
            },

            Self::I8Value { value } => {
                NativeManifestValue::I8 { value: *value }
            }
            Self::I16Value { value } => {
                NativeManifestValue::I16 { value: *value }
            }
            Self::I32Value { value } => {
                NativeManifestValue::I32 { value: *value }
            }
            Self::I64Value { value } => {
                NativeManifestValue::I64 { value: *value }
            }
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
                    .map(|item| item.to_native(name_record))
                    .collect::<Result<_>>()?,
            },
            Self::ArrayValue {
                element_value_kind,
                elements,
            } => NativeManifestValue::Array {
                element_value_kind: (*element_value_kind).into(),
                elements: elements
                    .iter()
                    .map(|item| item.to_native(name_record))
                    .collect::<Result<_>>()?,
            },
            Self::TupleValue { fields } => NativeManifestValue::Tuple {
                fields: fields
                    .iter()
                    .map(|item| item.to_native(name_record))
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
                    .map(|ManifestBuilderMapEntry { key, value }| {
                        key.to_native(name_record).and_then(|key| {
                            value
                                .to_native(name_record)
                                .map(|value| (key, value))
                        })
                    })
                    .collect::<Result<_>>()?,
            },
            Self::AddressValue { value } => NativeManifestValue::Custom {
                value: NativeManifestCustomValue::Address(
                    value.to_native(name_record)?,
                ),
            },
            Self::BucketValue { value } => NativeManifestValue::Custom {
                value: NativeManifestCustomValue::Bucket(
                    value.to_native(name_record)?,
                ),
            },
            Self::ProofValue { value } => NativeManifestValue::Custom {
                value: NativeManifestCustomValue::Proof(
                    value.to_native(name_record)?,
                ),
            },
            Self::AddressReservationValue { value } => {
                NativeManifestValue::Custom {
                    value: NativeManifestCustomValue::AddressReservation(
                        value.to_native(name_record)?,
                    ),
                }
            }
            Self::ExpressionValue { value } => NativeManifestValue::Custom {
                value: NativeManifestCustomValue::Expression((*value).into()),
            },
            Self::BlobValue { value } => NativeManifestValue::Custom {
                value: NativeManifestCustomValue::Blob(value.clone().into()),
            },
            Self::DecimalValue { value } => NativeManifestValue::Custom {
                value: NativeManifestCustomValue::Decimal(native_from_decimal(
                    &value.0,
                )),
            },
            Self::PreciseDecimalValue { value } => {
                NativeManifestValue::Custom {
                    value: NativeManifestCustomValue::PreciseDecimal(
                        native_from_precise_decimal(&value.0),
                    ),
                }
            }
            Self::NonFungibleLocalIdValue { value } => {
                NativeManifestValue::Custom {
                    value: NativeManifestCustomValue::NonFungibleLocalId(
                        native_from_non_fungible_local_id(
                            value.clone().try_into()?,
                        ),
                    ),
                }
            }
        };
        Ok(value)
    }
}
