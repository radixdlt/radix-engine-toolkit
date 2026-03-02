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

        let subintent = SerializableSubintentV2::from_native(
            &subintent,
            *network_id,
            (),
        )?;

        Ok(subintent)
    }
}

export_function!(SubintentV2Decompile as subintent_v2_decompile);
export_jni_function!(SubintentV2Decompile as subintentV2Decompile);
