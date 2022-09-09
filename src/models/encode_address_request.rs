use crate::models::Address;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct EncodeAddressRequest {
    #[serde(with = "hex::serde")]
    pub address: Vec<u8>,

    pub network_id: u8,
}

pub type EncodeAddressResponse = Address;
