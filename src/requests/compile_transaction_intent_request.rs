use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::error::Error;
use crate::export_request;
use crate::models::TransactionIntent;
use crate::traits::{CompilableIntent, Request, Validate};
use crate::validation::validate_transaction_intent;

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
        validate_transaction_intent(&self.transaction_intent)?;
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
        let compiled_intent: Vec<u8> = self.transaction_intent.compile()?;

        Ok(CompileTransactionIntentResponse { compiled_intent })
    }
}

export_request!(CompileTransactionIntentRequest as compile_transaction_intent);

// ======
// Tests
// ======

#[cfg(test)]
mod tests {
    // TODO: Unit tests for this request type
}
