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

use super::{MetadataEntry, MetadataValue, MetadataValueConversionError};
use scrypto::api::node_modules::metadata::{
    MetadataEntry as NativeMetadataEntry, MetadataValue as NativeMetadataValue, Url,
};
use scrypto::prelude::*;

impl MetadataEntry {
    pub fn to_metadata_entry(
        &self,
        bech32_coder: &Bech32Coder,
    ) -> Result<NativeMetadataEntry, MetadataValueConversionError> {
        match self {
            Self::List(entries) => entries
                .iter()
                .map(|entry| entry.to_metadata_value(bech32_coder))
                .collect::<Result<_, _>>()
                .map(NativeMetadataEntry::List),
            Self::Value(entry) => entry
                .to_metadata_value(bech32_coder)
                .map(NativeMetadataEntry::Value),
        }
    }

    pub fn from_metadata_entry(value: &NativeMetadataEntry, bech32_coder: &Bech32Coder) -> Self {
        match value {
            NativeMetadataEntry::List(entries) => Self::List(
                entries
                    .iter()
                    .map(|entry| MetadataValue::from_metadata_value(entry, bech32_coder))
                    .collect(),
            ),
            NativeMetadataEntry::Value(entry) => {
                Self::Value(MetadataValue::from_metadata_value(entry, bech32_coder))
            }
        }
    }
}

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
            Self::Address { value } => {
                NativeMetadataValue::Address(GlobalAddress::new_unchecked(value.0))
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
            NativeMetadataValue::Address(value) => Self::Address {
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
        }
    }
}
