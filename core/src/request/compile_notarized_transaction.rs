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
use crate::traits::{CompilableIntent, ValueRef};
use serializable::serializable;

// =================
// Model Definition
// =================

/// This function does the opposite of the compile_signed_transaction_intent function. This function
/// takes in a compiled signed transaction intent and decompiles it into its transaction intent and
/// signatures.
#[serializable]
pub struct CompileNotarizedTransactionRequest {
    #[serde(flatten)]
    pub notarized_intent: NotarizedTransaction,
}

/// The response from [`CompileNotarizedTransactionRequest`].
#[serializable]
pub struct CompileNotarizedTransactionResponse {
    #[serde_as(as = "serde_with::hex::Hex")]
    pub compiled_intent: Vec<u8>,
}

// ===============
// Implementation
// ===============

struct CompileNotarizedTransactionHandler;

impl Handler<CompileNotarizedTransactionRequest, CompileNotarizedTransactionResponse>
    for CompileNotarizedTransactionHandler
{
    fn pre_process(
        request: CompileNotarizedTransactionRequest,
    ) -> Result<CompileNotarizedTransactionRequest> {
        // Validate all `Value`s in the request. Ensure that:
        //     1. All addresses are of the network provided in the request.
        //     2. All single-type collections are of a single kind.
        request
            .borrow_values()
            .iter()
            .map(|value| {
                value.validate(Some(
                    request
                        .notarized_intent
                        .signed_intent
                        .intent
                        .header
                        .network_id,
                ))
            })
            .collect::<Result<Vec<_>>>()?;
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
    ) -> CompileNotarizedTransactionResponse {
        response
    }
}

impl ValueRef for CompileNotarizedTransactionRequest {
    fn borrow_values(&self) -> Vec<&crate::Value> {
        self.notarized_intent.borrow_values()
    }

    fn borrow_values_mut(&mut self) -> Vec<&mut crate::Value> {
        self.notarized_intent.borrow_values_mut()
    }
}
