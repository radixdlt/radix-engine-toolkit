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
use crate::model::SignedTransactionIntent;
use crate::traits::{CompilableIntent, Request, Validate};

// ==========================
// Request & Response Models
// ==========================

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompileSignedTransactionIntentRequest {
    #[serde(flatten)]
    pub signed_intent: SignedTransactionIntent,
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompileSignedTransactionIntentResponse {
    #[serde_as(as = "serde_with::hex::Hex")]
    pub compiled_signed_intent: Vec<u8>,
}

// ===========
// Validation
// ===========

impl Validate for CompileSignedTransactionIntentRequest {
    fn validate(&self) -> Result<(), Error> {
        self.signed_intent.validate()?;
        Ok(())
    }
}

impl Validate for CompileSignedTransactionIntentResponse {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

// =======================
// Request Implementation
// =======================

impl<'r> Request<'r, CompileSignedTransactionIntentResponse>
    for CompileSignedTransactionIntentRequest
{
    fn handle_request(self) -> Result<CompileSignedTransactionIntentResponse, Error> {
        let compiled_signed_intent = self.signed_intent.compile()?;

        Ok(CompileSignedTransactionIntentResponse {
            compiled_signed_intent,
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
