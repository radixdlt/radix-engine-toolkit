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

use indexmap::{indexmap, IndexMap};

pub fn generate_serializable_model_examples(
) -> IndexMap<&'static str, IndexMap<String, Vec<serde_json::Value>>> {
    indexmap! {
        "cryptographic/public_key" => model_examples![
            SerializablePublicKey,
            SerializableEcdsaSecp256k1PublicKey,
            SerializableEddsaEd25519PublicKey
        ],
        "network" => model_examples![
            SerializableNetworkId,
            SerializableOlympiaNetwork,
        ],
        "non_fungible" => model_examples![
            SerializableNonFungibleGlobalId,
        ],
        "node_id" => model_examples![
            SerializableNodeId,
        ],
    }
}

macro_rules! model_examples {
    (
        $( $function: ident ),* $(,)?
    ) => {
        {
            use $crate::serializable_models::traits::HasExamples;

            let mut map = indexmap::IndexMap::new();

            $(
                let name = $crate::utils::snake_case_type_name::<$function>();
                let examples = $function::serde_value_examples().into_iter().collect::<Vec<_>>();

                map.insert(name, examples);
            )*

            map
        }
    };
}
use model_examples;
use native_json_library::models::{
    cryptographic::public_key::{
        SerializableEcdsaSecp256k1PublicKey, SerializableEddsaEd25519PublicKey,
        SerializablePublicKey,
    },
    network::{network_id::SerializableNetworkId, olympia_network::SerializableOlympiaNetwork},
    node_id::SerializableNodeId,
    non_fungible_global_id::SerializableNonFungibleGlobalId,
};
