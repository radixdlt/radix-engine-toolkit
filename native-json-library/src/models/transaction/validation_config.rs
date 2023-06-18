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

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use transaction::validation::ValidationConfig;

use crate::prelude::*;

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct SerializableValidationConfig {
    pub network_id: SerializableU8,
    pub max_notarized_payload_size: SerializableU64,
    pub min_cost_unit_limit: SerializableU32,
    pub max_cost_unit_limit: SerializableU32,
    pub min_tip_percentage: SerializableU16,
    pub max_tip_percentage: SerializableU16,
    pub max_epoch_range: SerializableU64,
}

impl From<ValidationConfig> for SerializableValidationConfig {
    fn from(value: ValidationConfig) -> Self {
        Self {
            network_id: value.network_id.into(),
            max_notarized_payload_size: (value.max_notarized_payload_size as u64).into(),
            min_cost_unit_limit: value.min_cost_unit_limit.into(),
            max_cost_unit_limit: value.max_cost_unit_limit.into(),
            min_tip_percentage: value.min_tip_percentage.into(),
            max_tip_percentage: value.max_tip_percentage.into(),
            max_epoch_range: value.max_epoch_range.into(),
        }
    }
}

impl From<SerializableValidationConfig> for ValidationConfig {
    fn from(value: SerializableValidationConfig) -> Self {
        Self {
            network_id: *value.network_id,
            max_notarized_payload_size: *value.max_notarized_payload_size as usize,
            min_cost_unit_limit: *value.min_cost_unit_limit,
            max_cost_unit_limit: *value.max_cost_unit_limit,
            min_tip_percentage: *value.min_tip_percentage,
            max_tip_percentage: *value.max_tip_percentage,
            max_epoch_range: *value.max_epoch_range,
        }
    }
}
