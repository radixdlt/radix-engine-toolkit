use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::error::Error;
use crate::export_request;
use crate::models::NotarizedTransaction;
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
        let compiled_notarized_intent: Vec<u8> = self.notarized_transaction.compile()?;

        Ok(CompileNotarizedTransactionIntentResponse {
            compiled_notarized_intent,
        })
    }
}

export_request!(CompileNotarizedTransactionIntentRequest as compile_notarized_transaction_intent);

// ======
// Tests
// ======

#[cfg(test)]
mod tests {
    // TODO: Unit tests for this request type
}
