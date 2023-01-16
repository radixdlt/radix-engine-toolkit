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
use crate::traits::ValueRef;
use crate::value::Value;
use serializable::serializable;

// =================
// Model Definition
// =================

#[serializable]
pub struct SborEncodeRequest {
    #[serde(flatten)]
    pub value: Value,
}

#[serializable]
pub struct SborEncodeResponse {
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
        // Validate all `Value`s in the request. Ensure that:
        //     1. All addresses are of the network provided in the request.
        //     2. All single-type collections are of a single kind.
        request
            .borrow_values()
            .iter()
            .map(|value| value.validate(None))
            .collect::<Result<Vec<_>>>()?;
        Ok(request)
    }

    fn handle(request: &SborEncodeRequest) -> Result<SborEncodeResponse> {
        request
            .value
            .encode()
            .map(|encoded_value| SborEncodeResponse { encoded_value })
    }

    fn post_process(_: &SborEncodeRequest, response: SborEncodeResponse) -> SborEncodeResponse {
        response
    }
}

impl ValueRef for SborEncodeRequest {
    fn borrow_values(&self) -> Vec<&Value> {
        vec![&self.value]
    }

    fn borrow_values_mut(&mut self) -> Vec<&mut Value> {
        vec![&mut self.value]
    }
}
