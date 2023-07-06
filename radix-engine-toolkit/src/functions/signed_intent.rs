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

//====================
// Signed Intent Hash
//====================

pub type SignedIntentHashInput = SerializableSignedIntent;
pub type SignedIntentHashOutput = SerializableHash;

pub struct SignedIntentHash;
impl<'f> Function<'f> for SignedIntentHash {
    type Input = SignedIntentHashInput;
    type Output = SignedIntentHashOutput;

    fn handle(
        signed_intent: Self::Input,
    ) -> Result<Self::Output, crate::error::InvocationHandlingError> {
        let signed_intent = signed_intent.to_native(*signed_intent.intent.header.network_id)?;
        let hash = radix_engine_toolkit_core::functions::signed_intent::hash(&signed_intent)
            .map_err(|error| {
                InvocationHandlingError::EncodeError(
                    debug_string(error),
                    debug_string(signed_intent),
                )
            })?;
        Ok(hash.into())
    }
}

export_function!(SignedIntentHash as signed_intent_hash);
export_jni_function!(SignedIntentHash as signedIntentHash);

//=======================
// Signed Intent Compile
//=======================

pub type SignedIntentCompileInput = SerializableSignedIntent;
pub type SignedIntentCompileOutput = SerializableBytes;

pub struct SignedIntentCompile;
impl<'f> Function<'f> for SignedIntentCompile {
    type Input = SignedIntentCompileInput;
    type Output = SignedIntentCompileOutput;

    fn handle(
        signed_intent: Self::Input,
    ) -> Result<Self::Output, crate::error::InvocationHandlingError> {
        let signed_intent = signed_intent.to_native(*signed_intent.intent.header.network_id)?;
        let compile = radix_engine_toolkit_core::functions::signed_intent::compile(&signed_intent)
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

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct SignedIntentDecompileInput {
    pub compiled: SerializableBytes,
    pub instructions_kind: SerializableInstructionsKind,
}
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
        let signed_intent = radix_engine_toolkit_core::functions::signed_intent::decompile(
            &**compiled,
        )
        .map_err(|error| {
            InvocationHandlingError::EncodeError(debug_string(error), debug_string(compiled))
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

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct SignedIntentStaticallyValidateInput {
    pub signed_intent: SerializableSignedIntent,
    pub validation_config: SerializableValidationConfig,
}

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
        SignedIntentStaticallyValidateInput {
            signed_intent,
            validation_config,
        }: Self::Input,
    ) -> Result<Self::Output, InvocationHandlingError> {
        let signed_intent = signed_intent.to_native(*signed_intent.intent.header.network_id)?;
        let validation_config = validation_config.into();

        match radix_engine_toolkit_core::functions::signed_intent::statically_validate(
            &signed_intent,
            validation_config,
        ) {
            Ok(..) => Ok(Self::Output::Valid),
            Err(error) => Ok(Self::Output::Invalid(debug_string(error))),
        }
    }
}

export_function!(SignedIntentStaticallyValidate as signed_intent_statically_validate);
export_jni_function!(SignedIntentStaticallyValidate as signedIntentStaticallyValidate);
