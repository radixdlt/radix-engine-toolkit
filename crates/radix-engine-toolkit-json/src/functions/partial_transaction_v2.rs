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

//=============================
// Partial Transaction V2 Hash
//=============================

#[typeshare::typeshare]
pub type PartialTransactionV2HashInput = SerializablePartialTransactionV2;
#[typeshare::typeshare]
pub type PartialTransactionV2HashOutput = SerializableTransactionHash;

pub struct PartialTransactionV2Hash;
impl<'f> Function<'f> for PartialTransactionV2Hash {
    type Input = PartialTransactionV2HashInput;
    type Output = PartialTransactionV2HashOutput;

    fn handle(
        partial_transaction: Self::Input,
    ) -> Result<Self::Output, crate::error::InvocationHandlingError> {
        let network_id =
            *partial_transaction.root_subintent.intent_core.header.network_id;
        let partial_transaction =
            partial_transaction.to_native(network_id)?;
        let hash =
            radix_engine_toolkit::functions::transaction_v2::partial_transaction::hash(
                &partial_transaction,
            )
            .map_err(|error| {
                InvocationHandlingError::EncodeError(
                    debug_string(error),
                    debug_string(partial_transaction),
                )
            })?;
        Ok(hash.into())
    }
}

export_function!(PartialTransactionV2Hash as partial_transaction_v2_hash);
export_jni_function!(PartialTransactionV2Hash as partialTransactionV2Hash);

//================================
// Partial Transaction V2 Compile
//================================

#[typeshare::typeshare]
pub type PartialTransactionV2CompileInput = SerializablePartialTransactionV2;
#[typeshare::typeshare]
pub type PartialTransactionV2CompileOutput = SerializableBytes;

pub struct PartialTransactionV2Compile;
impl<'f> Function<'f> for PartialTransactionV2Compile {
    type Input = PartialTransactionV2CompileInput;
    type Output = PartialTransactionV2CompileOutput;

    fn handle(
        partial_transaction: Self::Input,
    ) -> Result<Self::Output, crate::error::InvocationHandlingError> {
        let network_id =
            *partial_transaction.root_subintent.intent_core.header.network_id;
        let partial_transaction =
            partial_transaction.to_native(network_id)?;
        let compile =
            radix_engine_toolkit::functions::transaction_v2::partial_transaction::to_payload_bytes(
                &partial_transaction,
            )
            .map_err(|error| {
                InvocationHandlingError::EncodeError(
                    debug_string(error),
                    debug_string(partial_transaction),
                )
            })?;
        Ok(compile.into())
    }
}

export_function!(PartialTransactionV2Compile as partial_transaction_v2_compile);
export_jni_function!(
    PartialTransactionV2Compile as partialTransactionV2Compile
);

//==================================
// Partial Transaction V2 Decompile
//==================================

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct PartialTransactionV2DecompileInput {
    pub compiled: SerializableBytes,
    pub network_id: SerializableU8,
}
#[typeshare::typeshare]
pub type PartialTransactionV2DecompileOutput =
    SerializablePartialTransactionV2;

pub struct PartialTransactionV2Decompile;
impl<'a> Function<'a> for PartialTransactionV2Decompile {
    type Input = PartialTransactionV2DecompileInput;
    type Output = PartialTransactionV2DecompileOutput;

    fn handle(
        PartialTransactionV2DecompileInput {
            compiled,
            network_id,
        }: Self::Input,
    ) -> Result<Self::Output, InvocationHandlingError> {
        let partial_transaction =
            radix_engine_toolkit::functions::transaction_v2::partial_transaction::from_payload_bytes(
                &**compiled,
            )
            .map_err(|error| {
                InvocationHandlingError::DecodeError(
                    debug_string(error),
                    debug_string(compiled),
                )
            })?;

        let partial_transaction =
            SerializablePartialTransactionV2::from_native(
                &partial_transaction,
                *network_id,
                (),
            )?;

        Ok(partial_transaction)
    }
}

export_function!(
    PartialTransactionV2Decompile as partial_transaction_v2_decompile
);
export_jni_function!(
    PartialTransactionV2Decompile as partialTransactionV2Decompile
);
