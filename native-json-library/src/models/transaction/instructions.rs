use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::instruction::SerializableInstruction;

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
#[serde(tag = "kind", content = "value")]
pub enum SerializableInstructions {
    String(String),
    Parsed(Vec<SerializableInstruction>),
}

