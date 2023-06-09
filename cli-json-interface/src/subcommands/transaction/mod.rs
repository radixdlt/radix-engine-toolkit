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

mod analyze_manifest;
mod convert_manifest;
mod decompile;

/// A subcommand for all transaction related commands.
#[derive(clap::Subcommand, Debug)]
pub enum Transaction {
    AnalyzeManifest(analyze_manifest::AnalyzeManifest),
    ConvertManifest(convert_manifest::ConvertManifest),
    Decompile(decompile::Decompile),
}

impl Transaction {
    pub fn run<O: std::io::Write>(&self, out: &mut O) -> crate::error::Result<()> {
        match self {
            Self::AnalyzeManifest(cmd) => cmd.run(out),
            Self::ConvertManifest(cmd) => cmd.run(out),
            Self::Decompile(cmd) => cmd.run(out),
        }
    }
}