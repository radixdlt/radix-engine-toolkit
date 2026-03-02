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

use radix_common::types::Epoch;
use radix_transactions::prelude::IntentHeaderV2;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct SerializableIntentHeaderV2 {
    pub network_id: SerializableU8,
    pub start_epoch_inclusive: SerializableU64,
    pub end_epoch_exclusive: SerializableU64,
    pub min_proposer_timestamp_inclusive: Option<SerializableI64>,
    pub max_proposer_timestamp_exclusive: Option<SerializableI64>,
    pub intent_discriminator: SerializableU64,
}

impl From<SerializableIntentHeaderV2> for IntentHeaderV2 {
    fn from(value: SerializableIntentHeaderV2) -> Self {
        Self {
            network_id: *value.network_id,
            start_epoch_inclusive: Epoch::of(*value.start_epoch_inclusive),
            end_epoch_exclusive: Epoch::of(*value.end_epoch_exclusive),
            min_proposer_timestamp_inclusive: value
                .min_proposer_timestamp_inclusive
                .map(|v| radix_common::prelude::Instant::new(*v)),
            max_proposer_timestamp_exclusive: value
                .max_proposer_timestamp_exclusive
                .map(|v| radix_common::prelude::Instant::new(*v)),
            intent_discriminator: *value.intent_discriminator,
        }
    }
}

impl From<IntentHeaderV2> for SerializableIntentHeaderV2 {
    fn from(value: IntentHeaderV2) -> Self {
        Self {
            network_id: value.network_id.into(),
            start_epoch_inclusive: value.start_epoch_inclusive.number().into(),
            end_epoch_exclusive: value.end_epoch_exclusive.number().into(),
            min_proposer_timestamp_inclusive: value
                .min_proposer_timestamp_inclusive
                .map(|v| v.seconds_since_unix_epoch.into()),
            max_proposer_timestamp_exclusive: value
                .max_proposer_timestamp_exclusive
                .map(|v| v.seconds_since_unix_epoch.into()),
            intent_discriminator: value.intent_discriminator.into(),
        }
    }
}
