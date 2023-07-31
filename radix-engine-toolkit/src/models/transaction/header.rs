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

use radix_engine_common::types::Epoch;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use transaction::prelude::TransactionHeaderV1;

use crate::prelude::*;

#[typeshare::typeshare]
type SerializableEpoch = SerializableU64;

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct SerializableTransactionHeader {
    pub network_id: SerializableU8,
    pub start_epoch_inclusive: SerializableEpoch,
    pub end_epoch_exclusive: SerializableEpoch,
    pub nonce: SerializableU32,
    pub notary_public_key: SerializablePublicKey,
    pub notary_is_signatory: bool,
    pub tip_percentage: SerializableU16,
}

impl From<SerializableTransactionHeader> for TransactionHeaderV1 {
    fn from(value: SerializableTransactionHeader) -> Self {
        Self {
            network_id: *value.network_id,
            start_epoch_inclusive: Epoch::of(*value.start_epoch_inclusive),
            end_epoch_exclusive: Epoch::of(*value.end_epoch_exclusive),
            nonce: *value.nonce,
            notary_public_key: value.notary_public_key.into(),
            notary_is_signatory: value.notary_is_signatory,
            tip_percentage: *value.tip_percentage,
        }
    }
}

impl From<TransactionHeaderV1> for SerializableTransactionHeader {
    fn from(value: TransactionHeaderV1) -> Self {
        Self {
            network_id: value.network_id.into(),
            start_epoch_inclusive: value.start_epoch_inclusive.number().into(),
            end_epoch_exclusive: value.end_epoch_exclusive.number().into(),
            nonce: value.nonce.into(),
            notary_public_key: value.notary_public_key.into(),
            notary_is_signatory: value.notary_is_signatory,
            tip_percentage: value.tip_percentage.into(),
        }
    }
}
