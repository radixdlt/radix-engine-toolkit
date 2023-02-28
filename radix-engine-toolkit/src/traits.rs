use crate::error::Result;
use crate::model::transaction::InstructionKind;
use scrypto::prelude::{hash, Hash};

/// A trait that defines the common interface of all compile-able intents
pub trait CompilableIntent {
    fn compile(&self) -> Result<Vec<u8>>;

    fn decompile<T>(data: &T, instructions_kind: InstructionKind) -> Result<Self>
    where
        Self: Sized,
        T: AsRef<[u8]>;

    fn hash(&self) -> Result<Hash> {
        self.compile().map(hash)
    }
}
