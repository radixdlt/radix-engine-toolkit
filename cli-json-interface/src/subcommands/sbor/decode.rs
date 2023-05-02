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

/// Decodes a Manifest and Scrypto SBOR encoded payloads.
#[derive(Parser, Debug)]
pub struct Decode {
    /// The SBOR encoded payload to decode
    #[clap(short, long)]
    payload: String,

    /// The network id to use. This is primarily used for decoding addresses
    #[clap(short, long)]
    network_id: u8,
}

impl Decode {
    pub fn run<O: std::io::Write>(&self, out: &mut O) -> Result<()> {
        let input = sbor_decode::Input {
            encoded_value: hex::decode(&self.payload)?,
            network_id: self.network_id,
        };
        let output = sbor_decode::Handler::fulfill(input).map_err(|error| {
            RETError::InvocationHandlingError(InvocationHandlingError::SborDecodeError(error))
        })?;
        pretty_print(&output, out)
    }
}
