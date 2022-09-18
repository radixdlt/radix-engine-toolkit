use crate::error::Error;
use crate::export_handler;
use crate::models::manifest::ManifestInstructionsKind;
use crate::models::serde::TransactionIntent;
use crate::traits::Validate;
use crate::validation::validate_transaction_intent;
use scrypto::prelude::scrypto_decode;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

// ==========================
// Request & Response Models
// ==========================

#[derive(Serialize, Deserialize, Clone)]
pub struct DecompileTransactionIntentRequest {
    /// Defines the output format that we would like the manifest to be in after this request is
    /// performed.
    pub manifest_instructions_output_format: ManifestInstructionsKind,

    /// The compiled intent which we wish to decompile.
    #[serde(with = "hex::serde")]
    pub compiled_intent: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone)]
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
        validate_transaction_intent(&self.transaction_intent)?;
        Ok(())
    }
}

// ========
// Handler
// ========

pub fn handle_decompile_transaction_intent(
    request: DecompileTransactionIntentRequest,
) -> Result<DecompileTransactionIntentResponse, Error> {
    let transaction_intent: TransactionIntent =
        scrypto_decode::<transaction::model::TransactionIntent>(&request.compiled_intent)?
            .try_into()?;
    let transaction_intent: TransactionIntent = transaction_intent
        .convert_manifest_instructions_kind(request.manifest_instructions_output_format)?;

    let response: DecompileTransactionIntentResponse =
        DecompileTransactionIntentResponse { transaction_intent };

    Ok(response)
}

export_handler!(
    handle_decompile_transaction_intent(DecompileTransactionIntentRequest)
        as decompile_transaction_intent
);

// ======
// Tests
// ======

#[cfg(test)]
mod tests {
    // TODO: Unit tests for this request type
}
