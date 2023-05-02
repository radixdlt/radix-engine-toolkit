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

use crate::error::Result;
use crate::utils::pretty_print;
use clap::Parser;
use radix_engine_toolkit::error::{InvocationHandlingError, RETError};
use radix_engine_toolkit::functions::*;

#[derive(Parser, Debug)]
/// Hashes some data using the hashing algorithm of Scrypto and the Radix Engine. Currently, this
/// is Blake2b with 256 bit digests.
pub struct Hash {
    /// A hex-encoded string of the data to hash
    #[clap(short, long)]
    data: String,
}

impl Hash {
    pub fn run<O: std::io::Write>(&self, out: &mut O) -> Result<()> {
        let input = hash::Input {
            payload: hex::decode(&self.data)?,
        };
        let output = hash::Handler::fulfill(input).map_err(|error| {
            RETError::InvocationHandlingError(InvocationHandlingError::HashError(error))
        })?;
        pretty_print(&output, out)
    }
}
