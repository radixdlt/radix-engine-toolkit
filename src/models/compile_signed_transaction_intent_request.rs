use crate::models::serde::SignedTransactionIntent;
use serde::{Deserialize, Serialize};

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
