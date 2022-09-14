use crate::models::manifest::ManifestInstructionsKind;
use crate::models::serde::SignedTransactionIntent;
use radix_engine::types::Signature;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct DecompileNotarizedTransactionIntentRequest {
    pub manifest_instructions_output_format: ManifestInstructionsKind,
    #[serde(with = "hex::serde")]
    pub compiled_notarized_intent: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DecompileNotarizedTransactionIntentResponse {
    pub signed_intent: SignedTransactionIntent,
    pub notary_signature: Signature,
}
