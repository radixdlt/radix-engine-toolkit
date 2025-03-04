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

pub enum OlympiaNetwork {
    Mainnet,
    Stokenet,
    Releasenet,
    RCNet,
    Milestonenet,
    Devopsnet,
    Sandpitnet,
    Localnet,
}

impl OlympiaNetwork {
    pub const fn hrp(&self) -> &str {
        match self {
            Self::Mainnet => "rdx",
            Self::Stokenet => "tdx",
            Self::Releasenet => "tdx3",
            Self::RCNet => "tdx4",
            Self::Milestonenet => "tdx5",
            Self::Devopsnet => "tdx6",
            Self::Sandpitnet => "tdx7",
            Self::Localnet => "ddx",
        }
    }
}
