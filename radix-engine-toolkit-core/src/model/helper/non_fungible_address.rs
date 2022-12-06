use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use scrypto::prelude::{
    FromPublicKey, NonFungibleAddress as NativeNonFungibleAddress, NonFungibleId, PublicKey,
};

use super::ValueSerializationProxy;
use crate::model::NetworkAwareResourceAddress;

#[serde_as]
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct NonFungibleAddress {
    #[serde_as(as = "ValueSerializationProxy")]
    pub resource_address: NetworkAwareResourceAddress,

    #[serde_as(as = "ValueSerializationProxy")]
    pub non_fungible_id: NonFungibleId,
}

impl NonFungibleAddress {
    pub fn new(
        resource_address: NetworkAwareResourceAddress,
        non_fungible_id: NonFungibleId,
    ) -> Self {
        Self {
            resource_address,
            non_fungible_id,
        }
    }

    pub fn from_public_key<P: Into<PublicKey> + Clone>(public_key: &P, network_id: u8) -> Self {
        let native_non_fungible_address = NativeNonFungibleAddress::from_public_key(public_key);
        Self {
            resource_address: NetworkAwareResourceAddress {
                network_id,
                address: native_non_fungible_address.resource_address(),
            },
            non_fungible_id: native_non_fungible_address.non_fungible_id().clone(),
        }
    }
}

impl From<NonFungibleAddress> for scrypto::prelude::NonFungibleAddress {
    fn from(value: NonFungibleAddress) -> Self {
        scrypto::prelude::NonFungibleAddress::new(
            value.resource_address.address,
            value.non_fungible_id,
        )
    }
}
