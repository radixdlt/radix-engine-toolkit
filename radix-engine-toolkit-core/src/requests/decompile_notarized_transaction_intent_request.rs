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

use crate::model::manifest_instructions::ManifestInstructionsKind;
use crate::model::NotarizedTransaction;
use crate::traits::{CompilableIntent, Request, Validate};

// ==========================
// Request & Response Models
// ==========================

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DecompileNotarizedTransactionIntentRequest {
    pub manifest_instructions_output_format: ManifestInstructionsKind,
    #[serde_as(as = "serde_with::hex::Hex")]
    pub compiled_notarized_intent: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DecompileNotarizedTransactionIntentResponse {
    #[serde(flatten)]
    pub notarized_transaction: NotarizedTransaction,
}

// ===========
// Validation
// ===========

impl Validate for DecompileNotarizedTransactionIntentRequest {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

impl Validate for DecompileNotarizedTransactionIntentResponse {
    fn validate(&self) -> Result<(), Error> {
        self.notarized_transaction.validate()?;
        Ok(())
    }
}

// =======================
// Request Implementation
// =======================

impl<'r> Request<'r, DecompileNotarizedTransactionIntentResponse>
    for DecompileNotarizedTransactionIntentRequest
{
    fn handle_request(self) -> Result<DecompileNotarizedTransactionIntentResponse, Error> {
        let notarized_transaction = NotarizedTransaction::decompile(
            &self.compiled_notarized_intent,
            self.manifest_instructions_output_format,
        )?;

        Ok(DecompileNotarizedTransactionIntentResponse {
            notarized_transaction,
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
