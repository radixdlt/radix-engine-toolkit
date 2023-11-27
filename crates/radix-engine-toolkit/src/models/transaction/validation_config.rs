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
use transaction::validation::{MessageValidationConfig, ValidationConfig};

use crate::prelude::*;

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct SerializableValidationConfig {
    pub network_id: SerializableU8,
    pub max_notarized_payload_size: SerializableU64,
    pub min_tip_percentage: SerializableU16,
    pub max_tip_percentage: SerializableU16,
    pub max_epoch_range: SerializableU64,
    pub message_validation: SerializableMessageValidationConfig,
}

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct SerializableMessageValidationConfig {
    pub max_plaintext_message_length: SerializableU64,
    pub max_encrypted_message_length: SerializableU64,
    pub max_mime_type_length: SerializableU64,
    pub max_decryptors: SerializableU64,
}

impl From<ValidationConfig> for SerializableValidationConfig {
    fn from(value: ValidationConfig) -> Self {
        Self {
            network_id: value.network_id.into(),
            max_notarized_payload_size: (value.max_notarized_payload_size
                as u64)
                .into(),
            min_tip_percentage: value.min_tip_percentage.into(),
            max_tip_percentage: value.max_tip_percentage.into(),
            max_epoch_range: value.max_epoch_range.into(),
            message_validation: value.message_validation.into(),
        }
    }
}

impl From<SerializableValidationConfig> for ValidationConfig {
    fn from(value: SerializableValidationConfig) -> Self {
        Self {
            network_id: *value.network_id,
            max_notarized_payload_size: *value.max_notarized_payload_size
                as usize,
            min_tip_percentage: *value.min_tip_percentage,
            max_tip_percentage: *value.max_tip_percentage,
            max_epoch_range: *value.max_epoch_range,
            message_validation: MessageValidationConfig::from(
                value.message_validation,
            ),
        }
    }
}

impl From<MessageValidationConfig> for SerializableMessageValidationConfig {
    fn from(value: MessageValidationConfig) -> Self {
        Self {
            max_plaintext_message_length: (value.max_plaintext_message_length
                as u64)
                .into(),
            max_encrypted_message_length: (value.max_encrypted_message_length
                as u64)
                .into(),
            max_mime_type_length: (value.max_mime_type_length as u64).into(),
            max_decryptors: (value.max_decryptors as u64).into(),
        }
    }
}

impl From<SerializableMessageValidationConfig> for MessageValidationConfig {
    fn from(value: SerializableMessageValidationConfig) -> Self {
        Self {
            max_plaintext_message_length: *value.max_plaintext_message_length
                as usize,
            max_encrypted_message_length: *value.max_encrypted_message_length
                as usize,
            max_mime_type_length: *value.max_mime_type_length as usize,
            max_decryptors: *value.max_decryptors as usize,
        }
    }
}
