use crate::error::Error;
use crate::export_handler;
use crate::models::manifest::ManifestInstructionsKind;
use crate::requests::*;
use crate::traits::Validate;
use serde::{Deserialize, Serialize};

// ==========================
// Request & Response Models
// ==========================

#[derive(Serialize, Deserialize, Clone)]
pub struct DecompileUnknownTransactionIntentRequest {
    pub manifest_instructions_output_format: ManifestInstructionsKind,
    #[serde(with = "hex::serde")]
    pub compiled_unknown_intent: Vec<u8>,
}

impl Into<DecompileTransactionIntentRequest> for DecompileUnknownTransactionIntentRequest {
    fn into(self) -> DecompileTransactionIntentRequest {
        DecompileTransactionIntentRequest {
            compiled_intent: self.compiled_unknown_intent,
            manifest_instructions_output_format: self.manifest_instructions_output_format,
        }
    }
}

impl Into<DecompileSignedTransactionIntentRequest> for DecompileUnknownTransactionIntentRequest {
    fn into(self) -> DecompileSignedTransactionIntentRequest {
        DecompileSignedTransactionIntentRequest {
            compiled_signed_intent: self.compiled_unknown_intent,
            manifest_instructions_output_format: self.manifest_instructions_output_format,
        }
    }
}

impl Into<DecompileNotarizedTransactionIntentRequest> for DecompileUnknownTransactionIntentRequest {
    fn into(self) -> DecompileNotarizedTransactionIntentRequest {
        DecompileNotarizedTransactionIntentRequest {
            compiled_notarized_intent: self.compiled_unknown_intent,
            manifest_instructions_output_format: self.manifest_instructions_output_format,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum DecompileUnknownTransactionIntentResponse {
    TransactionIntent(DecompileTransactionIntentResponse),
    SignedTransactionIntent(DecompileSignedTransactionIntentResponse),
    NotarizedTransactionIntent(DecompileNotarizedTransactionIntentResponse),
}

impl From<DecompileTransactionIntentResponse> for DecompileUnknownTransactionIntentResponse {
    fn from(response: DecompileTransactionIntentResponse) -> Self {
        Self::TransactionIntent(response)
    }
}

impl From<DecompileSignedTransactionIntentResponse> for DecompileUnknownTransactionIntentResponse {
    fn from(response: DecompileSignedTransactionIntentResponse) -> Self {
        Self::SignedTransactionIntent(response)
    }
}

impl From<DecompileNotarizedTransactionIntentResponse>
    for DecompileUnknownTransactionIntentResponse
{
    fn from(response: DecompileNotarizedTransactionIntentResponse) -> Self {
        Self::NotarizedTransactionIntent(response)
    }
}

// ===========
// Validation
// ===========

impl Validate for DecompileUnknownTransactionIntentRequest {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

impl Validate for DecompileUnknownTransactionIntentResponse {
    fn validate(&self) -> Result<(), Error> {
        match self {
            Self::TransactionIntent(response) => response.validate(),
            Self::SignedTransactionIntent(response) => response.validate(),
            Self::NotarizedTransactionIntent(response) => response.validate(),
        }
    }
}

// ========
// Handler
// ========

pub fn handle_decompile_unknown_transaction_intent(
    request: DecompileUnknownTransactionIntentRequest,
) -> Result<DecompileUnknownTransactionIntentResponse, Error> {
    let response: DecompileUnknownTransactionIntentResponse = if let Ok(response) =
        handle_decompile_transaction_intent(request.clone().into())
    {
        Ok(response.into())
    } else if let Ok(response) = handle_decompile_signed_transaction_intent(request.clone().into())
    {
        Ok(response.into())
    } else if let Ok(response) = handle_decompile_notarized_transaction_intent(request.into()) {
        Ok(response.into())
    } else {
        Err(Error::UnrecognizedCompiledIntentFormat)
    }?;

    Ok(response)
}

export_handler!(handle_decompile_unknown_transaction_intent(
    DecompileUnknownTransactionIntentRequest
) as decompile_unknown_transaction_intent);

// ======
// Tests
// ======

#[cfg(test)]
mod tests {
    // TODO: Unit tests for this request type
}
