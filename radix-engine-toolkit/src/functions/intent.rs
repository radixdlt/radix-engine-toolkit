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

//=============
// Intent Hash
//=============

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct IntentHashInput {
    pub intent: SerializableIntent,
    pub network_id: SerializableU8,
}
pub type IntentHashOutput = SerializableHash;

pub struct IntentHash;
impl<'f> Function<'f> for IntentHash {
    type Input = IntentHashInput;
    type Output = IntentHashOutput;

    fn handle(
        IntentHashInput { intent, network_id }: Self::Input,
    ) -> Result<Self::Output, crate::error::InvocationHandlingError> {
        let intent = intent.to_native(*network_id)?;
        let hash =
            radix_engine_toolkit_core::functions::intent::hash(&intent).map_err(|error| {
                InvocationHandlingError::EncodeError(debug_string(error), debug_string(intent))
            })?;
        Ok(hash.into())
    }
}

export_function!(IntentHash as intent_hash);
export_jni_function!(IntentHash as intentHash);

//================
// Intent Compile
//================

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct IntentCompileInput {
    pub intent: SerializableIntent,
    pub network_id: SerializableU8,
}
pub type IntentCompileOutput = SerializableBytes;

pub struct IntentCompile;
impl<'f> Function<'f> for IntentCompile {
    type Input = IntentCompileInput;
    type Output = IntentCompileOutput;

    fn handle(
        IntentCompileInput { intent, network_id }: Self::Input,
    ) -> Result<Self::Output, crate::error::InvocationHandlingError> {
        let intent = intent.to_native(*network_id)?;
        let compile =
            radix_engine_toolkit_core::functions::intent::compile(&intent).map_err(|error| {
                InvocationHandlingError::EncodeError(debug_string(error), debug_string(intent))
            })?;
        Ok(compile.into())
    }
}

export_function!(IntentCompile as intent_compile);
export_jni_function!(IntentCompile as intentCompile);

//==================
// Intent Decompile
//==================

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct IntentDecompileInput {
    pub compiled: SerializableBytes,
    pub network_id: SerializableU8,
    pub instructions_kind: SerializableInstructionsKind,
}
pub type IntentDecompileOutput = SerializableIntent;

pub struct IntentDecompile;
impl<'a> Function<'a> for IntentDecompile {
    type Input = IntentDecompileInput;
    type Output = IntentDecompileOutput;

    fn handle(
        IntentDecompileInput {
            compiled,
            network_id,
            instructions_kind,
        }: Self::Input,
    ) -> Result<Self::Output, InvocationHandlingError> {
        let intent = radix_engine_toolkit_core::functions::intent::decompile(&**compiled).map_err(
            |error| {
                InvocationHandlingError::EncodeError(debug_string(error), debug_string(compiled))
            },
        )?;

        let intent = SerializableIntent::from_native(&intent, *network_id, instructions_kind)?;

        Ok(intent)
    }
}

export_function!(IntentDecompile as intent_decompile);
export_jni_function!(IntentDecompile as intentDecompile);

//============================
// Intent Statically Validate
//============================

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct IntentStaticallyValidateInput {
    pub intent: SerializableIntent,
    pub network_id: SerializableU8,
    pub validation_config: SerializableValidationConfig,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
#[serde(tag = "kind", content = "value")]
pub enum IntentStaticallyValidateOutput {
    Valid,
    Invalid(String),
}

pub struct IntentStaticallyValidate;
impl<'a> Function<'a> for IntentStaticallyValidate {
    type Input = IntentStaticallyValidateInput;
    type Output = IntentStaticallyValidateOutput;

    fn handle(
        IntentStaticallyValidateInput {
            intent,
            network_id,
            validation_config,
        }: Self::Input,
    ) -> Result<Self::Output, InvocationHandlingError> {
        let intent = intent.to_native(*network_id)?;
        let validation_config = validation_config.into();

        match radix_engine_toolkit_core::functions::intent::statically_validate(
            &intent,
            validation_config,
        ) {
            Ok(..) => Ok(Self::Output::Valid),
            Err(error) => Ok(Self::Output::Invalid(debug_string(error))),
        }
    }
}

export_function!(IntentStaticallyValidate as intent_statically_validate);
export_jni_function!(IntentStaticallyValidate as intentStaticallyValidate);
