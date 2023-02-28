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

use crate::error::Result;
use crate::request::traits::Handler;
use toolkit_derive::serializable;

// =================
// Model Definition
// =================

/// This request takes in a [`Value`] and attempts to SBOR encode it and return back an SBOR byte
/// array.
#[serializable]
pub struct SborEncodeRequest {
    // /// The value to SBOR encode.
    // #[serde(flatten)]
    // pub value: Value,
}

/// The response from the [`SborEncodeRequest`].
#[serializable]
pub struct SborEncodeResponse {
    /// A byte array serialized as a hex string of the SBOR encoded value.
    #[schemars(with = "String")]
    #[schemars(regex(pattern = "[0-9a-fA-F]+"))]
    #[serde_as(as = "serde_with::hex::Hex")]
    pub encoded_value: Vec<u8>,
}

// ===============
// Implementation
// ===============

pub struct SborEncodeHandler;

impl Handler<SborEncodeRequest, SborEncodeResponse> for SborEncodeHandler {
    fn pre_process(request: SborEncodeRequest) -> Result<SborEncodeRequest> {
        Ok(request)
    }

    fn handle(_request: &SborEncodeRequest) -> Result<SborEncodeResponse> {
        todo!()
    }

    fn post_process(
        _: &SborEncodeRequest,
        response: SborEncodeResponse,
    ) -> Result<SborEncodeResponse> {
        Ok(response)
    }
}
