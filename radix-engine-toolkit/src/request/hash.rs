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

use crate::error::Result;
use crate::model::constants::RADIX_ENGINE_HASH_LENGTH;

use scrypto::prelude::hash;
use toolkit_derive::serializable;

use super::traits::Handler;

// =================
// Model Definition
// =================

/// Hashes data using the hashing algorithm used in Scrypto and the Radix Engine
#[serializable]
pub struct HashRequest {
    /// The payload to hash
    #[schemars(with = "String")]
    #[schemars(regex(pattern = "[0-9a-fA-F]+"))]
    #[serde_as(as = "serde_with::hex::Hex")]
    pub payload: Vec<u8>,
}

/// The response of the [`HashRequest`]
#[serializable]
pub struct HashResponse {
    #[schemars(with = "String")]
    #[schemars(regex(pattern = "[0-9a-fA-F]+"))]
    #[serde_as(as = "serde_with::hex::Hex")]
    pub value: [u8; RADIX_ENGINE_HASH_LENGTH],
}

// ===============
// Implementation
// ===============

pub struct HashHandler;

impl Handler<HashRequest, HashResponse> for HashHandler {
    fn pre_process(request: HashRequest) -> Result<HashRequest> {
        Ok(request)
    }

    fn handle(request: &HashRequest) -> Result<HashResponse> {
        let response = HashResponse {
            value: hash(&request.payload).0,
        };
        Ok(response)
    }

    fn post_process(_: &HashRequest, response: HashResponse) -> Result<HashResponse> {
        Ok(response)
    }
}
