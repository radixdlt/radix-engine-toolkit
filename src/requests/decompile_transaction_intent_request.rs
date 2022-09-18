use crate::address::Bech32Manager;
use crate::error::Error;
use crate::export_handler;
use crate::models::manifest::{ManifestInstructions, ManifestInstructionsKind};
use crate::models::serde::{TransactionIntent, TransactionManifest};
use crate::traits::Validate;
use crate::validation::validate_transaction_intent;
use scrypto::prelude::scrypto_decode;
use serde::{Deserialize, Serialize};

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
    let transaction_intent: transaction::model::TransactionIntent =
        scrypto_decode(&request.compiled_intent)?;
    let manifest_instructions: ManifestInstructions =
        ManifestInstructions::from_scrypto_transaction_manifest(
            &transaction_intent.manifest,
            &Bech32Manager::new(transaction_intent.header.network_id),
            request.manifest_instructions_output_format,
        )?;

    let response: DecompileTransactionIntentResponse = DecompileTransactionIntentResponse {
        transaction_intent: TransactionIntent {
            header: transaction_intent.header,
            manifest: TransactionManifest {
                instructions: manifest_instructions,
                blobs: transaction_intent.manifest.blobs,
            },
        },
    };

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
