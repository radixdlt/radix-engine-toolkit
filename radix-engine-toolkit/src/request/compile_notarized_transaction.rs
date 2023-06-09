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

use super::traits::Handler;
use crate::error::Result;
use crate::model::instruction::Instruction;
use crate::model::transaction::{InstructionList, NotarizedTransaction};
use crate::traits::CompilableIntent;
use crate::visitor::{traverse_instruction, ValueNetworkAggregatorVisitor};
use toolkit_derive::serializable;

// =================
// Model Definition
// =================

/// This function does the opposite of the compile_signed_transaction_intent function. This function
/// takes in a compiled signed transaction intent and decompiles it into its transaction intent and
/// signatures.
#[serializable]
pub struct CompileNotarizedTransactionRequest {
    /// The notarized transaction intent to compile
    #[serde(flatten)]
    pub notarized_intent: NotarizedTransaction,
}

/// The response from [`CompileNotarizedTransactionRequest`].
#[serializable]
pub struct CompileNotarizedTransactionResponse {
    /// A byte array serialized as a hex string which represents the compiled notarized transaction
    /// intent.
    #[schemars(with = "String")]
    #[schemars(regex(pattern = "[0-9a-fA-F]+"))]
    #[serde_as(as = "serde_with::hex::Hex")]
    pub compiled_intent: Vec<u8>,
}

// ===============
// Implementation
// ===============

pub struct CompileNotarizedTransactionHandler;

impl Handler<CompileNotarizedTransactionRequest, CompileNotarizedTransactionResponse>
    for CompileNotarizedTransactionHandler
{
    fn pre_process(
        mut request: CompileNotarizedTransactionRequest,
    ) -> Result<CompileNotarizedTransactionRequest> {
        // Visitors
        let mut network_aggregator_visitor = ValueNetworkAggregatorVisitor::default();

        // Instructions
        let instructions: &mut [Instruction] = match request
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
                traverse_instruction(instruction, &mut [&mut network_aggregator_visitor], &mut [])
            })
            .collect::<Result<Vec<_>>>()?;

        // Check for network mismatches
        let expected_network_id = request
            .notarized_intent
            .signed_intent
            .intent
            .header
            .network_id;
        if let Some(network_id) = network_aggregator_visitor
            .0
            .iter()
            .find(|network_id| **network_id != expected_network_id)
        {
            return Err(crate::error::Error::NetworkMismatchError {
                found: *network_id,
                expected: expected_network_id,
            });
        }
        Ok(request)
    }

    fn handle(
        request: &CompileNotarizedTransactionRequest,
    ) -> Result<CompileNotarizedTransactionResponse> {
        request
            .notarized_intent
            .compile()
            .map(|compiled_intent| CompileNotarizedTransactionResponse { compiled_intent })
    }

    fn post_process(
        _: &CompileNotarizedTransactionRequest,
        response: CompileNotarizedTransactionResponse,
    ) -> Result<CompileNotarizedTransactionResponse> {
        Ok(response)
    }
}