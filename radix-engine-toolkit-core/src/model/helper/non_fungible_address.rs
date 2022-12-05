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
            non_fungible_id: native_non_fungible_address.non_fungible_id(),
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
