use std::collections::HashMap;
use std::path::PathBuf;

use convert_case::Casing;

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
    generate_json_schema().expect("Failed to generate schema")
}

pub fn generate_json_schema() -> Result<(), JsonSchemaGenerationError> {
    // Creating the schema for all of the request and response types through the generate schema
    // macro
    let schema_map = generate_schema_hashmap!(
        core::request::InformationRequest,
        core::request::InformationResponse,
        core::request::ConvertManifestRequest,
        core::request::ConvertManifestResponse,
        core::request::CompileTransactionIntentRequest,
        core::request::CompileTransactionIntentResponse,
        core::request::DecompileTransactionIntentRequest,
        core::request::DecompileTransactionIntentResponse,
        core::request::CompileSignedTransactionIntentRequest,
        core::request::CompileSignedTransactionIntentResponse,
        core::request::DecompileSignedTransactionIntentRequest,
        core::request::DecompileSignedTransactionIntentResponse,
        core::request::CompileNotarizedTransactionRequest,
        core::request::CompileNotarizedTransactionResponse,
        core::request::DecompileNotarizedTransactionRequest,
        core::request::DecompileNotarizedTransactionResponse,
        core::request::DecompileUnknownTransactionIntentRequest,
        core::request::DecompileUnknownTransactionIntentResponse,
        core::request::DecodeAddressRequest,
        core::request::DecodeAddressResponse,
        core::request::EncodeAddressRequest,
        core::request::EncodeAddressResponse,
        core::request::DecodeAddressRequest,
        core::request::DecodeAddressResponse,
        core::request::SborEncodeRequest,
        core::request::SborEncodeResponse,
        core::request::SborDecodeRequest,
        core::request::SborDecodeResponse,
        core::request::DeriveVirtualAccountAddressRequest,
        core::request::DeriveVirtualAccountAddressResponse,
        core::request::KnownEntityAddressesRequest,
        core::request::KnownEntityAddressesResponse,
        core::request::StaticallyValidateTransactionRequest,
        core::request::StaticallyValidateTransactionResponse
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
                .map_err(JsonSchemaGenerationError::SerializationError)
                .and_then(|schema_string| {
                    std::fs::write(path, schema_string).map_err(JsonSchemaGenerationError::IOError)
                })
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(())
}

#[derive(Debug)]
pub enum JsonSchemaGenerationError {
    IOError(std::io::Error),
    SerializationError(serde_json::Error),
}
