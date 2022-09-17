use scrypto::prelude::{NonFungibleAddress, NonFungibleId, ResourceAddress};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Serialize, Deserialize, Clone)]
pub struct DeriveNonFungibleAddressRequest {
    #[serde_as(as = "DisplayFromStr")]
    pub resource_address: ResourceAddress,
    #[serde_as(as = "DisplayFromStr")]
    pub non_fungible_id: NonFungibleId,
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone)]
pub struct DeriveNonFungibleAddressResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub non_fungible_address: NonFungibleAddress,
}
