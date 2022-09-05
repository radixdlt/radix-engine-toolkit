use serde::de::Error as DeserializationError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::{serde_as, FromInto};

use scrypto::engine::types::VaultId;
use scrypto::prelude::{EcdsaPublicKey, EcdsaSignature, Hash, Vault};
use transaction::model::TransactionHeader;

use crate::models::manifest::Manifest;

#[derive(Serialize, Deserialize, Debug)]
#[serde(remote = "TransactionHeader")]
pub struct TransactionHeaderDef {
    pub version: u8,
    pub network_id: u8,
    pub start_epoch_inclusive: u64,
    pub end_epoch_exclusive: u64,
    pub nonce: u64,
    #[serde(with = "EcdsaPublicKeyDef")]
    pub notary_public_key: EcdsaPublicKey,
    pub notary_as_signatory: bool,
    pub cost_unit_limit: u32,
    pub tip_percentage: u32,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(remote = "EcdsaPublicKey")]
pub struct EcdsaPublicKeyDef(#[serde(with = "hex::serde")] pub [u8; EcdsaPublicKey::LENGTH]);

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(remote = "EcdsaSignature")]
pub struct EcdsaSignatureDef(#[serde(with = "hex::serde")] pub [u8; EcdsaSignature::LENGTH]);

#[derive(Serialize, Deserialize, Debug)]
#[serde(remote = "Hash")]
pub struct HashDef(#[serde(with = "hex::serde")] pub [u8; Hash::LENGTH]);

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
#[serde(remote = "Vault")]
pub struct VaultDef(#[serde_as(as = "FromInto<VaultIdProxy>")] pub VaultId);

pub struct VaultIdProxy(pub VaultId);

impl From<VaultId> for VaultIdProxy {
    fn from(value: VaultId) -> Self {
        Self(value)
    }
}

impl Into<VaultId> for VaultIdProxy {
    fn into(self) -> VaultId {
        self.0
    }
}

impl Serialize for VaultIdProxy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Ok(serializer.serialize_str(&hex::encode(&Vault(self.0).to_vec()))?)
    }
}

impl<'de> Deserialize<'de> for VaultIdProxy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let vault_id_string: &str = Deserialize::deserialize(deserializer)?;
        let decoded_vault_id: Vec<u8> = hex::decode(vault_id_string).map_err(|_| {
            DeserializationError::custom(format!(
                "Could not decode vault id as hex. Vault id: {}",
                vault_id_string
            ))
        })?;
        let vault: Vault = Vault::try_from(decoded_vault_id.as_slice()).map_err(|_| {
            DeserializationError::custom(format!(
                "Could not create vault from the given vault id. Vault id: {}",
                vault_id_string
            ))
        })?;

        Ok(VaultIdProxy(vault.0))
    }
}

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
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
        #[derive(Debug, Clone, Hash, Eq, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Signature {
    #[serde(with = "EcdsaPublicKeyDef")]
    pub public_key: EcdsaPublicKey,
    #[serde(with = "EcdsaSignatureDef")]
    pub signature: EcdsaSignature,
}
