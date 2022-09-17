use scrypto::prelude::{NonFungibleAddress, PublicKey};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Serialize, Deserialize, Clone)]
pub struct DeriveNonFungibleAddressFromPublicKeyRequest {
    pub public_key: PublicKey,
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone)]
pub struct DeriveNonFungibleAddressFromPublicKeyResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub non_fungible_address: NonFungibleAddress,
}
