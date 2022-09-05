use crate::models::manifest::ManifestKind;
use crate::models::serde::SignedTransactionIntent;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct DecompileNotarizedTransactionIntentRequest {
    pub manifest_output_format: ManifestKind,
    #[serde(with = "hex::serde")]
    pub compiled_notarized_intent: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DecompileNotarizedTransactionIntentResponse {
    pub signed_intent: SignedTransactionIntent,
    #[serde(with = "crate::models::serde::EcdsaSignatureDef")]
    pub notary_signature: scrypto::prelude::EcdsaSignature,
}
