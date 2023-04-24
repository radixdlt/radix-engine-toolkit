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

use super::{model::*, ManifestSborValueConversionError};

use crate::model::address::NetworkAwareNodeId;
use crate::utils::checked_copy_u8_slice;

use scrypto::prelude::{
    BytesNonFungibleLocalId, Decimal, IntegerNonFungibleLocalId, ManifestCustomValue,
    ManifestCustomValueKind, ManifestValue, ManifestValueKind, NonFungibleLocalId, PreciseDecimal,
    StringNonFungibleLocalId, UUIDNonFungibleLocalId,
};
use scrypto::prelude::{
    ManifestAddress, ManifestBlobRef, ManifestBucket, ManifestDecimal, ManifestNonFungibleLocalId,
    ManifestPreciseDecimal, ManifestProof,
};

impl From<ManifestValueKind> for ManifestSborValueKind {
    fn from(value: ManifestValueKind) -> Self {
        match value {
            ManifestValueKind::Bool => ManifestSborValueKind::Bool,

            ManifestValueKind::U8 => ManifestSborValueKind::U8,
            ManifestValueKind::U16 => ManifestSborValueKind::U16,
            ManifestValueKind::U32 => ManifestSborValueKind::U32,
            ManifestValueKind::U64 => ManifestSborValueKind::U64,
            ManifestValueKind::U128 => ManifestSborValueKind::U128,

            ManifestValueKind::I8 => ManifestSborValueKind::I8,
            ManifestValueKind::I16 => ManifestSborValueKind::I16,
            ManifestValueKind::I32 => ManifestSborValueKind::I32,
            ManifestValueKind::I64 => ManifestSborValueKind::I64,
            ManifestValueKind::I128 => ManifestSborValueKind::I128,

            ManifestValueKind::String => ManifestSborValueKind::String,

            ManifestValueKind::Enum => ManifestSborValueKind::Enum,
            ManifestValueKind::Map => ManifestSborValueKind::Map,
            ManifestValueKind::Array => ManifestSborValueKind::Array,
            ManifestValueKind::Tuple => ManifestSborValueKind::Tuple,

            ManifestValueKind::Custom(ManifestCustomValueKind::Address) => {
                ManifestSborValueKind::Address
            }

            ManifestValueKind::Custom(ManifestCustomValueKind::Decimal) => {
                ManifestSborValueKind::Decimal
            }
            ManifestValueKind::Custom(ManifestCustomValueKind::PreciseDecimal) => {
                ManifestSborValueKind::PreciseDecimal
            }
            ManifestValueKind::Custom(ManifestCustomValueKind::NonFungibleLocalId) => {
                ManifestSborValueKind::NonFungibleLocalId
            }

            ManifestValueKind::Custom(ManifestCustomValueKind::Bucket) => {
                ManifestSborValueKind::Bucket
            }
            ManifestValueKind::Custom(ManifestCustomValueKind::Proof) => {
                ManifestSborValueKind::Proof
            }

            ManifestValueKind::Custom(ManifestCustomValueKind::Blob) => ManifestSborValueKind::Blob,
            ManifestValueKind::Custom(ManifestCustomValueKind::Expression) => {
                ManifestSborValueKind::Expression
            }
        }
    }
}

impl From<ManifestSborValueKind> for ManifestValueKind {
    fn from(value: ManifestSborValueKind) -> Self {
        match value {
            ManifestSborValueKind::Bool => ManifestValueKind::Bool,

            ManifestSborValueKind::U8 => ManifestValueKind::U8,
            ManifestSborValueKind::U16 => ManifestValueKind::U16,
            ManifestSborValueKind::U32 => ManifestValueKind::U32,
            ManifestSborValueKind::U64 => ManifestValueKind::U64,
            ManifestSborValueKind::U128 => ManifestValueKind::U128,

            ManifestSborValueKind::I8 => ManifestValueKind::I8,
            ManifestSborValueKind::I16 => ManifestValueKind::I16,
            ManifestSborValueKind::I32 => ManifestValueKind::I32,
            ManifestSborValueKind::I64 => ManifestValueKind::I64,
            ManifestSborValueKind::I128 => ManifestValueKind::I128,

            ManifestSborValueKind::String => ManifestValueKind::String,

            ManifestSborValueKind::Enum => ManifestValueKind::Enum,
            ManifestSborValueKind::Map => ManifestValueKind::Map,
            ManifestSborValueKind::Array => ManifestValueKind::Array,
            ManifestSborValueKind::Tuple => ManifestValueKind::Tuple,

            ManifestSborValueKind::Address => {
                ManifestValueKind::Custom(ManifestCustomValueKind::Address)
            }
            ManifestSborValueKind::Decimal => {
                ManifestValueKind::Custom(ManifestCustomValueKind::Decimal)
            }
            ManifestSborValueKind::PreciseDecimal => {
                ManifestValueKind::Custom(ManifestCustomValueKind::PreciseDecimal)
            }
            ManifestSborValueKind::NonFungibleLocalId => {
                ManifestValueKind::Custom(ManifestCustomValueKind::NonFungibleLocalId)
            }
            ManifestSborValueKind::Bucket => {
                ManifestValueKind::Custom(ManifestCustomValueKind::Bucket)
            }
            ManifestSborValueKind::Proof => {
                ManifestValueKind::Custom(ManifestCustomValueKind::Proof)
            }

            ManifestSborValueKind::Blob => ManifestValueKind::Custom(ManifestCustomValueKind::Blob),
            ManifestSborValueKind::Expression => {
                ManifestValueKind::Custom(ManifestCustomValueKind::Expression)
            }
        }
    }
}

impl ManifestSborValue {
    pub fn to_manifest_sbor_value(
        &self,
    ) -> Result<ManifestValue, ManifestSborValueConversionError> {
        let value = match self {
            Self::Bool { value } => ManifestValue::Bool { value: *value },

            Self::U8 { value } => ManifestValue::U8 { value: *value },
            Self::U16 { value } => ManifestValue::U16 { value: *value },
            Self::U32 { value } => ManifestValue::U32 { value: *value },
            Self::U64 { value } => ManifestValue::U64 { value: *value },
            Self::U128 { value } => ManifestValue::U128 { value: *value },

            Self::I8 { value } => ManifestValue::I8 { value: *value },
            Self::I16 { value } => ManifestValue::I16 { value: *value },
            Self::I32 { value } => ManifestValue::I32 { value: *value },
            Self::I64 { value } => ManifestValue::I64 { value: *value },
            Self::I128 { value } => ManifestValue::I128 { value: *value },

            Self::String { value } => ManifestValue::String {
                value: value.clone(),
            },
            Self::Enum { variant, fields } => ManifestValue::Enum {
                discriminator: *variant,
                fields: fields
                    .clone()
                    .unwrap_or_default()
                    .into_iter()
                    .map(|value| value.to_manifest_sbor_value())
                    .collect::<Result<Vec<_>, ManifestSborValueConversionError>>()?,
            },
            Self::Map {
                key_value_kind,
                value_value_kind,
                entries,
            } => ManifestValue::Map {
                key_value_kind: (*key_value_kind).into(),
                value_value_kind: (*value_value_kind).into(),
                entries: {
                    let mut scrypto_entries = Vec::new();
                    for (key, value) in entries {
                        scrypto_entries.push((
                            key.to_manifest_sbor_value()?,
                            value.to_manifest_sbor_value()?,
                        ))
                    }
                    scrypto_entries
                },
            },
            Self::Array {
                element_kind,
                elements,
            } => ManifestValue::Array {
                element_value_kind: (*element_kind).into(),
                elements: elements
                    .clone()
                    .into_iter()
                    .map(|value| value.to_manifest_sbor_value())
                    .collect::<Result<Vec<_>, ManifestSborValueConversionError>>()?,
            },
            Self::Tuple { elements } => ManifestValue::Tuple {
                fields: elements
                    .clone()
                    .into_iter()
                    .map(|value| value.to_manifest_sbor_value())
                    .collect::<Result<Vec<_>, ManifestSborValueConversionError>>()?,
            },

            Self::Address { address } => ManifestValue::Custom {
                value: ManifestCustomValue::Address(ManifestAddress((*address).into())),
            },

            Self::Decimal { value } => ManifestValue::Custom {
                value: ManifestCustomValue::Decimal(ManifestDecimal(
                    checked_copy_u8_slice(value.to_vec()).map_or(
                        Err(ManifestSborValueConversionError::InvalidLength {
                            expected: 32,
                            actual: value.to_vec().len(),
                        }),
                        Ok,
                    )?,
                )),
            },
            Self::PreciseDecimal { value } => ManifestValue::Custom {
                value: ManifestCustomValue::PreciseDecimal(ManifestPreciseDecimal(
                    checked_copy_u8_slice(value.to_vec()).map_or(
                        Err(ManifestSborValueConversionError::InvalidLength {
                            expected: 64,
                            actual: value.to_vec().len(),
                        }),
                        Ok,
                    )?,
                )),
            },

            Self::Bucket { identifier } => ManifestValue::Custom {
                value: ManifestCustomValue::Bucket(ManifestBucket(*identifier)),
            },
            Self::Proof { identifier } => ManifestValue::Custom {
                value: ManifestCustomValue::Proof(ManifestProof(*identifier)),
            },

            Self::Expression { value } => ManifestValue::Custom {
                value: ManifestCustomValue::Expression(*value),
            },
            Self::Blob { hash } => ManifestValue::Custom {
                value: ManifestCustomValue::Blob(ManifestBlobRef(hash.0)),
            },

            Self::NonFungibleLocalId { value } => ManifestValue::Custom {
                value: ManifestCustomValue::NonFungibleLocalId(match value {
                    NonFungibleLocalId::Integer(v) => {
                        ManifestNonFungibleLocalId::Integer(v.value())
                    }
                    NonFungibleLocalId::UUID(v) => ManifestNonFungibleLocalId::UUID(v.value()),
                    NonFungibleLocalId::String(v) => {
                        ManifestNonFungibleLocalId::String(v.value().to_owned())
                    }
                    NonFungibleLocalId::Bytes(v) => {
                        ManifestNonFungibleLocalId::Bytes(v.value().to_owned())
                    }
                }),
            },
        };
        Ok(value)
    }

    pub fn from_manifest_sbor_value(
        scrypto_value: &ManifestValue,
        network_id: u8,
    ) -> Result<Self, ManifestSborValueConversionError> {
        let value = match scrypto_value {
            ManifestValue::Bool { value } => Self::Bool { value: *value },

            ManifestValue::U8 { value } => Self::U8 { value: *value },
            ManifestValue::U16 { value } => Self::U16 { value: *value },
            ManifestValue::U32 { value } => Self::U32 { value: *value },
            ManifestValue::U64 { value } => Self::U64 { value: *value },
            ManifestValue::U128 { value } => Self::U128 { value: *value },

            ManifestValue::I8 { value } => Self::I8 { value: *value },
            ManifestValue::I16 { value } => Self::I16 { value: *value },
            ManifestValue::I32 { value } => Self::I32 { value: *value },
            ManifestValue::I64 { value } => Self::I64 { value: *value },
            ManifestValue::I128 { value } => Self::I128 { value: *value },

            ManifestValue::String { value } => Self::String {
                value: value.clone(),
            },

            ManifestValue::Enum {
                discriminator,
                fields,
            } => Self::Enum {
                variant: *discriminator,
                fields: if fields.is_empty() {
                    None
                } else {
                    Some(
                        fields
                            .clone()
                            .into_iter()
                            .map(|value| Self::from_manifest_sbor_value(&value, network_id))
                            .collect::<Result<Vec<_>, ManifestSborValueConversionError>>()?,
                    )
                },
            },
            ManifestValue::Map {
                key_value_kind,
                value_value_kind,
                entries,
            } => Self::Map {
                key_value_kind: (*key_value_kind).into(),
                value_value_kind: (*value_value_kind).into(),
                entries: {
                    let mut scrypto_entries = Vec::new();
                    for (key, value) in entries {
                        scrypto_entries.push((
                            Self::from_manifest_sbor_value(key, network_id)?,
                            Self::from_manifest_sbor_value(value, network_id)?,
                        ))
                    }
                    scrypto_entries
                },
            },
            ManifestValue::Array {
                element_value_kind,
                elements,
            } => Self::Array {
                element_kind: (*element_value_kind).into(),
                elements: elements
                    .clone()
                    .into_iter()
                    .map(|value| Self::from_manifest_sbor_value(&value, network_id))
                    .collect::<Result<Vec<_>, ManifestSborValueConversionError>>()?,
            },
            ManifestValue::Tuple { fields } => Self::Tuple {
                elements: fields
                    .clone()
                    .into_iter()
                    .map(|value| Self::from_manifest_sbor_value(&value, network_id))
                    .collect::<Result<Vec<_>, ManifestSborValueConversionError>>()?,
            },

            ManifestValue::Custom {
                value: ManifestCustomValue::Address(value),
            } => Self::Address {
                address: NetworkAwareNodeId(value.0 .0, network_id),
            },

            ManifestValue::Custom {
                value: ManifestCustomValue::Bucket(bucket),
            } => Self::Bucket {
                identifier: bucket.0,
            },
            ManifestValue::Custom {
                value: ManifestCustomValue::Proof(proof),
            } => Self::Proof {
                identifier: proof.0,
            },

            ManifestValue::Custom {
                value: ManifestCustomValue::Blob(blob),
            } => Self::Blob {
                hash: ManifestBlobRef(blob.0),
            },
            ManifestValue::Custom {
                value: ManifestCustomValue::Expression(expression),
            } => Self::Expression { value: *expression },

            ManifestValue::Custom {
                value: ManifestCustomValue::Decimal(value),
            } => Self::Decimal {
                value: Decimal::try_from(value.0.as_slice())?,
            },
            ManifestValue::Custom {
                value: ManifestCustomValue::PreciseDecimal(value),
            } => Self::PreciseDecimal {
                value: PreciseDecimal::try_from(value.0.as_slice())?,
            },

            ManifestValue::Custom {
                value: ManifestCustomValue::NonFungibleLocalId(value),
            } => Self::NonFungibleLocalId {
                value: match value {
                    ManifestNonFungibleLocalId::Integer(v) => {
                        NonFungibleLocalId::Integer(IntegerNonFungibleLocalId::new(*v))
                    }
                    ManifestNonFungibleLocalId::UUID(v) => {
                        NonFungibleLocalId::UUID(UUIDNonFungibleLocalId::new(*v)?)
                    }
                    ManifestNonFungibleLocalId::String(v) => {
                        NonFungibleLocalId::String(StringNonFungibleLocalId::new(v.to_owned())?)
                    }
                    ManifestNonFungibleLocalId::Bytes(v) => {
                        NonFungibleLocalId::Bytes(BytesNonFungibleLocalId::new(v.to_owned())?)
                    }
                },
            },
        };
        Ok(value)
    }
}
