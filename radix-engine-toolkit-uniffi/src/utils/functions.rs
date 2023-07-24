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

#[uniffi::export]
pub fn known_addresses(network_id: u8) -> KnownAddresses {
    KnownAddresses {
        resource_addresses: ResourceAddresses::new_from_network(network_id),
        package_addresses: PackageAddresses::new_from_network(network_id),
        component_addresses: ComponentAddresses::new_from_network(network_id),
    }
}

#[uniffi::export]
pub fn hash(data: Vec<u8>) -> Arc<Hash> {
    Hash::from_unhashed_bytes(data)
}

#[derive(Clone, Debug, Record)]
pub struct KnownAddresses {
    pub resource_addresses: ResourceAddresses,
    pub package_addresses: PackageAddresses,
    pub component_addresses: ComponentAddresses,
}

impl_new_from_network! {
    #[derive(Clone, Debug, Record)]
    pub struct ResourceAddresses {
        pub xrd: Arc<Address>,
        pub secp256k1_signature_virtual_badge: Arc<Address>,
        pub ed25519_signature_virtual_badge: Arc<Address>,
        pub package_of_direct_caller_virtual_badge: Arc<Address>,
        pub global_caller_virtual_badge: Arc<Address>,
        pub system_transaction_badge: Arc<Address>,
        pub package_owner_badge: Arc<Address>,
        pub validator_owner_badge: Arc<Address>,
        pub account_owner_badge: Arc<Address>,
        pub identity_owner_badge: Arc<Address>,
    }
}

impl_new_from_network! {
    #[derive(Clone, Debug, Record)]
    pub struct PackageAddresses {
        pub package_package: Arc<Address>,
        pub resource_package: Arc<Address>,
        pub account_package: Arc<Address>,
        pub identity_package: Arc<Address>,
        pub consensus_manager_package: Arc<Address>,
        pub access_controller_package: Arc<Address>,
        pub pool_package: Arc<Address>,
        pub transaction_processor_package: Arc<Address>,
        pub metadata_module_package: Arc<Address>,
        pub royalty_module_package: Arc<Address>,
        pub role_assignment_module_package: Arc<Address>,
        pub genesis_helper_package: Arc<Address>,
        pub faucet_package: Arc<Address>,
    }
}

impl_new_from_network! {
    #[derive(Clone, Debug, Record)]
    pub struct ComponentAddresses {
        pub consensus_manager: Arc<Address>,
        pub genesis_helper: Arc<Address>,
        pub faucet: Arc<Address>,
    }
}

macro_rules! impl_new_from_network {
    (
        $(#[$meta: meta])*
        $vis: vis struct $ident: ident {
            $(
                $item_vis: vis $item_ident: ident: $item_type: ty
            ),* $(,)?
        }
    ) => {
        paste::paste! {
            $(#[$meta])*
            $vis struct $ident {
                $(
                    $item_vis $item_ident: $item_type,
                )*
            }

            impl $ident {
                pub fn new_from_network(network_id: u8) -> Self {
                    Self {
                        $(
                            $item_ident: Arc::new(Address( scrypto::prelude::[< $item_ident: upper >].into_node_id(), network_id )),
                        )*
                    }
                }
            }
        }
    };
}
use impl_new_from_network;
