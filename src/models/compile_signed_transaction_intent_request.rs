use crate::models::serde::{Signature, TransactionIntent};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct CompileSignedTransactionIntentRequest {
    pub transaction_intent: TransactionIntent,
    pub signatures: Vec<Signature>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CompileSignedTransactionIntentResponse {
    #[serde(with = "hex::serde")]
    pub compiled_signed_intent: Vec<u8>,
}
