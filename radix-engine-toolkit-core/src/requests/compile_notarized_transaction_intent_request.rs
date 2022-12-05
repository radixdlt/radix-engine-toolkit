// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at

//   http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::error::Error;
use crate::model::NotarizedTransaction;
use crate::traits::{CompilableIntent, Request, Validate};

// ==========================
// Request & Response Models
// ==========================

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompileNotarizedTransactionIntentRequest {
    #[serde(flatten)]
    pub notarized_transaction: NotarizedTransaction,
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompileNotarizedTransactionIntentResponse {
    #[serde_as(as = "serde_with::hex::Hex")]
    pub compiled_notarized_intent: Vec<u8>,
}

// ===========
// Validation
// ===========

impl Validate for CompileNotarizedTransactionIntentRequest {
    fn validate(&self) -> Result<(), Error> {
        self.notarized_transaction.validate()?;
        Ok(())
    }
}

impl Validate for CompileNotarizedTransactionIntentResponse {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

// =======================
// Request Implementation
// =======================

impl<'r> Request<'r, CompileNotarizedTransactionIntentResponse>
    for CompileNotarizedTransactionIntentRequest
{
    fn handle_request(self) -> Result<CompileNotarizedTransactionIntentResponse, Error> {
        let compiled_notarized_intent = self.notarized_transaction.compile()?;

        Ok(CompileNotarizedTransactionIntentResponse {
            compiled_notarized_intent,
        })
    }
}

// ======
// Tests
// ======

#[cfg(test)]
mod tests {
    // TODO: Unit tests for this request type
}
