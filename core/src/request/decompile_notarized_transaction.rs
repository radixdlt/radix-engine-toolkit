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
use crate::InstructionKind;
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

    #[serde_as(as = "serde_with::hex::Hex")]
    pub compiled_notarized_intent: Vec<u8>,
}

/// The response from [`DecompileNotarizedTransactionRequest`].
#[serializable]
pub struct DecompileNotarizedTransactionResponse {
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
    ) -> DecompileNotarizedTransactionResponse {
        for value in response.borrow_values_mut().iter_mut() {
            value.alias();
        }
        response
    }
}

impl ValueRef for DecompileNotarizedTransactionResponse {
    fn borrow_values(&self) -> Vec<&crate::Value> {
        self.notarized_intent.borrow_values()
    }

    fn borrow_values_mut(&mut self) -> Vec<&mut crate::Value> {
        self.notarized_intent.borrow_values_mut()
    }
}
