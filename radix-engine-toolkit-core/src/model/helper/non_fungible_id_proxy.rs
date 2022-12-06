use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

use scrypto::prelude::NonFungibleId;

#[serde_as]
#[derive(Serialize, Deserialize)]
#[serde(tag = "variant", content = "value")]
pub enum NonFungibleIdProxy {
    String(String),
    U32(#[serde_as(as = "DisplayFromStr")] u32),
    U64(#[serde_as(as = "DisplayFromStr")] u64),
    Bytes(#[serde_as(as = "serde_with::hex::Hex")] Vec<u8>),
    UUID(#[serde_as(as = "DisplayFromStr")] u128),
}

impl From<NonFungibleId> for NonFungibleIdProxy {
    fn from(value: NonFungibleId) -> Self {
        match value {
            NonFungibleId::U32(value) => Self::U32(value),
            NonFungibleId::U64(value) => Self::U64(value),
            NonFungibleId::UUID(value) => Self::UUID(value),
            NonFungibleId::String(value) => Self::String(value),
            NonFungibleId::Bytes(value) => Self::Bytes(value),
        }
    }
}

impl From<NonFungibleIdProxy> for NonFungibleId {
    fn from(value: NonFungibleIdProxy) -> Self {
        match value {
            NonFungibleIdProxy::U32(value) => Self::U32(value),
            NonFungibleIdProxy::U64(value) => Self::U64(value),
            NonFungibleIdProxy::UUID(value) => Self::UUID(value),
            NonFungibleIdProxy::String(value) => Self::String(value),
            NonFungibleIdProxy::Bytes(value) => Self::Bytes(value),
        }
    }
}
