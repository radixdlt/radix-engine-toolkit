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

mod address;

mod command;
mod error;
mod utils;

use crate::error::Result;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None, name = "ret-cli")]
pub struct RetCli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Information(command::Information),
    DecodeAddress(command::DecodeAddress),
    EncodeAddress(command::EncodeAddress),
    ConvertManifest(command::ConvertManifest),
    AnalyzeManifest(command::AnalyzeManifest),
}

pub fn main() -> Result<()> {
    let cli = RetCli::parse();
    let mut out = std::io::stdout();

    match cli.command {
        Command::Information(cmd) => cmd.run(&mut out)?,
        Command::ConvertManifest(cmd) => cmd.run(&mut out)?,
        Command::DecodeAddress(cmd) => cmd.run(&mut out)?,
        Command::EncodeAddress(cmd) => cmd.run(&mut out)?,
        Command::AnalyzeManifest(cmd) => cmd.run(&mut out)?,
    };

    Ok(())
}
