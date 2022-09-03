use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct InformationRequest {}

#[derive(Serialize, Deserialize, Clone)]
pub struct InformationResponse {
    pub package_version: String,
}
