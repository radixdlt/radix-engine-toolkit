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

impl FromNativeWithNetworkContext for MetadataValue {
    type Native = engine::MetadataValue;

    fn from_native(native: Self::Native, network_id: u8) -> Self {
        Self::from_native(&native, network_id)
    }
}

impl MetadataValue {
    pub fn from_native(native: &engine::MetadataValue, network_id: u8) -> Self {
        match native.clone() {
            engine::MetadataValue::String(value) => Self::StringValue { value },
            engine::MetadataValue::Bool(value) => Self::BoolValue { value },

            engine::MetadataValue::U8(value) => Self::U8Value { value },
            engine::MetadataValue::U32(value) => Self::U32Value { value },
            engine::MetadataValue::U64(value) => Self::U64Value { value },
            engine::MetadataValue::I32(value) => Self::I32Value { value },
            engine::MetadataValue::I64(value) => Self::I64Value { value },

            engine::MetadataValue::Decimal(value) => Self::DecimalValue {
                value: Arc::new(Decimal(value)),
            },
            engine::MetadataValue::GlobalAddress(value) => {
                Self::GlobalAddressValue {
                    value: Arc::new(Address::from_node_id(value, network_id)),
                }
            }
            engine::MetadataValue::PublicKey(value) => Self::PublicKeyValue {
                value: value.into(),
            },
            engine::MetadataValue::PublicKeyHash(value) => {
                Self::PublicKeyHashValue {
                    value: value.into(),
                }
            }
            engine::MetadataValue::NonFungibleGlobalId(value) => {
                Self::NonFungibleGlobalIdValue {
                    value: Arc::new(NonFungibleGlobalId(value, network_id)),
                }
            }
            engine::MetadataValue::NonFungibleLocalId(value) => {
                Self::NonFungibleLocalIdValue {
                    value: value.into(),
                }
            }
            engine::MetadataValue::Instant(value) => Self::InstantValue {
                value: value.seconds_since_unix_epoch,
            },
            engine::MetadataValue::Url(value) => {
                Self::UrlValue { value: value.0 }
            }
            engine::MetadataValue::Origin(value) => {
                Self::OriginValue { value: value.0 }
            }

            engine::MetadataValue::StringArray(value) => {
                Self::StringArrayValue { value }
            }
            engine::MetadataValue::BoolArray(value) => {
                Self::BoolArrayValue { value }
            }

            engine::MetadataValue::U8Array(value) => {
                Self::U8ArrayValue { value }
            }
            engine::MetadataValue::U32Array(value) => {
                Self::U32ArrayValue { value }
            }
            engine::MetadataValue::U64Array(value) => {
                Self::U64ArrayValue { value }
            }
            engine::MetadataValue::I32Array(value) => {
                Self::I32ArrayValue { value }
            }
            engine::MetadataValue::I64Array(value) => {
                Self::I64ArrayValue { value }
            }

            engine::MetadataValue::DecimalArray(value) => {
                Self::DecimalArrayValue {
                    value: value
                        .into_iter()
                        .map(|value| Arc::new(Decimal(value)))
                        .collect(),
                }
            }
            engine::MetadataValue::GlobalAddressArray(value) => {
                Self::GlobalAddressArrayValue {
                    value: value
                        .into_iter()
                        .map(|value| {
                            Arc::new(Address::from_node_id(value, network_id))
                        })
                        .collect(),
                }
            }
            engine::MetadataValue::PublicKeyArray(value) => {
                Self::PublicKeyArrayValue {
                    value: value.into_iter().map(Into::into).collect(),
                }
            }
            engine::MetadataValue::PublicKeyHashArray(value) => {
                Self::PublicKeyHashArrayValue {
                    value: value.into_iter().map(Into::into).collect(),
                }
            }
            engine::MetadataValue::NonFungibleGlobalIdArray(value) => {
                Self::NonFungibleGlobalIdArrayValue {
                    value: value
                        .into_iter()
                        .map(|value| {
                            Arc::new(NonFungibleGlobalId(value, network_id))
                        })
                        .collect(),
                }
            }
            engine::MetadataValue::NonFungibleLocalIdArray(value) => {
                Self::NonFungibleLocalIdArrayValue {
                    value: value.into_iter().map(Into::into).collect(),
                }
            }
            engine::MetadataValue::InstantArray(value) => {
                Self::InstantArrayValue {
                    value: value
                        .into_iter()
                        .map(|value| value.seconds_since_unix_epoch)
                        .collect(),
                }
            }
            engine::MetadataValue::UrlArray(value) => Self::UrlArrayValue {
                value: value.into_iter().map(|value| value.0).collect(),
            },
            engine::MetadataValue::OriginArray(value) => {
                Self::OriginArrayValue {
                    value: value.into_iter().map(|value| value.0).collect(),
                }
            }
        }
    }

    pub fn to_native(&self) -> Result<engine::MetadataValue> {
        let value = match self {
            Self::StringValue { value } => {
                engine::MetadataValue::String(value.to_owned())
            }
            Self::BoolValue { value } => engine::MetadataValue::Bool(*value),

            Self::U8Value { value } => engine::MetadataValue::U8(*value),
            Self::U32Value { value } => engine::MetadataValue::U32(*value),
            Self::U64Value { value } => engine::MetadataValue::U64(*value),
            Self::I32Value { value } => engine::MetadataValue::I32(*value),
            Self::I64Value { value } => engine::MetadataValue::I64(*value),

            Self::DecimalValue { value } => {
                engine::MetadataValue::Decimal(value.0)
            }
            Self::GlobalAddressValue { value } => {
                engine::MetadataValue::GlobalAddress(
                    engine::GlobalAddress::try_from(value.as_bytes())?,
                )
            }
            Self::PublicKeyValue { value } => {
                engine::MetadataValue::PublicKey(value.clone().try_into()?)
            }
            Self::PublicKeyHashValue { value } => {
                engine::MetadataValue::PublicKeyHash(value.clone().try_into()?)
            }
            Self::NonFungibleGlobalIdValue { value } => {
                engine::MetadataValue::NonFungibleGlobalId(value.0.clone())
            }
            Self::NonFungibleLocalIdValue { value } => {
                engine::MetadataValue::NonFungibleLocalId(
                    value.clone().try_into()?,
                )
            }
            Self::InstantValue { value } => {
                engine::MetadataValue::Instant(engine::Instant::new(*value))
            }
            Self::UrlValue { value } => {
                engine::MetadataValue::Url(engine::UncheckedUrl::of(value))
            }
            Self::OriginValue { value } => engine::MetadataValue::Origin(
                engine::UncheckedOrigin::of(value),
            ),

            Self::StringArrayValue { value } => {
                engine::MetadataValue::StringArray(value.clone())
            }
            Self::BoolArrayValue { value } => {
                engine::MetadataValue::BoolArray(value.clone())
            }
            Self::U8ArrayValue { value } => {
                engine::MetadataValue::U8Array(value.clone())
            }
            Self::U32ArrayValue { value } => {
                engine::MetadataValue::U32Array(value.clone())
            }
            Self::U64ArrayValue { value } => {
                engine::MetadataValue::U64Array(value.clone())
            }
            Self::I32ArrayValue { value } => {
                engine::MetadataValue::I32Array(value.clone())
            }
            Self::I64ArrayValue { value } => {
                engine::MetadataValue::I64Array(value.clone())
            }

            Self::DecimalArrayValue { value } => {
                engine::MetadataValue::DecimalArray(
                    value.iter().map(|value| value.0).collect(),
                )
            }
            Self::PublicKeyArrayValue { value } => {
                engine::MetadataValue::PublicKeyArray(
                    value
                        .iter()
                        .map(|value| value.clone().try_into())
                        .collect::<Result<_>>()?,
                )
            }
            Self::PublicKeyHashArrayValue { value } => {
                engine::MetadataValue::PublicKeyHashArray(
                    value
                        .iter()
                        .map(|value| value.clone().try_into())
                        .collect::<Result<_>>()?,
                )
            }
            Self::NonFungibleGlobalIdArrayValue { value } => {
                engine::MetadataValue::NonFungibleGlobalIdArray(
                    value.iter().map(|value| value.0.clone()).collect(),
                )
            }
            Self::NonFungibleLocalIdArrayValue { value } => {
                engine::MetadataValue::NonFungibleLocalIdArray(
                    value
                        .iter()
                        .map(|value| value.clone().try_into())
                        .collect::<Result<_>>()?,
                )
            }
            Self::GlobalAddressArrayValue { value } => {
                engine::MetadataValue::GlobalAddressArray(
                    value
                        .iter()
                        .map(|value| {
                            engine::GlobalAddress::try_from(value.as_bytes())
                                .map_err(Into::into)
                        })
                        .collect::<Result<_>>()?,
                )
            }
            Self::InstantArrayValue { value } => {
                engine::MetadataValue::InstantArray(
                    value
                        .iter()
                        .map(|value| engine::Instant::new(*value))
                        .collect(),
                )
            }
            Self::UrlArrayValue { value } => engine::MetadataValue::UrlArray(
                value.iter().map(engine::UncheckedUrl::of).collect(),
            ),
            Self::OriginArrayValue { value } => {
                engine::MetadataValue::OriginArray(
                    value.iter().map(engine::UncheckedOrigin::of).collect(),
                )
            }
        };
        Ok(value)
    }
}

// ==================
// Exposed "Methods"
// ==================

#[uniffi::export]
pub fn metadata_sbor_decode(
    bytes: Vec<u8>,
    network_id: u8,
) -> Result<MetadataValue> {
    let native = match bytes.first().copied() {
        Some(engine::SCRYPTO_SBOR_V1_PAYLOAD_PREFIX) => {
            engine::scrypto_decode::<engine::MetadataValue>(&bytes)
                .map_err(Into::into)
        }
        Some(engine::MANIFEST_SBOR_V1_PAYLOAD_PREFIX) => {
            engine::manifest_decode::<engine::MetadataValue>(&bytes)
                .map_err(Into::into)
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
    Ok(engine::scrypto_encode(&native).expect("Can't fail"))
}
