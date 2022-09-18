use crate::error::Error;
use crate::export_handler;
use crate::models::manifest::ManifestInstructionsKind;
use crate::models::serde::NotarizedTransaction;
use crate::traits::Validate;
use crate::validation::validate_notarized_transaction;
use scrypto::prelude::scrypto_decode;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

// ==========================
// Request & Response Models
// ==========================

#[derive(Serialize, Deserialize, Clone)]
pub struct DecompileNotarizedTransactionIntentRequest {
    pub manifest_instructions_output_format: ManifestInstructionsKind,
    #[serde(with = "hex::serde")]
    pub compiled_notarized_intent: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone)]
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

// ========
// Handler
// ========

pub fn handle_decompile_notarized_transaction_intent(
    request: DecompileNotarizedTransactionIntentRequest,
) -> Result<DecompileNotarizedTransactionIntentResponse, Error> {
    let notarized_transaction: NotarizedTransaction = scrypto_decode::<
        transaction::model::NotarizedTransaction,
    >(&request.compiled_notarized_intent)?
    .try_into()?;
    let notarized_transaction: NotarizedTransaction = notarized_transaction
        .convert_manifest_instructions_kind(request.manifest_instructions_output_format)?;

    let response: DecompileNotarizedTransactionIntentResponse =
        DecompileNotarizedTransactionIntentResponse {
            notarized_transaction,
        };
    Ok(response)
}

export_handler!(handle_decompile_notarized_transaction_intent(
    DecompileNotarizedTransactionIntentRequest
) as decompile_notarized_transaction_intent);

// ======
// Tests
// ======

#[cfg(test)]
mod tests {
    // TODO: Unit tests for this request type
}
