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
use radix_transactions::prelude::SubintentManifestV2;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

//===================
// Subintent V2 Hash
//===================

#[typeshare::typeshare]
pub type SubintentV2HashInput = SerializableSubintentV2;
#[typeshare::typeshare]
pub type SubintentV2HashOutput = SerializableTransactionHash;

pub struct SubintentV2Hash;
impl<'f> Function<'f> for SubintentV2Hash {
    type Input = SubintentV2HashInput;
    type Output = SubintentV2HashOutput;

    fn handle(
        subintent: Self::Input,
    ) -> Result<Self::Output, crate::error::InvocationHandlingError> {
        let network_id = *subintent.intent_core.header.network_id;
        let subintent = subintent.to_native(network_id)?;
        let hash =
            radix_engine_toolkit::functions::transaction_v2::subintent::hash(
                &subintent,
            )
            .map_err(|error| {
                InvocationHandlingError::EncodeError(
                    debug_string(error),
                    debug_string(subintent),
                )
            })?;
        Ok(hash.into())
    }
}

export_function!(SubintentV2Hash as subintent_v2_hash);
export_jni_function!(SubintentV2Hash as subintentV2Hash);

//======================
// Subintent V2 Compile
//======================

#[typeshare::typeshare]
pub type SubintentV2CompileInput = SerializableSubintentV2;
#[typeshare::typeshare]
pub type SubintentV2CompileOutput = SerializableBytes;

pub struct SubintentV2Compile;
impl<'f> Function<'f> for SubintentV2Compile {
    type Input = SubintentV2CompileInput;
    type Output = SubintentV2CompileOutput;

    fn handle(
        subintent: Self::Input,
    ) -> Result<Self::Output, crate::error::InvocationHandlingError> {
        let network_id = *subintent.intent_core.header.network_id;
        let subintent = subintent.to_native(network_id)?;
        let compile =
            radix_engine_toolkit::functions::transaction_v2::subintent::to_payload_bytes(
                &subintent,
            )
            .map_err(|error| {
                InvocationHandlingError::EncodeError(
                    debug_string(error),
                    debug_string(subintent),
                )
            })?;
        Ok(compile.into())
    }
}

export_function!(SubintentV2Compile as subintent_v2_compile);
export_jni_function!(SubintentV2Compile as subintentV2Compile);

//========================
// Subintent V2 Decompile
//========================

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct SubintentV2DecompileInput {
    pub compiled: SerializableBytes,
    pub network_id: SerializableU8,
}
#[typeshare::typeshare]
pub type SubintentV2DecompileOutput = SerializableSubintentV2;

pub struct SubintentV2Decompile;
impl<'a> Function<'a> for SubintentV2Decompile {
    type Input = SubintentV2DecompileInput;
    type Output = SubintentV2DecompileOutput;

    fn handle(
        SubintentV2DecompileInput {
            compiled,
            network_id,
        }: Self::Input,
    ) -> Result<Self::Output, InvocationHandlingError> {
        let subintent =
            radix_engine_toolkit::functions::transaction_v2::subintent::from_payload_bytes(
                &**compiled,
            )
            .map_err(|error| {
                InvocationHandlingError::DecodeError(
                    debug_string(error),
                    debug_string(compiled),
                )
            })?;

        let subintent =
            SerializableSubintentV2::from_native(&subintent, *network_id, ())?;

        Ok(subintent)
    }
}

export_function!(SubintentV2Decompile as subintent_v2_decompile);
export_jni_function!(SubintentV2Decompile as subintentV2Decompile);

//=================================
// Subintent V2 Statically Analyze
//=================================

#[typeshare::typeshare]
pub type SubintentV2StaticallyAnalyzeInput = SerializableSubintentV2;
#[typeshare::typeshare]
pub type SubintentV2StaticallyAnalyzeOutput = ManifestStaticallyAnalyzeOutput;

pub struct SubintentV2StaticallyAnalyze;
impl<'a> Function<'a> for SubintentV2StaticallyAnalyze {
    type Input = SubintentV2StaticallyAnalyzeInput;
    type Output = SubintentV2StaticallyAnalyzeOutput;

    fn handle(
        subintent: Self::Input,
    ) -> Result<Self::Output, InvocationHandlingError> {
        let network_id = *subintent.intent_core.header.network_id;
        let subintent = subintent.to_native(network_id)?;
        let manifest = SubintentManifestV2 {
            instructions: subintent.intent_core.instructions.0,
            blobs: subintent.intent_core.blobs.into(),
            object_names: Default::default(),
            children: subintent
                .intent_core
                .children
                .children
                .into_iter()
                .collect(),
        };
        let analysis =
            radix_engine_toolkit::functions::transaction_v2::subintent_manifest::statically_analyze(
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

export_function!(
    SubintentV2StaticallyAnalyze as subintent_v2_statically_analyze
);
export_jni_function!(
    SubintentV2StaticallyAnalyze as subintentV2StaticallyAnalyze
);
