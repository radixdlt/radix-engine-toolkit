use crate::models::{Address, AddressKind};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct AddressInformationRequest {
    pub address: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AddressInformationResponse {
    pub network_id: u8,
    pub entity_type: AddressKind,
    #[serde(with = "hex::serde")]
    pub data: Vec<u8>,
    pub hrp: String,
    pub address: Address,
}
