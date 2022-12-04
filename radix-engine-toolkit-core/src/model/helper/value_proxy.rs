use serde::de::Error as DeserializationError;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde_with::{SerializeAs, DeserializeAs};

use crate::model::value::Value;
use crate::error::Error;

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