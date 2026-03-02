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

use sbor_json::utils::network_definition_from_network_id;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

//====================================
// Notarized Transaction V2 Hash
//====================================

#[typeshare::typeshare]
pub type NotarizedTransactionV2HashInput =
    SerializableNotarizedTransactionV2;
#[typeshare::typeshare]
pub type NotarizedTransactionV2HashOutput = SerializableTransactionHash;

pub struct NotarizedTransactionV2Hash;
impl<'f> Function<'f> for NotarizedTransactionV2Hash {
    type Input = NotarizedTransactionV2HashInput;
    type Output = NotarizedTransactionV2HashOutput;

    fn handle(
        notarized_transaction: Self::Input,
    ) -> Result<Self::Output, crate::error::InvocationHandlingError> {
        let network_id = *notarized_transaction
            .signed_transaction_intent
            .transaction_intent
            .root_intent_core
            .header
            .network_id;
        let notarized_transaction =
            notarized_transaction.to_native(network_id)?;
        let hash =
            radix_engine_toolkit::functions::transaction_v2::notarized_transaction::hash(
                &notarized_transaction,
            )
            .map_err(|error| {
                InvocationHandlingError::EncodeError(
                    debug_string(error),
                    debug_string(notarized_transaction),
                )
            })?;
        Ok(hash.into())
    }
}

export_function!(
    NotarizedTransactionV2Hash as notarized_transaction_v2_hash
);
export_jni_function!(
    NotarizedTransactionV2Hash as notarizedTransactionV2Hash
);

//=======================================
// Notarized Transaction V2 Compile
//=======================================

#[typeshare::typeshare]
pub type NotarizedTransactionV2CompileInput =
    SerializableNotarizedTransactionV2;
#[typeshare::typeshare]
pub type NotarizedTransactionV2CompileOutput = SerializableBytes;

pub struct NotarizedTransactionV2Compile;
impl<'f> Function<'f> for NotarizedTransactionV2Compile {
    type Input = NotarizedTransactionV2CompileInput;
    type Output = NotarizedTransactionV2CompileOutput;

    fn handle(
        notarized_transaction: Self::Input,
    ) -> Result<Self::Output, crate::error::InvocationHandlingError> {
        let network_id = *notarized_transaction
            .signed_transaction_intent
            .transaction_intent
            .root_intent_core
            .header
            .network_id;
        let notarized_transaction =
            notarized_transaction.to_native(network_id)?;
        let compile =
            radix_engine_toolkit::functions::transaction_v2::notarized_transaction::to_payload_bytes(
                &notarized_transaction,
            )
            .map_err(|error| {
                InvocationHandlingError::EncodeError(
                    debug_string(error),
                    debug_string(notarized_transaction),
                )
            })?;
        Ok(compile.into())
    }
}

export_function!(
    NotarizedTransactionV2Compile as notarized_transaction_v2_compile
);
export_jni_function!(
    NotarizedTransactionV2Compile as notarizedTransactionV2Compile
);

//=========================================
// Notarized Transaction V2 Decompile
//=========================================

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct NotarizedTransactionV2DecompileInput {
    pub compiled: SerializableBytes,
    pub network_id: SerializableU8,
}
#[typeshare::typeshare]
pub type NotarizedTransactionV2DecompileOutput =
    SerializableNotarizedTransactionV2;

pub struct NotarizedTransactionV2Decompile;
impl<'a> Function<'a> for NotarizedTransactionV2Decompile {
    type Input = NotarizedTransactionV2DecompileInput;
    type Output = NotarizedTransactionV2DecompileOutput;

    fn handle(
        NotarizedTransactionV2DecompileInput {
            compiled,
            network_id,
        }: Self::Input,
    ) -> Result<Self::Output, InvocationHandlingError> {
        let notarized_transaction =
            radix_engine_toolkit::functions::transaction_v2::notarized_transaction::from_payload_bytes(
                &**compiled,
            )
            .map_err(|error| {
                InvocationHandlingError::DecodeError(
                    debug_string(error),
                    debug_string(compiled),
                )
            })?;

        let notarized_transaction =
            SerializableNotarizedTransactionV2::from_native(
                &notarized_transaction,
                *network_id,
                (),
            )?;

        Ok(notarized_transaction)
    }
}

export_function!(
    NotarizedTransactionV2Decompile
        as notarized_transaction_v2_decompile
);
export_jni_function!(
    NotarizedTransactionV2Decompile as notarizedTransactionV2Decompile
);

//====================================================
// Notarized Transaction V2 Statically Validate
//====================================================

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct NotarizedTransactionV2StaticallyValidateInput {
    pub notarized_transaction: SerializableNotarizedTransactionV2,
    pub network_id: SerializableU8,
}

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
#[serde(tag = "kind", content = "value")]
pub enum NotarizedTransactionV2StaticallyValidateOutput {
    Valid,
    Invalid(String),
}

pub struct NotarizedTransactionV2StaticallyValidate;
impl<'a> Function<'a> for NotarizedTransactionV2StaticallyValidate {
    type Input = NotarizedTransactionV2StaticallyValidateInput;
    type Output = NotarizedTransactionV2StaticallyValidateOutput;

    fn handle(
        NotarizedTransactionV2StaticallyValidateInput {
            notarized_transaction,
            network_id,
        }: Self::Input,
    ) -> Result<Self::Output, InvocationHandlingError> {
        let notarized_transaction =
            notarized_transaction.to_native(*network_id)?;
        let network_definition =
            network_definition_from_network_id(*network_id);

        match radix_engine_toolkit::functions::transaction_v2::notarized_transaction::statically_validate(
            &notarized_transaction,
            &network_definition,
        ) {
            Ok(..) => Ok(Self::Output::Valid),
            Err(error) => Ok(Self::Output::Invalid(debug_string(error))),
        }
    }
}

export_function!(
    NotarizedTransactionV2StaticallyValidate
        as notarized_transaction_v2_statically_validate
);
export_jni_function!(
    NotarizedTransactionV2StaticallyValidate
        as notarizedTransactionV2StaticallyValidate
);
