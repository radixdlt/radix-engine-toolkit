use crate::models::Value;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct SBORDecodeRequest {
    #[serde(with = "hex::serde")]
    pub encoded_value: Vec<u8>,
    pub network_id: u8,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SBORDecodeResponse {
    #[serde(flatten)]
    pub value: Value,
}
