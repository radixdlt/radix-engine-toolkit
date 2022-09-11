use crate::models::Value;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct SBOREncodeRequest {
    #[serde(flatten)]
    pub value: Value,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SBOREncodeResponse {
    #[serde(with = "hex::serde")]
    pub encoded_value: Vec<u8>,
}
