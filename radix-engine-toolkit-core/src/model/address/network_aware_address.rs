use std::convert::TryFrom;
use std::fmt::Display;
use std::str::FromStr;

use serde::de::Error as DeserializationError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::error::Error;
use crate::model::address::bech32_coder::Bech32Coder;

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
                let address_string: String = Deserialize::deserialize(deserializer)?;
                let address = address_string
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
                let bech32_coder = Bech32Coder::new(self.network_id);
                write!(
                    f,
                    "{}",
                    bech32_coder.encoder.$encoding_method_ident(&self.address)
                )
            }
        }

        impl FromStr for $network_aware_struct_ident {
            type Err = Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let bech32_coder = Bech32Coder::new_from_address(s)?;
                Ok(Self {
                    address: bech32_coder.decoder.$decoding_method_ident(s)?,
                    network_id: bech32_coder.network_id(),
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
