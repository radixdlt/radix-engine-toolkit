use crate::models::serde::SignedTransactionIntent;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct CompileNotarizedTransactionIntentRequest {
    pub signed_intent: SignedTransactionIntent,
    #[serde(with = "crate::models::serde::EcdsaSignatureDef")]
    pub notary_signature: scrypto::prelude::EcdsaSignature,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CompileNotarizedTransactionIntentResponse {
    #[serde(with = "hex::serde")]
    pub compiled_notarized_intent: Vec<u8>,
}
