use std::str::FromStr;

use serde::de::Error as DeserializationError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use scrypto::prelude::Hash;
use scrypto_utils::copy_u8_array;

use crate::error::Error;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeIdentifier(pub (Hash, u32));

impl NodeIdentifier {
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

impl From<(Hash, u32)> for NodeIdentifier {
    fn from(value: (Hash, u32)) -> Self {
        Self(value)
    }
}

impl From<NodeIdentifier> for (Hash, u32) {
    fn from(entity_id: NodeIdentifier) -> Self {
        entity_id.0
    }
}

impl Serialize for NodeIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for NodeIdentifier {
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

impl ToString for NodeIdentifier {
    fn to_string(&self) -> String {
        hex::encode(self.to_bytes())
    }
}

impl FromStr for NodeIdentifier {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let node_id_bytes = hex::decode(s)
            .map_err(|_| Error::DeserializationError(format!("Failed to decode node id: {}", s)))?;

        // TODO: Should not do a copy u8 array without first checking the length
        Ok(Self::from_bytes(copy_u8_array(&node_id_bytes)))
    }
}
