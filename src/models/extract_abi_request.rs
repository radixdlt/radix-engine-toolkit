use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ExtractAbiRequest {
    #[serde(with = "hex::serde")]
    pub package_wasm: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ExtractAbiResponse {
    #[serde(with = "hex::serde")]
    pub code: Vec<u8>,
    #[serde(with = "hex::serde")]
    pub abi: Vec<u8>,
}
