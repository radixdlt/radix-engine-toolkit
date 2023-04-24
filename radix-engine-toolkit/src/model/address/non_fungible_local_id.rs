// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use scrypto::prelude::{
    IntegerNonFungibleLocalId, NonFungibleLocalId as ScryptoNonFungibleLocalId,
};
use toolkit_derive::serializable;

use crate::{impl_display_as_debug, utils::debug_string};

#[serializable]
#[serde(tag = "type", content = "value")]
/// Represents non-fungible ids which is a discriminated union of the different types that
/// non-fungible ids may be.
pub enum NonFungibleLocalId {
    /// A 64 bit unsigned integer non-fungible id type which is serialized as a string
    #[schemars(example = "crate::example::address::non_fungible::non_fungible_local_integer")]
    Integer(
        #[schemars(regex(pattern = "[0-9]+"))]
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        u64,
    ),

    // TODO: Should this be serialized as a GUID?
    /// A 128 bit unsigned integer UUID non-fungible id type which is serialized as a string
    #[schemars(example = "crate::example::address::non_fungible::non_fungible_local_uuid")]
    UUID(
        #[schemars(regex(pattern = "[0-9]+"))]
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        u128,
    ),

    /// An byte array non-fungible id type which is serialized as a hex string. This can be between
    /// 1 and 64 bytes in length which translates to a length range of 2 and 128 when hex-encoded.
    #[schemars(example = "crate::example::address::non_fungible::non_fungible_local_uuid")]
    Bytes(
        #[schemars(regex(pattern = "[0-9a-fA-F]+"))]
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::hex::Hex")]
        #[schemars(length(min = 2, max = 128))]
        Vec<u8>,
    ),

    /// A string non-fungible id. This can be between 1 and 64 characters long.
    #[schemars(example = "crate::example::address::non_fungible::non_fungible_local_string")]
    String(#[schemars(length(min = 1, max = 64))] String),
}

impl TryFrom<ScryptoNonFungibleLocalId> for NonFungibleLocalId {
    type Error = NonFungibleLocalIdConversionError;

    fn try_from(value: ScryptoNonFungibleLocalId) -> Result<Self, Self::Error> {
        match value {
            ScryptoNonFungibleLocalId::Integer(value) => Ok(Self::Integer(value.value())),
            ScryptoNonFungibleLocalId::UUID(value) => Ok(Self::UUID(value.value())),
            ScryptoNonFungibleLocalId::String(value) => Ok(Self::String(value.value().to_owned())),
            ScryptoNonFungibleLocalId::Bytes(value) => Ok(Self::Bytes(value.value().to_owned())),
        }
    }
}

impl TryFrom<NonFungibleLocalId> for ScryptoNonFungibleLocalId {
    type Error = NonFungibleLocalIdConversionError;

    fn try_from(value: NonFungibleLocalId) -> Result<Self, Self::Error> {
        match value {
            NonFungibleLocalId::Integer(value) => {
                Ok(Self::Integer(IntegerNonFungibleLocalId::new(value)))
            }
            NonFungibleLocalId::UUID(value) => ScryptoNonFungibleLocalId::uuid(value)
                .map_err(|error| NonFungibleLocalIdConversionError(debug_string(error))),
            NonFungibleLocalId::String(value) => ScryptoNonFungibleLocalId::string(value)
                .map_err(|error| NonFungibleLocalIdConversionError(debug_string(error))),
            NonFungibleLocalId::Bytes(value) => ScryptoNonFungibleLocalId::bytes(value)
                .map_err(|error| NonFungibleLocalIdConversionError(debug_string(error))),
        }
    }
}

#[serializable]
pub struct NonFungibleLocalIdConversionError(String);
impl_display_as_debug!(NonFungibleLocalIdConversionError);
