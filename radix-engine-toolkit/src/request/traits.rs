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

/// A trait describing request handlers - their main responsibility is handling request
/// preprocessing, handling, and postprocessing.
pub trait Handler<I, O> {
    /// Performs request preprocessing - example, validation of requests to ensure that values and
    /// instructions all follow expected format.
    fn pre_process(request: I) -> Result<I>;

    /// The main request handler describing how to fulfill this request
    fn handle(request: &I) -> Result<O>;

    /// Performs all post processing of requests - example, aliasing values.
    fn post_process(request: &I, response: O) -> O;

    /// Fulfills the request by performing preprocessing, handling, and post processing
    fn fulfill(request: I) -> Result<O> {
        // pre-process request
        let request = Self::pre_process(request)?;

        // handle request and perform post-processing
        Self::handle(&request).map(|response| Self::post_process(&request, response))
    }
}
