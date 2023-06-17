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
use native_json_library::models::cryptographic::public_key::{
    SerializableEd25519PublicKey, SerializablePublicKey, SerializableSecp256k1PublicKey,
};
use native_json_library::prelude::*;

pub fn generate_serializable_model_examples(
) -> IndexMap<&'static str, IndexMap<String, Vec<serde_json::Value>>> {
    indexmap! {
        "cryptographic/public_key" => model_examples![
            SerializablePublicKey,
            SerializableSecp256k1PublicKey,
            SerializableEd25519PublicKey
        ],
        "network" => model_examples![
            SerializableOlympiaNetwork,
        ],
        "non_fungible" => model_examples![
            SerializableNonFungibleGlobalId,
            SerializableNonFungibleLocalId
        ],
        "node_id" => model_examples![
            SerializableNodeId,
        ],
        "numbers" => model_examples![
            SerializableU8 as serializable_u8,
            SerializableU16 as serializable_u16,
            SerializableU32 as serializable_u32,
            SerializableU64 as serializable_u64,
            SerializableU128 as serializable_u128,
            SerializableI8 as serializable_i8,
            SerializableI16 as serializable_i16,
            SerializableI32 as serializable_i32,
            SerializableI64 as serializable_i64,
            SerializableI128 as serializable_i128,
            SerializableDecimal,
            SerializablePreciseDecimal,
        ]
    }
}

macro_rules! model_examples {
    (
        $( $function: ident $(as $name: ident)? ),* $(,)?
    ) => {
        {
            use $crate::serializable_models::traits::HasExamples;

            let mut map = indexmap::IndexMap::new();

            $(
                #[allow(unused_variables)]
                let name = $crate::utils::snake_case_type_name::<$function>();
                $(
                    let name = stringify!($name).to_owned();
                )?
                let examples = $function::serde_value_examples().into_iter().collect::<Vec<_>>();

                map.insert(name, examples);
            )*

            map
        }
    };
}
use model_examples;
