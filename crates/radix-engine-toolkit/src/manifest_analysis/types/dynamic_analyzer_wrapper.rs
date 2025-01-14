use crate::internal_prelude::*;

/// A wrapper type that provides a blanket implementation of the dynamic
/// analyzer for types that implement the [`ManifestStaticAnalyzer`] trait.
///
/// Currently, not all the analyzers implement the [`ManifestDynamicAnalyzer`]
/// trait. The ones that only require static analysis do not implement it. But,
/// the primary traversal logic we have in the code base assumes that we have
/// analyzers that all implement [`ManifestDynamicAnalyzer`].
///
/// Therefore, we add this type as a wrapper to any [`ManifestStaticAnalyzer`]
/// and this type itself implements the [`ManifestStaticAnalyzer`] in a way
/// where all of the calls are passed to the underlying type. This type also
/// implements the [`ManifestDynamicAnalyzer`] trait in a way where the no
/// actions are done at all.
pub struct DynamicAnalyzerWrapper<A: ManifestStaticAnalyzer>(A);

impl<A: ManifestStaticAnalyzer> ManifestStaticAnalyzer
    for DynamicAnalyzerWrapper<A>
{
    type Initializer = A::Initializer;
    type Output = A::Output;
    type PermissionState = A::PermissionState;
    type RequirementState = A::RequirementState;

    fn new(
        initializer: Self::Initializer,
    ) -> (Self, Self::PermissionState, Self::RequirementState) {
        let (analyzer, permission_state, requirement_state) =
            A::new(initializer);
        (Self(analyzer), permission_state, requirement_state)
    }

    fn output(self) -> Self::Output {
        self.0.output()
    }

    fn process_permission(
        &mut self,
        permission_state: &mut Self::PermissionState,
        named_address_store: &NamedAddressStore,
        instruction: &GroupedInstruction,
        maybe_typed_invocation: Option<(
            &ManifestInvocationReceiver,
            &TypedManifestNativeInvocation,
        )>,
    ) {
        self.0.process_permission(
            permission_state,
            named_address_store,
            instruction,
            maybe_typed_invocation,
        );
    }

    fn process_requirement(
        &mut self,
        requirement_state: &mut Self::RequirementState,
        named_address_store: &NamedAddressStore,
        instruction: &GroupedInstruction,
        maybe_typed_invocation: Option<(
            &ManifestInvocationReceiver,
            &TypedManifestNativeInvocation,
        )>,
    ) {
        self.0.process_requirement(
            requirement_state,
            named_address_store,
            instruction,
            maybe_typed_invocation,
        );
    }

    fn process_instruction(
        &mut self,
        named_address_store: &NamedAddressStore,
        instruction: &GroupedInstruction,
        maybe_typed_invocation: Option<(
            &ManifestInvocationReceiver,
            &TypedManifestNativeInvocation,
        )>,
    ) {
        self.0.process_instruction(
            named_address_store,
            instruction,
            maybe_typed_invocation,
        );
    }
}

impl<A: ManifestStaticAnalyzer> ManifestDynamicAnalyzer
    for DynamicAnalyzerWrapper<A>
{
    type Output = ();
    type RequirementState = ConstState<true>;

    fn new(
        initializer: Self::Initializer,
    ) -> (
        Self,
        <Self as ManifestStaticAnalyzer>::PermissionState,
        <Self as ManifestStaticAnalyzer>::RequirementState,
        <Self as ManifestDynamicAnalyzer>::RequirementState,
    ) {
        let (analyzer, permission_state, requirement_state) =
            <Self as ManifestStaticAnalyzer>::new(initializer);
        (
            analyzer,
            permission_state,
            requirement_state,
            ConstState::<true>,
        )
    }

    fn output(
        self,
    ) -> CombinedAnalysisOutput<
        <Self as ManifestStaticAnalyzer>::Output,
        <Self as ManifestDynamicAnalyzer>::Output,
    > {
        CombinedAnalysisOutput {
            static_analyzer_output: self.0.output(),
            dynamic_analyzer_output: (),
        }
    }

    fn process_requirement(
        &mut self,
        _: &mut <Self as ManifestDynamicAnalyzer>::RequirementState,
        _: &NamedAddressStore,
        _: &GroupedInstruction,
        _: Option<(
            &ManifestInvocationReceiver,
            &TypedManifestNativeInvocation,
        )>,
    ) {
    }

    fn process_instruction(
        &mut self,
        _: &NamedAddressStore,
        _: &GroupedInstruction,
        _: Option<(
            &ManifestInvocationReceiver,
            &TypedManifestNativeInvocation,
        )>,
    ) {
    }
}
