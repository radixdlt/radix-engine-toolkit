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

use crate::prelude::*;

#[derive(Clone, Debug, Object)]
pub struct OlympiaAddress(pub(crate) String);

#[uniffi::export]
impl OlympiaAddress {
    #[uniffi::constructor]
    pub fn new(address: String) -> Arc<Self> {
        Arc::new(Self(address))
    }

    pub fn as_str(&self) -> String {
        self.0.clone()
    }

    pub fn public_key(&self) -> Result<PublicKey> {
        derive_public_key_from_olympia_account_address(Arc::new(self.clone()))
    }
}

#[derive(Clone, Debug, Enum)]
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

impl From<toolkit::OlympiaNetwork> for OlympiaNetwork {
    fn from(value: toolkit::OlympiaNetwork) -> Self {
        match value {
            toolkit::OlympiaNetwork::Mainnet => Self::Mainnet,
            toolkit::OlympiaNetwork::Stokenet => Self::Stokenet,
            toolkit::OlympiaNetwork::Releasenet => Self::Releasenet,
            toolkit::OlympiaNetwork::RCNet => Self::RCNet,
            toolkit::OlympiaNetwork::Milestonenet => Self::Milestonenet,
            toolkit::OlympiaNetwork::Devopsnet => Self::Devopsnet,
            toolkit::OlympiaNetwork::Sandpitnet => Self::Sandpitnet,
            toolkit::OlympiaNetwork::Localnet => Self::Localnet,
        }
    }
}

impl From<OlympiaNetwork> for toolkit::OlympiaNetwork {
    fn from(value: OlympiaNetwork) -> Self {
        match value {
            OlympiaNetwork::Mainnet => Self::Mainnet,
            OlympiaNetwork::Stokenet => Self::Stokenet,
            OlympiaNetwork::Releasenet => Self::Releasenet,
            OlympiaNetwork::RCNet => Self::RCNet,
            OlympiaNetwork::Milestonenet => Self::Milestonenet,
            OlympiaNetwork::Devopsnet => Self::Devopsnet,
            OlympiaNetwork::Sandpitnet => Self::Sandpitnet,
            OlympiaNetwork::Localnet => Self::Localnet,
        }
    }
}
