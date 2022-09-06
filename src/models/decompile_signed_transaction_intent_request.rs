use crate::models::manifest::ManifestKind;
use crate::models::serde::SignedTransactionIntent;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct DecompileSignedTransactionIntentRequest {
    pub manifest_output_format: ManifestKind,
    
    #[serde(with = "hex::serde")]
    pub compiled_signed_intent: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DecompileSignedTransactionIntentResponse {
    #[serde(flatten)]
    pub signed_intent: SignedTransactionIntent,
}
