use std::convert::{TryFrom, TryInto};
use std::fmt::Display;
use std::str::FromStr;

use serde::de::Error as DeserializationError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::serde_as;

use scrypto::prelude::{Hash, Signature, SignatureWithPublicKey};

use crate::address::Bech32Manager;
use crate::error::Error;
use crate::models::manifest::ManifestInstructions;

use super::ManifestInstructionsKind;

pub type VaultId = EntityId;
pub type KeyValueStoreId = EntityId;

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionManifest {
    pub instructions: ManifestInstructions,
    #[serde_as(as = "Vec<serde_with::hex::Hex>")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub blobs: Vec<Vec<u8>>,
}

impl TransactionManifest {
    pub fn to_scrypto_transaction_manifest(
        &self,
        bech32_manager: &Bech32Manager,
    ) -> Result<transaction::model::TransactionManifest, Error> {
        self.instructions
            .to_scrypto_transaction_manifest(bech32_manager, self.blobs.clone())
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct EntityId(pub (Hash, u32));

impl From<(Hash, u32)> for EntityId {
    fn from(value: (Hash, u32)) -> Self {
        Self(value)
    }
}

impl From<EntityId> for (Hash, u32) {
    fn from(entity_id: EntityId) -> Self {
        entity_id.0
    }
}

impl Serialize for EntityId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut v = self.0 .0.to_vec();
        v.extend(self.0 .1.to_le_bytes());
        serializer.serialize_str(&hex::encode(&v))
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

                let address: Self = Self::from_str(address_string)
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
    pub header: transaction::model::TransactionHeader,
    pub manifest: TransactionManifest,
}

impl TransactionIntent {
    pub fn convert_manifest_instructions_kind(
        self,
        to: ManifestInstructionsKind,
    ) -> Result<Self, Error> {
        let bech32_manager: Bech32Manager = Bech32Manager::new(self.header.network_id);
        Ok(Self {
            header: self.header,
            manifest: TransactionManifest {
                instructions: self.manifest.instructions.to(
                    to,
                    &bech32_manager,
                    self.manifest.blobs.clone(),
                )?,
                blobs: self.manifest.blobs,
            },
        })
    }
}

impl TryInto<transaction::model::TransactionIntent> for TransactionIntent {
    type Error = Error;

    fn try_into(self) -> Result<transaction::model::TransactionIntent, Self::Error> {
        let bech32_manager: Bech32Manager = Bech32Manager::new(self.header.network_id);

        Ok(transaction::model::TransactionIntent {
            header: self.header,
            manifest: self
                .manifest
                .to_scrypto_transaction_manifest(&bech32_manager)?,
        })
    }
}

impl TryInto<TransactionIntent> for transaction::model::TransactionIntent {
    type Error = Error;

    fn try_into(self) -> Result<TransactionIntent, Self::Error> {
        let bech32_manager: Bech32Manager = Bech32Manager::new(self.header.network_id);

        Ok(TransactionIntent {
            header: self.header,
            manifest: TransactionManifest {
                instructions: ManifestInstructions::from_scrypto_transaction_manifest(
                    &self.manifest,
                    &bech32_manager,
                    super::ManifestInstructionsKind::JSON,
                )?,
                blobs: self.manifest.blobs,
            },
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignedTransactionIntent {
    pub transaction_intent: TransactionIntent,
    pub signatures: Vec<SignatureWithPublicKey>,
}

impl SignedTransactionIntent {
    pub fn convert_manifest_instructions_kind(
        self,
        to: ManifestInstructionsKind,
    ) -> Result<Self, Error> {
        Ok(Self {
            signatures: self.signatures,
            transaction_intent: self
                .transaction_intent
                .convert_manifest_instructions_kind(to)?,
        })
    }
}

impl TryInto<transaction::model::SignedTransactionIntent> for SignedTransactionIntent {
    type Error = Error;

    fn try_into(self) -> Result<transaction::model::SignedTransactionIntent, Self::Error> {
        Ok(transaction::model::SignedTransactionIntent {
            intent: self.transaction_intent.try_into()?,
            intent_signatures: self.signatures,
        })
    }
}

impl TryInto<SignedTransactionIntent> for transaction::model::SignedTransactionIntent {
    type Error = Error;

    fn try_into(self) -> Result<SignedTransactionIntent, Self::Error> {
        Ok(SignedTransactionIntent {
            transaction_intent: self.intent.try_into()?,
            signatures: self.intent_signatures,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NotarizedTransaction {
    pub signed_intent: SignedTransactionIntent,
    pub notary_signature: Signature,
}

impl NotarizedTransaction {
    pub fn convert_manifest_instructions_kind(
        self,
        to: ManifestInstructionsKind,
    ) -> Result<Self, Error> {
        Ok(Self {
            signed_intent: self.signed_intent.convert_manifest_instructions_kind(to)?,
            notary_signature: self.notary_signature,
        })
    }
}

impl TryInto<transaction::model::NotarizedTransaction> for NotarizedTransaction {
    type Error = Error;

    fn try_into(self) -> Result<transaction::model::NotarizedTransaction, Self::Error> {
        Ok(transaction::model::NotarizedTransaction {
            signed_intent: self.signed_intent.try_into()?,
            notary_signature: self.notary_signature,
        })
    }
}

impl TryInto<NotarizedTransaction> for transaction::model::NotarizedTransaction {
    type Error = Error;

    fn try_into(self) -> Result<NotarizedTransaction, Self::Error> {
        Ok(NotarizedTransaction {
            signed_intent: self.signed_intent.try_into()?,
            notary_signature: self.notary_signature,
        })
    }
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

    AccountComponent,
    SystemComponent,
    NormalComponent,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "variant")]
pub enum OptionProxy<T> {
    Some { field: T },
    None,
}

impl<T> From<Option<T>> for OptionProxy<T> {
    fn from(option: Option<T>) -> Self {
        match option {
            Option::Some(field) => Self::Some { field },
            Option::None => Self::None,
        }
    }
}

impl<T> From<OptionProxy<T>> for Option<T> {
    fn from(option: OptionProxy<T>) -> Self {
        match option {
            OptionProxy::Some { field } => Self::Some(field),
            OptionProxy::None => Self::None,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "variant")]
pub enum ResultProxy<O, E> {
    Ok { field: O },
    Err { field: E },
}

impl<O, E> From<ResultProxy<O, E>> for Result<O, E> {
    fn from(result: ResultProxy<O, E>) -> Self {
        match result {
            ResultProxy::Ok { field } => Result::Ok(field),
            ResultProxy::Err { field } => Result::Err(field),
        }
    }
}

impl<O, E> From<Result<O, E>> for ResultProxy<O, E> {
    fn from(result: Result<O, E>) -> Self {
        match result {
            Result::Ok(field) => ResultProxy::Ok { field },
            Result::Err(field) => ResultProxy::Err { field },
        }
    }
}
