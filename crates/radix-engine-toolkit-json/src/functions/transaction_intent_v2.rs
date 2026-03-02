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
use radix_transactions::prelude::TransactionManifestV2;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

//===============================
// Transaction Intent V2 Hash
//===============================

#[typeshare::typeshare]
pub type TransactionIntentV2HashInput = SerializableTransactionIntentV2;
#[typeshare::typeshare]
pub type TransactionIntentV2HashOutput = SerializableTransactionHash;

pub struct TransactionIntentV2Hash;
impl<'f> Function<'f> for TransactionIntentV2Hash {
    type Input = TransactionIntentV2HashInput;
    type Output = TransactionIntentV2HashOutput;

    fn handle(
        transaction_intent: Self::Input,
    ) -> Result<Self::Output, crate::error::InvocationHandlingError> {
        let network_id = *transaction_intent.root_intent_core.header.network_id;
        let transaction_intent = transaction_intent.to_native(network_id)?;
        let hash =
            radix_engine_toolkit::functions::transaction_v2::transaction_intent::hash(
                &transaction_intent,
            )
            .map_err(|error| {
                InvocationHandlingError::EncodeError(
                    debug_string(error),
                    debug_string(transaction_intent),
                )
            })?;
        Ok(hash.into())
    }
}

export_function!(TransactionIntentV2Hash as transaction_intent_v2_hash);
export_jni_function!(TransactionIntentV2Hash as transactionIntentV2Hash);

//==================================
// Transaction Intent V2 Compile
//==================================

#[typeshare::typeshare]
pub type TransactionIntentV2CompileInput = SerializableTransactionIntentV2;
#[typeshare::typeshare]
pub type TransactionIntentV2CompileOutput = SerializableBytes;

pub struct TransactionIntentV2Compile;
impl<'f> Function<'f> for TransactionIntentV2Compile {
    type Input = TransactionIntentV2CompileInput;
    type Output = TransactionIntentV2CompileOutput;

    fn handle(
        transaction_intent: Self::Input,
    ) -> Result<Self::Output, crate::error::InvocationHandlingError> {
        let network_id = *transaction_intent.root_intent_core.header.network_id;
        let transaction_intent = transaction_intent.to_native(network_id)?;
        let compile =
            radix_engine_toolkit::functions::transaction_v2::transaction_intent::to_payload_bytes(
                &transaction_intent,
            )
            .map_err(|error| {
                InvocationHandlingError::EncodeError(
                    debug_string(error),
                    debug_string(transaction_intent),
                )
            })?;
        Ok(compile.into())
    }
}

export_function!(TransactionIntentV2Compile as transaction_intent_v2_compile);
export_jni_function!(TransactionIntentV2Compile as transactionIntentV2Compile);

//====================================
// Transaction Intent V2 Decompile
//====================================

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct TransactionIntentV2DecompileInput {
    pub compiled: SerializableBytes,
    pub network_id: SerializableU8,
}
#[typeshare::typeshare]
pub type TransactionIntentV2DecompileOutput = SerializableTransactionIntentV2;

pub struct TransactionIntentV2Decompile;
impl<'a> Function<'a> for TransactionIntentV2Decompile {
    type Input = TransactionIntentV2DecompileInput;
    type Output = TransactionIntentV2DecompileOutput;

    fn handle(
        TransactionIntentV2DecompileInput {
            compiled,
            network_id,
        }: Self::Input,
    ) -> Result<Self::Output, InvocationHandlingError> {
        let transaction_intent =
            radix_engine_toolkit::functions::transaction_v2::transaction_intent::from_payload_bytes(
                &**compiled,
            )
            .map_err(|error| {
                InvocationHandlingError::DecodeError(
                    debug_string(error),
                    debug_string(compiled),
                )
            })?;

        let transaction_intent = SerializableTransactionIntentV2::from_native(
            &transaction_intent,
            *network_id,
            (),
        )?;

        Ok(transaction_intent)
    }
}

export_function!(
    TransactionIntentV2Decompile as transaction_intent_v2_decompile
);
export_jni_function!(
    TransactionIntentV2Decompile as transactionIntentV2Decompile
);

//==========================================
// Transaction Intent V2 Statically Analyze
//==========================================

#[typeshare::typeshare]
pub type TransactionIntentV2StaticallyAnalyzeInput =
    SerializableTransactionIntentV2;
#[typeshare::typeshare]
pub type TransactionIntentV2StaticallyAnalyzeOutput =
    ManifestStaticallyAnalyzeOutput;

pub struct TransactionIntentV2StaticallyAnalyze;
impl<'a> Function<'a> for TransactionIntentV2StaticallyAnalyze {
    type Input = TransactionIntentV2StaticallyAnalyzeInput;
    type Output = TransactionIntentV2StaticallyAnalyzeOutput;

    fn handle(
        transaction_intent: Self::Input,
    ) -> Result<Self::Output, InvocationHandlingError> {
        let network_id = *transaction_intent.root_intent_core.header.network_id;
        let transaction_intent = transaction_intent.to_native(network_id)?;
        let manifest = TransactionManifestV2 {
            instructions: transaction_intent.root_intent_core.instructions.0,
            blobs: transaction_intent.root_intent_core.blobs.into(),
            object_names: Default::default(),
            children: transaction_intent
                .root_intent_core
                .children
                .children
                .into_iter()
                .collect(),
        };
        let analysis =
            radix_engine_toolkit::functions::transaction_v2::transaction_manifest::statically_analyze(
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
    TransactionIntentV2StaticallyAnalyze
        as transaction_intent_v2_statically_analyze
);
export_jni_function!(
    TransactionIntentV2StaticallyAnalyze
        as transactionIntentV2StaticallyAnalyze
);
