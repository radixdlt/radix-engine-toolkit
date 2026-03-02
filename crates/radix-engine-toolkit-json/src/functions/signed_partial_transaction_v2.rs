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
// Signed Partial Transaction V2 Hash
//====================================

#[typeshare::typeshare]
pub type SignedPartialTransactionV2HashInput =
    SerializableSignedPartialTransactionV2;
#[typeshare::typeshare]
pub type SignedPartialTransactionV2HashOutput = SerializableTransactionHash;

pub struct SignedPartialTransactionV2Hash;
impl<'f> Function<'f> for SignedPartialTransactionV2Hash {
    type Input = SignedPartialTransactionV2HashInput;
    type Output = SignedPartialTransactionV2HashOutput;

    fn handle(
        signed_partial: Self::Input,
    ) -> Result<Self::Output, crate::error::InvocationHandlingError> {
        let network_id = *signed_partial
            .partial_transaction
            .root_subintent
            .intent_core
            .header
            .network_id;
        let signed_partial = signed_partial.to_native(network_id)?;
        let hash =
            radix_engine_toolkit::functions::transaction_v2::signed_partial_transaction::hash(
                &signed_partial,
            )
            .map_err(|error| {
                InvocationHandlingError::EncodeError(
                    debug_string(error),
                    debug_string(signed_partial),
                )
            })?;
        Ok(hash.into())
    }
}

export_function!(
    SignedPartialTransactionV2Hash as signed_partial_transaction_v2_hash
);
export_jni_function!(
    SignedPartialTransactionV2Hash as signedPartialTransactionV2Hash
);

//=======================================
// Signed Partial Transaction V2 Compile
//=======================================

#[typeshare::typeshare]
pub type SignedPartialTransactionV2CompileInput =
    SerializableSignedPartialTransactionV2;
#[typeshare::typeshare]
pub type SignedPartialTransactionV2CompileOutput = SerializableBytes;

pub struct SignedPartialTransactionV2Compile;
impl<'f> Function<'f> for SignedPartialTransactionV2Compile {
    type Input = SignedPartialTransactionV2CompileInput;
    type Output = SignedPartialTransactionV2CompileOutput;

    fn handle(
        signed_partial: Self::Input,
    ) -> Result<Self::Output, crate::error::InvocationHandlingError> {
        let network_id = *signed_partial
            .partial_transaction
            .root_subintent
            .intent_core
            .header
            .network_id;
        let signed_partial = signed_partial.to_native(network_id)?;
        let compile =
            radix_engine_toolkit::functions::transaction_v2::signed_partial_transaction::to_payload_bytes(
                &signed_partial,
            )
            .map_err(|error| {
                InvocationHandlingError::EncodeError(
                    debug_string(error),
                    debug_string(signed_partial),
                )
            })?;
        Ok(compile.into())
    }
}

export_function!(
    SignedPartialTransactionV2Compile
        as signed_partial_transaction_v2_compile
);
export_jni_function!(
    SignedPartialTransactionV2Compile as signedPartialTransactionV2Compile
);

//=========================================
// Signed Partial Transaction V2 Decompile
//=========================================

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct SignedPartialTransactionV2DecompileInput {
    pub compiled: SerializableBytes,
    pub network_id: SerializableU8,
}
#[typeshare::typeshare]
pub type SignedPartialTransactionV2DecompileOutput =
    SerializableSignedPartialTransactionV2;

pub struct SignedPartialTransactionV2Decompile;
impl<'a> Function<'a> for SignedPartialTransactionV2Decompile {
    type Input = SignedPartialTransactionV2DecompileInput;
    type Output = SignedPartialTransactionV2DecompileOutput;

    fn handle(
        SignedPartialTransactionV2DecompileInput {
            compiled,
            network_id,
        }: Self::Input,
    ) -> Result<Self::Output, InvocationHandlingError> {
        let signed_partial =
            radix_engine_toolkit::functions::transaction_v2::signed_partial_transaction::from_payload_bytes(
                &**compiled,
            )
            .map_err(|error| {
                InvocationHandlingError::DecodeError(
                    debug_string(error),
                    debug_string(compiled),
                )
            })?;

        let signed_partial =
            SerializableSignedPartialTransactionV2::from_native(
                &signed_partial,
                *network_id,
                (),
            )?;

        Ok(signed_partial)
    }
}

export_function!(
    SignedPartialTransactionV2Decompile
        as signed_partial_transaction_v2_decompile
);
export_jni_function!(
    SignedPartialTransactionV2Decompile
        as signedPartialTransactionV2Decompile
);

//====================================================
// Signed Partial Transaction V2 Statically Validate
//====================================================

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct SignedPartialTransactionV2StaticallyValidateInput {
    pub signed_partial_transaction: SerializableSignedPartialTransactionV2,
    pub network_id: SerializableU8,
}

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
#[serde(tag = "kind", content = "value")]
pub enum SignedPartialTransactionV2StaticallyValidateOutput {
    Valid,
    Invalid(String),
}

pub struct SignedPartialTransactionV2StaticallyValidate;
impl<'a> Function<'a> for SignedPartialTransactionV2StaticallyValidate {
    type Input = SignedPartialTransactionV2StaticallyValidateInput;
    type Output = SignedPartialTransactionV2StaticallyValidateOutput;

    fn handle(
        SignedPartialTransactionV2StaticallyValidateInput {
            signed_partial_transaction,
            network_id,
        }: Self::Input,
    ) -> Result<Self::Output, InvocationHandlingError> {
        let signed_partial =
            signed_partial_transaction.to_native(*network_id)?;
        let network_definition =
            network_definition_from_network_id(*network_id);

        match radix_engine_toolkit::functions::transaction_v2::signed_partial_transaction::statically_validate(
            &signed_partial,
            &network_definition,
        ) {
            Ok(..) => Ok(Self::Output::Valid),
            Err(error) => Ok(Self::Output::Invalid(debug_string(error))),
        }
    }
}

export_function!(
    SignedPartialTransactionV2StaticallyValidate
        as signed_partial_transaction_v2_statically_validate
);
export_jni_function!(
    SignedPartialTransactionV2StaticallyValidate
        as signedPartialTransactionV2StaticallyValidate
);
