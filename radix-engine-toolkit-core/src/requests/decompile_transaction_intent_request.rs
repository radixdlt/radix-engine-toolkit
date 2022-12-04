use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::error::Error;
use crate::model::manifest_instructions::ManifestInstructionsKind;
use crate::model::TransactionIntent;
use crate::traits::{CompilableIntent, Request, Validate};

// ==========================
// Request & Response Models
// ==========================

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DecompileTransactionIntentRequest {
    /// Defines the output format that we would like the manifest to be in after this request is
    /// performed.
    pub manifest_instructions_output_format: ManifestInstructionsKind,

    /// The compiled intent which we wish to decompile.
    #[serde_as(as = "serde_with::hex::Hex")]
    pub compiled_intent: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DecompileTransactionIntentResponse {
    #[serde(flatten)]
    pub transaction_intent: TransactionIntent,
}

// ===========
// Validation
// ===========

impl Validate for DecompileTransactionIntentRequest {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

impl Validate for DecompileTransactionIntentResponse {
    fn validate(&self) -> Result<(), Error> {
        self.transaction_intent.validate()?;
        Ok(())
    }
}

// =======================
// Request Implementation
// =======================

impl<'r> Request<'r, DecompileTransactionIntentResponse> for DecompileTransactionIntentRequest {
    fn handle_request(self) -> Result<DecompileTransactionIntentResponse, Error> {
        let transaction_intent = TransactionIntent::decompile(
            &self.compiled_intent,
            self.manifest_instructions_output_format,
        )?;

        Ok(DecompileTransactionIntentResponse { transaction_intent })
    }
}

// ======
// Tests
// ======

#[cfg(test)]
mod tests {
    // TODO: Unit tests for this request type
}
