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

use radix_transactions::manifest::InterpreterValidationRulesetSpecifier;
use radix_transactions::prelude::PreparationSettings;
use radix_transactions::validation::{
    ManifestValidationRuleset, MessageValidationConfig,
    TransactionValidationConfig,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct SerializableValidationConfig {
    pub max_signer_signatures_per_intent: SerializableU64,
    pub max_references_per_intent: SerializableU64,
    pub min_tip_percentage: SerializableU16,
    pub max_tip_percentage: SerializableU16,
    pub max_epoch_range: SerializableU64,
    pub max_instructions: SerializableU64,
    pub message_validation: SerializableMessageValidationConfig,
    pub v1_transactions_allow_notary_to_duplicate_signer: bool,
    pub preparation_settings: SerializablePreparationSettings,
    pub manifest_validation: SerializableManifestValidationRuleset,
    pub v2_transactions_allowed: bool,
    pub min_tip_basis_points: SerializableU32,
    pub max_tip_basis_points: SerializableU32,
    pub max_subintent_depth: SerializableU64,
    pub max_total_signature_validations: SerializableU64,
    pub max_total_references: SerializableU64,
}

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct SerializableMessageValidationConfig {
    pub max_plaintext_message_length: SerializableU64,
    pub max_encrypted_message_length: SerializableU64,
    pub max_mime_type_length: SerializableU64,
    pub max_decryptors: SerializableU64,
}

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct SerializablePreparationSettings {
    pub v2_transactions_permitted: bool,
    pub max_user_payload_length: SerializableU64,
    pub max_ledger_payload_length: SerializableU64,
    pub max_child_subintents_per_intent: SerializableU64,
    pub max_subintents_per_transaction: SerializableU64,
    pub max_blobs: SerializableU64,
}

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
#[serde(tag = "kind", content = "value")]
pub enum SerializableManifestValidationRuleset {
    BabylonBasicValidator,
    Interpreter(SerializableInterpreterValidationRulesetSpecifier),
}

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub enum SerializableInterpreterValidationRulesetSpecifier {
    AllValidations,
    Cuttlefish,
}

impl From<TransactionValidationConfig> for SerializableValidationConfig {
    fn from(value: TransactionValidationConfig) -> Self {
        Self {
            max_signer_signatures_per_intent: (value
                .max_signer_signatures_per_intent
                as u64)
                .into(),
            max_references_per_intent: (value.max_references_per_intent as u64)
                .into(),
            min_tip_percentage: value.min_tip_percentage.into(),
            max_tip_percentage: value.max_tip_percentage.into(),
            max_epoch_range: value.max_epoch_range.into(),
            max_instructions: (value.max_instructions as u64).into(),
            message_validation: value.message_validation.into(),
            v1_transactions_allow_notary_to_duplicate_signer: value
                .v1_transactions_allow_notary_to_duplicate_signer,
            preparation_settings: value.preparation_settings.into(),
            manifest_validation: value.manifest_validation.into(),
            v2_transactions_allowed: value.v2_transactions_allowed,
            min_tip_basis_points: value.min_tip_basis_points.into(),
            max_tip_basis_points: value.max_tip_basis_points.into(),
            max_subintent_depth: (value.max_subintent_depth as u64).into(),
            max_total_signature_validations: (value
                .max_total_signature_validations
                as u64)
                .into(),
            max_total_references: (value.max_total_references as u64).into(),
        }
    }
}

impl From<SerializableValidationConfig> for TransactionValidationConfig {
    fn from(value: SerializableValidationConfig) -> Self {
        Self {
            max_signer_signatures_per_intent: *value
                .max_signer_signatures_per_intent
                as usize,
            max_references_per_intent: *value.max_references_per_intent
                as usize,
            min_tip_percentage: *value.min_tip_percentage,
            max_tip_percentage: *value.max_tip_percentage,
            max_epoch_range: *value.max_epoch_range,
            max_instructions: *value.max_instructions as usize,
            message_validation: MessageValidationConfig::from(
                value.message_validation,
            ),
            v1_transactions_allow_notary_to_duplicate_signer: value
                .v1_transactions_allow_notary_to_duplicate_signer,
            preparation_settings: PreparationSettings::from(
                value.preparation_settings,
            ),
            manifest_validation: ManifestValidationRuleset::from(
                value.manifest_validation,
            ),
            v2_transactions_allowed: value.v2_transactions_allowed,
            min_tip_basis_points: *value.min_tip_basis_points,
            max_tip_basis_points: *value.max_tip_basis_points,
            max_subintent_depth: *value.max_subintent_depth as usize,
            max_total_signature_validations: *value
                .max_total_signature_validations
                as usize,
            max_total_references: *value.max_total_references as usize,
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

impl From<PreparationSettings> for SerializablePreparationSettings {
    fn from(value: PreparationSettings) -> Self {
        Self {
            v2_transactions_permitted: value.v2_transactions_permitted,
            max_user_payload_length: (value.max_user_payload_length as u64)
                .into(),
            max_ledger_payload_length: (value.max_ledger_payload_length as u64)
                .into(),
            max_child_subintents_per_intent: (value
                .max_child_subintents_per_intent
                as u64)
                .into(),
            max_subintents_per_transaction: (value
                .max_subintents_per_transaction
                as u64)
                .into(),
            max_blobs: (value.max_blobs as u64).into(),
        }
    }
}

impl From<SerializablePreparationSettings> for PreparationSettings {
    fn from(value: SerializablePreparationSettings) -> Self {
        Self {
            v2_transactions_permitted: value.v2_transactions_permitted,
            max_user_payload_length: *value.max_user_payload_length as usize,
            max_ledger_payload_length: *value.max_ledger_payload_length
                as usize,
            max_child_subintents_per_intent: *value
                .max_child_subintents_per_intent
                as usize,
            max_subintents_per_transaction: *value
                .max_subintents_per_transaction
                as usize,
            max_blobs: *value.max_blobs as usize,
        }
    }
}

impl From<ManifestValidationRuleset> for SerializableManifestValidationRuleset {
    fn from(value: ManifestValidationRuleset) -> Self {
        match value {
            ManifestValidationRuleset::BabylonBasicValidator => {
                Self::BabylonBasicValidator
            }
            ManifestValidationRuleset::Interpreter(specifier) => {
                Self::Interpreter(specifier.into())
            }
        }
    }
}

impl From<SerializableManifestValidationRuleset> for ManifestValidationRuleset {
    fn from(value: SerializableManifestValidationRuleset) -> Self {
        match value {
            SerializableManifestValidationRuleset::BabylonBasicValidator => {
                Self::BabylonBasicValidator
            }
            SerializableManifestValidationRuleset::Interpreter(specifier) => {
                Self::Interpreter(specifier.into())
            }
        }
    }
}

impl From<InterpreterValidationRulesetSpecifier>
    for SerializableInterpreterValidationRulesetSpecifier
{
    fn from(value: InterpreterValidationRulesetSpecifier) -> Self {
        match value {
            InterpreterValidationRulesetSpecifier::AllValidations => {
                Self::AllValidations
            }
            InterpreterValidationRulesetSpecifier::Cuttlefish => {
                Self::Cuttlefish
            }
        }
    }
}

impl From<SerializableInterpreterValidationRulesetSpecifier>
    for InterpreterValidationRulesetSpecifier
{
    fn from(
        value: SerializableInterpreterValidationRulesetSpecifier,
    ) -> Self {
        match value {
            SerializableInterpreterValidationRulesetSpecifier::AllValidations => {
                Self::AllValidations
            }
            SerializableInterpreterValidationRulesetSpecifier::Cuttlefish => {
                Self::Cuttlefish
            }
        }
    }
}
