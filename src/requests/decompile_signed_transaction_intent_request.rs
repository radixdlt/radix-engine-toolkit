use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::error::Error;
use crate::export_request;
use crate::models::manifest_instructions::ManifestInstructionsKind;
use crate::models::SignedTransactionIntent;
use crate::traits::{CompilableIntent, Request, Validate};

// ==========================
// Request & Response Models
// ==========================

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DecompileSignedTransactionIntentRequest {
    pub manifest_instructions_output_format: ManifestInstructionsKind,

    #[serde_as(as = "serde_with::hex::Hex")]
    pub compiled_signed_intent: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DecompileSignedTransactionIntentResponse {
    #[serde(flatten)]
    pub signed_intent: SignedTransactionIntent,
}

// ===========
// Validation
// ===========

impl Validate for DecompileSignedTransactionIntentRequest {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

impl Validate for DecompileSignedTransactionIntentResponse {
    fn validate(&self) -> Result<(), Error> {
        self.signed_intent.validate()?;
        Ok(())
    }
}

// =======================
// Request Implementation
// =======================

impl<'r> Request<'r, DecompileSignedTransactionIntentResponse>
    for DecompileSignedTransactionIntentRequest
{
    fn handle_request(self) -> Result<DecompileSignedTransactionIntentResponse, Error> {
        let signed_intent: SignedTransactionIntent = SignedTransactionIntent::decompile(
            &self.compiled_signed_intent,
            self.manifest_instructions_output_format,
        )?;

        Ok(DecompileSignedTransactionIntentResponse { signed_intent })
    }
}

export_request!(DecompileSignedTransactionIntentRequest as decompile_signed_transaction_intent);

// ======
// Tests
// ======

#[cfg(test)]
mod tests {
    // TODO: Unit tests for this request type
}
