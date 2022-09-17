use crate::address::Bech32Manager;
use crate::error::Error;
use crate::export_handler;
use crate::models::manifest::{ManifestInstructions, ManifestInstructionsKind};
use crate::models::serde::{
    NotarizedTransaction, SignedTransactionIntent, TransactionIntent, TransactionManifest,
};
use crate::traits::Validate;
use crate::validation::validate_notarized_transaction;
use scrypto::prelude::{scrypto_decode, SignatureWithPublicKey};
use serde::{Deserialize, Serialize};

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
    let notarized_transaction_intent: transaction::model::NotarizedTransaction =
        scrypto_decode(&request.compiled_notarized_intent)?;

    let signatures: Vec<SignatureWithPublicKey> =
        notarized_transaction_intent.signed_intent.intent_signatures;
    let manifest_instructions: ManifestInstructions =
        ManifestInstructions::from_scrypto_transaction_manifest(
            &notarized_transaction_intent.signed_intent.intent.manifest,
            &Bech32Manager::new(
                notarized_transaction_intent
                    .signed_intent
                    .intent
                    .header
                    .network_id,
            ),
            request.manifest_instructions_output_format,
        )?;

    let response: DecompileNotarizedTransactionIntentResponse =
        DecompileNotarizedTransactionIntentResponse {
            notarized_transaction: NotarizedTransaction {
                signed_intent: SignedTransactionIntent {
                    signatures,
                    transaction_intent: TransactionIntent {
                        header: notarized_transaction_intent.signed_intent.intent.header,
                        manifest: TransactionManifest {
                            instructions: manifest_instructions,
                            blobs: notarized_transaction_intent
                                .signed_intent
                                .intent
                                .manifest
                                .blobs
                                .clone(),
                        },
                    },
                },
                notary_signature: notarized_transaction_intent.notary_signature,
            },
        };
    Ok(response)
}

export_handler!(
    handle_decompile_notarized_transaction_intent as decompile_notarized_transaction_intent
);

// ======
// Tests
// ======

#[cfg(test)]
mod tests {
    // TODO: Unit tests for this request type
}
