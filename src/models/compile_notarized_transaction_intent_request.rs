use crate::models::serde::SignedTransactionIntent;
use radix_engine::types::Signature;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct CompileNotarizedTransactionIntentRequest {
    pub signed_intent: SignedTransactionIntent,
    pub notary_signature: Signature,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CompileNotarizedTransactionIntentResponse {
    #[serde(with = "hex::serde")]
    pub compiled_notarized_intent: Vec<u8>,
}
