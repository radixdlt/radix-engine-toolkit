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
pub enum MetadataValue {
    StringValue {
        value: String,
    },
    BoolValue {
        value: bool,
    },
    U8Value {
        value: u8,
    },
    U32Value {
        value: u32,
    },
    U64Value {
        value: u64,
    },
    I32Value {
        value: i32,
    },
    I64Value {
        value: i64,
    },
    DecimalValue {
        value: Arc<Decimal>,
    },
    GlobalAddressValue {
        value: Arc<Address>,
    },
    PublicKeyValue {
        value: PublicKey,
    },
    NonFungibleGlobalIdValue {
        value: Arc<NonFungibleGlobalId>,
    },
    NonFungibleLocalIdValue {
        value: NonFungibleLocalId,
    },
    InstantValue {
        value: i64,
    },
    UrlValue {
        value: String,
    },
    OriginValue {
        value: String,
    },
    PublicKeyHashValue {
        value: PublicKeyHash,
    },

    StringArrayValue {
        value: Vec<String>,
    },
    BoolArrayValue {
        value: Vec<bool>,
    },
    U8ArrayValue {
        value: Vec<u8>,
    },
    U32ArrayValue {
        value: Vec<u32>,
    },
    U64ArrayValue {
        value: Vec<u64>,
    },
    I32ArrayValue {
        value: Vec<i32>,
    },
    I64ArrayValue {
        value: Vec<i64>,
    },
    DecimalArrayValue {
        value: Vec<Arc<Decimal>>,
    },
    GlobalAddressArrayValue {
        value: Vec<Arc<Address>>,
    },
    PublicKeyArrayValue {
        value: Vec<PublicKey>,
    },
    NonFungibleGlobalIdArrayValue {
        value: Vec<Arc<NonFungibleGlobalId>>,
    },
    NonFungibleLocalIdArrayValue {
        value: Vec<NonFungibleLocalId>,
    },
    InstantArrayValue {
        value: Vec<i64>,
    },
    UrlArrayValue {
        value: Vec<String>,
    },
    OriginArrayValue {
        value: Vec<String>,
    },
    PublicKeyHashArrayValue {
        value: Vec<PublicKeyHash>,
    },
}

impl MetadataValue {
    pub fn from_native(native: &NativeMetadataValue, network_id: u8) -> Self {
        match native.clone() {
            NativeMetadataValue::String(value) => Self::StringValue { value },
            NativeMetadataValue::Bool(value) => Self::BoolValue { value },

            NativeMetadataValue::U8(value) => Self::U8Value { value },
            NativeMetadataValue::U32(value) => Self::U32Value { value },
            NativeMetadataValue::U64(value) => Self::U64Value { value },
            NativeMetadataValue::I32(value) => Self::I32Value { value },
            NativeMetadataValue::I64(value) => Self::I64Value { value },

            NativeMetadataValue::Decimal(value) => Self::DecimalValue {
                value: Arc::new(Decimal(value)),
            },
            NativeMetadataValue::GlobalAddress(value) => Self::GlobalAddressValue {
                value: Arc::new(Address::from_typed_node_id(value, network_id)),
            },
            NativeMetadataValue::PublicKey(value) => Self::PublicKeyValue {
                value: value.into(),
            },
            NativeMetadataValue::PublicKeyHash(value) => Self::PublicKeyHashValue {
                value: value.into(),
            },
            NativeMetadataValue::NonFungibleGlobalId(value) => Self::NonFungibleGlobalIdValue {
                value: Arc::new(NonFungibleGlobalId(value, network_id)),
            },
            NativeMetadataValue::NonFungibleLocalId(value) => Self::NonFungibleLocalIdValue {
                value: value.into(),
            },
            NativeMetadataValue::Instant(value) => Self::InstantValue {
                value: value.seconds_since_unix_epoch,
            },
            NativeMetadataValue::Url(value) => Self::UrlValue { value: value.0 },
            NativeMetadataValue::Origin(value) => Self::OriginValue { value: value.0 },

            NativeMetadataValue::StringArray(value) => Self::StringArrayValue { value },
            NativeMetadataValue::BoolArray(value) => Self::BoolArrayValue { value },

            NativeMetadataValue::U8Array(value) => Self::U8ArrayValue { value },
            NativeMetadataValue::U32Array(value) => Self::U32ArrayValue { value },
            NativeMetadataValue::U64Array(value) => Self::U64ArrayValue { value },
            NativeMetadataValue::I32Array(value) => Self::I32ArrayValue { value },
            NativeMetadataValue::I64Array(value) => Self::I64ArrayValue { value },

            NativeMetadataValue::DecimalArray(value) => Self::DecimalArrayValue {
                value: value
                    .into_iter()
                    .map(|value| Arc::new(Decimal(value)))
                    .collect(),
            },
            NativeMetadataValue::GlobalAddressArray(value) => Self::GlobalAddressArrayValue {
                value: value
                    .into_iter()
                    .map(|value| Arc::new(Address::from_typed_node_id(value, network_id)))
                    .collect(),
            },
            NativeMetadataValue::PublicKeyArray(value) => Self::PublicKeyArrayValue {
                value: value.into_iter().map(Into::into).collect(),
            },
            NativeMetadataValue::PublicKeyHashArray(value) => Self::PublicKeyHashArrayValue {
                value: value.into_iter().map(Into::into).collect(),
            },
            NativeMetadataValue::NonFungibleGlobalIdArray(value) => {
                Self::NonFungibleGlobalIdArrayValue {
                    value: value
                        .into_iter()
                        .map(|value| Arc::new(NonFungibleGlobalId(value, network_id)))
                        .collect(),
                }
            }
            NativeMetadataValue::NonFungibleLocalIdArray(value) => {
                Self::NonFungibleLocalIdArrayValue {
                    value: value.into_iter().map(Into::into).collect(),
                }
            }
            NativeMetadataValue::InstantArray(value) => Self::InstantArrayValue {
                value: value
                    .into_iter()
                    .map(|value| value.seconds_since_unix_epoch)
                    .collect(),
            },
            NativeMetadataValue::UrlArray(value) => Self::UrlArrayValue {
                value: value.into_iter().map(|value| value.0).collect(),
            },
            NativeMetadataValue::OriginArray(value) => Self::OriginArrayValue {
                value: value.into_iter().map(|value| value.0).collect(),
            },
        }
    }

    pub fn to_native(&self) -> Result<NativeMetadataValue> {
        let value = match self {
            Self::StringValue { value } => NativeMetadataValue::String(value.to_owned()),
            Self::BoolValue { value } => NativeMetadataValue::Bool(*value),

            Self::U8Value { value } => NativeMetadataValue::U8(*value),
            Self::U32Value { value } => NativeMetadataValue::U32(*value),
            Self::U64Value { value } => NativeMetadataValue::U64(*value),
            Self::I32Value { value } => NativeMetadataValue::I32(*value),
            Self::I64Value { value } => NativeMetadataValue::I64(*value),

            Self::DecimalValue { value } => NativeMetadataValue::Decimal(value.0),
            Self::GlobalAddressValue { value } => {
                NativeMetadataValue::GlobalAddress(NativeGlobalAddress::try_from(value.as_bytes())?)
            }
            Self::PublicKeyValue { value } => {
                NativeMetadataValue::PublicKey(value.clone().try_into()?)
            }
            Self::PublicKeyHashValue { value } => {
                NativeMetadataValue::PublicKeyHash(value.clone().try_into()?)
            }
            Self::NonFungibleGlobalIdValue { value } => {
                NativeMetadataValue::NonFungibleGlobalId(value.0.clone())
            }
            Self::NonFungibleLocalIdValue { value } => {
                NativeMetadataValue::NonFungibleLocalId(value.clone().try_into()?)
            }
            Self::InstantValue { value } => {
                NativeMetadataValue::Instant(NativeInstant::new(*value))
            }
            Self::UrlValue { value } => NativeMetadataValue::Url(NativeUncheckedUrl::of(value)),
            Self::OriginValue { value } => {
                NativeMetadataValue::Origin(NativeUncheckedOrigin::of(value))
            }

            Self::StringArrayValue { value } => NativeMetadataValue::StringArray(value.clone()),
            Self::BoolArrayValue { value } => NativeMetadataValue::BoolArray(value.clone()),
            Self::U8ArrayValue { value } => NativeMetadataValue::U8Array(value.clone()),
            Self::U32ArrayValue { value } => NativeMetadataValue::U32Array(value.clone()),
            Self::U64ArrayValue { value } => NativeMetadataValue::U64Array(value.clone()),
            Self::I32ArrayValue { value } => NativeMetadataValue::I32Array(value.clone()),
            Self::I64ArrayValue { value } => NativeMetadataValue::I64Array(value.clone()),

            Self::DecimalArrayValue { value } => {
                NativeMetadataValue::DecimalArray(value.iter().map(|value| value.0).collect())
            }
            Self::PublicKeyArrayValue { value } => NativeMetadataValue::PublicKeyArray(
                value
                    .iter()
                    .map(|value| value.clone().try_into())
                    .collect::<Result<_>>()?,
            ),
            Self::PublicKeyHashArrayValue { value } => NativeMetadataValue::PublicKeyHashArray(
                value
                    .iter()
                    .map(|value| value.clone().try_into())
                    .collect::<Result<_>>()?,
            ),
            Self::NonFungibleGlobalIdArrayValue { value } => {
                NativeMetadataValue::NonFungibleGlobalIdArray(
                    value.iter().map(|value| value.0.clone()).collect(),
                )
            }
            Self::NonFungibleLocalIdArrayValue { value } => {
                NativeMetadataValue::NonFungibleLocalIdArray(
                    value
                        .iter()
                        .map(|value| value.clone().try_into())
                        .collect::<Result<_>>()?,
                )
            }
            Self::GlobalAddressArrayValue { value } => NativeMetadataValue::GlobalAddressArray(
                value
                    .iter()
                    .map(|value| {
                        NativeGlobalAddress::try_from(value.as_bytes()).map_err(Into::into)
                    })
                    .collect::<Result<_>>()?,
            ),
            Self::InstantArrayValue { value } => NativeMetadataValue::InstantArray(
                value
                    .iter()
                    .map(|value| NativeInstant::new(*value))
                    .collect(),
            ),
            Self::UrlArrayValue { value } => {
                NativeMetadataValue::UrlArray(value.iter().map(NativeUncheckedUrl::of).collect())
            }
            Self::OriginArrayValue { value } => NativeMetadataValue::OriginArray(
                value.iter().map(NativeUncheckedOrigin::of).collect(),
            ),
        };
        Ok(value)
    }
}

// ==================
// Exposed "Methods"
// ==================

#[uniffi::export]
pub fn metadata_sbor_decode(bytes: Vec<u8>, network_id: u8) -> Result<MetadataValue> {
    let native = match bytes.first().copied() {
        Some(NATIVE_SCRYPTO_SBOR_V1_PAYLOAD_PREFIX) => {
            native_scrypto_decode::<NativeMetadataValue>(&bytes).map_err(Into::into)
        }
        Some(NATIVE_MANIFEST_SBOR_V1_PAYLOAD_PREFIX) => {
            native_manifest_decode::<NativeMetadataValue>(&bytes).map_err(Into::into)
        }
        v => Err(RadixEngineToolkitError::DecodeError {
            error: format!("Invalid index byte: {v:?}"),
        }),
    }?;
    Ok(MetadataValue::from_native(&native, network_id))
}

#[uniffi::export]
pub fn metadata_sbor_encode(value: MetadataValue) -> Result<Vec<u8>> {
    let native = value.to_native()?;
    Ok(native_scrypto_encode(&native).expect("Can't fail"))
}
