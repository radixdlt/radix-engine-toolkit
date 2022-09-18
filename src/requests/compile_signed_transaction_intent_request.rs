use crate::address::Bech32Manager;
use crate::error::Error;
use crate::export_handler;
use crate::models::serde::{SignedTransactionIntent, TransactionManifest};
use crate::traits::Validate;
use crate::validation::validate_transaction_intent;
use scrypto::prelude::{scrypto_encode, SignatureWithPublicKey};
use serde::{Deserialize, Serialize};

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
    request.validate()?;

    let bech32_manager: Bech32Manager =
        Bech32Manager::new(request.signed_intent.transaction_intent.header.network_id);

    let manifest: TransactionManifest = request.signed_intent.transaction_intent.manifest;
    let manifest: transaction::model::TransactionManifest = manifest
        .instructions
        .to_scrypto_transaction_manifest(&bech32_manager, manifest.blobs)?;
    let transaction_intent: transaction::model::TransactionIntent =
        transaction::model::TransactionIntent {
            header: request.signed_intent.transaction_intent.header,
            manifest,
        };

    let signatures: Vec<SignatureWithPublicKey> = request.signed_intent.signatures;
    let signed_transaction_intent: transaction::model::SignedTransactionIntent =
        transaction::model::SignedTransactionIntent {
            intent: transaction_intent,
            intent_signatures: signatures,
        };
    let compiled_signed_intent: Vec<u8> = scrypto_encode(&signed_transaction_intent);

    let response: CompileSignedTransactionIntentResponse = CompileSignedTransactionIntentResponse {
        compiled_signed_intent,
    };

    response.validate()?;
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
