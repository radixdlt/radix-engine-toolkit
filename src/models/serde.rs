use serde::de::Error as DeserializationError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use scrypto::prelude::{EcdsaPublicKey, EcdsaSignature, Hash};
use transaction::model::TransactionHeader;

use crate::models::manifest::Manifest;
use serde_with::{serde_as, DisplayFromStr};
#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
#[serde(remote = "TransactionHeader")]
pub struct TransactionHeaderDef {
    pub version: u8,
    pub network_id: u8,
    pub start_epoch_inclusive: u64,
    pub end_epoch_exclusive: u64,
    pub nonce: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub notary_public_key: EcdsaPublicKey,
    pub notary_as_signatory: bool,
    pub cost_unit_limit: u32,
    pub tip_percentage: u32,
}

pub type VaultId = EntityId;
pub type KeyValueStoreId = EntityId;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct EntityId(pub (Hash, u32));

impl From<(Hash, u32)> for EntityId {
    fn from(value: (Hash, u32)) -> Self {
        Self(value)
    }
}

impl Into<(Hash, u32)> for EntityId {
    fn into(self) -> (Hash, u32) {
        self.0
    }
}

impl Serialize for EntityId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut v = self.0 .0.to_vec();
        v.extend(self.0 .1.to_le_bytes());
        Ok(serializer.serialize_str(&hex::encode(&v))?)
    }
}

impl<'de> Deserialize<'de> for EntityId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let vault_id_string: &str = Deserialize::deserialize(deserializer)?;
        let decoded_vault_id: Vec<u8> = hex::decode(vault_id_string).map_err(|_| {
            DeserializationError::custom(format!(
                "Could not decode entity identifier as hex: {}",
                vault_id_string
            ))
        })?;
        match decoded_vault_id.len() {
            36 => Ok(Self((
                Hash(scrypto::misc::copy_u8_array(&decoded_vault_id[0..32])),
                u32::from_le_bytes(scrypto::misc::copy_u8_array(&decoded_vault_id[32..])),
            ))),
            _ => Err(DeserializationError::custom(format!(
                "Could not create vault from the given entity identifier: {}",
                vault_id_string
            ))),
        }
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

        impl<'de> Deserialize<'de> for $network_aware_struct_ident {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let address_string: &str = Deserialize::deserialize(deserializer)?;

                let network_id: u8 =
                    crate::utils::network_id_from_address_string(address_string)
                        .map_err(|error| DeserializationError::custom(format!("{:?}", error)))?;
                let network_definition: scrypto::core::NetworkDefinition =
                    crate::utils::network_definition_from_network_id(network_id);
                let bech32_decoder = scrypto::address::Bech32Decoder::new(&network_definition);

                let address: $underlying_type = bech32_decoder
                    .$decoding_method_ident(address_string)
                    .map_err(|error| DeserializationError::custom(format!("{:?}", error)))?;

                Ok(Self {
                    network_id,
                    address,
                })
            }
        }

        impl Serialize for $network_aware_struct_ident {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let network_definition: scrypto::core::NetworkDefinition =
                    crate::utils::network_definition_from_network_id(self.network_id);
                let bech32_encoder: scrypto::address::Bech32Encoder =
                    scrypto::address::Bech32Encoder::new(&network_definition);
                let encoded_address: String = bech32_encoder.$encoding_method_ident(&self.address);
                Ok(serializer.serialize_str(&encoded_address)?)
            }
        }

        impl Into<$underlying_type> for $network_aware_struct_ident {
            fn into(self) -> $underlying_type {
                self.address.clone()
            }
        }

        impl Into<$underlying_type> for &$network_aware_struct_ident {
            fn into(self) -> $underlying_type {
                self.address.clone()
            }
        }
    };
}

define_network_aware_address!(
    scrypto::prelude::ComponentAddress => NetworkAwareComponentAddress,
    encode_component_address,
    validate_and_decode_component_address
);
define_network_aware_address!(
    scrypto::prelude::PackageAddress => NetworkAwarePackageAddress,
    encode_package_address,
    validate_and_decode_package_address
);
define_network_aware_address!(
    scrypto::prelude::ResourceAddress => NetworkAwareResourceAddress,
    encode_resource_address,
    validate_and_decode_resource_address
);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionIntent {
    #[serde(with = "crate::models::serde::TransactionHeaderDef")]
    pub header: transaction::model::TransactionHeader,
    pub manifest: Manifest,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignedTransactionIntent {
    pub transaction_intent: TransactionIntent,
    pub signatures: Vec<Signature>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Signature {
    #[serde_as(as = "DisplayFromStr")]
    pub public_key: EcdsaPublicKey,
    #[serde_as(as = "DisplayFromStr")]
    pub signature: EcdsaSignature,
}

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
                scrypto::prelude::ComponentAddress::Account(_) => AddressKind::AccountComponent,
                scrypto::prelude::ComponentAddress::System(_) => AddressKind::SystemComponent,
                scrypto::prelude::ComponentAddress::Normal(_) => AddressKind::NormalComponent,
            },
            Self::ResourceAddress(resource_address) => match resource_address.address {
                scrypto::prelude::ResourceAddress::Normal(_) => AddressKind::Resource,
            },
            Self::PackageAddress(package_address) => match package_address.address {
                scrypto::prelude::PackageAddress::Normal(_) => AddressKind::Package,
            },
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AddressKind {
    Resource,
    Package,

    AccountComponent,
    SystemComponent,
    NormalComponent,
}
