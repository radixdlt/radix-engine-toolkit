// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at

//   http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use serde::de::Error as DeserializationError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::{DeserializeAs, SerializeAs};

use crate::error::Error;
use crate::model::value::Value;

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
