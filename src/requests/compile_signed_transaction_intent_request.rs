use crate::error::Error;
use crate::export_handler;
use crate::models::serde::SignedTransactionIntent;
use crate::traits::Validate;
use crate::validation::validate_transaction_intent;
use scrypto::prelude::scrypto_encode;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

// ==========================
// Request & Response Models
// ==========================

#[derive(Serialize, Deserialize, Clone)]
pub struct CompileSignedTransactionIntentRequest {
    #[serde(flatten)]
    pub signed_intent: SignedTransactionIntent,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CompileSignedTransactionIntentResponse {
    #[serde(with = "hex::serde")]
    pub compiled_signed_intent: Vec<u8>,
}

// ===========
// Validation
// ===========

impl Validate for CompileSignedTransactionIntentRequest {
    fn validate(&self) -> Result<(), Error> {
        validate_transaction_intent(&self.signed_intent.transaction_intent)?;
        Ok(())
    }
}

impl Validate for CompileSignedTransactionIntentResponse {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

// ========
// Handler
// ========

pub fn handle_compile_signed_transaction_intent(
    request: CompileSignedTransactionIntentRequest,
) -> Result<CompileSignedTransactionIntentResponse, Error> {
    let signed_transaction_intent: transaction::model::SignedTransactionIntent =
        request.signed_intent.try_into()?;
    let compiled_signed_intent: Vec<u8> = scrypto_encode(&signed_transaction_intent);

    let response: CompileSignedTransactionIntentResponse = CompileSignedTransactionIntentResponse {
        compiled_signed_intent,
    };
    Ok(response)
}

export_handler!(
    handle_compile_signed_transaction_intent(CompileSignedTransactionIntentRequest)
        as compile_signed_transaction_intent
);

// ======
// Tests
// ======

#[cfg(test)]
mod tests {
    // TODO: Unit tests for this request type
}
