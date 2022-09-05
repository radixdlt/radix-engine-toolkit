use crate::models::manifest::Manifest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct CompileTransactionIntentRequest {
    /// The transaction header of the intent
    #[serde(with = "crate::models::serde::TransactionHeaderDef")]
    pub header: transaction::model::TransactionHeader,

    /// The transaction manifest that will be compiled
    pub manifest: Manifest,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CompileTransactionIntentResponse {
    /// The compiled intent
    #[serde(with = "hex::serde")]
    pub compiled_intent: Vec<u8>,
}
