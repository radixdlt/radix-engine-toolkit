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
use radix_engine_toolkit::{
    error::{InvocationHandlingError, RETError},
    functions::*,
};

/// Decodes the Bech32 address revealing some information on what exactly does it address.
#[derive(Parser, Debug)]
pub struct Decode {
    /// The Bech32m encoded address to decode.
    #[clap(short, long)]
    address: String,
}

impl Decode {
    pub fn run<O: std::io::Write>(&self, out: &mut O) -> Result<()> {
        let request = decode_address::Input {
            address: self.address.clone().parse().unwrap(),
        };
        let response = decode_address::Handler::fulfill(request).map_err(|error| {
            RETError::InvocationHandlingError(InvocationHandlingError::DecodeAddressError(error))
        })?;
        pretty_print(&response, out)
    }
}
