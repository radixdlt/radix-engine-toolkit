// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at

//   http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use radix_transaction::validation::ValidationConfig;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct ValidationConfigProxy {
    #[serde_as(as = "DisplayFromStr")]
    pub network_id: u8,

    #[serde_as(as = "DisplayFromStr")]
    pub min_cost_unit_limit: u32,

    #[serde_as(as = "DisplayFromStr")]
    pub max_cost_unit_limit: u32,

    #[serde_as(as = "DisplayFromStr")]
    pub min_tip_percentage: u8,

    #[serde_as(as = "DisplayFromStr")]
    pub max_tip_percentage: u8,

    #[serde_as(as = "DisplayFromStr")]
    pub max_epoch_range: u64,
}

impl From<ValidationConfig> for ValidationConfigProxy {
    fn from(value: ValidationConfig) -> Self {
        Self {
            network_id: value.network_id,
            min_cost_unit_limit: value.min_cost_unit_limit,
            max_cost_unit_limit: value.max_cost_unit_limit,
            min_tip_percentage: value.min_tip_percentage,
            max_tip_percentage: value.max_tip_percentage,
            max_epoch_range: value.max_epoch_range,
        }
    }
}

impl From<ValidationConfigProxy> for ValidationConfig {
    fn from(value: ValidationConfigProxy) -> Self {
        Self {
            network_id: value.network_id,
            min_cost_unit_limit: value.min_cost_unit_limit,
            max_cost_unit_limit: value.max_cost_unit_limit,
            min_tip_percentage: value.min_tip_percentage,
            max_tip_percentage: value.max_tip_percentage,
            max_epoch_range: value.max_epoch_range,
        }
    }
}
