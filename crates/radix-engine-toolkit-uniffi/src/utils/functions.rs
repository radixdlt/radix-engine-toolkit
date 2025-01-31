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
pub fn get_known_addresses(network_id: u8) -> KnownAddresses {
    KnownAddresses::new_from_network(network_id)
}

#[uniffi::export]
pub fn get_hash(data: Vec<u8>) -> Arc<Hash> {
    Hash::from_unhashed_bytes(data)
}

macro_rules! define_known_addresses {
    (
        $(
            $ty: ident => [
                $(
                    $ident: ident
                ),* $(,)?
            ]
        ),* $(,)?
    ) => {
        paste::paste! {
            #[derive(Clone, Debug, Record)]
            pub struct KnownAddresses {
                $(
                    pub $ty: [< $ty: camel >],
                )*
            }

            impl KnownAddresses {
                pub fn new_from_network(network_id: u8) -> Self {
                    Self {
                        $(
                            $ty: [< $ty: camel >]::new_from_network(network_id),
                        )*
                    }
                }
            }

            $(
                #[derive(Clone, Debug, Record)]
                pub struct [< $ty: camel >] {
                    $(
                        pub $ident: $crate::prelude::Arc<$crate::prelude::Address>,
                    )*
                }

                impl [< $ty: camel >] {
                    pub fn new_from_network(network_id: u8) -> Self {
                        Self {
                            $(
                                $ident: $crate::prelude::Arc::new(Address::from_node_id( scrypto::prelude::[< $ident: upper >],network_id )),
                            )*
                        }
                    }
                }
            )*
        }
    };
}
define_known_addresses! {
    resource_addresses => [
        xrd,
        secp256k1_signature_resource,
        ed25519_signature_resource,
        package_of_direct_caller_resource,
        global_caller_resource,
        system_execution_resource,
        package_owner_badge,
        validator_owner_badge,
        account_owner_badge,
        identity_owner_badge,
    ],
    package_addresses => [
        package_package,
        resource_package,
        account_package,
        identity_package,
        consensus_manager_package,
        access_controller_package,
        pool_package,
        transaction_processor_package,
        metadata_module_package,
        royalty_module_package,
        role_assignment_module_package,
        genesis_helper_package,
        faucet_package,
    ],
    component_addresses => [
        consensus_manager,
        genesis_helper,
        faucet,
    ]
}

#[uniffi::export]
pub fn test_panic(message: String) -> Result<()> {
    panic!("{}", message)
}
