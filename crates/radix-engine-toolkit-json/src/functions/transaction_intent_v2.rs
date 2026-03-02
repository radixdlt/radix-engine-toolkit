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
        let network_id =
            *transaction_intent.root_intent_core.header.network_id;
        let transaction_intent =
            transaction_intent.to_native(network_id)?;
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
        let network_id =
            *transaction_intent.root_intent_core.header.network_id;
        let transaction_intent =
            transaction_intent.to_native(network_id)?;
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

export_function!(
    TransactionIntentV2Compile as transaction_intent_v2_compile
);
export_jni_function!(
    TransactionIntentV2Compile as transactionIntentV2Compile
);

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

        let transaction_intent =
            SerializableTransactionIntentV2::from_native(
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
