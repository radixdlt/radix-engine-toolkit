use crate::model::Value;
use crate::{error::Result, InstructionKind};
use scrypto::prelude::{hash, Hash};

/// A trait that defines a set of methods for aggregating value refs from objects
pub trait ValueRef {
    /// Used to borrow all values immutably
    fn borrow_values(&self) -> Vec<&Value>;

    /// Borrows all values mutably
    fn borrow_values_mut(&mut self) -> Vec<&mut Value>;
}

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
