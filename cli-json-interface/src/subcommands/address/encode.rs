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

use crate::utils::pretty_print;
use clap::Parser;
use radix_engine_toolkit::request::{EncodeAddressHandler, EncodeAddressRequest, Handler};

#[derive(Parser, Debug)]
/// Encodes a raw address into a Bech32 encoded address.
pub struct Encode {
    /// The raw address to Bech32m encode. This is 27-byte long raw address serialized as a 54
    /// character long hexadecimal string.
    #[clap(short, long)]
    raw_address: String,

    /// The network id to use for encoding the address.
    #[clap(short, long)]
    network_id: u8,
}

impl Encode {
    pub fn run<O: std::io::Write>(&self, out: &mut O) -> Result<()> {
        let request = EncodeAddressRequest {
            address_bytes: hex::decode(&self.raw_address)?,
            network_id: self.network_id,
        };
        let response = EncodeAddressHandler::fulfill(request)?;
        pretty_print(&response, out)
    }
}
