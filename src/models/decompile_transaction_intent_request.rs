use crate::models::manifest::{Manifest, ManifestKind};
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
    /// The transaction header of the intent
    #[serde(with = "crate::models::serde::TransactionHeaderDef")]
    pub header: transaction::model::TransactionHeader,

    /// The transaction manifest that will be compiled
    pub manifest: Manifest,
}
