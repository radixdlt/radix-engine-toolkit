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

use scrypto::prelude::{ComponentAddress, PublicKey};
use serializable::serializable;

use crate::error::Result;
use crate::{Handler, NetworkAwareComponentAddress};

// =================
// Model Definition
// =================

#[serializable]
pub struct DeriveVirtualAccountAddressRequest {
    #[schemars(with = "String")]
    #[schemars(regex(pattern = "[0-9]+"))]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub network_id: u8,

    #[schemars(with = "crate::model::crypto::PublicKey")]
    #[serde_as(as = "serde_with::FromInto<crate::model::crypto::PublicKey>")]
    pub public_key: PublicKey,
}

#[serializable]
pub struct DeriveVirtualAccountAddressResponse {
    #[schemars(with = "crate::model::Value")]
    #[serde_as(as = "serde_with::TryFromInto<crate::model::Value>")]
    pub virtual_account_address: NetworkAwareComponentAddress,
}

// ===============
// Implementation
// ===============

pub struct DeriveVirtualAccountAddressHandler;

impl Handler<DeriveVirtualAccountAddressRequest, DeriveVirtualAccountAddressResponse>
    for DeriveVirtualAccountAddressHandler
{
    fn pre_process(
        request: DeriveVirtualAccountAddressRequest,
    ) -> Result<DeriveVirtualAccountAddressRequest> {
        Ok(request)
    }

    fn handle(
        request: &DeriveVirtualAccountAddressRequest,
    ) -> Result<DeriveVirtualAccountAddressResponse> {
        Ok(DeriveVirtualAccountAddressResponse {
            virtual_account_address: NetworkAwareComponentAddress {
                network_id: request.network_id,
                address: ComponentAddress::virtual_account_from_public_key(&request.public_key),
            },
        })
    }

    fn post_process(
        _: &DeriveVirtualAccountAddressRequest,
        response: DeriveVirtualAccountAddressResponse,
    ) -> DeriveVirtualAccountAddressResponse {
        response
    }
}
