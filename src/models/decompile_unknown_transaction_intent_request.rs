use crate::models::manifest::ManifestKind;
use crate::models::{
    DecompileNotarizedTransactionIntentRequest, DecompileNotarizedTransactionIntentResponse,
    DecompileSignedTransactionIntentRequest, DecompileSignedTransactionIntentResponse,
    DecompileTransactionIntentRequest, DecompileTransactionIntentResponse,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct DecompileUnknownTransactionIntentRequest {
    pub manifest_output_format: ManifestKind,
    #[serde(with = "hex::serde")]
    pub compiled_unknown_intent: Vec<u8>,
}

impl Into<DecompileTransactionIntentRequest> for DecompileUnknownTransactionIntentRequest {
    fn into(self) -> DecompileTransactionIntentRequest {
        DecompileTransactionIntentRequest {
            compiled_intent: self.compiled_unknown_intent,
            manifest_output_format: self.manifest_output_format,
        }
    }
}

impl Into<DecompileSignedTransactionIntentRequest> for DecompileUnknownTransactionIntentRequest {
    fn into(self) -> DecompileSignedTransactionIntentRequest {
        DecompileSignedTransactionIntentRequest {
            compiled_signed_intent: self.compiled_unknown_intent,
            manifest_output_format: self.manifest_output_format,
        }
    }
}

impl Into<DecompileNotarizedTransactionIntentRequest> for DecompileUnknownTransactionIntentRequest {
    fn into(self) -> DecompileNotarizedTransactionIntentRequest {
        DecompileNotarizedTransactionIntentRequest {
            compiled_notarized_intent: self.compiled_unknown_intent,
            manifest_output_format: self.manifest_output_format,
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
