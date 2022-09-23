use crate::error::Error;
use crate::export_request;
use crate::models::serde::NotarizedTransaction;
use crate::traits::{Request, Validate};
use crate::validation::validate_notarized_transaction;
use scrypto::prelude::scrypto_encode;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

// ==========================
// Request & Response Models
// ==========================

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompileNotarizedTransactionIntentRequest {
    #[serde(flatten)]
    pub notarized_transaction: NotarizedTransaction,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompileNotarizedTransactionIntentResponse {
    #[serde(with = "hex::serde")]
    pub compiled_notarized_intent: Vec<u8>,
}

// ===========
// Validation
// ===========

impl Validate for CompileNotarizedTransactionIntentRequest {
    fn validate(&self) -> Result<(), Error> {
        validate_notarized_transaction(&self.notarized_transaction)?;
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
        let notarized_transaction: transaction::model::NotarizedTransaction =
            self.notarized_transaction.try_into()?;
        let compiled_notarized_intent: Vec<u8> = scrypto_encode(&notarized_transaction);

        let response: CompileNotarizedTransactionIntentResponse =
            CompileNotarizedTransactionIntentResponse {
                compiled_notarized_intent,
            };
        Ok(response)
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
