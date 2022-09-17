use crate::error::Error;
use crate::export_handler;
use crate::models::serde::NotarizedTransaction;
use crate::traits::Validate;
use crate::validation::validate_notarized_transaction;
use scrypto::prelude::{scrypto_encode, SignatureWithPublicKey};
use serde::{Deserialize, Serialize};

// ==========================
// Request & Response Models
// ==========================

#[derive(Serialize, Deserialize, Clone)]
pub struct CompileNotarizedTransactionIntentRequest {
    pub notarized_transaction: NotarizedTransaction,
}

#[derive(Serialize, Deserialize, Clone)]
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

// ========
// Handler
// ========

pub fn handle_compile_notarized_transaction_intent(
    request: CompileNotarizedTransactionIntentRequest,
) -> Result<CompileNotarizedTransactionIntentResponse, Error> {
    let transaction_intent: transaction::model::TransactionIntent = request
        .notarized_transaction
        .signed_intent
        .transaction_intent
        .try_into()?;

    let signatures: Vec<SignatureWithPublicKey> =
        request.notarized_transaction.signed_intent.signatures;
    let notarized_transaction: transaction::model::NotarizedTransaction =
        transaction::model::NotarizedTransaction {
            signed_intent: transaction::model::SignedTransactionIntent {
                intent: transaction_intent,
                intent_signatures: signatures,
            },
            notary_signature: request.notarized_transaction.notary_signature,
        };
    let compiled_notarized_intent: Vec<u8> = scrypto_encode(&notarized_transaction);

    let response: CompileNotarizedTransactionIntentResponse =
        CompileNotarizedTransactionIntentResponse {
            compiled_notarized_intent,
        };

    Ok(response)
}

export_handler!(
    handle_compile_notarized_transaction_intent as compile_notarized_transaction_intent
);

// ======
// Tests
// ======

#[cfg(test)]
mod tests {
    // TODO: Unit tests for this request type
}
