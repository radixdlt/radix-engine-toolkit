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
use crate::request::Handler;
use crate::value::Value;
use crate::{traverse_value, ValueAliasingVisitor};
use serializable::serializable;

// =================
// Model Definition
// =================

/// Takes in a byte array of SBOR byte and attempts to decode it to a [`Value`]. Since some of the
/// types in the [`Value`] model are network aware, this request also takes in a network id which
/// is primarily used for the Bech32m encoding of addresses.
#[serializable]
pub struct SborDecodeRequest {
    /// A byte array serialized as a hex string of the SBOR buffer to attempt to decode as a
    /// [`Value`]
    #[schemars(with = "String")]
    #[schemars(regex(pattern = "[0-9a-fA-F]+"))]
    #[serde_as(as = "serde_with::hex::Hex")]
    pub encoded_value: Vec<u8>,

    /// An 8 bit unsigned integer serialized as a string which represents the id of the network
    /// that the decoded data will be used on. This is primarily used for the Bech32m encoding of
    /// addresses.
    #[schemars(with = "String")]
    #[schemars(regex(pattern = "[0-9]+"))]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub network_id: u8,
}

/// The response from the [`SborDecodeRequest`].
#[serializable]
pub struct SborDecodeResponse {
    /// A value representing the SBOR decoded form of the passed SBOR buffer.
    pub value: Value,
}

// ===============
// Implementation
// ===============

pub struct SborDecodeHandler;

impl Handler<SborDecodeRequest, SborDecodeResponse> for SborDecodeHandler {
    fn pre_process(request: SborDecodeRequest) -> Result<SborDecodeRequest> {
        Ok(request)
    }

    fn handle(request: &SborDecodeRequest) -> Result<SborDecodeResponse> {
        Value::decode(&request.encoded_value, request.network_id)
            .map(|value| SborDecodeResponse { value })
    }

    fn post_process(
        _: &SborDecodeRequest,
        mut response: SborDecodeResponse,
    ) -> Result<SborDecodeResponse> {
        // Visitors
        let mut aliasing_visitor = ValueAliasingVisitor::default();

        // Traverse value with visitors
        traverse_value(&mut response.value, &mut [&mut aliasing_visitor])?;

        // The aliasing visitor performs all of the modifications in place as it meets them. Nothing
        // else needs to be done here.

        Ok(response)
    }
}
