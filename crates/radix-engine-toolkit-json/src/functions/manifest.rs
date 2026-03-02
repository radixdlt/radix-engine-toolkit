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

use radix_common::prelude::{ManifestAddress, ManifestGlobalAddress};
use sbor_json::utils::network_definition_from_network_id;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

//===============
// Manifest Hash
//===============

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct ManifestHashInput {
    pub manifest: SerializableTransactionManifest,
    pub network_id: SerializableU8,
}
#[typeshare::typeshare]
pub type ManifestHashOutput = SerializableHash;

pub struct ManifestHash;
impl<'f> Function<'f> for ManifestHash {
    type Input = ManifestHashInput;
    type Output = ManifestHashOutput;

    fn handle(
        ManifestHashInput {
            manifest,
            network_id,
        }: Self::Input,
    ) -> Result<Self::Output, crate::error::InvocationHandlingError> {
        let manifest = manifest.to_native(*network_id)?;
        let compiled = radix_engine_toolkit::functions::transaction_v1::manifest::to_payload_bytes(&manifest)
            .map_err(|error| {
                InvocationHandlingError::EncodeError(
                    debug_string(error),
                    debug_string(&manifest),
                )
            })?;
        let hash = radix_common::crypto::hash(&compiled);
        Ok(hash.into())
    }
}

export_function!(ManifestHash as manifest_hash);
export_jni_function!(ManifestHash as manifestHash);

//==================
// Manifest Compile
//==================

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct ManifestCompileInput {
    pub manifest: SerializableTransactionManifest,
    pub network_id: SerializableU8,
}
#[typeshare::typeshare]
pub type ManifestCompileOutput = SerializableBytes;

pub struct ManifestCompile;
impl<'f> Function<'f> for ManifestCompile {
    type Input = ManifestCompileInput;
    type Output = ManifestCompileOutput;

    fn handle(
        ManifestCompileInput {
            manifest,
            network_id,
        }: Self::Input,
    ) -> Result<Self::Output, crate::error::InvocationHandlingError> {
        let manifest = manifest.to_native(*network_id)?;
        let compile =
            radix_engine_toolkit::functions::transaction_v1::manifest::to_payload_bytes(&manifest)
                .map_err(|error| {
                    InvocationHandlingError::EncodeError(
                        debug_string(error),
                        debug_string(&manifest),
                    )
                })?;
        Ok(compile.into())
    }
}

export_function!(ManifestCompile as manifest_compile);
export_jni_function!(ManifestCompile as manifestCompile);

//====================
// Manifest Decompile
//====================

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct ManifestDecompileInput {
    pub compiled: SerializableBytes,
    pub network_id: SerializableU8,
    pub instructions_kind: SerializableInstructionsKind,
}
#[typeshare::typeshare]
pub type ManifestDecompileOutput = SerializableTransactionManifest;

pub struct ManifestDecompile;
impl<'a> Function<'a> for ManifestDecompile {
    type Input = ManifestDecompileInput;
    type Output = ManifestDecompileOutput;

    fn handle(
        ManifestDecompileInput {
            compiled,
            network_id,
            instructions_kind,
        }: Self::Input,
    ) -> Result<Self::Output, InvocationHandlingError> {
        let manifest =
            radix_engine_toolkit::functions::transaction_v1::manifest::from_payload_bytes(&**compiled)
                .map_err(|error| {
                InvocationHandlingError::DecodeError(
                    debug_string(error),
                    debug_string(&compiled),
                )
            })?;

        let manifest = SerializableTransactionManifest::from_native(
            &manifest,
            *network_id,
            instructions_kind,
        )?;

        Ok(manifest)
    }
}

export_function!(ManifestDecompile as manifest_decompile);
export_jni_function!(ManifestDecompile as manifestDecompile);

//==============================
// Manifest Statically Validate
//==============================

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct ManifestStaticallyValidateInput {
    pub manifest: SerializableTransactionManifest,
    pub network_id: SerializableU8,
}

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
#[serde(tag = "kind", content = "value")]
pub enum ManifestStaticallyValidateOutput {
    Valid,
    Invalid(String),
}

pub struct ManifestStaticallyValidate;
impl<'a> Function<'a> for ManifestStaticallyValidate {
    type Input = ManifestStaticallyValidateInput;
    type Output = ManifestStaticallyValidateOutput;

    fn handle(
        ManifestStaticallyValidateInput {
            manifest,
            network_id,
        }: Self::Input,
    ) -> Result<Self::Output, InvocationHandlingError> {
        let manifest = manifest.to_native(*network_id)?;
        let network_definition =
            network_definition_from_network_id(*network_id);

        match radix_engine_toolkit::functions::transaction_v1::manifest::statically_validate(
            &manifest,
            &network_definition,
        ) {
            Ok(..) => Ok(Self::Output::Valid),
            Err(error) => Ok(Self::Output::Invalid(debug_string(error))),
        }
    }
}

export_function!(ManifestStaticallyValidate as manifest_statically_validate);
export_jni_function!(ManifestStaticallyValidate as manifestStaticallyValidate);

//=============================
// Manifest Statically Analyze
//=============================

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct ManifestStaticallyAnalyzeInput {
    pub manifest: SerializableTransactionManifest,
    pub network_id: SerializableU8,
}

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct ManifestStaticallyAnalyzeOutput {
    pub encountered_entities: Vec<String>,
    pub accounts_requiring_auth: Vec<String>,
    pub accounts_withdrawn_from: Vec<String>,
    pub accounts_deposited_into: Vec<String>,
    pub classification: Vec<String>,
    pub reserved_instructions: Vec<String>,
}

pub struct ManifestStaticallyAnalyze;
impl<'a> Function<'a> for ManifestStaticallyAnalyze {
    type Input = ManifestStaticallyAnalyzeInput;
    type Output = ManifestStaticallyAnalyzeOutput;

    fn handle(
        ManifestStaticallyAnalyzeInput {
            manifest,
            network_id,
        }: Self::Input,
    ) -> Result<Self::Output, InvocationHandlingError> {
        let network_id = *network_id;
        let manifest = manifest.to_native(network_id)?;
        let analysis = radix_engine_toolkit::functions::transaction_v1::manifest::statically_analyze(
            &manifest,
        )
        .map_err(|error| {
            InvocationHandlingError::DecodeError(
                debug_string(error),
                debug_string(&manifest),
            )
        })?;

        let encountered_entities = analysis
            .entities_encountered_summary
            .entities
            .into_iter()
            .filter_map(|address| match address {
                ManifestAddress::Static(node_id) => Some(
                    SerializableNodeId::new(node_id, network_id).0.to_string(),
                ),
                ManifestAddress::Named(..) => None,
            })
            .collect();

        let accounts_requiring_auth = analysis
            .entities_requiring_auth_summary
            .accounts
            .into_iter()
            .filter_map(|address| match address {
                ManifestGlobalAddress::Static(global_address) => Some(
                    SerializableNodeId::from_global_address(
                        global_address,
                        network_id,
                    )
                    .0
                    .to_string(),
                ),
                ManifestGlobalAddress::Named(..) => None,
            })
            .collect();

        let accounts_withdrawn_from = analysis
            .account_interactions_summary
            .accounts_withdrawn_from
            .into_iter()
            .filter_map(|address| match address {
                ManifestGlobalAddress::Static(global_address) => Some(
                    SerializableNodeId::from_global_address(
                        global_address,
                        network_id,
                    )
                    .0
                    .to_string(),
                ),
                ManifestGlobalAddress::Named(..) => None,
            })
            .collect();

        let accounts_deposited_into = analysis
            .account_interactions_summary
            .accounts_deposited_into
            .into_iter()
            .filter_map(|address| match address {
                ManifestGlobalAddress::Static(global_address) => Some(
                    SerializableNodeId::from_global_address(
                        global_address,
                        network_id,
                    )
                    .0
                    .to_string(),
                ),
                ManifestGlobalAddress::Named(..) => None,
            })
            .collect();

        let classification = analysis
            .manifest_classification
            .into_iter()
            .map(|item| format!("{item:?}"))
            .collect();

        let reserved = analysis.reserved_instructions_summary;
        let mut reserved_instructions = Vec::new();
        if !reserved.account_lock_fee_invocations.is_empty() {
            reserved_instructions.push("AccountLockFee".to_owned());
        }
        if !reserved.account_securify_invocations.is_empty() {
            reserved_instructions.push("AccountSecurify".to_owned());
        }
        if !reserved
            .account_lock_owner_keys_metadata_field_invocations
            .is_empty()
        {
            reserved_instructions
                .push("AccountLockOwnerKeysMetadataField".to_owned());
        }
        if !reserved
            .account_update_owner_keys_metadata_field_invocations
            .is_empty()
        {
            reserved_instructions
                .push("AccountUpdateOwnerKeysMetadataField".to_owned());
        }
        if !reserved.identity_securify_invocations.is_empty() {
            reserved_instructions.push("IdentitySecurify".to_owned());
        }
        if !reserved
            .identity_lock_owner_keys_metadata_field_invocations
            .is_empty()
        {
            reserved_instructions
                .push("IdentityLockOwnerKeysMetadataField".to_owned());
        }
        if !reserved
            .identity_update_owner_keys_metadata_field_invocations
            .is_empty()
        {
            reserved_instructions
                .push("IdentityUpdateOwnerKeysMetadataField".to_owned());
        }
        if !reserved.access_controller_invocations.is_empty() {
            reserved_instructions.push("AccessControllerInvocation".to_owned());
        }

        Ok(Self::Output {
            encountered_entities,
            accounts_requiring_auth,
            accounts_withdrawn_from,
            accounts_deposited_into,
            classification,
            reserved_instructions,
        })
    }
}

export_function!(ManifestStaticallyAnalyze as manifest_statically_analyze);
export_jni_function!(ManifestStaticallyAnalyze as manifestStaticallyAnalyze);
