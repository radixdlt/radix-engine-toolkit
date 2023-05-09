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

use scrypto::prelude::NonFungibleLocalId as ScryptoNonFungibleLocalId;
use toolkit_derive::serializable;

use crate::{impl_display_as_debug, utils::debug_string};

/// Represents non-fungible ids which is a discriminated union of the different types that
/// non-fungible ids may be.
#[serializable]
#[schemars(example = "example")]
pub struct NonFungibleLocalId(String);

impl ToString for NonFungibleLocalId {
    fn to_string(&self) -> String {
        self.0.to_owned()
    }
}

impl TryFrom<ScryptoNonFungibleLocalId> for NonFungibleLocalId {
    type Error = NonFungibleLocalIdConversionError;

    fn try_from(value: ScryptoNonFungibleLocalId) -> Result<Self, Self::Error> {
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<NonFungibleLocalId> for ScryptoNonFungibleLocalId {
    type Error = NonFungibleLocalIdConversionError;

    fn try_from(value: NonFungibleLocalId) -> Result<Self, Self::Error> {
        value
            .to_string()
            .parse()
            .map_err(|error| NonFungibleLocalIdConversionError(debug_string(error)))
    }
}

#[serializable]
pub struct NonFungibleLocalIdConversionError(String);
impl_display_as_debug!(NonFungibleLocalIdConversionError);

fn example() -> NonFungibleLocalId {
    NonFungibleLocalId::try_from(ScryptoNonFungibleLocalId::integer(1)).unwrap()
}
