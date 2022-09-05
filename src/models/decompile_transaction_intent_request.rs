use crate::models::manifest::ManifestKind;
use crate::models::serde::TransactionIntent;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct DecompileTransactionIntentRequest {
    /// Defines the output format that we would like the manifest to be in after this request is
    /// performed.
    pub manifest_output_format: ManifestKind,

    /// The compiled intent which we wish to decompile.
    #[serde(with = "hex::serde")]
    pub compiled_intent: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DecompileTransactionIntentResponse {
    #[serde(flatten)]
    pub transaction_intent: TransactionIntent,
}
