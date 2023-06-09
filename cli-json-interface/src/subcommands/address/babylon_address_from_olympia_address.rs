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
use radix_engine_toolkit::request::{
    DeriveBabylonAddressFromOlympiaAddressHandler, DeriveBabylonAddressFromOlympiaAddressRequest,
    Handler,
};

#[derive(Parser, Debug)]
/// Derives the Babylon account address for a given Olympia account address.
pub struct BabylonAddressFromOlympiaAddress {
    /// The Olympia account address to derive the Babylon address for.
    #[clap(short, long)]
    olympia_account_address: String,

    /// The network id to derive the known addresses for.
    #[clap(short, long)]
    network_id: u8,
}

impl BabylonAddressFromOlympiaAddress {
    pub fn run<O: std::io::Write>(&self, out: &mut O) -> Result<()> {
        let request = DeriveBabylonAddressFromOlympiaAddressRequest {
            network_id: self.network_id,
            olympia_account_address: self.olympia_account_address.clone(),
        };
        let response = DeriveBabylonAddressFromOlympiaAddressHandler::fulfill(request)?;
        pretty_print(&response, out)
    }
}