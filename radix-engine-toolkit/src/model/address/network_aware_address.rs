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

use std::fmt::Display;
use std::str::FromStr;

use crate::address::Bech32Coder;
use crate::error::{Error, Result};

// Defines a network aware address. This is needed for the serialization and deserialization using
// serde.
macro_rules! define_network_aware_address {
    (
        $underlying_type: ty => $network_aware_struct_ident: ident,
        $encoding_method_ident: ident,
        $decoding_method_ident: ident
    ) => {
        #[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
        pub struct $network_aware_struct_ident {
            pub network_id: u8,
            pub address: $underlying_type,
        }
        impl From<$network_aware_struct_ident> for $underlying_type {
            fn from(address: $network_aware_struct_ident) -> $underlying_type {
                address.address
            }
        }

        impl $network_aware_struct_ident {
            pub fn from_u8_array(data: &[u8], network_id: u8) -> Result<Self> {
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

        impl Display for $network_aware_struct_ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let bech32_coder = Bech32Coder::new(self.network_id);
                write!(f, "{}", bech32_coder.$encoding_method_ident(&self.address))
            }
        }

        impl FromStr for $network_aware_struct_ident {
            type Err = Error;

            fn from_str(s: &str) -> Result<Self> {
                let bech32_coder = Bech32Coder::new_from_address(s)?;
                Ok(Self {
                    address: bech32_coder.$decoding_method_ident(s)?,
                    network_id: bech32_coder.network_id(),
                })
            }
        }
    };
}

define_network_aware_address!(
    scrypto::prelude::ComponentAddress => NetworkAwareComponentAddress,
    encode_component_address,
    decode_component_address
);
define_network_aware_address!(
    scrypto::prelude::PackageAddress => NetworkAwarePackageAddress,
    encode_package_address,
    decode_package_address
);
define_network_aware_address!(
    scrypto::prelude::ResourceAddress => NetworkAwareResourceAddress,
    encode_resource_address,
    decode_resource_address
);
define_network_aware_address!(
    scrypto::prelude::SystemAddress => NetworkAwareSystemAddress,
    encode_system_address,
    decode_system_address
);
