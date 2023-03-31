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

mod babylon_address_from_olympia_address;
mod decode;
mod encode;
mod known_addresses;
mod virtual_account_address;
mod virtual_identity_address;

/// A subcommand for all address related commands.
#[derive(clap::Subcommand, Debug)]
pub enum Address {
    Encode(encode::Encode),
    Decode(decode::Decode),
    KnownAddresses(known_addresses::KnownAddresses),
    VirtualAccountAddress(virtual_account_address::VirtualAccountAddress),
    VirtualIdentityAddress(virtual_identity_address::VirtualIdentityAddress),
    BabylonAccountAddressFromOlympiaAddress(
        babylon_address_from_olympia_address::BabylonAddressFromOlympiaAddress,
    ),
}

impl Address {
    pub fn run<O: std::io::Write>(&self, out: &mut O) -> crate::error::Result<()> {
        match self {
            Self::Encode(cmd) => cmd.run(out),
            Self::Decode(cmd) => cmd.run(out),
            Self::KnownAddresses(cmd) => cmd.run(out),
            Self::VirtualAccountAddress(cmd) => cmd.run(out),
            Self::VirtualIdentityAddress(cmd) => cmd.run(out),
            Self::BabylonAccountAddressFromOlympiaAddress(cmd) => cmd.run(out),
        }
    }
}
