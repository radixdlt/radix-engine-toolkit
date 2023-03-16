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
        InformationRequest,
        InformationResponse,
        ConvertManifestRequest,
        ConvertManifestResponse,
        AnalyzeManifestRequest,
        AnalyzeManifestResponse,
        AnalyzeManifestWithPreviewContextRequest,
        AnalyzeManifestWithPreviewContextResponse,
        CompileTransactionIntentRequest,
        CompileTransactionIntentResponse,
        DecompileTransactionIntentRequest,
        DecompileTransactionIntentResponse,
        CompileSignedTransactionIntentRequest,
        CompileSignedTransactionIntentResponse,
        DecompileSignedTransactionIntentRequest,
        DecompileSignedTransactionIntentResponse,
        CompileNotarizedTransactionRequest,
        CompileNotarizedTransactionResponse,
        DecompileNotarizedTransactionRequest,
        DecompileNotarizedTransactionResponse,
        DecompileUnknownTransactionIntentRequest,
        DecompileUnknownTransactionIntentResponse,
        DecodeAddressRequest,
        DecodeAddressResponse,
        EncodeAddressRequest,
        EncodeAddressResponse,
        DecodeAddressRequest,
        DecodeAddressResponse,
        SborEncodeRequest,
        SborEncodeResponse,
        SborDecodeRequest,
        SborDecodeResponse,
        DeriveVirtualAccountAddressRequest,
        DeriveVirtualAccountAddressResponse,
        DeriveVirtualIdentityAddressRequest,
        DeriveVirtualIdentityAddressResponse,
        DeriveNonFungibleGlobalIdFromPublicKeyRequest,
        DeriveNonFungibleGlobalIdFromPublicKeyResponse,
        KnownEntityAddressesRequest,
        KnownEntityAddressesResponse,
        StaticallyValidateTransactionRequest,
        StaticallyValidateTransactionResponse,
        HashRequest,
        HashResponse
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
        .add_example::<InformationHandler, _, _>()
        .add_example::<ConvertManifestHandler, _, _>()
        .add_example::<AnalyzeManifestHandler, _, _>()
        .add_example::<CompileTransactionIntentHandler, _, _>()
        .add_example::<DecompileTransactionIntentHandler, _, _>()
        .add_example::<CompileSignedTransactionIntentHandler, _, _>()
        .add_example::<DecompileSignedTransactionIntentHandler, _, _>()
        .add_example::<CompileNotarizedTransactionHandler, _, _>()
        .add_example::<DecompileNotarizedTransactionHandler, _, _>()
        .add_example::<DecompileUnknownTransactionIntentHandler, _, _>()
        .add_example::<EncodeAddressHandler, _, _>()
        .add_example::<DecodeAddressHandler, _, _>()
        .add_example::<SborEncodeHandler, _, _>()
        .add_example::<SborDecodeHandler, _, _>()
        .add_example::<DeriveVirtualAccountAddressHandler, _, _>()
        .add_example::<DeriveVirtualIdentityAddressHandler, _, _>()
        .add_example::<DeriveNonFungibleGlobalIdFromPublicKeyHandler, _, _>()
        .add_example::<StaticallyValidateTransactionHandler, _, _>()
        .add_example::<KnownEntityAddressesHandler, _, _>()
        .add_example::<HashHandler, _, _>()
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
