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

#[derive(Clone, Debug, Object)]
pub struct ValidationConfig {
    pub network_id: u8,
    pub max_notarized_payload_size: u64,
    pub min_cost_unit_limit: u32,
    pub max_cost_unit_limit: u32,
    pub min_tip_percentage: u16,
    pub max_tip_percentage: u16,
    pub max_epoch_range: u64,
}

#[uniffi::export]
impl ValidationConfig {
    #[allow(clippy::too_many_arguments)]
    #[uniffi::constructor]
    pub fn new(
        network_id: u8,
        max_notarized_payload_size: u64,
        min_cost_unit_limit: u32,
        max_cost_unit_limit: u32,
        min_tip_percentage: u16,
        max_tip_percentage: u16,
        max_epoch_range: u64,
    ) -> Arc<Self> {
        Arc::new(Self {
            network_id,
            max_notarized_payload_size,
            min_cost_unit_limit,
            max_cost_unit_limit,
            min_tip_percentage,
            max_tip_percentage,
            max_epoch_range,
        })
    }

    #[uniffi::constructor]
    pub fn default(network_id: u8) -> Arc<Self> {
        Arc::new(NativeValidationConfig::default(network_id).into())
    }

    pub fn network_id(&self) -> u8 {
        self.network_id
    }

    pub fn max_notarized_payload_size(&self) -> u64 {
        self.max_notarized_payload_size
    }

    pub fn min_cost_unit_limit(&self) -> u32 {
        self.min_cost_unit_limit
    }

    pub fn max_cost_unit_limit(&self) -> u32 {
        self.max_cost_unit_limit
    }

    pub fn min_tip_percentage(&self) -> u16 {
        self.min_tip_percentage
    }

    pub fn max_tip_percentage(&self) -> u16 {
        self.max_tip_percentage
    }

    pub fn max_epoch_range(&self) -> u64 {
        self.max_epoch_range
    }
}

//============
// From Impls
//============

impl From<ValidationConfig> for NativeValidationConfig {
    fn from(value: ValidationConfig) -> Self {
        Self {
            network_id: value.network_id,
            max_notarized_payload_size: value.max_notarized_payload_size as usize,
            min_cost_unit_limit: value.min_cost_unit_limit,
            max_cost_unit_limit: value.max_cost_unit_limit,
            min_tip_percentage: value.min_tip_percentage,
            max_tip_percentage: value.max_tip_percentage,
            max_epoch_range: value.max_epoch_range,
        }
    }
}

impl From<NativeValidationConfig> for ValidationConfig {
    fn from(value: NativeValidationConfig) -> Self {
        Self {
            network_id: value.network_id,
            max_notarized_payload_size: value.max_notarized_payload_size as u64,
            min_cost_unit_limit: value.min_cost_unit_limit,
            max_cost_unit_limit: value.max_cost_unit_limit,
            min_tip_percentage: value.min_tip_percentage,
            max_tip_percentage: value.max_tip_percentage,
            max_epoch_range: value.max_epoch_range,
        }
    }
}
