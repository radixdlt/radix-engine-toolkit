use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::error::Error;
use crate::export_request;
use crate::models::SignedTransactionIntent;
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
        let compiled_signed_intent: Vec<u8> = self.signed_intent.compile()?;

        Ok(CompileSignedTransactionIntentResponse {
            compiled_signed_intent,
        })
    }
}

export_request!(CompileSignedTransactionIntentRequest as compile_signed_transaction_intent);

// ======
// Tests
// ======

#[cfg(test)]
mod tests {
    // TODO: Unit tests for this request type
}
