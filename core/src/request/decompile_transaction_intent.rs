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
use crate::traits::{CompilableIntent, ValueRef};
use crate::InstructionKind;
use serializable::serializable;

// =================
// Model Definition
// =================

/// This function does the opposite of the compile_transaction_intent function. It takes in a
/// compiled transaction intent and decompiles it into its human-readable / machine-readable format.
#[serializable]
pub struct DecompileTransactionIntentRequest {
    /// Defines the output format that we would like the manifest to be in after this request is
    /// performed.
    pub instructions_output_kind: InstructionKind,

    #[serde_as(as = "serde_with::hex::Hex")]
    pub compiled_intent: Vec<u8>,
}

/// The response from [`DecompileTransactionIntentRequest`].
#[serializable]
pub struct DecompileTransactionIntentResponse {
    #[serde(flatten)]
    pub transaction_intent: TransactionIntent,
}

// ===============
// Implementation
// ===============

pub struct DecompileTransactionIntentHandler;

impl Handler<DecompileTransactionIntentRequest, DecompileTransactionIntentResponse>
    for DecompileTransactionIntentHandler
{
    fn pre_process(
        request: DecompileTransactionIntentRequest,
    ) -> Result<DecompileTransactionIntentRequest> {
        Ok(request)
    }

    fn handle(
        request: &DecompileTransactionIntentRequest,
    ) -> Result<DecompileTransactionIntentResponse> {
        TransactionIntent::decompile(&request.compiled_intent, request.instructions_output_kind)
            .map(|transaction_intent| DecompileTransactionIntentResponse { transaction_intent })
    }

    fn post_process(
        _: &DecompileTransactionIntentRequest,
        mut response: DecompileTransactionIntentResponse,
    ) -> DecompileTransactionIntentResponse {
        for value in response.borrow_values_mut().iter_mut() {
            value.alias();
        }
        response
    }
}

impl ValueRef for DecompileTransactionIntentResponse {
    fn borrow_values(&self) -> Vec<&crate::Value> {
        self.transaction_intent.borrow_values()
    }

    fn borrow_values_mut(&mut self) -> Vec<&mut crate::Value> {
        self.transaction_intent.borrow_values_mut()
    }
}
