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
use crate::model::TransactionIntent;
use crate::traits::{CompilableIntent, Request, Validate};

// ==========================
// Request & Response Models
// ==========================

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompileTransactionIntentRequest {
    #[serde(flatten)]
    pub transaction_intent: TransactionIntent,
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompileTransactionIntentResponse {
    #[serde_as(as = "serde_with::hex::Hex")]
    pub compiled_intent: Vec<u8>,
}

// ===========
// Validation
// ===========

impl Validate for CompileTransactionIntentRequest {
    fn validate(&self) -> Result<(), Error> {
        self.transaction_intent.validate()?;
        Ok(())
    }
}

impl Validate for CompileTransactionIntentResponse {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

// =======================
// Request Implementation
// =======================

impl<'r> Request<'r, CompileTransactionIntentResponse> for CompileTransactionIntentRequest {
    fn handle_request(self) -> Result<CompileTransactionIntentResponse, Error> {
        let compiled_intent = self.transaction_intent.compile()?;

        Ok(CompileTransactionIntentResponse { compiled_intent })
    }
}

// ======
// Tests
// ======

#[cfg(test)]
mod tests {
    // TODO: Unit tests for this request type
}
