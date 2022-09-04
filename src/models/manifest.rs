use crate::models::instruction::Instruction;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "value")]
pub enum Manifest {
    String(String),
    JSON(Vec<Instruction>),
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ManifestKind {
    String,
    JSON,
}
