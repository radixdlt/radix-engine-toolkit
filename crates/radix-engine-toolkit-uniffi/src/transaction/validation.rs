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
    pub min_tip_percentage: u16,
    pub max_tip_percentage: u16,
    pub max_epoch_range: u64,
    pub message_validation: Arc<MessageValidationConfig>,
}

#[derive(Clone, Debug, Object)]
pub struct MessageValidationConfig {
    pub max_plaintext_message_length: u64,
    pub max_encrypted_message_length: u64,
    pub max_mime_type_length: u64,
    pub max_decryptors: u64,
}

#[uniffi::export]
impl ValidationConfig {
    #[allow(clippy::too_many_arguments)]
    #[uniffi::constructor]
    pub fn new(
        network_id: u8,
        max_notarized_payload_size: u64,
        min_tip_percentage: u16,
        max_tip_percentage: u16,
        max_epoch_range: u64,
        message_validation: Arc<MessageValidationConfig>,
    ) -> Arc<Self> {
        Arc::new(Self {
            network_id,
            max_notarized_payload_size,
            min_tip_percentage,
            max_tip_percentage,
            max_epoch_range,
            message_validation,
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

    pub fn min_tip_percentage(&self) -> u16 {
        self.min_tip_percentage
    }

    pub fn max_tip_percentage(&self) -> u16 {
        self.max_tip_percentage
    }

    pub fn max_epoch_range(&self) -> u64 {
        self.max_epoch_range
    }

    pub fn message_validation(&self) -> Arc<MessageValidationConfig> {
        self.message_validation.clone()
    }
}

#[uniffi::export]
impl MessageValidationConfig {
    #[uniffi::constructor]
    pub fn new(
        max_plaintext_message_length: u64,
        max_encrypted_message_length: u64,
        max_mime_type_length: u64,
        max_decryptors: u64,
    ) -> Arc<Self> {
        Arc::new(Self {
            max_plaintext_message_length,
            max_encrypted_message_length,
            max_mime_type_length,
            max_decryptors,
        })
    }

    #[allow(clippy::should_implement_trait)]
    #[uniffi::constructor]
    pub fn default() -> Arc<Self> {
        Arc::new(NativeMessageValidationConfig::default().into())
    }

    pub fn max_plaintext_message_length(&self) -> u64 {
        self.max_plaintext_message_length
    }

    pub fn max_encrypted_message_length(&self) -> u64 {
        self.max_encrypted_message_length
    }

    pub fn max_mime_type_length(&self) -> u64 {
        self.max_mime_type_length
    }

    pub fn max_decryptors(&self) -> u64 {
        self.max_decryptors
    }
}

//============
// From Impls
//============

impl From<ValidationConfig> for NativeValidationConfig {
    fn from(value: ValidationConfig) -> Self {
        Self {
            network_id: value.network_id,
            max_notarized_payload_size: value.max_notarized_payload_size
                as usize,
            min_tip_percentage: value.min_tip_percentage,
            max_tip_percentage: value.max_tip_percentage,
            max_epoch_range: value.max_epoch_range,
            message_validation: value
                .message_validation
                .as_ref()
                .clone()
                .into(),
        }
    }
}

impl From<NativeValidationConfig> for ValidationConfig {
    fn from(value: NativeValidationConfig) -> Self {
        Self {
            network_id: value.network_id,
            max_notarized_payload_size: value.max_notarized_payload_size as u64,
            min_tip_percentage: value.min_tip_percentage,
            max_tip_percentage: value.max_tip_percentage,
            max_epoch_range: value.max_epoch_range,
            message_validation: Arc::new(value.message_validation.into()),
        }
    }
}

impl From<MessageValidationConfig> for NativeMessageValidationConfig {
    fn from(value: MessageValidationConfig) -> Self {
        Self {
            max_plaintext_message_length: value.max_plaintext_message_length
                as usize,
            max_encrypted_message_length: value.max_encrypted_message_length
                as usize,
            max_mime_type_length: value.max_mime_type_length as usize,
            max_decryptors: value.max_decryptors as usize,
        }
    }
}

impl From<NativeMessageValidationConfig> for MessageValidationConfig {
    fn from(value: NativeMessageValidationConfig) -> Self {
        Self {
            max_plaintext_message_length: value.max_plaintext_message_length
                as u64,
            max_encrypted_message_length: value.max_encrypted_message_length
                as u64,
            max_mime_type_length: value.max_mime_type_length as u64,
            max_decryptors: value.max_decryptors as u64,
        }
    }
}
