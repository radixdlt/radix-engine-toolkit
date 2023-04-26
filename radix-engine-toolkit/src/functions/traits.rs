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

use std::fmt::Debug;

use crate::error::InvocationHandlingError;

/// A trait describing invocation handlers - their main responsibility is handling invocation
/// preprocessing, handling, and postprocessing.
pub trait InvocationHandler<I, O> {
    type Error: Debug + Into<InvocationHandlingError>;

    /// Performs pre processing on the invocation input
    fn pre_process(invocation: I) -> Result<I, Self::Error>;

    /// The main invocation handler describing how to fulfill this invocation
    fn handle(invocation: &I) -> Result<O, Self::Error>;

    /// Performs all post processing of invocation output - example, aliasing values.
    fn post_process(invocation: &I, response: O) -> Result<O, Self::Error>;

    /// Fulfills the invocation by performing preprocessing, handling, and post processing
    fn fulfill(invocation: I) -> Result<O, Self::Error> {
        // pre-process invocation
        let invocation = Self::pre_process(invocation)?;

        // handle invocation and perform post-processing
        Self::handle(&invocation).and_then(|response| Self::post_process(&invocation, response))
    }
}
