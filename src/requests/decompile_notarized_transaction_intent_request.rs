use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::error::Error;
use crate::export_request;
use crate::models::manifest_instructions::ManifestInstructionsKind;
use crate::models::NotarizedTransaction;
use crate::traits::{CompilableIntent, Request, Validate};
use crate::validation::validate_notarized_transaction;

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
        validate_notarized_transaction(&self.notarized_transaction)?;
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
        let notarized_transaction: NotarizedTransaction = NotarizedTransaction::decompile(
            &self.compiled_notarized_intent,
            self.manifest_instructions_output_format,
        )?;

        Ok(DecompileNotarizedTransactionIntentResponse {
            notarized_transaction,
        })
    }
}

export_request!(
    DecompileNotarizedTransactionIntentRequest as decompile_notarized_transaction_intent
);

// ======
// Tests
// ======

#[cfg(test)]
mod tests {
    // TODO: Unit tests for this request type
}
