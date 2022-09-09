use crate::models::{Address, AddressKind};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct DecodeAddressRequest {
    pub address: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DecodeAddressResponse {
    pub network_id: u8,
    pub network_name: String,
    pub entity_type: AddressKind,
    #[serde(with = "hex::serde")]
    pub data: Vec<u8>,
    pub hrp: String,
    pub address: Address,
}
