use std::convert::TryFrom;

use serde::{Serialize, Deserialize};
use serde_with::serde_as;

use scrypto::prelude::scrypto_decode;

use crate::error::Error;

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct NonFungibleIdProxy {
    #[serde_as(as = "serde_with::hex::Hex")]
    value: Vec<u8>,
}

impl TryFrom<scrypto::prelude::NonFungibleId> for NonFungibleIdProxy {
    type Error = Error;

    fn try_from(value: scrypto::prelude::NonFungibleId) -> Result<Self, Self::Error> {
        Ok(Self {
            value: value.to_vec(),
        })
    }
}

impl TryFrom<NonFungibleIdProxy> for scrypto::prelude::NonFungibleId {
    type Error = Error;

    fn try_from(value: NonFungibleIdProxy) -> Result<Self, Self::Error> {
        Ok(scrypto_decode(&value.value)?)
    }
}
