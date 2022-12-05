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

//! Defines the request and response models used in an information request. This is a simple request
//! which has no arguments and returns an information response containing the current version of the
//! package. You may treat this request as a "hello world" request of sorts as it can be used to
//! check if the communication with this library is successful or not.

use crate::error::Error;
use crate::traits::{Request, Validate};

use serde::{Deserialize, Serialize};

// ==========================
// Request & Response Models
// ==========================

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InformationRequest {}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InformationResponse {
    pub package_version: String,
}

// ===========
// Validation
// ===========

impl Validate for InformationRequest {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

impl Validate for InformationResponse {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

// =======================
// Request Implementation
// =======================

impl<'r> Request<'r, InformationResponse> for InformationRequest {
    fn handle_request(self) -> Result<InformationResponse, Error> {
        Ok(InformationResponse {
            package_version: env!("CARGO_PKG_VERSION").into(),
        })
    }
}

// ======
// Tests
// ======

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn information_handler_returns_ok() {
        let response = InformationRequest {}.fulfill_request();
        assert!(matches!(response, Ok(_)));
    }
}
