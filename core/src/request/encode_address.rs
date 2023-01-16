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

use crate::{model::address::EntityAddress, Handler};
use serializable::serializable;

// =================
// Model Definition
// =================

#[serializable]
pub struct EncodeAddressRequest {
    #[schemars(with = "String")]
    #[schemars(length(equal = 54))]
    #[schemars(regex(pattern = "[0-9a-fA-F]+"))]
    #[serde_as(as = "serde_with::hex::Hex")]
    pub address_bytes: Vec<u8>,

    #[schemars(with = "String")]
    #[schemars(regex(pattern = "[0-9]+"))]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub network_id: u8,
}

#[serializable]
pub struct EncodeAddressResponse {
    #[serde(flatten)]
    address: EntityAddress,
}

// ===============
// Implementation
// ===============

struct EncodeAddressHandler;

impl Handler<EncodeAddressRequest, EncodeAddressResponse> for EncodeAddressHandler {
    fn pre_process(request: EncodeAddressRequest) -> crate::Result<EncodeAddressRequest> {
        Ok(request)
    }

    fn handle(request: &EncodeAddressRequest) -> crate::Result<EncodeAddressResponse> {
        EntityAddress::from_u8_array(&request.address_bytes, request.network_id)
            .map(|address| EncodeAddressResponse { address })
    }

    fn post_process(
        _: &EncodeAddressRequest,
        response: EncodeAddressResponse,
    ) -> EncodeAddressResponse {
        response
    }
}
