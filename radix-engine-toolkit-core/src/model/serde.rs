// TODO: Refactor this module and separate it out as needed. Right now it is a place where any kind
// of serialization model or logic exists. Perhaps there could be better ways to do this.

use std::convert::TryFrom;
use std::str::FromStr;

use scrypto_utils::copy_u8_array;

use serde::de::Error as DeserializationError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use scrypto::prelude::{scrypto_decode, Hash};
use serde_with::{serde_as, DeserializeAs, SerializeAs};

use crate::error::Error;

use super::Value;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeId(pub (Hash, u32));

impl NodeId {
    pub fn to_bytes(&self) -> [u8; 36] {
        let mut node_id_bytes = self.0 .0.to_vec();
        node_id_bytes.extend(self.0 .1.to_le_bytes());
        copy_u8_array(&node_id_bytes)
    }

    pub fn from_bytes(vec: [u8; 36]) -> Self {
        let hash_bytes = &vec[0..32];
        let index_bytes = &vec[32..];

        let hash = Hash(copy_u8_array(hash_bytes));
        let index = u32::from_le_bytes(copy_u8_array(index_bytes));

        Self((hash, index))
    }
}

impl From<(Hash, u32)> for NodeId {
    fn from(value: (Hash, u32)) -> Self {
        Self(value)
    }
}

impl From<NodeId> for (Hash, u32) {
    fn from(entity_id: NodeId) -> Self {
        entity_id.0
    }
}

impl Serialize for NodeId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for NodeId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let node_id_string: String = Deserialize::deserialize(deserializer)?;
        node_id_string
            .parse()
            .map_err(|_| DeserializationError::custom("Failed to parse node id from string"))
    }
}

impl ToString for NodeId {
    fn to_string(&self) -> String {
        hex::encode(self.to_bytes())
    }
}

impl FromStr for NodeId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let node_id_bytes = hex::decode(s)
            .map_err(|_| Error::DeserializationError(format!("Failed to decode node id: {}", s)))?;

        // TODO: Should not do a copy u8 array without first checking the length
        Ok(Self::from_bytes(copy_u8_array(&node_id_bytes)))
    }
}

/// A serde-as serializer that serializes and deserializes object as a [Value]. This is useful for
/// consistent returns from the toolkit.
pub struct ValueSerializationProxy;

impl<T> SerializeAs<T> for ValueSerializationProxy
where
    T: Into<Value> + Clone,
{
    fn serialize_as<S>(source: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = source.clone().into();
        value.serialize(serializer)
    }
}

impl<'de, T> DeserializeAs<'de, T> for ValueSerializationProxy
where
    T: TryFrom<Value, Error = Error> + Clone,
{
    fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
    {
        Value::deserialize(deserializer)?
            .try_into()
            .map_err(|err| D::Error::custom(format!("{:?}", err)))
    }
}

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct NonFungibleIdData {
    #[serde_as(as = "serde_with::hex::Hex")]
    value: Vec<u8>,
}

impl TryFrom<scrypto::prelude::NonFungibleId> for NonFungibleIdData {
    type Error = Error;

    fn try_from(value: scrypto::prelude::NonFungibleId) -> Result<Self, Self::Error> {
        Ok(Self {
            value: value.to_vec(),
        })
    }
}

impl TryFrom<NonFungibleIdData> for scrypto::prelude::NonFungibleId {
    type Error = Error;

    fn try_from(value: NonFungibleIdData) -> Result<Self, Self::Error> {
        Ok(scrypto_decode(&value.value)?)
    }
}
