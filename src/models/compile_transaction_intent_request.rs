use crate::models::serde::TransactionIntent;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct CompileTransactionIntentRequest {
    #[serde(flatten)]
    pub transaction_intent: TransactionIntent,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CompileTransactionIntentResponse {
    #[serde(with = "hex::serde")]
    pub compiled_intent: Vec<u8>,
}
