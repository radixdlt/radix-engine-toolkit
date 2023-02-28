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

use radix_engine_toolkit::request::analyze_manifest::{
    AnalyzeManifestHandler, AnalyzeManifestRequest, AnalyzeManifestResponse,
};
use radix_engine_toolkit::request::compile_notarized_transaction::{
    CompileNotarizedTransactionHandler, CompileNotarizedTransactionRequest,
    CompileNotarizedTransactionResponse,
};
use radix_engine_toolkit::request::compile_signed_transaction_intent::{
    CompileSignedTransactionIntentHandler, CompileSignedTransactionIntentRequest,
    CompileSignedTransactionIntentResponse,
};
use radix_engine_toolkit::request::compile_transaction_intent::{
    CompileTransactionIntentHandler, CompileTransactionIntentRequest,
    CompileTransactionIntentResponse,
};
use radix_engine_toolkit::request::convert_manifest::{
    ConvertManifestHandler, ConvertManifestRequest, ConvertManifestResponse,
};
use radix_engine_toolkit::request::decode_address::{
    DecodeAddressHandler, DecodeAddressRequest, DecodeAddressResponse,
};
use radix_engine_toolkit::request::decompile_notarized_transaction::{
    DecompileNotarizedTransactionHandler, DecompileNotarizedTransactionRequest,
    DecompileNotarizedTransactionResponse,
};
use radix_engine_toolkit::request::decompile_signed_transaction_intent::{
    DecompileSignedTransactionIntentHandler, DecompileSignedTransactionIntentRequest,
    DecompileSignedTransactionIntentResponse,
};
use radix_engine_toolkit::request::decompile_transaction_intent::{
    DecompileTransactionIntentHandler, DecompileTransactionIntentRequest,
    DecompileTransactionIntentResponse,
};
use radix_engine_toolkit::request::decompile_unknown_intent::{
    DecompileUnknownTransactionIntentHandler, DecompileUnknownTransactionIntentRequest,
    DecompileUnknownTransactionIntentResponse,
};
use radix_engine_toolkit::request::derive_non_fungible_global_id_from_public_key::{
    DeriveNonFungibleGlobalIdFromPublicKeyHandler, DeriveNonFungibleGlobalIdFromPublicKeyRequest,
    DeriveNonFungibleGlobalIdFromPublicKeyResponse,
};
use radix_engine_toolkit::request::derive_virtual_account_address::{
    DeriveVirtualAccountAddressHandler, DeriveVirtualAccountAddressRequest,
    DeriveVirtualAccountAddressResponse,
};
use radix_engine_toolkit::request::derive_virtual_identity_address::{
    DeriveVirtualIdentityAddressHandler, DeriveVirtualIdentityAddressRequest,
    DeriveVirtualIdentityAddressResponse,
};
use radix_engine_toolkit::request::encode_address::{
    EncodeAddressHandler, EncodeAddressRequest, EncodeAddressResponse,
};
use radix_engine_toolkit::request::information::{
    InformationHandler, InformationRequest, InformationResponse,
};
use radix_engine_toolkit::request::known_entity_addresses::{
    KnownEntityAddressesHandler, KnownEntityAddressesRequest, KnownEntityAddressesResponse,
};
use radix_engine_toolkit::request::sbor_decode::{
    SborDecodeHandler, SborDecodeRequest, SborDecodeResponse,
};
use radix_engine_toolkit::request::sbor_encode::{
    SborEncodeHandler, SborEncodeRequest, SborEncodeResponse,
};
use radix_engine_toolkit::request::statically_validate_transaction::{
    StaticallyValidateTransactionHandler, StaticallyValidateTransactionRequest,
    StaticallyValidateTransactionResponse,
};

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
        StaticallyValidateTransactionResponse
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
        .add_example::<AnalyzeManifestHandler, AnalyzeManifestRequest, AnalyzeManifestResponse>()
        .add_example::<CompileTransactionIntentHandler, CompileTransactionIntentRequest, CompileTransactionIntentResponse>()
        .add_example::<DecompileTransactionIntentHandler, DecompileTransactionIntentRequest, DecompileTransactionIntentResponse>()
        .add_example::<CompileSignedTransactionIntentHandler, CompileSignedTransactionIntentRequest, CompileSignedTransactionIntentResponse>()
        .add_example::<DecompileSignedTransactionIntentHandler, DecompileSignedTransactionIntentRequest, DecompileSignedTransactionIntentResponse>()
        .add_example::<CompileNotarizedTransactionHandler, CompileNotarizedTransactionRequest, CompileNotarizedTransactionResponse>()
        .add_example::<DecompileNotarizedTransactionHandler, DecompileNotarizedTransactionRequest, DecompileNotarizedTransactionResponse>()
        .add_example::<DecompileUnknownTransactionIntentHandler, DecompileUnknownTransactionIntentRequest, DecompileUnknownTransactionIntentResponse>()
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
