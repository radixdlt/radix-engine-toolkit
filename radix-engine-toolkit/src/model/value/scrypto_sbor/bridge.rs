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

use super::{model::*, ScryptoSborValueConversionError};
use crate::model::address::NetworkAwareNodeId;

use scrypto::prelude::{
    NodeId, Own, ScryptoCustomValue, ScryptoCustomValueKind, ScryptoValue, ScryptoValueKind,
};

impl From<ScryptoValueKind> for ScryptoSborValueKind {
    fn from(value: ScryptoValueKind) -> Self {
        match value {
            ScryptoValueKind::Bool => ScryptoSborValueKind::Bool,

            ScryptoValueKind::U8 => ScryptoSborValueKind::U8,
            ScryptoValueKind::U16 => ScryptoSborValueKind::U16,
            ScryptoValueKind::U32 => ScryptoSborValueKind::U32,
            ScryptoValueKind::U64 => ScryptoSborValueKind::U64,
            ScryptoValueKind::U128 => ScryptoSborValueKind::U128,

            ScryptoValueKind::I8 => ScryptoSborValueKind::I8,
            ScryptoValueKind::I16 => ScryptoSborValueKind::I16,
            ScryptoValueKind::I32 => ScryptoSborValueKind::I32,
            ScryptoValueKind::I64 => ScryptoSborValueKind::I64,
            ScryptoValueKind::I128 => ScryptoSborValueKind::I128,

            ScryptoValueKind::String => ScryptoSborValueKind::String,

            ScryptoValueKind::Enum => ScryptoSborValueKind::Enum,
            ScryptoValueKind::Map => ScryptoSborValueKind::Map,
            ScryptoValueKind::Array => ScryptoSborValueKind::Array,
            ScryptoValueKind::Tuple => ScryptoSborValueKind::Tuple,

            ScryptoValueKind::Custom(ScryptoCustomValueKind::Decimal) => {
                ScryptoSborValueKind::Decimal
            }
            ScryptoValueKind::Custom(ScryptoCustomValueKind::PreciseDecimal) => {
                ScryptoSborValueKind::PreciseDecimal
            }
            ScryptoValueKind::Custom(ScryptoCustomValueKind::NonFungibleLocalId) => {
                ScryptoSborValueKind::NonFungibleLocalId
            }
            ScryptoValueKind::Custom(ScryptoCustomValueKind::Own) => ScryptoSborValueKind::Own,
            ScryptoValueKind::Custom(ScryptoCustomValueKind::Reference) => {
                ScryptoSborValueKind::Reference
            }
        }
    }
}

impl From<ScryptoSborValueKind> for ScryptoValueKind {
    fn from(value: ScryptoSborValueKind) -> Self {
        match value {
            ScryptoSborValueKind::Bool => ScryptoValueKind::Bool,

            ScryptoSborValueKind::U8 => ScryptoValueKind::U8,
            ScryptoSborValueKind::U16 => ScryptoValueKind::U16,
            ScryptoSborValueKind::U32 => ScryptoValueKind::U32,
            ScryptoSborValueKind::U64 => ScryptoValueKind::U64,
            ScryptoSborValueKind::U128 => ScryptoValueKind::U128,

            ScryptoSborValueKind::I8 => ScryptoValueKind::I8,
            ScryptoSborValueKind::I16 => ScryptoValueKind::I16,
            ScryptoSborValueKind::I32 => ScryptoValueKind::I32,
            ScryptoSborValueKind::I64 => ScryptoValueKind::I64,
            ScryptoSborValueKind::I128 => ScryptoValueKind::I128,

            ScryptoSborValueKind::String => ScryptoValueKind::String,

            ScryptoSborValueKind::Enum => ScryptoValueKind::Enum,
            ScryptoSborValueKind::Map => ScryptoValueKind::Map,
            ScryptoSborValueKind::Array | ScryptoSborValueKind::Bytes => ScryptoValueKind::Array,
            ScryptoSborValueKind::Tuple => ScryptoValueKind::Tuple,

            ScryptoSborValueKind::Decimal => {
                ScryptoValueKind::Custom(ScryptoCustomValueKind::Decimal)
            }
            ScryptoSborValueKind::PreciseDecimal => {
                ScryptoValueKind::Custom(ScryptoCustomValueKind::PreciseDecimal)
            }
            ScryptoSborValueKind::NonFungibleLocalId => {
                ScryptoValueKind::Custom(ScryptoCustomValueKind::NonFungibleLocalId)
            }
            ScryptoSborValueKind::Own => ScryptoValueKind::Custom(ScryptoCustomValueKind::Own),
            ScryptoSborValueKind::Reference => {
                ScryptoValueKind::Custom(ScryptoCustomValueKind::Reference)
            }
        }
    }
}

impl ScryptoSborValue {
    pub fn to_scrypto_sbor_value(&self) -> Result<ScryptoValue, ScryptoSborValueConversionError> {
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
            Self::Enum { variant_id, fields } => ScryptoValue::Enum {
                discriminator: *variant_id,
                fields: fields
                    .clone()
                    .into_iter()
                    .map(|value| value.to_scrypto_sbor_value())
                    .collect::<Result<Vec<_>, _>>()?,
            },
            Self::Map {
                key_kind: key_value_kind,
                value_kind: value_value_kind,
                entries,
            } => ScryptoValue::Map {
                key_value_kind: (*key_value_kind).into(),
                value_value_kind: (*value_value_kind).into(),
                entries: entries
                    .iter()
                    .map(|map_entry| map_entry.to_scrypto_value_tuple())
                    .collect::<Result<Vec<_>, _>>()?,
            },
            Self::Array {
                element_kind,
                elements,
            } => ScryptoValue::Array {
                element_value_kind: (*element_kind).into(),
                elements: elements
                    .clone()
                    .into_iter()
                    .map(|value| value.to_scrypto_sbor_value())
                    .collect::<Result<Vec<_>, _>>()?,
            },
            Self::Tuple { fields: elements } => ScryptoValue::Tuple {
                fields: elements
                    .clone()
                    .into_iter()
                    .map(|value| value.to_scrypto_sbor_value())
                    .collect::<Result<Vec<_>, _>>()?,
            },

            Self::Decimal { value } => ScryptoValue::Custom {
                value: ScryptoCustomValue::Decimal(*value),
            },
            Self::PreciseDecimal { value } => ScryptoValue::Custom {
                value: ScryptoCustomValue::PreciseDecimal(*value),
            },

            Self::NonFungibleLocalId { value } => ScryptoValue::Custom {
                value: ScryptoCustomValue::NonFungibleLocalId(value.clone()),
            },

            Self::Own { value } => ScryptoValue::Custom {
                value: ScryptoCustomValue::Own(Own(NodeId(value.0))),
            },
            Self::Reference { value } => ScryptoValue::Custom {
                value: ScryptoCustomValue::Reference(
                    radix_engine_common::data::scrypto::model::Reference(value.0.into()),
                ),
            },
            Self::Bytes { hex, .. } => ScryptoValue::Array {
                element_value_kind: ScryptoValueKind::U8,
                elements: hex
                    .iter()
                    .map(|value| ScryptoValue::U8 { value: *value })
                    .collect(),
            },
        };
        Ok(value)
    }

    pub fn from_scrypto_sbor_value(
        scrypto_value: &ScryptoValue,
        network_id: u8,
    ) -> Result<Self, ScryptoSborValueConversionError> {
        let value = match scrypto_value {
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
                variant_id: *discriminator,
                fields: fields
                    .clone()
                    .into_iter()
                    .map(|value| Self::from_scrypto_sbor_value(&value, network_id))
                    .collect::<Result<Vec<_>, _>>()?,
            },
            ScryptoValue::Map {
                key_value_kind,
                value_value_kind,
                entries,
            } => Self::Map {
                key_kind: (*key_value_kind).into(),
                value_kind: (*value_value_kind).into(),
                entries: entries
                    .clone()
                    .into_iter()
                    .map(|(key, value)| MapEntry::from_scrypto_value(network_id, key, value))
                    .collect::<Result<Vec<_>, _>>()?,
            },
            ScryptoValue::Array {
                element_value_kind: ScryptoValueKind::U8,
                elements,
            } => {
                let bytes = elements
                    .iter()
                    .map(|element| {
                        if let ScryptoValue::U8 { value } = element {
                            Ok(*value)
                        } else {
                            Err(ScryptoSborValueConversionError::InvalidType {
                                expected: ScryptoSborValueKind::U8,
                            })
                        }
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                Self::Bytes {
                    hex: bytes,
                    element_kind: ScryptoSborValueKind::U8,
                }
            }
            ScryptoValue::Array {
                element_value_kind,
                elements,
            } => Self::Array {
                element_kind: (*element_value_kind).into(),
                elements: elements
                    .clone()
                    .into_iter()
                    .map(|value| Self::from_scrypto_sbor_value(&value, network_id))
                    .collect::<Result<Vec<_>, _>>()?,
            },
            ScryptoValue::Tuple { fields } => Self::Tuple {
                fields: fields
                    .clone()
                    .into_iter()
                    .map(|value| Self::from_scrypto_sbor_value(&value, network_id))
                    .collect::<Result<Vec<_>, _>>()?,
            },

            ScryptoValue::Custom {
                value: ScryptoCustomValue::Decimal(value),
            } => Self::Decimal { value: *value },
            ScryptoValue::Custom {
                value: ScryptoCustomValue::PreciseDecimal(value),
            } => Self::PreciseDecimal { value: *value },

            ScryptoValue::Custom {
                value: ScryptoCustomValue::NonFungibleLocalId(value),
            } => Self::NonFungibleLocalId {
                value: value.clone(),
            },

            ScryptoValue::Custom {
                value: ScryptoCustomValue::Own(value),
            } => Self::Own {
                value: NetworkAwareNodeId(value.0 .0, network_id),
            },
            ScryptoValue::Custom {
                value: ScryptoCustomValue::Reference(value),
            } => Self::Reference {
                value: NetworkAwareNodeId(value.0 .0, network_id),
            },
        };
        Ok(value)
    }
}
