use crate::address::Bech32Manager;
use crate::error::Error;
use crate::export_handler;
use crate::models::manifest::{ManifestInstructions, ManifestInstructionsKind};
use crate::models::serde::{SignedTransactionIntent, TransactionIntent, TransactionManifest};
use crate::traits::Validate;
use crate::validation::validate_transaction_intent;
use scrypto::prelude::{scrypto_decode, SignatureWithPublicKey};
use serde::{Deserialize, Serialize};

// ==========================
// Request & Response Models
// ==========================

#[derive(Serialize, Deserialize, Clone)]
pub struct DecompileSignedTransactionIntentRequest {
    pub manifest_instructions_output_format: ManifestInstructionsKind,

    #[serde(with = "hex::serde")]
    pub compiled_signed_intent: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone)]
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
        validate_transaction_intent(&self.signed_intent.transaction_intent)?;
        Ok(())
    }
}

// ========
// Handler
// ========

pub fn handle_decompile_signed_transaction_intent(
    request: DecompileSignedTransactionIntentRequest,
) -> Result<DecompileSignedTransactionIntentResponse, Error> {
    let signed_transaction_intent: transaction::model::SignedTransactionIntent =
        scrypto_decode(&request.compiled_signed_intent)?;

    let signatures: Vec<SignatureWithPublicKey> = signed_transaction_intent.intent_signatures;
    let manifest_instructions: ManifestInstructions =
        ManifestInstructions::from_scrypto_transaction_manifest(
            &signed_transaction_intent.intent.manifest,
            &Bech32Manager::new(signed_transaction_intent.intent.header.network_id),
            request.manifest_instructions_output_format,
        )?;

    let response: DecompileSignedTransactionIntentResponse =
        DecompileSignedTransactionIntentResponse {
            signed_intent: SignedTransactionIntent {
                signatures,
                transaction_intent: TransactionIntent {
                    header: signed_transaction_intent.intent.header,
                    manifest: TransactionManifest {
                        instructions: manifest_instructions,
                        blobs: signed_transaction_intent.intent.manifest.blobs,
                    },
                },
            },
        };

    Ok(response)
}

export_handler!(
    handle_decompile_signed_transaction_intent(DecompileSignedTransactionIntentRequest)
        as decompile_signed_transaction_intent
);

// ======
// Tests
// ======

#[cfg(test)]
mod tests {
    // TODO: Unit tests for this request type
}
