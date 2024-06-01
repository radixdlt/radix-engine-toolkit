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

use radix_common::prelude::*;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::common::address::SerializableNodeId;
use crate::common::map_entry::MapEntry;

#[serde_as]
#[derive(
    Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[serde(tag = "kind")]
pub enum ProgrammaticScryptoValue {
    Bool {
        value: bool,
    },
    I8 {
        #[serde_as(as = "serde_with::DisplayFromStr")]
        value: i8,
    },
    I16 {
        #[serde_as(as = "serde_with::DisplayFromStr")]
        value: i16,
    },
    I32 {
        #[serde_as(as = "serde_with::DisplayFromStr")]
        value: i32,
    },
    I64 {
        #[serde_as(as = "serde_with::DisplayFromStr")]
        value: i64,
    },
    I128 {
        #[serde_as(as = "serde_with::DisplayFromStr")]
        value: i128,
    },
    U8 {
        #[serde_as(as = "serde_with::DisplayFromStr")]
        value: u8,
    },
    U16 {
        #[serde_as(as = "serde_with::DisplayFromStr")]
        value: u16,
    },
    U32 {
        #[serde_as(as = "serde_with::DisplayFromStr")]
        value: u32,
    },
    U64 {
        #[serde_as(as = "serde_with::DisplayFromStr")]
        value: u64,
    },
    U128 {
        #[serde_as(as = "serde_with::DisplayFromStr")]
        value: u128,
    },
    String {
        value: String,
    },
    Enum {
        #[serde(rename = "variant_id")]
        discriminator: u8,
        fields: Vec<ProgrammaticScryptoValue>,
    },
    Array {
        #[serde(rename = "element_kind")]
        element_value_kind: ProgrammaticScryptoValueKind,
        elements: Vec<ProgrammaticScryptoValue>,
    },
    Tuple {
        fields: Vec<ProgrammaticScryptoValue>,
    },
    Map {
        #[serde(rename = "key_kind")]
        key_value_kind: ProgrammaticScryptoValueKind,
        #[serde(rename = "value_kind")]
        value_value_kind: ProgrammaticScryptoValueKind,
        #[serde_as(
            as = "Vec<serde_with::FromInto<MapEntry<ProgrammaticScryptoValue>>>"
        )]
        entries: Vec<(ProgrammaticScryptoValue, ProgrammaticScryptoValue)>,
    },
    Reference {
        #[serde_as(as = "serde_with::DisplayFromStr")]
        value: SerializableNodeId,
    },
    Own {
        #[serde_as(as = "serde_with::DisplayFromStr")]
        value: SerializableNodeId,
    },
    Decimal {
        #[serde_as(as = "serde_with::DisplayFromStr")]
        value: Decimal,
    },
    PreciseDecimal {
        #[serde_as(as = "serde_with::DisplayFromStr")]
        value: PreciseDecimal,
    },
    NonFungibleLocalId {
        #[serde_as(as = "serde_with::DisplayFromStr")]
        value: NonFungibleLocalId,
    },
    Bytes {
        #[serde(rename = "element_kind")]
        element_value_kind: ProgrammaticScryptoValueKind,

        #[serde_as(as = "serde_with::hex::Hex")]
        #[serde(rename = "hex")]
        value: Vec<u8>,
    },
}

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub enum ProgrammaticScryptoValueKind {
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
    Reference,
    Own,
    Decimal,
    PreciseDecimal,
    NonFungibleLocalId,
}

impl From<ProgrammaticScryptoValueKind> for ScryptoValueKind {
    fn from(value: ProgrammaticScryptoValueKind) -> Self {
        match value {
            ProgrammaticScryptoValueKind::Bool => Self::Bool,
            ProgrammaticScryptoValueKind::I8 => Self::I8,
            ProgrammaticScryptoValueKind::I16 => Self::I16,
            ProgrammaticScryptoValueKind::I32 => Self::I32,
            ProgrammaticScryptoValueKind::I64 => Self::I64,
            ProgrammaticScryptoValueKind::I128 => Self::I128,
            ProgrammaticScryptoValueKind::U8 => Self::U8,
            ProgrammaticScryptoValueKind::U16 => Self::U16,
            ProgrammaticScryptoValueKind::U32 => Self::U32,
            ProgrammaticScryptoValueKind::U64 => Self::U64,
            ProgrammaticScryptoValueKind::U128 => Self::U128,
            ProgrammaticScryptoValueKind::String => Self::String,
            ProgrammaticScryptoValueKind::Enum => Self::Enum,
            ProgrammaticScryptoValueKind::Array => Self::Array,
            ProgrammaticScryptoValueKind::Tuple => Self::Tuple,
            ProgrammaticScryptoValueKind::Map => Self::Map,
            ProgrammaticScryptoValueKind::Reference => {
                Self::Custom(ScryptoCustomValueKind::Reference)
            }
            ProgrammaticScryptoValueKind::Own => {
                Self::Custom(ScryptoCustomValueKind::Own)
            }
            ProgrammaticScryptoValueKind::Decimal => {
                Self::Custom(ScryptoCustomValueKind::Decimal)
            }
            ProgrammaticScryptoValueKind::PreciseDecimal => {
                Self::Custom(ScryptoCustomValueKind::PreciseDecimal)
            }
            ProgrammaticScryptoValueKind::NonFungibleLocalId => {
                Self::Custom(ScryptoCustomValueKind::NonFungibleLocalId)
            }
        }
    }
}

impl From<ScryptoValueKind> for ProgrammaticScryptoValueKind {
    fn from(value: ScryptoValueKind) -> Self {
        match value {
            ScryptoValueKind::Bool => Self::Bool,
            ScryptoValueKind::I8 => Self::I8,
            ScryptoValueKind::I16 => Self::I16,
            ScryptoValueKind::I32 => Self::I32,
            ScryptoValueKind::I64 => Self::I64,
            ScryptoValueKind::I128 => Self::I128,
            ScryptoValueKind::U8 => Self::U8,
            ScryptoValueKind::U16 => Self::U16,
            ScryptoValueKind::U32 => Self::U32,
            ScryptoValueKind::U64 => Self::U64,
            ScryptoValueKind::U128 => Self::U128,
            ScryptoValueKind::String => Self::String,
            ScryptoValueKind::Enum => Self::Enum,
            ScryptoValueKind::Array => Self::Array,
            ScryptoValueKind::Tuple => Self::Tuple,
            ScryptoValueKind::Map => Self::Map,
            ScryptoValueKind::Custom(ScryptoCustomValueKind::Reference) => {
                Self::Reference
            }
            ScryptoValueKind::Custom(ScryptoCustomValueKind::Own) => Self::Own,
            ScryptoValueKind::Custom(ScryptoCustomValueKind::Decimal) => {
                Self::Decimal
            }
            ScryptoValueKind::Custom(
                ScryptoCustomValueKind::PreciseDecimal,
            ) => Self::PreciseDecimal,
            ScryptoValueKind::Custom(
                ScryptoCustomValueKind::NonFungibleLocalId,
            ) => Self::NonFungibleLocalId,
        }
    }
}

impl ProgrammaticScryptoValue {
    pub fn to_scrypto_value(&self) -> ScryptoValue {
        match self {
            Self::Bool { value } => ScryptoValue::Bool { value: *value },
            Self::I8 { value } => ScryptoValue::I8 { value: *value },
            Self::I16 { value } => ScryptoValue::I16 { value: *value },
            Self::I32 { value } => ScryptoValue::I32 { value: *value },
            Self::I64 { value } => ScryptoValue::I64 { value: *value },
            Self::I128 { value } => ScryptoValue::I128 { value: *value },
            Self::U8 { value } => ScryptoValue::U8 { value: *value },
            Self::U16 { value } => ScryptoValue::U16 { value: *value },
            Self::U32 { value } => ScryptoValue::U32 { value: *value },
            Self::U64 { value } => ScryptoValue::U64 { value: *value },
            Self::U128 { value } => ScryptoValue::U128 { value: *value },
            Self::String { value } => ScryptoValue::String {
                value: value.clone(),
            },
            Self::Enum {
                discriminator,
                fields,
            } => ScryptoValue::Enum {
                discriminator: *discriminator,
                fields: fields.iter().map(Self::to_scrypto_value).collect(),
            },
            Self::Array {
                element_value_kind,
                elements,
            } => ScryptoValue::Array {
                element_value_kind: (*element_value_kind).into(),
                elements: elements.iter().map(Self::to_scrypto_value).collect(),
            },
            Self::Tuple { fields } => ScryptoValue::Tuple {
                fields: fields.iter().map(Self::to_scrypto_value).collect(),
            },
            Self::Map {
                key_value_kind,
                value_value_kind,
                entries,
            } => ScryptoValue::Map {
                key_value_kind: (*key_value_kind).into(),
                value_value_kind: (*value_value_kind).into(),
                entries: entries
                    .iter()
                    .map(|(key, value)| {
                        (
                            Self::to_scrypto_value(key),
                            Self::to_scrypto_value(value),
                        )
                    })
                    .collect(),
            },
            Self::Reference { value } => ScryptoValue::Custom {
                value: ScryptoCustomValue::Reference(Reference(value.0)),
            },
            Self::Own { value } => ScryptoValue::Custom {
                value: ScryptoCustomValue::Own(Own(value.0)),
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
            Self::Bytes {
                element_value_kind,
                value,
            } => ScryptoValue::Array {
                element_value_kind: (*element_value_kind).into(),
                elements: value
                    .iter()
                    .map(|value| ScryptoValue::U8 { value: *value })
                    .collect(),
            },
        }
    }

    pub fn from_scrypto_value(value: &ScryptoValue, network_id: u8) -> Self {
        match value {
            SborValue::Bool { value } => Self::Bool { value: *value },
            SborValue::I8 { value } => Self::I8 { value: *value },
            SborValue::I16 { value } => Self::I16 { value: *value },
            SborValue::I32 { value } => Self::I32 { value: *value },
            SborValue::I64 { value } => Self::I64 { value: *value },
            SborValue::I128 { value } => Self::I128 { value: *value },
            SborValue::U8 { value } => Self::U8 { value: *value },
            SborValue::U16 { value } => Self::U16 { value: *value },
            SborValue::U32 { value } => Self::U32 { value: *value },
            SborValue::U64 { value } => Self::U64 { value: *value },
            SborValue::U128 { value } => Self::U128 { value: *value },
            SborValue::String { value } => Self::String {
                value: value.to_owned(),
            },
            SborValue::Enum {
                discriminator,
                fields,
            } => Self::Enum {
                discriminator: *discriminator,
                fields: fields
                    .iter()
                    .map(|value| Self::from_scrypto_value(value, network_id))
                    .collect(),
            },
            SborValue::Array {
                element_value_kind,
                elements,
            } if elements
                .iter()
                .all(|element| matches!(element, ScryptoValue::U8 { .. })) =>
            {
                Self::Bytes {
                    element_value_kind: (*element_value_kind).into(),
                    value: elements
                        .iter()
                        .map_while(|value| match value {
                            ScryptoValue::U8 { value } => Some(*value),
                            _ => None,
                        })
                        .collect::<Vec<u8>>(),
                }
            }
            SborValue::Array {
                element_value_kind,
                elements,
            } => Self::Array {
                element_value_kind: (*element_value_kind).into(),
                elements: elements
                    .iter()
                    .map(|value| Self::from_scrypto_value(value, network_id))
                    .collect(),
            },
            SborValue::Tuple { fields } => Self::Tuple {
                fields: fields
                    .iter()
                    .map(|value| Self::from_scrypto_value(value, network_id))
                    .collect(),
            },
            SborValue::Map {
                key_value_kind,
                value_value_kind,
                entries,
            } => Self::Map {
                key_value_kind: (*key_value_kind).into(),
                value_value_kind: (*value_value_kind).into(),
                entries: entries
                    .iter()
                    .map(|(key, value)| {
                        (
                            Self::from_scrypto_value(key, network_id),
                            Self::from_scrypto_value(value, network_id),
                        )
                    })
                    .collect(),
            },
            SborValue::Custom {
                value: ScryptoCustomValue::Reference(value),
            } => Self::Reference {
                value: SerializableNodeId(value.0, network_id),
            },
            SborValue::Custom {
                value: ScryptoCustomValue::Own(value),
            } => Self::Own {
                value: SerializableNodeId(value.0, network_id),
            },
            SborValue::Custom {
                value: ScryptoCustomValue::Decimal(value),
            } => Self::Decimal { value: *value },
            SborValue::Custom {
                value: ScryptoCustomValue::PreciseDecimal(value),
            } => Self::PreciseDecimal { value: *value },
            SborValue::Custom {
                value: ScryptoCustomValue::NonFungibleLocalId(value),
            } => Self::NonFungibleLocalId {
                value: value.clone(),
            },
        }
    }
}
