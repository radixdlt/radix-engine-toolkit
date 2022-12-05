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
use crate::model::helper::ValueSerializationProxy;
use crate::traits::{Request, Validate};

use scrypto::prelude::{FromPublicKey, NonFungibleAddress, PublicKey};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

// ==========================
// Request & Response Models
// ==========================

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeriveNonFungibleAddressFromPublicKeyRequest {
    #[serde(flatten)]
    pub public_key: PublicKey,
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeriveNonFungibleAddressFromPublicKeyResponse {
    #[serde_as(as = "ValueSerializationProxy")]
    pub non_fungible_address: NonFungibleAddress,
}

// ===========
// Validation
// ===========

impl Validate for DeriveNonFungibleAddressFromPublicKeyRequest {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

impl Validate for DeriveNonFungibleAddressFromPublicKeyResponse {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

// =======================
// Request Implementation
// =======================

impl<'r> Request<'r, DeriveNonFungibleAddressFromPublicKeyResponse>
    for DeriveNonFungibleAddressFromPublicKeyRequest
{
    fn handle_request(self) -> Result<DeriveNonFungibleAddressFromPublicKeyResponse, Error> {
        let non_fungible_address = NonFungibleAddress::from_public_key(&self.public_key);

        Ok(DeriveNonFungibleAddressFromPublicKeyResponse {
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
