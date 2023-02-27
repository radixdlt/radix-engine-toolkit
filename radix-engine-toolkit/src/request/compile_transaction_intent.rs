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
use crate::model::transaction::TransactionIntent;
use crate::request::Handler;
use crate::traits::CompilableIntent;
use crate::{traverse_instruction, Instruction, InstructionList, ValueNetworkAggregatorVisitor};
use toolkit_derive::serializable;

// =================
// Model Definition
// =================

/// Takes a transaction intent and compiles it by SBOR encoding it and returning it back to the
/// caller. This is mainly useful when creating a transaction.
#[serializable]
pub struct CompileTransactionIntentRequest {
    /// The transaction intent to compile.
    #[serde(flatten)]
    pub transaction_intent: TransactionIntent,
}

/// The response from [`CompileTransactionIntentRequest`].
#[serializable]
pub struct CompileTransactionIntentResponse {
    /// A byte array serialized as a hex string which represents the compiled transaction intent.
    #[schemars(with = "String")]
    #[schemars(regex(pattern = "[0-9a-fA-F]+"))]
    #[serde_as(as = "serde_with::hex::Hex")]
    pub compiled_intent: Vec<u8>,
}

// ===============
// Implementation
// ===============

pub struct CompileTransactionIntentHandler;

impl Handler<CompileTransactionIntentRequest, CompileTransactionIntentResponse>
    for CompileTransactionIntentHandler
{
    fn pre_process(
        mut request: CompileTransactionIntentRequest,
    ) -> Result<CompileTransactionIntentRequest> {
        // Visitors
        let mut network_aggregator_visitor = ValueNetworkAggregatorVisitor::default();

        // Instructions
        let instructions: &mut [Instruction] =
            match request.transaction_intent.manifest.instructions {
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
        let expected_network_id = request.transaction_intent.header.network_id;
        if let Some(network_id) = network_aggregator_visitor
            .0
            .iter()
            .find(|network_id| **network_id != expected_network_id)
        {
            return Err(crate::Error::NetworkMismatchError {
                found: *network_id,
                expected: expected_network_id,
            });
        }
        Ok(request)
    }

    fn handle(
        request: &CompileTransactionIntentRequest,
    ) -> Result<CompileTransactionIntentResponse> {
        request
            .transaction_intent
            .compile()
            .map(|compiled_intent| CompileTransactionIntentResponse { compiled_intent })
    }

    fn post_process(
        _: &CompileTransactionIntentRequest,
        response: CompileTransactionIntentResponse,
    ) -> Result<CompileTransactionIntentResponse> {
        Ok(response)
    }
}
