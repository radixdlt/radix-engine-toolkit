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

use convert_case::Casing;
use examples_builder::InMemoryExamplesBuilder;
use radix_engine_toolkit::request::*;

/// Generates a Schema HashMap where the key is the class name and the value is the schema
macro_rules! generate_schema_hashmap {
    ($($type: ty),*) => {{
        let mut map = HashMap::new();
        $(
            {
                // Converting type path to a type name by splitting it at the double colon and
                // getting the type name
                let type_name = stringify!($type).split("::").last().unwrap().trim().to_owned();

                // Getting the schema for the type
                let schema = schemars::schema_for!($type);

                // Adding it to the map
                map.insert(type_name, schema);
            }
        )*

        map
    }};
}

fn main() {
    generate_json_schema().expect("Failed to generate schema");
    generate_request_examples().expect("Failed to generate request examples");
}

pub fn generate_json_schema() -> Result<(), GenerationError> {
    // Creating the schema for all of the request and response types through the generate schema
    // macro
    let schema_map = generate_schema_hashmap!(
        radix_engine_toolkit::request::InformationRequest,
        radix_engine_toolkit::request::InformationResponse,
        radix_engine_toolkit::request::ConvertManifestRequest,
        radix_engine_toolkit::request::ConvertManifestResponse,
        radix_engine_toolkit::request::CompileTransactionIntentRequest,
        radix_engine_toolkit::request::CompileTransactionIntentResponse,
        radix_engine_toolkit::request::DecompileTransactionIntentRequest,
        radix_engine_toolkit::request::DecompileTransactionIntentResponse,
        radix_engine_toolkit::request::CompileSignedTransactionIntentRequest,
        radix_engine_toolkit::request::CompileSignedTransactionIntentResponse,
        radix_engine_toolkit::request::DecompileSignedTransactionIntentRequest,
        radix_engine_toolkit::request::DecompileSignedTransactionIntentResponse,
        radix_engine_toolkit::request::CompileNotarizedTransactionRequest,
        radix_engine_toolkit::request::CompileNotarizedTransactionResponse,
        radix_engine_toolkit::request::DecompileNotarizedTransactionRequest,
        radix_engine_toolkit::request::DecompileNotarizedTransactionResponse,
        radix_engine_toolkit::request::DecompileUnknownTransactionIntentRequest,
        radix_engine_toolkit::request::DecompileUnknownTransactionIntentResponse,
        radix_engine_toolkit::request::DecodeAddressRequest,
        radix_engine_toolkit::request::DecodeAddressResponse,
        radix_engine_toolkit::request::EncodeAddressRequest,
        radix_engine_toolkit::request::EncodeAddressResponse,
        radix_engine_toolkit::request::DecodeAddressRequest,
        radix_engine_toolkit::request::DecodeAddressResponse,
        radix_engine_toolkit::request::SborEncodeRequest,
        radix_engine_toolkit::request::SborEncodeResponse,
        radix_engine_toolkit::request::SborDecodeRequest,
        radix_engine_toolkit::request::SborDecodeResponse,
        radix_engine_toolkit::request::DeriveVirtualAccountAddressRequest,
        radix_engine_toolkit::request::DeriveVirtualAccountAddressResponse,
        radix_engine_toolkit::request::DeriveVirtualIdentityAddressRequest,
        radix_engine_toolkit::request::DeriveVirtualIdentityAddressResponse,
        radix_engine_toolkit::request::DeriveNonFungibleGlobalIdFromPublicKeyRequest,
        radix_engine_toolkit::request::DeriveNonFungibleGlobalIdFromPublicKeyResponse,
        radix_engine_toolkit::request::KnownEntityAddressesRequest,
        radix_engine_toolkit::request::KnownEntityAddressesResponse,
        radix_engine_toolkit::request::StaticallyValidateTransactionRequest,
        radix_engine_toolkit::request::StaticallyValidateTransactionResponse
    );

    // Iterating over the HashMap, modifying the class name to be in snake case and writing the
    // schema to the file system
    schema_map
        .iter()
        .map(|(struct_ident, schema)| {
            (
                format!("{}.json", struct_ident.to_case(convert_case::Case::Snake)),
                schema,
            )
        })
        .map(|(file_name, schema)| {
            let path = {
                let mut path = PathBuf::from(".");
                path.push("out");
                path.push("schema");
                path.push(file_name);
                path
            };

            serde_json::to_string_pretty(schema)
                .map_err(GenerationError::SerializationError)
                .and_then(|schema_string| {
                    std::fs::write(path, schema_string).map_err(GenerationError::IOError)
                })
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(())
}

fn generate_request_examples() -> Result<(), GenerationError> {
    let examples = InMemoryExamplesBuilder::new()
        .add_example::<InformationHandler, InformationRequest, InformationResponse>()
        .add_example::<ConvertManifestHandler, ConvertManifestRequest, ConvertManifestResponse>()
        .add_example::<CompileTransactionIntentHandler, CompileTransactionIntentRequest, CompileTransactionIntentResponse>()
        .add_example::<DecompileTransactionIntentHandler, DecompileTransactionIntentRequest, DecompileTransactionIntentResponse>()
        .add_example::<CompileSignedTransactionIntentHandler, CompileSignedTransactionIntentRequest, CompileSignedTransactionIntentResponse>()
        .add_example::<DecompileSignedTransactionIntentHandler, DecompileSignedTransactionIntentRequest, DecompileSignedTransactionIntentResponse>()
        .add_example::<CompileNotarizedTransactionHandler, CompileNotarizedTransactionRequest, CompileNotarizedTransactionResponse>()
        .add_example::<DecompileNotarizedTransactionHandler, DecompileNotarizedTransactionRequest, DecompileNotarizedTransactionResponse>()
        .add_example::<EncodeAddressHandler, EncodeAddressRequest, EncodeAddressResponse>()
        .add_example::<DecodeAddressHandler, DecodeAddressRequest, DecodeAddressResponse>()
        .add_example::<SborEncodeHandler, SborEncodeRequest, SborEncodeResponse>()
        .add_example::<SborDecodeHandler, SborDecodeRequest, SborDecodeResponse>()
        .add_example::<DeriveVirtualAccountAddressHandler, DeriveVirtualAccountAddressRequest, DeriveVirtualAccountAddressResponse>()
        .add_example::<DeriveVirtualIdentityAddressHandler, DeriveVirtualIdentityAddressRequest, DeriveVirtualIdentityAddressResponse>()
        .add_example::<DeriveNonFungibleGlobalIdFromPublicKeyHandler, DeriveNonFungibleGlobalIdFromPublicKeyRequest, DeriveNonFungibleGlobalIdFromPublicKeyResponse>()
        .add_example::<StaticallyValidateTransactionHandler, StaticallyValidateTransactionRequest, StaticallyValidateTransactionResponse>()
        .add_example::<KnownEntityAddressesHandler, KnownEntityAddressesRequest, KnownEntityAddressesResponse>()
        .build();

    let path = {
        let mut path = PathBuf::from(".");
        path.push("out");
        path.push("examples");
        path.push("request-examples.md");
        path
    };

    std::fs::write(path, examples).map_err(GenerationError::IOError)
}

#[derive(Debug)]
pub enum GenerationError {
    IOError(std::io::Error),
    SerializationError(serde_json::Error),
}
