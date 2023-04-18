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

// Defines a network aware address. This is needed for the serialization and deserialization using
// serde.
macro_rules! define_network_aware_address {
    (
        $underlying_type: ty => $network_aware_struct_ident: ident,
        $check_fn_ident: ident
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

        impl TryFrom<$crate::model::engine_identifier::NetworkAwareNodeId>
            for $network_aware_struct_ident
        {
            type Error = $crate::model::address::AddressError;

            fn try_from(
                node_id: $crate::model::engine_identifier::NetworkAwareNodeId,
            ) -> Result<Self, Self::Error> {
                let native_node_id = radix_engine_common::types::NodeId(node_id.0);
                if native_node_id.$check_fn_ident() {
                    let address = <$underlying_type>::new_unchecked(node_id.0);
                    Ok($network_aware_struct_ident {
                        network_id: node_id.1,
                        address,
                    })
                } else {
                    Err($crate::model::address::AddressError::AddressError {
                        message: "Invalid Address".into(),
                    })
                }
            }
        }

        impl From<$network_aware_struct_ident>
            for $crate::model::engine_identifier::NetworkAwareNodeId
        {
            fn from(address: $network_aware_struct_ident) -> Self {
                Self(address.address.as_node_id().0, address.network_id)
            }
        }

        impl $network_aware_struct_ident {
            pub fn from_u8_array(data: &[u8], network_id: u8) -> $crate::error::Result<Self> {
                if let Ok(address) = <$underlying_type>::try_from(data) {
                    Ok($network_aware_struct_ident {
                        network_id,
                        address,
                    })
                } else {
                    Err($crate::model::address::AddressError::UnrecognizedAddressFormat)
                }
            }

            pub fn network_aware_node_id(
                &self,
            ) -> $crate::model::engine_identifier::NetworkAwareNodeId {
                $crate::model::engine_identifier::NetworkAwareNodeId(
                    self.address.as_node_id().0,
                    self.network_id,
                )
            }
        }

        impl Display for $network_aware_struct_ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let node_id = self.network_aware_node_id();
                node_id.fmt(f)
            }
        }

        impl FromStr for $network_aware_struct_ident {
            type Err = $crate::model::address::AddressError;

            fn from_str(s: &str) -> $crate::error::Result<Self> {
                let node_id = $crate::model::engine_identifier::NetworkAwareNodeId::from_str(s)?;
                Ok(Self {
                    address: <$underlying_type>::new_unchecked(node_id.0),
                    network_id: node_id.1,
                })
            }
        }

        /// An implementation of borrow which allows the network aware types to be borrowed as
        /// non-network aware types. Useful for Bech32 encoding.
        impl std::borrow::Borrow<$underlying_type> for $network_aware_struct_ident {
            fn borrow(&self) -> &$underlying_type {
                &self.address
            }
        }

        impl std::borrow::Borrow<$underlying_type> for &$network_aware_struct_ident {
            fn borrow(&self) -> &$underlying_type {
                &self.address
            }
        }

        impl std::borrow::Borrow<$underlying_type> for &mut $network_aware_struct_ident {
            fn borrow(&self) -> &$underlying_type {
                &self.address
            }
        }
    };
}

define_network_aware_address!(
    scrypto::prelude::ComponentAddress => NetworkAwareComponentAddress,
    is_global_component
);
define_network_aware_address!(
    scrypto::prelude::PackageAddress => NetworkAwarePackageAddress,
    is_global_package
);
define_network_aware_address!(
    scrypto::prelude::ResourceAddress => NetworkAwareResourceAddress,
    is_global_resource
);
