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

//============================
// Notarized Transaction Hash
//============================

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct NotarizedTransactionHashInput {
    pub notarized_transaction: SerializableNotarizedTransaction,
    pub network_id: SerializableU8,
}
pub type NotarizedTransactionHashOutput = SerializableHash;

pub struct NotarizedTransactionHash;
impl<'f> Function<'f> for NotarizedTransactionHash {
    type Input = NotarizedTransactionHashInput;
    type Output = NotarizedTransactionHashOutput;

    fn handle(
        NotarizedTransactionHashInput {
            notarized_transaction,
            network_id,
        }: Self::Input,
    ) -> Result<Self::Output, crate::error::InvocationHandlingError> {
        let notarized_transaction = notarized_transaction.to_native(*network_id)?;
        let hash =
            radix_engine_toolkit::functions::notarized_transaction::hash(&notarized_transaction)
                .map_err(|error| {
                    InvocationHandlingError::EncodeError(
                        debug_string(error),
                        debug_string(notarized_transaction),
                    )
                })?;
        Ok(hash.into())
    }
}

export_function!(NotarizedTransactionHash as notarized_transaction_hash);
export_jni_function!(NotarizedTransactionHash as notarizedTransactionHash);

//===============================
// Notarized Transaction Compile
//===============================

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct NotarizedTransactionCompileInput {
    pub notarized_transaction: SerializableNotarizedTransaction,
    pub network_id: SerializableU8,
}
pub type NotarizedTransactionCompileOutput = SerializableBytes;

pub struct NotarizedTransactionCompile;
impl<'f> Function<'f> for NotarizedTransactionCompile {
    type Input = NotarizedTransactionCompileInput;
    type Output = NotarizedTransactionCompileOutput;

    fn handle(
        NotarizedTransactionCompileInput {
            notarized_transaction,
            network_id,
        }: Self::Input,
    ) -> Result<Self::Output, crate::error::InvocationHandlingError> {
        let notarized_transaction = notarized_transaction.to_native(*network_id)?;
        let compile =
            radix_engine_toolkit::functions::notarized_transaction::compile(&notarized_transaction)
                .map_err(|error| {
                    InvocationHandlingError::EncodeError(
                        debug_string(error),
                        debug_string(notarized_transaction),
                    )
                })?;
        Ok(compile.into())
    }
}

export_function!(NotarizedTransactionCompile as notarized_transaction_compile);
export_jni_function!(NotarizedTransactionCompile as notarizedTransactionCompile);

//=================================
// Notarized Transaction Decompile
//=================================

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct NotarizedTransactionDecompileInput {
    pub compiled: SerializableBytes,
    pub network_id: SerializableU8,
    pub instructions_kind: SerializableInstructionsKind,
}
pub type NotarizedTransactionDecompileOutput = SerializableNotarizedTransaction;

pub struct NotarizedTransactionDecompile;
impl<'a> Function<'a> for NotarizedTransactionDecompile {
    type Input = NotarizedTransactionDecompileInput;
    type Output = NotarizedTransactionDecompileOutput;

    fn handle(
        NotarizedTransactionDecompileInput {
            compiled,
            network_id,
            instructions_kind,
        }: Self::Input,
    ) -> Result<Self::Output, InvocationHandlingError> {
        let notarized_transaction =
            radix_engine_toolkit::functions::notarized_transaction::decompile(&**compiled)
                .map_err(|error| {
                    InvocationHandlingError::EncodeError(
                        debug_string(error),
                        debug_string(compiled),
                    )
                })?;

        let notarized_transaction = SerializableNotarizedTransaction::from_native(
            &notarized_transaction,
            *network_id,
            instructions_kind,
        )?;

        Ok(notarized_transaction)
    }
}

export_function!(NotarizedTransactionDecompile as notarized_transaction_decompile);
export_jni_function!(NotarizedTransactionDecompile as notarizedTransactionDecompile);

//===========================================
// Notarized Transaction Statically Validate
//===========================================

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct NotarizedTransactionStaticallyValidateInput {
    pub notarized_transaction: SerializableNotarizedTransaction,
    pub network_id: SerializableU8,
    pub validation_config: SerializableValidationConfig,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
#[serde(tag = "kind", content = "value")]
pub enum NotarizedTransactionStaticallyValidateOutput {
    Valid,
    Invalid(String),
}

pub struct NotarizedTransactionStaticallyValidate;
impl<'a> Function<'a> for NotarizedTransactionStaticallyValidate {
    type Input = NotarizedTransactionStaticallyValidateInput;
    type Output = NotarizedTransactionStaticallyValidateOutput;

    fn handle(
        NotarizedTransactionStaticallyValidateInput {
            notarized_transaction,
            network_id,
            validation_config,
        }: Self::Input,
    ) -> Result<Self::Output, InvocationHandlingError> {
        let notarized_transaction = notarized_transaction.to_native(*network_id)?;
        let validation_config = validation_config.into();

        match radix_engine_toolkit::functions::notarized_transaction::statically_validate(
            &notarized_transaction,
            validation_config,
        ) {
            Ok(..) => Ok(Self::Output::Valid),
            Err(error) => Ok(Self::Output::Invalid(debug_string(error))),
        }
    }
}

export_function!(
    NotarizedTransactionStaticallyValidate as notarized_transaction_statically_validate
);
export_jni_function!(
    NotarizedTransactionStaticallyValidate as notarizedTransactionStaticallyValidate
);
