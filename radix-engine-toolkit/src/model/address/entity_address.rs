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

use crate::error::{Error, Result};
use crate::model::address::network_aware_address::*;
use scrypto::radix_engine_interface::address::EntityType;
use serializable::serializable;
use std::fmt::Display;
use std::str::FromStr;

// =================
// Model Definition
// =================

/// A discriminated union of entity addresses where addresses are serialized as a Bech32m encoded
/// string.
#[serializable]
#[serde(tag = "type")]
pub enum EntityAddress {
    /// Represents a Bech32m encoded human-readable component address. This address is serialized
    /// as a human-readable bech32m encoded string.
    ComponentAddress {
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        address: NetworkAwareComponentAddress,
    },

    /// Represents a Bech32m encoded human-readable resource address. This address is serialized
    /// as a human-readable bech32m encoded string.
    ResourceAddress {
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        address: NetworkAwareResourceAddress,
    },

    /// Represents a Bech32m encoded human-readable package address. This address is serialized
    /// as a human-readable bech32m encoded string.
    PackageAddress {
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        address: NetworkAwarePackageAddress,
    },
}

// ===============
// Implementation
// ===============

impl EntityAddress {
    pub fn kind(&self) -> EntityType {
        match self {
            Self::ComponentAddress { address } => match address.address {
                scrypto::prelude::ComponentAddress::Normal(_)
                | scrypto::prelude::ComponentAddress::AccessController(_) => {
                    EntityType::NormalComponent
                }
                scrypto::prelude::ComponentAddress::Account(_) => EntityType::AccountComponent,
                scrypto::prelude::ComponentAddress::EcdsaSecp256k1VirtualAccount(_) => {
                    EntityType::EcdsaSecp256k1VirtualAccountComponent
                }
                scrypto::prelude::ComponentAddress::EddsaEd25519VirtualAccount(_) => {
                    EntityType::EddsaEd25519VirtualAccountComponent
                }
                scrypto::prelude::ComponentAddress::Identity(_) => EntityType::IdentityComponent,
                scrypto::prelude::ComponentAddress::EcdsaSecp256k1VirtualIdentity(_) => {
                    EntityType::EcdsaSecp256k1VirtualIdentityComponent
                }
                scrypto::prelude::ComponentAddress::EddsaEd25519VirtualIdentity(_) => {
                    EntityType::EddsaEd25519VirtualIdentityComponent
                }
                scrypto::prelude::ComponentAddress::Clock(_) => EntityType::Clock,
                scrypto::prelude::ComponentAddress::EpochManager(_) => EntityType::EpochManager,
                scrypto::prelude::ComponentAddress::Validator(_) => EntityType::Validator,
            },
            Self::ResourceAddress { address } => match address.address {
                scrypto::prelude::ResourceAddress::Normal(_) => EntityType::Resource,
            },
            Self::PackageAddress { address } => match address.address {
                scrypto::prelude::PackageAddress::Normal(_) => EntityType::Package,
            },
        }
    }

    pub fn network_id(&self) -> u8 {
        match self {
            Self::ComponentAddress { address } => address.network_id,
            Self::ResourceAddress { address } => address.network_id,
            Self::PackageAddress { address } => address.network_id,
        }
    }

    pub fn from_u8_array(array: &[u8], network_id: u8) -> Result<Self> {
        if let Ok(address) = NetworkAwareComponentAddress::from_u8_array(array, network_id) {
            Ok(Self::ComponentAddress { address })
        } else if let Ok(address) = NetworkAwareResourceAddress::from_u8_array(array, network_id) {
            Ok(Self::ResourceAddress { address })
        } else if let Ok(address) = NetworkAwarePackageAddress::from_u8_array(array, network_id) {
            Ok(Self::PackageAddress { address })
        } else {
            Err(Error::UnrecognizedAddressFormat)
        }
    }
}

// =====
// Text
// =====

impl Display for EntityAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityAddress::ComponentAddress { address } => write!(f, "{}", address),
            EntityAddress::ResourceAddress { address } => write!(f, "{}", address),
            EntityAddress::PackageAddress { address } => write!(f, "{}", address),
        }
    }
}

impl FromStr for EntityAddress {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if let Ok(address) = NetworkAwareComponentAddress::from_str(s) {
            Ok(Self::ComponentAddress { address })
        } else if let Ok(address) = NetworkAwareResourceAddress::from_str(s) {
            Ok(Self::ResourceAddress { address })
        } else if let Ok(address) = NetworkAwarePackageAddress::from_str(s) {
            Ok(Self::PackageAddress { address })
        } else {
            Err(Error::UnrecognizedAddressFormat)
        }
    }
}
