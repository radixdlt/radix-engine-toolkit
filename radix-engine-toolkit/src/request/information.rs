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
use serializable::serializable;

// =================
// Model Definition
// =================

#[serializable]
/// The request provides information information on the currently in-use radix engine toolkit such
/// as the version of the radix engine toolkit. In most cases, this is the first function written
/// when integrating new clients; so, this function is often times seen as the "Hello World" example
/// of the radix engine toolkit.
pub struct InformationRequest {}

/// The response from [`InformationRequest`]s
#[serializable]
pub struct InformationResponse {
    /// A SemVer string of the version of the Radix Engine Toolkit. Ideally, if the toolkit is
    /// version X then that means that it is compatible with version X of Scrypto.
    pub package_version: String,
}

// ===============
// Implementation
// ===============

pub struct InformationHandler;

impl Handler<InformationRequest, InformationResponse> for InformationHandler {
    fn pre_process(request: InformationRequest) -> Result<InformationRequest> {
        Ok(request)
    }

    fn handle(_: &InformationRequest) -> Result<InformationResponse> {
        let response = InformationResponse {
            package_version: env!("CARGO_PKG_VERSION").into(),
        };
        Ok(response)
    }

    fn post_process(_: &InformationRequest, response: InformationResponse) -> InformationResponse {
        response
    }
}
