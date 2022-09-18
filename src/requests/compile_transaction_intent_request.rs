use crate::address::Bech32Manager;
use crate::error::Error;
use crate::export_handler;
use crate::models::serde::TransactionIntent;
use crate::traits::Validate;
use crate::validation::validate_transaction_intent;
use scrypto::prelude::scrypto_encode;
use serde::{Deserialize, Serialize};

// ==========================
// Request & Response Models
// ==========================

#[derive(Serialize, Deserialize, Clone)]
pub struct CompileTransactionIntentRequest {
    #[serde(flatten)]
    pub transaction_intent: TransactionIntent,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CompileTransactionIntentResponse {
    #[serde(with = "hex::serde")]
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

// ========
// Handler
// ========

pub fn handle_compile_transaction_intent(
    request: CompileTransactionIntentRequest,
) -> Result<CompileTransactionIntentResponse, Error> {
    let bech32_manager: Bech32Manager =
        Bech32Manager::new(request.transaction_intent.header.network_id);

    // Convert the instructions to a transaction manifest to then create a scrypto transaction
    // intent from it.
    let manifest: transaction::model::TransactionManifest = request
        .transaction_intent
        .manifest
        .instructions
        .to_scrypto_transaction_manifest(
            &bech32_manager,
            request.transaction_intent.manifest.blobs,
        )?;
    let transaction_intent: transaction::model::TransactionIntent =
        transaction::model::TransactionIntent {
            header: request.transaction_intent.header,
            manifest,
        };
    let compiled_intent: Vec<u8> = scrypto_encode(&transaction_intent);

    let response: CompileTransactionIntentResponse =
        CompileTransactionIntentResponse { compiled_intent };
    Ok(response)
}

export_handler!(
    handle_compile_transaction_intent(CompileTransactionIntentRequest)
        as compile_transaction_intent
);

// ======
// Tests
// ======

#[cfg(test)]
mod tests {
    // TODO: Unit tests for this request type
}
