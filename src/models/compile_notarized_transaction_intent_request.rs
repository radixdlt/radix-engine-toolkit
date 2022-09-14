use crate::models::serde::SignedTransactionIntent;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
#[serde_as]
#[derive(Serialize, Deserialize, Clone)]
pub struct CompileNotarizedTransactionIntentRequest {
    pub signed_intent: SignedTransactionIntent,
    #[serde_as(as = "DisplayFromStr")]
    pub notary_signature: scrypto::prelude::EcdsaSignature,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CompileNotarizedTransactionIntentResponse {
    #[serde(with = "hex::serde")]
    pub compiled_notarized_intent: Vec<u8>,
}
