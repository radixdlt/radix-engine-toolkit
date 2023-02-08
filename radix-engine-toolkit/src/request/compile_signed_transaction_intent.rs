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
use crate::model::transaction::SignedTransactionIntent;
use crate::request::Handler;
use crate::traits::CompilableIntent;
use crate::{traverse_instruction, Instruction, InstructionList, ValueNetworkAggregatorVisitor};
use serializable::serializable;

// =================
// Model Definition
// =================

/// Takes in a raw transaction intent as well as its signatures and compiles it. This is useful when
/// a notary wishes to notarize a signed transaction intent.
#[serializable]
pub struct CompileSignedTransactionIntentRequest {
    /// The signed transaction intent to compile
    #[serde(flatten)]
    pub signed_intent: SignedTransactionIntent,
}

/// The response from [`CompileSignedTransactionIntentRequest`].
#[serializable]
pub struct CompileSignedTransactionIntentResponse {
    /// A byte array serialized as a hex string which represents the compiled signed transaction
    /// intent.
    #[schemars(with = "String")]
    #[schemars(regex(pattern = "[0-9a-fA-F]+"))]
    #[serde_as(as = "serde_with::hex::Hex")]
    pub compiled_intent: Vec<u8>,
}

// ===============
// Implementation
// ===============

pub struct CompileSignedTransactionIntentHandler;

impl Handler<CompileSignedTransactionIntentRequest, CompileSignedTransactionIntentResponse>
    for CompileSignedTransactionIntentHandler
{
    fn pre_process(
        mut request: CompileSignedTransactionIntentRequest,
    ) -> Result<CompileSignedTransactionIntentRequest> {
        // Visitors
        let mut network_aggregator_visitor = ValueNetworkAggregatorVisitor::default();

        // Instructions
        let instructions: &mut [Instruction] =
            match request.signed_intent.intent.manifest.instructions {
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
        let expected_network_id = request.signed_intent.intent.header.network_id;
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
        request: &CompileSignedTransactionIntentRequest,
    ) -> Result<CompileSignedTransactionIntentResponse> {
        request
            .signed_intent
            .compile()
            .map(|compiled_intent| CompileSignedTransactionIntentResponse { compiled_intent })
    }

    fn post_process(
        _: &CompileSignedTransactionIntentRequest,
        response: CompileSignedTransactionIntentResponse,
    ) -> Result<CompileSignedTransactionIntentResponse> {
        Ok(response)
    }
}
