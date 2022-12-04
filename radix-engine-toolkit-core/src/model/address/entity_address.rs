use std::fmt::Display;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::network_aware_address::*;
use crate::{error::Error, traits::ValidateWithContext};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "address")]
pub enum EntityAddress {
    ComponentAddress(NetworkAwareComponentAddress),
    ResourceAddress(NetworkAwareResourceAddress),
    PackageAddress(NetworkAwarePackageAddress),
    SystemAddress(NetworkAwareSystemAddress),
}

impl EntityAddress {
    pub fn kind(&self) -> AddressKind {
        match self {
            Self::ComponentAddress(component_address) => match component_address.address {
                scrypto::prelude::ComponentAddress::Normal(_) => AddressKind::NormalComponent,
                scrypto::prelude::ComponentAddress::Account(_) => AddressKind::AccountComponent,
                scrypto::prelude::ComponentAddress::EcdsaSecp256k1VirtualAccount(_) => {
                    AddressKind::EcdsaSecp256k1VirtualAccount
                }
                scrypto::prelude::ComponentAddress::EddsaEd25519VirtualAccount(_) => {
                    AddressKind::EddsaEd25519VirtualAccount
                }
            },
            Self::ResourceAddress(resource_address) => match resource_address.address {
                scrypto::prelude::ResourceAddress::Normal(_) => AddressKind::Resource,
            },
            Self::PackageAddress(package_address) => match package_address.address {
                scrypto::prelude::PackageAddress::Normal(_) => AddressKind::Package,
            },
            Self::SystemAddress(system_address) => match system_address.address {
                scrypto::prelude::SystemAddress::EpochManager(_) => AddressKind::SystemEpochManager,
                scrypto::prelude::SystemAddress::Clock(_) => AddressKind::SystemClock,
            },
        }
    }

    pub fn network_id(&self) -> u8 {
        match self {
            Self::ComponentAddress(address) => address.network_id,
            Self::ResourceAddress(address) => address.network_id,
            Self::PackageAddress(address) => address.network_id,
            Self::SystemAddress(address) => address.network_id,
        }
    }

    pub fn from_u8_array(array: &[u8], network_id: u8) -> Result<Self, Error> {
        if let Ok(component_address) =
            NetworkAwareComponentAddress::from_u8_array(array, network_id)
        {
            Ok(Self::ComponentAddress(component_address))
        } else if let Ok(resource_address) =
            NetworkAwareResourceAddress::from_u8_array(array, network_id)
        {
            Ok(Self::ResourceAddress(resource_address))
        } else if let Ok(package_address) =
            NetworkAwarePackageAddress::from_u8_array(array, network_id)
        {
            Ok(Self::PackageAddress(package_address))
        } else {
            Err(Error::UnrecognizedAddressFormat)
        }
    }
}

impl From<NetworkAwareComponentAddress> for EntityAddress {
    fn from(address: NetworkAwareComponentAddress) -> Self {
        Self::ComponentAddress(address)
    }
}

impl From<NetworkAwareResourceAddress> for EntityAddress {
    fn from(address: NetworkAwareResourceAddress) -> Self {
        Self::ResourceAddress(address)
    }
}

impl From<NetworkAwarePackageAddress> for EntityAddress {
    fn from(address: NetworkAwarePackageAddress) -> Self {
        Self::PackageAddress(address)
    }
}

impl Display for EntityAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityAddress::ComponentAddress(address) => write!(f, "{}", address),
            EntityAddress::ResourceAddress(address) => write!(f, "{}", address),
            EntityAddress::PackageAddress(address) => write!(f, "{}", address),
            EntityAddress::SystemAddress(address) => write!(f, "{}", address),
        }
    }
}

impl FromStr for EntityAddress {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(component_address) = NetworkAwareComponentAddress::from_str(s) {
            Ok(Self::ComponentAddress(component_address))
        } else if let Ok(resource_address) = NetworkAwareResourceAddress::from_str(s) {
            Ok(Self::ResourceAddress(resource_address))
        } else if let Ok(package_address) = NetworkAwarePackageAddress::from_str(s) {
            Ok(Self::PackageAddress(package_address))
        } else {
            Err(Error::UnrecognizedAddressFormat)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AddressKind {
    Resource,
    Package,
    SystemClock,
    SystemEpochManager,

    NormalComponent,
    AccountComponent,
    EcdsaSecp256k1VirtualAccount,
    EddsaEd25519VirtualAccount,
}

impl ValidateWithContext<u8> for EntityAddress {
    fn validate(&self, network_id: u8) -> Result<(), Error> {
        if self.network_id() == network_id {
            Ok(())
        } else {
            Err(Error::NetworkMismatchError {
                expected: network_id,
                found: network_id,
            })
        }
    }
}
