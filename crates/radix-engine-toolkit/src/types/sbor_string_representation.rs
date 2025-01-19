use crate::internal_prelude::*;

#[derive(Debug, Clone)]
pub enum ScryptoSborStringRepresentation {
    ProgrammaticJson(String),
}

#[derive(Clone, Copy)]
pub enum ManifestSborStringRepresentation {
    ManifestString,
    JSON(SerializationMode),
}
