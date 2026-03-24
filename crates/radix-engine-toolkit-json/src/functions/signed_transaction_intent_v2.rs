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

//========================================
// Signed Transaction Intent V2 Hash
//========================================

#[typeshare::typeshare]
pub type SignedTransactionIntentV2HashInput =
    SerializableSignedTransactionIntentV2;
#[typeshare::typeshare]
pub type SignedTransactionIntentV2HashOutput = SerializableTransactionHash;

pub struct SignedTransactionIntentV2Hash;
impl<'f> Function<'f> for SignedTransactionIntentV2Hash {
    type Input = SignedTransactionIntentV2HashInput;
    type Output = SignedTransactionIntentV2HashOutput;

    fn handle(
        signed_intent: Self::Input,
    ) -> Result<Self::Output, crate::error::InvocationHandlingError> {
        let network_id = *signed_intent
            .transaction_intent
            .root_intent_core
            .header
            .network_id;
        let signed_intent = signed_intent.to_native(network_id)?;
        let hash =
            radix_engine_toolkit::functions::transaction_v2::signed_transaction_intent::hash(
                &signed_intent,
            )
            .map_err(|error| {
                InvocationHandlingError::EncodeError(
                    debug_string(error),
                    debug_string(signed_intent),
                )
            })?;
        Ok(hash.into())
    }
}

export_function!(
    SignedTransactionIntentV2Hash as signed_transaction_intent_v2_hash
);
export_jni_function!(
    SignedTransactionIntentV2Hash as signedTransactionIntentV2Hash
);

//===========================================
// Signed Transaction Intent V2 Compile
//===========================================

#[typeshare::typeshare]
pub type SignedTransactionIntentV2CompileInput =
    SerializableSignedTransactionIntentV2;
#[typeshare::typeshare]
pub type SignedTransactionIntentV2CompileOutput = SerializableBytes;

pub struct SignedTransactionIntentV2Compile;
impl<'f> Function<'f> for SignedTransactionIntentV2Compile {
    type Input = SignedTransactionIntentV2CompileInput;
    type Output = SignedTransactionIntentV2CompileOutput;

    fn handle(
        signed_intent: Self::Input,
    ) -> Result<Self::Output, crate::error::InvocationHandlingError> {
        let network_id = *signed_intent
            .transaction_intent
            .root_intent_core
            .header
            .network_id;
        let signed_intent = signed_intent.to_native(network_id)?;
        let compile =
            radix_engine_toolkit::functions::transaction_v2::signed_transaction_intent::to_payload_bytes(
                &signed_intent,
            )
            .map_err(|error| {
                InvocationHandlingError::EncodeError(
                    debug_string(error),
                    debug_string(signed_intent),
                )
            })?;
        Ok(compile.into())
    }
}

export_function!(
    SignedTransactionIntentV2Compile as signed_transaction_intent_v2_compile
);
export_jni_function!(
    SignedTransactionIntentV2Compile as signedTransactionIntentV2Compile
);

//=============================================
// Signed Transaction Intent V2 Decompile
//=============================================

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct SignedTransactionIntentV2DecompileInput {
    pub compiled: SerializableBytes,
    pub network_id: SerializableU8,
}
#[typeshare::typeshare]
pub type SignedTransactionIntentV2DecompileOutput =
    SerializableSignedTransactionIntentV2;

pub struct SignedTransactionIntentV2Decompile;
impl<'a> Function<'a> for SignedTransactionIntentV2Decompile {
    type Input = SignedTransactionIntentV2DecompileInput;
    type Output = SignedTransactionIntentV2DecompileOutput;

    fn handle(
        SignedTransactionIntentV2DecompileInput {
            compiled,
            network_id,
        }: Self::Input,
    ) -> Result<Self::Output, InvocationHandlingError> {
        let signed_intent =
            radix_engine_toolkit::functions::transaction_v2::signed_transaction_intent::from_payload_bytes(
                &**compiled,
            )
            .map_err(|error| {
                InvocationHandlingError::DecodeError(
                    debug_string(error),
                    debug_string(compiled),
                )
            })?;

        let signed_intent = SerializableSignedTransactionIntentV2::from_native(
            &signed_intent,
            *network_id,
            (),
        )?;

        Ok(signed_intent)
    }
}

export_function!(
    SignedTransactionIntentV2Decompile
        as signed_transaction_intent_v2_decompile
);
export_jni_function!(
    SignedTransactionIntentV2Decompile as signedTransactionIntentV2Decompile
);
