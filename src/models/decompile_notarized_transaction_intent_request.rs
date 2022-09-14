use crate::models::manifest::ManifestKind;
use crate::models::serde::SignedTransactionIntent;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Serialize, Deserialize, Clone)]
pub struct DecompileNotarizedTransactionIntentRequest {
    pub manifest_output_format: ManifestKind,
    #[serde(with = "hex::serde")]
    pub compiled_notarized_intent: Vec<u8>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone)]
pub struct DecompileNotarizedTransactionIntentResponse {
    pub signed_intent: SignedTransactionIntent,
    #[serde_as(as = "DisplayFromStr")]
    pub notary_signature: scrypto::prelude::EcdsaSignature,
}
