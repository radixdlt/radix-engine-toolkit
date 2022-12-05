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

use crate::error::Error;
use crate::model::address::NetworkAwareResourceAddress;
use crate::model::helper::ValueSerializationProxy;
use crate::traits::{Request, Validate};

use scrypto::prelude::{NonFungibleAddress, NonFungibleId};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

// ==========================
// Request & Response Models
// ==========================

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeriveNonFungibleAddressRequest {
    #[serde_as(as = "ValueSerializationProxy")]
    pub resource_address: NetworkAwareResourceAddress,

    #[serde_as(as = "ValueSerializationProxy")]
    pub non_fungible_id: NonFungibleId,
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeriveNonFungibleAddressResponse {
    #[serde_as(as = "ValueSerializationProxy")]
    pub non_fungible_address: NonFungibleAddress,
}

// ===========
// Validation
// ===========

impl Validate for DeriveNonFungibleAddressRequest {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

impl Validate for DeriveNonFungibleAddressResponse {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

// =======================
// Request Implementation
// =======================

impl<'r> Request<'r, DeriveNonFungibleAddressResponse> for DeriveNonFungibleAddressRequest {
    fn handle_request(self) -> Result<DeriveNonFungibleAddressResponse, Error> {
        let non_fungible_address =
            NonFungibleAddress::new(self.resource_address.address, self.non_fungible_id);

        Ok(DeriveNonFungibleAddressResponse {
            non_fungible_address,
        })
    }
}

// ======
// Tests
// ======

#[cfg(test)]
mod tests {
    // TODO: Unit tests for this request type
}
