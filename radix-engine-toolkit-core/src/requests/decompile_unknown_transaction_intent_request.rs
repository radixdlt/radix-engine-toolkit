use crate::error::Error;

use crate::model::manifest_instructions::ManifestInstructionsKind;
use crate::requests::*;
use crate::traits::{Request, Validate};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

// ==========================
// Request & Response Models
// ==========================

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DecompileUnknownTransactionIntentRequest {
    pub manifest_instructions_output_format: ManifestInstructionsKind,
    #[serde_as(as = "serde_with::hex::Hex")]
    pub compiled_unknown_intent: Vec<u8>,
}

impl From<DecompileUnknownTransactionIntentRequest> for DecompileTransactionIntentRequest {
    fn from(request: DecompileUnknownTransactionIntentRequest) -> Self {
        DecompileTransactionIntentRequest {
            compiled_intent: request.compiled_unknown_intent,
            manifest_instructions_output_format: request.manifest_instructions_output_format,
        }
    }
}

impl From<DecompileUnknownTransactionIntentRequest> for DecompileSignedTransactionIntentRequest {
    fn from(request: DecompileUnknownTransactionIntentRequest) -> Self {
        DecompileSignedTransactionIntentRequest {
            compiled_signed_intent: request.compiled_unknown_intent,
            manifest_instructions_output_format: request.manifest_instructions_output_format,
        }
    }
}

impl From<DecompileUnknownTransactionIntentRequest> for DecompileNotarizedTransactionIntentRequest {
    fn from(request: DecompileUnknownTransactionIntentRequest) -> Self {
        DecompileNotarizedTransactionIntentRequest {
            compiled_notarized_intent: request.compiled_unknown_intent,
            manifest_instructions_output_format: request.manifest_instructions_output_format,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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
        // Validation is not done here. The other request which fulfills this request will do the
        // validation on its own.
        Ok(())
    }
}

// =======================
// Request Implementation
// =======================

impl<'r> Request<'r, DecompileUnknownTransactionIntentResponse>
    for DecompileUnknownTransactionIntentRequest
{
    fn handle_request(self) -> Result<DecompileUnknownTransactionIntentResponse, Error> {
        if let Ok(response) =
            DecompileTransactionIntentRequest::from(self.clone()).fulfill_request()
        {
            Ok(response.into())
        } else if let Ok(response) =
            DecompileSignedTransactionIntentRequest::from(self.clone()).fulfill_request()
        {
            Ok(response.into())
        } else if let Ok(response) =
            DecompileNotarizedTransactionIntentRequest::from(self).fulfill_request()
        {
            Ok(response.into())
        } else {
            Err(Error::UnrecognizedCompiledIntentFormat)
        }
    }
}

// ======
// Tests
// ======

#[cfg(test)]
mod tests {
    // TODO: Unit tests for this request type
}
