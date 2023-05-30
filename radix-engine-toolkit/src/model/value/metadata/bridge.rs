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

use crate::model::address::{Bech32Coder, NetworkAwareNodeId};

use super::{MetadataValue, MetadataValueConversionError};
use scrypto::api::node_modules::metadata::{MetadataValue as NativeMetadataValue, Origin, Url};
use scrypto::prelude::*;

impl MetadataValue {
    pub fn to_metadata_value(
        &self,
        bech32_coder: &Bech32Coder,
    ) -> Result<NativeMetadataValue, MetadataValueConversionError> {
        let value = match self {
            Self::String { value } => NativeMetadataValue::String(value.to_owned()),
            Self::Bool { value } => NativeMetadataValue::Bool(*value),
            Self::U8 { value } => NativeMetadataValue::U8(*value),
            Self::U32 { value } => NativeMetadataValue::U32(*value),
            Self::U64 { value } => NativeMetadataValue::U64(*value),
            Self::I32 { value } => NativeMetadataValue::I32(*value),
            Self::I64 { value } => NativeMetadataValue::I64(*value),
            Self::Decimal { value } => NativeMetadataValue::Decimal(value.to_owned()),
            Self::GlobalAddress { value } => {
                NativeMetadataValue::GlobalAddress(unsafe { GlobalAddress::new_unchecked(value.0) })
            }
            Self::PublicKey { value } => NativeMetadataValue::PublicKey(value.to_owned()),
            Self::NonFungibleGlobalId { value } => {
                NonFungibleGlobalId::try_from_canonical_string(bech32_coder.decoder(), value)
                    .map(NativeMetadataValue::NonFungibleGlobalId)?
            }
            Self::NonFungibleLocalId { value } => {
                NonFungibleLocalId::from_str(value).map(NativeMetadataValue::NonFungibleLocalId)?
            }
            Self::Instant { value } => NativeMetadataValue::Instant(Instant {
                seconds_since_unix_epoch: *value,
            }),
            Self::Url { value } => NativeMetadataValue::Url(Url(value.to_owned())),
            Self::Origin { value } => NativeMetadataValue::Origin(Origin(value.to_owned())),
            Self::PublicKeyHash { value } => NativeMetadataValue::PublicKeyHash(value.to_owned()),

            Self::StringArray { value } => NativeMetadataValue::StringArray(value.clone()),
            Self::BoolArray { value } => NativeMetadataValue::BoolArray(value.clone()),
            Self::U8Array { value } => NativeMetadataValue::U8Array(value.clone()),
            Self::U32Array { value } => NativeMetadataValue::U32Array(value.clone()),
            Self::U64Array { value } => NativeMetadataValue::U64Array(value.clone()),
            Self::I32Array { value } => NativeMetadataValue::I32Array(value.clone()),
            Self::I64Array { value } => NativeMetadataValue::I64Array(value.clone()),
            Self::DecimalArray { value } => NativeMetadataValue::DecimalArray(value.clone()),
            Self::GlobalAddressArray { value } => NativeMetadataValue::GlobalAddressArray(
                value
                    .iter()
                    .map(|address| unsafe { GlobalAddress::new_unchecked(address.0) })
                    .collect(),
            ),
            Self::PublicKeyArray { value } => NativeMetadataValue::PublicKeyArray(value.clone()),
            Self::NonFungibleGlobalIdArray { value } => {
                NativeMetadataValue::NonFungibleGlobalIdArray(
                    value
                        .iter()
                        .map(|value| {
                            NonFungibleGlobalId::try_from_canonical_string(
                                bech32_coder.decoder(),
                                value,
                            )
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                )
            }
            Self::NonFungibleLocalIdArray { value } => {
                NativeMetadataValue::NonFungibleLocalIdArray(
                    value
                        .iter()
                        .map(|value| NonFungibleLocalId::from_str(value))
                        .collect::<Result<Vec<_>, _>>()?,
                )
            }
            Self::InstantArray { value } => NativeMetadataValue::InstantArray(
                value
                    .iter()
                    .map(|value| Instant {
                        seconds_since_unix_epoch: *value,
                    })
                    .collect(),
            ),
            Self::UrlArray { value } => NativeMetadataValue::UrlArray(
                value.iter().map(|value| Url(value.to_owned())).collect(),
            ),
            Self::OriginArray { value } => NativeMetadataValue::OriginArray(
                value.iter().map(|value| Origin(value.to_owned())).collect(),
            ),
            Self::PublicKeyHashArray { value } => {
                NativeMetadataValue::PublicKeyHashArray(value.clone())
            }
        };
        Ok(value)
    }

    pub fn from_metadata_value(value: &NativeMetadataValue, bech32_coder: &Bech32Coder) -> Self {
        match value {
            NativeMetadataValue::String(value) => Self::String {
                value: value.to_owned(),
            },
            NativeMetadataValue::Bool(value) => Self::Bool { value: *value },
            NativeMetadataValue::U8(value) => Self::U8 { value: *value },
            NativeMetadataValue::U32(value) => Self::U32 { value: *value },
            NativeMetadataValue::U64(value) => Self::U64 { value: *value },
            NativeMetadataValue::I32(value) => Self::I32 { value: *value },
            NativeMetadataValue::I64(value) => Self::I64 { value: *value },
            NativeMetadataValue::Decimal(value) => Self::Decimal {
                value: value.to_owned(),
            },
            NativeMetadataValue::GlobalAddress(value) => Self::GlobalAddress {
                value: NetworkAwareNodeId(value.as_node_id().0, bech32_coder.network_id()),
            },
            NativeMetadataValue::PublicKey(value) => Self::PublicKey {
                value: value.to_owned(),
            },
            NativeMetadataValue::NonFungibleGlobalId(value) => Self::NonFungibleGlobalId {
                value: value.to_canonical_string(bech32_coder.encoder()),
            },
            NativeMetadataValue::NonFungibleLocalId(value) => Self::NonFungibleLocalId {
                value: value.to_string(),
            },
            NativeMetadataValue::Instant(value) => Self::Instant {
                value: value.seconds_since_unix_epoch,
            },
            NativeMetadataValue::Url(value) => Self::Url {
                value: value.0.to_owned(),
            },
            NativeMetadataValue::Origin(value) => Self::Origin {
                value: value.0.to_owned(),
            },
            NativeMetadataValue::PublicKeyHash(value) => Self::PublicKeyHash {
                value: value.to_owned(),
            },

            NativeMetadataValue::StringArray(value) => Self::StringArray {
                value: value.to_owned(),
            },
            NativeMetadataValue::BoolArray(value) => Self::BoolArray { value: *value },
            NativeMetadataValue::U8Array(value) => Self::U8Array { value: *value },
            NativeMetadataValue::U32Array(value) => Self::U32Array { value: *value },
            NativeMetadataValue::U64Array(value) => Self::U64Array { value: *value },
            NativeMetadataValue::I32Array(value) => Self::I32Array { value: *value },
            NativeMetadataValue::I64Array(value) => Self::I64Array { value: *value },
            NativeMetadataValue::DecimalArray(value) => Self::DecimalArray {
                value: value.to_owned(),
            },
            NativeMetadataValue::GlobalAddressArray(value) => Self::GlobalAddressArray {
                value: value
                    .iter()
                    .map(|value| {
                        NetworkAwareNodeId(value.as_node_id().0, bech32_coder.network_id())
                    })
                    .collect(),
            },
            NativeMetadataValue::PublicKeyArray(value) => Self::PublicKeyArray {
                value: value.to_owned(),
            },
            NativeMetadataValue::NonFungibleGlobalIdArray(value) => {
                Self::NonFungibleGlobalIdArray {
                    value: value
                        .iter()
                        .map(|value| value.to_canonical_string(bech32_coder.encoder()))
                        .collect(),
                }
            }
            NativeMetadataValue::NonFungibleLocalIdArray(value) => Self::NonFungibleLocalIdArray {
                value: value.iter().map(ToString::to_string).collect(),
            },
            NativeMetadataValue::InstantArray(value) => Self::InstantArray {
                value: value
                    .iter()
                    .map(|value| value.seconds_since_unix_epoch)
                    .collect(),
            },
            NativeMetadataValue::UrlArray(value) => Self::UrlArray {
                value: value.iter().map(|value| value.0.to_owned()).collect(),
            },
            NativeMetadataValue::OriginArray(value) => Self::OriginArray {
                value: value.iter().map(|value| value.0.to_owned()).collect(),
            },
            NativeMetadataValue::PublicKeyHashArray(value) => Self::PublicKeyHashArray {
                value: value.to_owned(),
            },
        }
    }
}
