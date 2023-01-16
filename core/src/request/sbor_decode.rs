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
pub struct SborDecodeRequest {
    #[serde_as(as = "serde_with::hex::Hex")]
    pub encoded_value: Vec<u8>,

    #[schemars(with = "String")]
    #[schemars(regex(pattern = "[0-9]+"))]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub network_id: u8,
}

#[serializable]
pub struct SborDecodeResponse {
    #[serde(flatten)]
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

    fn post_process(_: &SborDecodeRequest, mut response: SborDecodeResponse) -> SborDecodeResponse {
        for value in response.borrow_values_mut().iter_mut() {
            value.alias();
        }
        response
    }
}

impl ValueRef for SborDecodeResponse {
    fn borrow_values(&self) -> Vec<&Value> {
        vec![&self.value]
    }

    fn borrow_values_mut(&mut self) -> Vec<&mut Value> {
        vec![&mut self.value]
    }
}
