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

mod examples;
mod examples_builder;

use std::collections::HashMap;
use std::path::PathBuf;

use examples_builder::InMemoryExamplesBuilder;
use radix_engine_toolkit::functions::*;
use scrypto_utils::hashmap;

macro_rules! generate_group_schema {
    ($($type: ty),*) => {
        {
            let mut map = HashMap::new();

            $(
                // Converting type path to a type name by splitting it at the double colon and
                // getting the type name
                let type_name = stringify!($type).split("::").last().unwrap().trim().to_owned();

                // Getting the schema for the type
                let schema = schemars::schema_for!($type);

                // Adding it to the map
                map.insert(type_name, schema);
            )*

            map
        }
    };
}

fn main() {
    generate_json_schema().expect("Failed to generate schema");
    generate_function_examples().expect("Failed to generate request examples");
}

pub fn generate_json_schema() -> Result<(), GenerationError> {
    let schema_map = hashmap!(
        "information" => generate_group_schema!(
            information::Input,
            information::Output,
            information::Error
        ),
        "convert_manifest" => generate_group_schema!(
            convert_manifest::Input,
            convert_manifest::Output,
            convert_manifest::Error
        ),
        "compile_transaction_intent" => generate_group_schema!(
            compile_transaction_intent::Input,
            compile_transaction_intent::Output,
            compile_transaction_intent::Error
        ),
        "decompile_transaction_intent" => generate_group_schema!(
            decompile_transaction_intent::Input,
            decompile_transaction_intent::Output,
            decompile_transaction_intent::Error
        ),
        "compile_signed_transaction_intent" => generate_group_schema!(
            compile_signed_transaction_intent::Input,
            compile_signed_transaction_intent::Output,
            compile_signed_transaction_intent::Error
        ),
        "decompile_signed_transaction_intent" => generate_group_schema!(
            decompile_signed_transaction_intent::Input,
            decompile_signed_transaction_intent::Output,
            decompile_signed_transaction_intent::Error
        ),
        "compile_notarized_transaction" => generate_group_schema!(
            compile_notarized_transaction::Input,
            compile_notarized_transaction::Output,
            compile_notarized_transaction::Error
        ),
        "decompile_notarized_transaction" => generate_group_schema!(
            decompile_notarized_transaction::Input,
            decompile_notarized_transaction::Output,
            decompile_notarized_transaction::Error
        ),
        "decompile_unknown_intent" => generate_group_schema!(
            decompile_unknown_intent::Input,
            decompile_unknown_intent::Output,
            decompile_unknown_intent::Error
        ),
        "decode_address" => generate_group_schema!(
            decode_address::Input,
            decode_address::Output,
            decode_address::Error
        ),
        "encode_address" => generate_group_schema!(
            encode_address::Input,
            encode_address::Output,
            encode_address::Error
        ),
        "sbor_decode" => generate_group_schema!(
            sbor_decode::Input,
            sbor_decode::Output,
            sbor_decode::Error
        ),
        "sbor_encode" => generate_group_schema!(
            sbor_encode::Input,
            sbor_encode::Output,
            sbor_encode::Error
        ),
        "derive_babylon_address_from_olympia_address" => generate_group_schema!(
            derive_babylon_address_from_olympia_address::Input,
            derive_babylon_address_from_olympia_address::Output,
            derive_babylon_address_from_olympia_address::Error
        ),
        "derive_olympia_address_from_public_key" => generate_group_schema!(
            derive_olympia_address_from_public_key::Input,
            derive_olympia_address_from_public_key::Output,
            derive_olympia_address_from_public_key::Error
        ),
        "derive_virtual_account_address" => generate_group_schema!(
            derive_virtual_account_address::Input,
            derive_virtual_account_address::Output,
            derive_virtual_account_address::Error
        ),
        "derive_virtual_identity_address" => generate_group_schema!(
            derive_virtual_identity_address::Input,
            derive_virtual_identity_address::Output,
            derive_virtual_identity_address::Error
        ),
        "known_entity_addresses" => generate_group_schema!(
            known_entity_addresses::Input,
            known_entity_addresses::Output,
            known_entity_addresses::Error
        ),
        "statically_validate_transaction" => generate_group_schema!(
            statically_validate_transaction::Input,
            statically_validate_transaction::Output,
            statically_validate_transaction::Error
        ),
        "hash" => generate_group_schema!(
            hash::Input,
            hash::Output,
            hash::Error
        ),
        "extract_addresses_from_manifest" => generate_group_schema!(
            extract_addresses_from_manifest::Input,
            extract_addresses_from_manifest::Output,
            extract_addresses_from_manifest::Error
        ),
        "analyze_transaction_execution" => generate_group_schema!(
            analyze_transaction_execution::Input,
            analyze_transaction_execution::Output,
            analyze_transaction_execution::Error
        ),
        "hash_transaction_intent" => generate_group_schema!(
            hash_transaction_intent::Input,
            hash_transaction_intent::Output,
            hash_transaction_intent::Error
        ),
        "hash_signed_transaction_intent" => generate_group_schema!(
            hash_signed_transaction_intent::Input,
            hash_signed_transaction_intent::Output,
            hash_signed_transaction_intent::Error
        ),
        "hash_notarized_transaction" => generate_group_schema!(
            hash_notarized_transaction::Input,
            hash_notarized_transaction::Output,
            hash_notarized_transaction::Error
        ),
    );

    for (function_name, type_schemas) in schema_map.iter() {
        let function_schema_directory = {
            let mut path = PathBuf::from(".");
            path.push("out");
            path.push("schema");
            path.push(function_name);
            path
        };
        std::fs::create_dir_all(&function_schema_directory).map_err(GenerationError::IOError)?;

        for (type_name, type_schema) in type_schemas {
            let type_schema_path = function_schema_directory.join(format!("{}.json", type_name));
            let serialized_schema = serde_json::to_string_pretty(&type_schema)
                .map_err(GenerationError::SerializationError)?;
            std::fs::write(type_schema_path, &serialized_schema)
                .map_err(GenerationError::IOError)?;
        }
    }

    Ok(())
}

fn generate_function_examples() -> Result<(), GenerationError> {
    let examples = InMemoryExamplesBuilder::new()
        .add_example::<information::Handler, _, _>()
        .add_example::<convert_manifest::Handler, _, _>()
        .add_example::<extract_addresses_from_manifest::Handler, _, _>()
        .add_example::<compile_transaction_intent::Handler, _, _>()
        .add_example::<decompile_transaction_intent::Handler, _, _>()
        .add_example::<compile_signed_transaction_intent::Handler, _, _>()
        .add_example::<decompile_signed_transaction_intent::Handler, _, _>()
        .add_example::<compile_notarized_transaction::Handler, _, _>()
        .add_example::<decompile_notarized_transaction::Handler, _, _>()
        .add_example::<decompile_unknown_intent::Handler, _, _>()
        .add_example::<encode_address::Handler, _, _>()
        .add_example::<decode_address::Handler, _, _>()
        .add_example::<sbor_encode::Handler, _, _>()
        .add_example::<sbor_decode::Handler, _, _>()
        .add_example::<derive_virtual_account_address::Handler, _, _>()
        .add_example::<derive_virtual_identity_address::Handler, _, _>()
        .add_example::<derive_babylon_address_from_olympia_address::Handler, _, _>()
        .add_example::<derive_olympia_address_from_public_key::Handler, _, _>()
        .add_example::<statically_validate_transaction::Handler, _, _>()
        .add_example::<known_entity_addresses::Handler, _, _>()
        .add_example::<hash::Handler, _, _>()
        .add_example::<hash_transaction_intent::Handler, _, _>()
        .add_example::<hash_signed_transaction_intent::Handler, _, _>()
        .add_example::<hash_notarized_transaction::Handler, _, _>()
        .build();

    let path = {
        let mut path = PathBuf::from(".");
        path.push("out");
        path.push("examples");
        path.push("function-examples.md");
        path
    };

    std::fs::write(path, examples).map_err(GenerationError::IOError)
}

#[derive(Debug)]
pub enum GenerationError {
    IOError(std::io::Error),
    SerializationError(serde_json::Error),
}
