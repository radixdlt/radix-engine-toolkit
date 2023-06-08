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

use native_json_library::{
    functions::derive::*,
    models::network::{SerializableNetworkId, SerializableOlympiaNetwork},
};
use radix_engine_common::prelude::PublicKey;
use transaction::validation::{EcdsaSecp256k1PrivateKey, EddsaEd25519PrivateKey};

use super::traits::HasExamples;

impl<'f> HasExamples<'f, 2> for DeriveVirtualAccountAddressFromPublicKey {
    fn example_inputs() -> [Self::Input; 2] {
        let public_key1 = EcdsaSecp256k1PrivateKey::from_u64(1).unwrap().public_key();
        let public_key2 = EddsaEd25519PrivateKey::from_u64(1).unwrap().public_key();

        [
            Self::Input {
                public_key: PublicKey::from(public_key1).into(),
                network_id: SerializableNetworkId(1),
            },
            Self::Input {
                public_key: PublicKey::from(public_key2).into(),
                network_id: SerializableNetworkId(1),
            },
        ]
    }
}

impl<'f> HasExamples<'f, 2> for DeriveVirtualIdentityAddressFromPublicKey {
    fn example_inputs() -> [Self::Input; 2] {
        let public_key1 = EcdsaSecp256k1PrivateKey::from_u64(1).unwrap().public_key();
        let public_key2 = EddsaEd25519PrivateKey::from_u64(1).unwrap().public_key();

        [
            Self::Input {
                public_key: PublicKey::from(public_key1).into(),
                network_id: SerializableNetworkId(1),
            },
            Self::Input {
                public_key: PublicKey::from(public_key2).into(),
                network_id: SerializableNetworkId(1),
            },
        ]
    }
}

impl<'f> HasExamples<'f, 2> for DeriveVirtualSignatureNonFungibleGlobalIdFromPublicKey {
    fn example_inputs() -> [Self::Input; 2] {
        let public_key1 = EcdsaSecp256k1PrivateKey::from_u64(1).unwrap().public_key();
        let public_key2 = EddsaEd25519PrivateKey::from_u64(1).unwrap().public_key();

        [
            Self::Input {
                public_key: PublicKey::from(public_key1).into(),
                network_id: SerializableNetworkId(1),
            },
            Self::Input {
                public_key: PublicKey::from(public_key2).into(),
                network_id: SerializableNetworkId(1),
            },
        ]
    }
}

impl<'f> HasExamples<'f, 1> for DeriveVirtualAccountAddressFromOlympiaAccountAddress {
    fn example_inputs() -> [Self::Input; 1] {
        [Self::Input {
            olympia_account_address:
                "rdx1qspx7zxmnrh36q33av24srdfzg7m3cj65968erpjuh7ja3rm3kmn6hq4j9842".to_string(),
            network_id: SerializableNetworkId(1),
        }]
    }
}

impl<'f> HasExamples<'f, 1> for DerivePublicKeyFromOlympiaAccountAddress {
    fn example_inputs() -> [Self::Input; 1] {
        ["rdx1qspx7zxmnrh36q33av24srdfzg7m3cj65968erpjuh7ja3rm3kmn6hq4j9842".to_string()]
    }
}

impl<'f> HasExamples<'f, 1> for DeriveOlympiaAccountAddressFromPublicKey {
    fn example_inputs() -> [Self::Input; 1] {
        let public_key = EcdsaSecp256k1PrivateKey::from_u64(1).unwrap().public_key();

        [Self::Input {
            public_key: public_key.into(),
            olympia_network: SerializableOlympiaNetwork::Mainnet,
        }]
    }
}

impl<'f> HasExamples<'f, 1> for DeriveNodeAddressFromPublicKey {
    fn example_inputs() -> [Self::Input; 1] {
        let public_key = EcdsaSecp256k1PrivateKey::from_u64(1).unwrap().public_key();

        [Self::Input {
            public_key: public_key.into(),
            network_id: SerializableNetworkId(1),
        }]
    }
}
