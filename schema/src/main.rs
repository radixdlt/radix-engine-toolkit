use std::io::Write;

use schemars::{schema_for, JsonSchema};

fn main() {
    generate_json_schema().expect("Failed to generate schema")
}

pub fn generate_json_schema() -> Result<(), JsonSchemaGenerationError> {
    // This type includes all of the top level interfaces that the Radix Engine Toolkit has.

    #[allow(dead_code)]
    #[derive(JsonSchema, serde::Serialize)]
    #[serde(untagged)]
    /// This schema describes the types that the Radix Engine Toolkit accepts and returns. In this
    /// schema you will find a number of models, the most basic one of them is the `Value` model
    /// which describes how primitive types and Scrypto values are represented when communicating
    /// with the Radix Engine Toolkit. In addition to that, there are additional models which
    /// describe what instructions look like, transactions, and any other models.
    ///
    /// This schema has been automatically generated and therefore should hopefully always be the
    /// ground truth for the Radix Engine Toolkit models
    pub enum RadixEngineToolkit {
        Value(core::Value),
        Error(core::Error),
        Instruction(core::Instruction),
    }

    // Generating the Radix Engine Toolkit schema
    let schema = schema_for!(RadixEngineToolkit);

    // Serialize the Schema to a JSON pretty (formatted) text
    let serialized_schema = serde_json::to_string_pretty(&schema)
        .map_err(JsonSchemaGenerationError::SerializationError)?;

    // Write the Schema to a file
    std::fs::File::create("schema.json")
        .map_err(JsonSchemaGenerationError::IOError)?
        .write_all(serialized_schema.as_bytes())
        .map_err(JsonSchemaGenerationError::IOError)?;

    Ok(())
}

#[derive(Debug)]
pub enum JsonSchemaGenerationError {
    IOError(std::io::Error),
    SerializationError(serde_json::Error),
}
