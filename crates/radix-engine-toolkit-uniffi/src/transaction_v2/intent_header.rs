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

#[derive(Clone, Debug, Record)]
pub struct IntentHeaderV2 {
    pub network_id: u8,

    pub start_epoch_inclusive: u64,
    pub end_epoch_exclusive: u64,
    pub min_proposer_timestamp_inclusive: Option<i64>,
    pub max_proposer_timestamp_exclusive: Option<i64>,

    pub intent_discriminator: u64,
}

impl TryFrom<IntentHeaderV2> for NativeIntentHeaderV2 {
    type Error = RadixEngineToolkitError;

    fn try_from(value: IntentHeaderV2) -> Result<Self> {
        Ok(Self {
            network_id: value.network_id,

            start_epoch_inclusive: NativeEpoch::of(value.start_epoch_inclusive),
            end_epoch_exclusive: NativeEpoch::of(value.end_epoch_exclusive),
            min_proposer_timestamp_inclusive: value
                .min_proposer_timestamp_inclusive
                .map(NativeInstant::new),
            max_proposer_timestamp_exclusive: value
                .max_proposer_timestamp_exclusive
                .map(NativeInstant::new),

            intent_discriminator: value.intent_discriminator,
        })
    }
}

impl From<NativeIntentHeaderV2> for IntentHeaderV2 {
    fn from(value: NativeIntentHeaderV2) -> Self {
        Self {
            network_id: value.network_id,
            start_epoch_inclusive: value.start_epoch_inclusive.number(),
            end_epoch_exclusive: value.end_epoch_exclusive.number(),
            min_proposer_timestamp_inclusive: value
                .min_proposer_timestamp_inclusive
                .map(|value| value.seconds_since_unix_epoch),
            max_proposer_timestamp_exclusive: value
                .max_proposer_timestamp_exclusive
                .map(|value| value.seconds_since_unix_epoch),
            intent_discriminator: value.intent_discriminator,
        }
    }
}
