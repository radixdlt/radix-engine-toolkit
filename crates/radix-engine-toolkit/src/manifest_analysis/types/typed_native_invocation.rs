use crate::internal_prelude::*;

/// A type representing a "fully qualified" manifest native invocation that has
/// the receiver and the typed invocation.
#[derive(Debug)]
pub struct TypedNativeInvocation {
    pub receiver: ManifestInvocationReceiver,
    pub invocation: TypedManifestNativeInvocation,
}
