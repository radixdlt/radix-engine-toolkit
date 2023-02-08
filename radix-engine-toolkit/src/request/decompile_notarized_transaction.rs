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

use crate::error::Result;
use crate::model::transaction::NotarizedTransaction;
use crate::request::Handler;
use crate::traits::CompilableIntent;
use crate::{
    traverse_instruction, Instruction, InstructionKind, InstructionList, ValueAliasingVisitor,
};
use serializable::serializable;

// =================
// Model Definition
// =================

/// This function does the opposite of the compile_notarized_transaction_intent function. This
/// function takes in a compiled notarized transaction intent and decompiles it into its signed
/// transaction intent and notary signature.
#[serializable]
pub struct DecompileNotarizedTransactionRequest {
    /// Defines the output format that we would like the manifest to be in after this request is
    /// performed.
    pub instructions_output_kind: InstructionKind,

    /// A byte array serialized as a hex string which represents the compiled notarized transaction
    /// intent to decompile.
    #[schemars(with = "String")]
    #[schemars(regex(pattern = "[0-9a-fA-F]+"))]
    #[serde_as(as = "serde_with::hex::Hex")]
    pub compiled_notarized_intent: Vec<u8>,
}

/// The response from [`DecompileNotarizedTransactionRequest`].
#[serializable]
pub struct DecompileNotarizedTransactionResponse {
    /// The decompiled notarized transaction intent where the instructions are in the format
    /// specified in the request.
    #[serde(flatten)]
    pub notarized_intent: NotarizedTransaction,
}

// ===============
// Implementation
// ===============

pub struct DecompileNotarizedTransactionHandler;

impl Handler<DecompileNotarizedTransactionRequest, DecompileNotarizedTransactionResponse>
    for DecompileNotarizedTransactionHandler
{
    fn pre_process(
        request: DecompileNotarizedTransactionRequest,
    ) -> Result<DecompileNotarizedTransactionRequest> {
        Ok(request)
    }

    fn handle(
        request: &DecompileNotarizedTransactionRequest,
    ) -> Result<DecompileNotarizedTransactionResponse> {
        NotarizedTransaction::decompile(
            &request.compiled_notarized_intent,
            request.instructions_output_kind,
        )
        .map(|notarized_intent| DecompileNotarizedTransactionResponse { notarized_intent })
    }

    fn post_process(
        _: &DecompileNotarizedTransactionRequest,
        mut response: DecompileNotarizedTransactionResponse,
    ) -> Result<DecompileNotarizedTransactionResponse> {
        // Visitors
        let mut aliasing_visitor = ValueAliasingVisitor::default();

        // Instructions
        let instructions: &mut [Instruction] = match response
            .notarized_intent
            .signed_intent
            .intent
            .manifest
            .instructions
        {
            InstructionList::Parsed(ref mut instructions) => instructions,
            InstructionList::String(..) => &mut [],
        };

        // Traverse instructions with visitors
        instructions
            .iter_mut()
            .map(|instruction| {
                traverse_instruction(instruction, &mut [&mut aliasing_visitor], &mut [])
            })
            .collect::<Result<Vec<_>>>()?;

        // The aliasing visitor performs all of the modifications in place as it meets them. Nothing
        // else needs to be done here.

        Ok(response)
    }
}
