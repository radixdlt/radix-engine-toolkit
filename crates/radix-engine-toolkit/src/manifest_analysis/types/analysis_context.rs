use crate::internal_prelude::*;

/// Context given by the traverser to the visitors when calling them.
#[derive(Clone, Copy, Debug)]
pub enum InstructionContext<'a> {
    /// The instruction is an invocation instruction and therefore it has
    /// additional context compared to other instructions.
    InvocationInstruction {
        /// A store that contains the [`ManifestNamedAddress`]es from all of the
        /// address allocations that took place in the manifest and their
        /// associated [`BlueprintId`]s.
        named_address_store: &'a NamedAddressStore,
        /// The index of the instruction that the visitor is currently at.
        instruction_index: &'a InstructionIndex,
        /// The current instruction being processed by the visitor. This
        /// instruction is given as a [`GroupedInstruction`] converted from the
        /// [`AnyInstruction`]s found in the manifest.
        instruction: &'a GroupedInstruction,
        /// If conversion into a  [`TypedManifestNativeInvocation`] succeeds
        /// then this field will be provided and will be [`Some`]. If it's not
        /// provided then the invocation is to a blueprint we don't currently
        /// support typing for, or is not a native invocation.
        typed_native_invocation: Option<&'a TypedNativeInvocation>,
        /// The inputs and outputs of the invocation as seen by the static
        /// analyzer.
        static_analysis_invocation_io: &'a InvocationIo<TrackedResources>,
        /// The inputs and outputs of the invocation as seen by the dynamic
        /// analyzer. If not provided then no dynamic analysis context was
        /// given to the traverser. This field *WILL* be provided in the case
        /// that the dynamic analysis context is given and even when no IO has
        /// happened.
        dynamic_analysis_invocation_io:
            Option<&'a InvocationIo<InvocationIoItems>>,
    },
    /// The instruction is an non-invocation instruction and therefore it has
    /// the base set of information we provide for instructions.
    NonInvocationInstruction {
        /// A store that contains the [`ManifestNamedAddress`]es from all of the
        /// address allocations that took place in the manifest and their
        /// associated [`BlueprintId`]s.
        named_address_store: &'a NamedAddressStore,
        /// The index of the instruction that the visitor is currently at.
        instruction_index: &'a InstructionIndex,
        /// The current instruction being processed by the visitor. This
        /// instruction is given as a [`GroupedInstruction`] converted from the
        /// [`AnyInstruction`]s found in the manifest.
        instruction: &'a GroupedInstruction,
    },
}

impl<'a> InstructionContext<'a> {
    pub fn named_address_store(&self) -> &'a NamedAddressStore {
        match self {
            Self::InvocationInstruction {
                named_address_store,
                ..
            }
            | Self::NonInvocationInstruction {
                named_address_store,
                ..
            } => named_address_store,
        }
    }

    pub fn instruction(&self) -> &'a GroupedInstruction {
        match self {
            Self::InvocationInstruction { instruction, .. }
            | Self::NonInvocationInstruction { instruction, .. } => instruction,
        }
    }

    pub fn instruction_index(&self) -> &'a InstructionIndex {
        match self {
            Self::InvocationInstruction {
                instruction_index, ..
            }
            | Self::NonInvocationInstruction {
                instruction_index, ..
            } => instruction_index,
        }
    }
}
