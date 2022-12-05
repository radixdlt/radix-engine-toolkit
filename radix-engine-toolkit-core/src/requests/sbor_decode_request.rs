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

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SBORDecodeRequest {
    #[serde_as(as = "serde_with::hex::Hex")]
    pub encoded_value: Vec<u8>,
    pub network_id: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SBORDecodeResponse {
    #[serde(flatten)]
    pub value: Value,
}

// ===========
// Validation
// ===========

impl Validate for SBORDecodeRequest {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

impl Validate for SBORDecodeResponse {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

// =======================
// Request Implementation
// =======================

impl<'r> Request<'r, SBORDecodeResponse> for SBORDecodeRequest {
    fn handle_request(self) -> Result<SBORDecodeResponse, Error> {
        Ok(SBORDecodeResponse {
            value: Value::decode(&self.encoded_value, self.network_id)?,
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
