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

//====================
// Signed Intent Hash
//====================

#[typeshare::typeshare]
pub type SignedIntentHashInput = SerializableSignedIntent;
#[typeshare::typeshare]
pub type SignedIntentHashOutput = SerializableTransactionHash;

pub struct SignedTransactionIntentHash;
impl<'f> Function<'f> for SignedTransactionIntentHash {
    type Input = SignedIntentHashInput;
    type Output = SignedIntentHashOutput;

    fn handle(
        signed_intent: Self::Input,
    ) -> Result<Self::Output, crate::error::InvocationHandlingError> {
        let signed_intent =
            signed_intent.to_native(*signed_intent.intent.header.network_id)?;
        let hash = radix_engine_toolkit::functions::transaction_v1::signed_intent::hash(
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

export_function!(SignedTransactionIntentHash as signed_intent_hash);
export_jni_function!(
    SignedTransactionIntentHash as SignedTransactionIntentHash
);

//=======================
// Signed Intent Compile
//=======================

#[typeshare::typeshare]
pub type SignedIntentCompileInput = SerializableSignedIntent;
#[typeshare::typeshare]
pub type SignedIntentCompileOutput = SerializableBytes;

pub struct SignedIntentCompile;
impl<'f> Function<'f> for SignedIntentCompile {
    type Input = SignedIntentCompileInput;
    type Output = SignedIntentCompileOutput;

    fn handle(
        signed_intent: Self::Input,
    ) -> Result<Self::Output, crate::error::InvocationHandlingError> {
        let signed_intent =
            signed_intent.to_native(*signed_intent.intent.header.network_id)?;
        let compile = radix_engine_toolkit::functions::transaction_v1::signed_intent::to_payload_bytes(
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

export_function!(SignedIntentCompile as signed_intent_compile);
export_jni_function!(SignedIntentCompile as signedIntentCompile);

//=========================
// Signed Intent Decompile
//=========================

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct SignedIntentDecompileInput {
    pub compiled: SerializableBytes,
    pub instructions_kind: SerializableInstructionsKind,
}
#[typeshare::typeshare]
pub type SignedIntentDecompileOutput = SerializableSignedIntent;

pub struct SignedIntentDecompile;
impl<'a> Function<'a> for SignedIntentDecompile {
    type Input = SignedIntentDecompileInput;
    type Output = SignedIntentDecompileOutput;

    fn handle(
        SignedIntentDecompileInput {
            compiled,
            instructions_kind,
        }: Self::Input,
    ) -> Result<Self::Output, InvocationHandlingError> {
        let signed_intent =
            radix_engine_toolkit::functions::transaction_v1::signed_intent::from_payload_bytes(
                &**compiled,
            )
            .map_err(|error| {
                InvocationHandlingError::DecodeError(
                    debug_string(error),
                    debug_string(&compiled),
                )
            })?;

        let signed_intent = SerializableSignedIntent::from_native(
            &signed_intent,
            signed_intent.intent.header.network_id,
            instructions_kind,
        )?;

        Ok(signed_intent)
    }
}

export_function!(SignedIntentDecompile as signed_intent_decompile);
export_jni_function!(SignedIntentDecompile as signedIntentDecompile);

//===================================
// Signed Intent Statically Validate
//===================================

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct SignedIntentStaticallyValidateInput {
    pub signed_intent: SerializableSignedIntent,
}

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
#[serde(tag = "kind", content = "value")]
pub enum SignedIntentStaticallyValidateOutput {
    Valid,
    Invalid(String),
}

pub struct SignedIntentStaticallyValidate;
impl<'a> Function<'a> for SignedIntentStaticallyValidate {
    type Input = SignedIntentStaticallyValidateInput;
    type Output = SignedIntentStaticallyValidateOutput;

    fn handle(
        SignedIntentStaticallyValidateInput { signed_intent }: Self::Input,
    ) -> Result<Self::Output, InvocationHandlingError> {
        let network_id = *signed_intent.intent.header.network_id;
        let signed_intent = signed_intent.to_native(network_id)?;
        let network_definition = network_definition_from_network_id(network_id);

        match radix_engine_toolkit::functions::transaction_v1::signed_intent::statically_validate(
            &signed_intent,
            &network_definition,
        ) {
            Ok(..) => Ok(Self::Output::Valid),
            Err(error) => Ok(Self::Output::Invalid(debug_string(error))),
        }
    }
}

export_function!(
    SignedIntentStaticallyValidate as signed_intent_statically_validate
);
export_jni_function!(
    SignedIntentStaticallyValidate as signedIntentStaticallyValidate
);
