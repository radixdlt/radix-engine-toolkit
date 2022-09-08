use crate::error::Error;
use crate::utils::{
    network_definition_from_network_id, network_id_from_address_string, network_id_from_hrp,
};

pub struct Bech32Manager {
    pub network_id: u8,
    pub encoder: scrypto::address::Bech32Encoder,
    pub decoder: scrypto::address::Bech32Decoder,
}

impl Bech32Manager {
    pub fn new(network_id: u8) -> Self {
        let network_definition: scrypto::core::NetworkDefinition =
            network_definition_from_network_id(network_id);
        Self {
            network_id,
            encoder: scrypto::address::Bech32Encoder::new(&network_definition),
            decoder: scrypto::address::Bech32Decoder::new(&network_definition),
        }
    }

    pub fn new_from_hrp(hrp: &str) -> Result<Self, Error> {
        Ok(Self::new(network_id_from_hrp(hrp)?))
    }

    pub fn new_from_address(address: &str) -> Result<Self, Error> {
        Ok(Self::new(network_id_from_address_string(address)?))
    }
}
