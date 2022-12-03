use std::convert::TryFrom;
use std::fmt::Display;
use std::str::FromStr;

use scrypto_utils::copy_u8_array;

use serde::de::Error as DeserializationError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use scrypto::prelude::Hash;
use serde_with::{DeserializeAs, SerializeAs};

use crate::address::Bech32Manager;
use crate::error::Error;

use super::Value;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeId(pub (Hash, u32));

impl NodeId {
    pub fn to_bytes(&self) -> [u8; 36] {
        let mut node_id_bytes: Vec<u8> = self.0 .0.to_vec();
        node_id_bytes.extend(self.0 .1.to_le_bytes());
        copy_u8_array(&node_id_bytes)
    }

    pub fn from_bytes(vec: [u8; 36]) -> Self {
        let hash_bytes: &[u8] = &vec[0..32];
        let index_bytes: &[u8] = &vec[32..];

        let hash: Hash = Hash(copy_u8_array(hash_bytes));
        let index: u32 = u32::from_le_bytes(copy_u8_array(index_bytes));

        Self((hash, index))
    }
}

impl From<(Hash, u32)> for NodeId {
    fn from(value: (Hash, u32)) -> Self {
        Self(value)
    }
}

impl From<NodeId> for (Hash, u32) {
    fn from(entity_id: NodeId) -> Self {
        entity_id.0
    }
}

impl Serialize for NodeId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for NodeId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let node_id_string: &str = Deserialize::deserialize(deserializer)?;
        node_id_string
            .parse()
            .map_err(|_| DeserializationError::custom("Failed to parse node id from string"))
    }
}

impl ToString for NodeId {
    fn to_string(&self) -> String {
        hex::encode(self.to_bytes())
    }
}

impl FromStr for NodeId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let node_id_bytes: Vec<u8> = hex::decode(s)
            .map_err(|_| Error::DeserializationError(format!("Failed to decode node id: {}", s)))?;

        // TODO: Should not do a copy u8 array without first checking the length
        Ok(Self::from_bytes(copy_u8_array(&node_id_bytes)))
    }
}

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone, PartialOrd, Ord)]
#[serde(untagged)]
pub enum Identifier {
    String(String),
    U32(u32),
}

// Defines a network aware address. This is needed for the serialization and deserialization using
// serde.
macro_rules! define_network_aware_address {
    (
        $underlying_type: ty => $network_aware_struct_ident: ident,
        $encoding_method_ident: ident,
        $decoding_method_ident: ident
    ) => {
        #[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
        pub struct $network_aware_struct_ident {
            pub network_id: u8,
            pub address: $underlying_type,
        }

        impl $network_aware_struct_ident {
            pub fn from_u8_array(data: &[u8], network_id: u8) -> Result<Self, Error> {
                if let Ok(address) = <$underlying_type>::try_from(data) {
                    Ok($network_aware_struct_ident {
                        network_id,
                        address,
                    })
                } else {
                    Err(Error::UnrecognizedAddressFormat)
                }
            }
        }

        impl<'de> Deserialize<'de> for $network_aware_struct_ident {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let address_string: &str = Deserialize::deserialize(deserializer)?;

                let address: Self = address_string
                    .parse()
                    .map_err(|err| DeserializationError::custom(format!("{:?}", err)))?;
                Ok(address)
            }
        }

        impl Serialize for $network_aware_struct_ident {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_str(&self.to_string())
            }
        }

        impl From<$network_aware_struct_ident> for $underlying_type {
            fn from(address: $network_aware_struct_ident) -> $underlying_type {
                address.address
            }
        }

        impl From<&$network_aware_struct_ident> for $underlying_type {
            fn from(address: &$network_aware_struct_ident) -> $underlying_type {
                address.address
            }
        }

        impl Display for $network_aware_struct_ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let bech32_manager: Bech32Manager = Bech32Manager::new(self.network_id);
                write!(
                    f,
                    "{}",
                    bech32_manager.encoder.$encoding_method_ident(&self.address)
                )
            }
        }

        impl FromStr for $network_aware_struct_ident {
            type Err = Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let bech32_manager: Bech32Manager = Bech32Manager::new_from_address(s)?;
                Ok(Self {
                    address: bech32_manager.decoder.$decoding_method_ident(s)?,
                    network_id: bech32_manager.network_id(),
                })
            }
        }
    };
}

define_network_aware_address!(
    scrypto::prelude::ComponentAddress => NetworkAwareComponentAddress,
    encode_component_address_to_string,
    validate_and_decode_component_address
);
define_network_aware_address!(
    scrypto::prelude::PackageAddress => NetworkAwarePackageAddress,
    encode_package_address_to_string,
    validate_and_decode_package_address
);
define_network_aware_address!(
    scrypto::prelude::ResourceAddress => NetworkAwareResourceAddress,
    encode_resource_address_to_string,
    validate_and_decode_resource_address
);
define_network_aware_address!(
    scrypto::prelude::SystemAddress => NetworkAwareSystemAddress,
    encode_system_address_to_string,
    validate_and_decode_system_address
);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "address")]
pub enum Address {
    ComponentAddress(NetworkAwareComponentAddress),
    ResourceAddress(NetworkAwareResourceAddress),
    PackageAddress(NetworkAwarePackageAddress),
}

impl Address {
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
        }
    }

    pub fn network_id(&self) -> u8 {
        match self {
            Self::ComponentAddress(address) => address.network_id,
            Self::ResourceAddress(address) => address.network_id,
            Self::PackageAddress(address) => address.network_id,
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

impl From<NetworkAwareComponentAddress> for Address {
    fn from(address: NetworkAwareComponentAddress) -> Self {
        Self::ComponentAddress(address)
    }
}

impl From<NetworkAwareResourceAddress> for Address {
    fn from(address: NetworkAwareResourceAddress) -> Self {
        Self::ResourceAddress(address)
    }
}

impl From<NetworkAwarePackageAddress> for Address {
    fn from(address: NetworkAwarePackageAddress) -> Self {
        Self::PackageAddress(address)
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Address::ComponentAddress(address) => write!(f, "{}", address),
            Address::ResourceAddress(address) => write!(f, "{}", address),
            Address::PackageAddress(address) => write!(f, "{}", address),
        }
    }
}

impl FromStr for Address {
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

    NormalComponent,
    AccountComponent,
    EcdsaSecp256k1VirtualAccount,
    EddsaEd25519VirtualAccount,
}

/// A serde-as serializer that serializes and deserializes object as a [Value]. This is useful for
/// consistent returns from the toolkit.
pub struct ValueSerializationProxy;

impl<T> SerializeAs<T> for ValueSerializationProxy
where
    T: Into<Value> + Clone,
{
    fn serialize_as<S>(source: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value: Value = source.clone().into();
        value.serialize(serializer)
    }
}

impl<'de, T> DeserializeAs<'de, T> for ValueSerializationProxy
where
    T: TryFrom<Value, Error = Error> + Clone,
{
    fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
    {
        Value::deserialize(deserializer)?
            .try_into()
            .map_err(|err| D::Error::custom(format!("{:?}", err)))
    }
}
