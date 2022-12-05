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
use crate::model::Value;
use crate::traits::{Request, Validate};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

// ==========================
// Request & Response Models
// ==========================

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SBOREncodeRequest {
    #[serde(flatten)]
    pub value: Value,
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SBOREncodeResponse {
    #[serde_as(as = "serde_with::hex::Hex")]
    pub encoded_value: Vec<u8>,
}

// ===========
// Validation
// ===========

impl Validate for SBOREncodeRequest {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

impl Validate for SBOREncodeResponse {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

// =======================
// Request Implementation
// =======================

impl<'r> Request<'r, SBOREncodeResponse> for SBOREncodeRequest {
    fn handle_request(self) -> Result<SBOREncodeResponse, Error> {
        Ok(SBOREncodeResponse {
            encoded_value: self.value.encode()?,
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
