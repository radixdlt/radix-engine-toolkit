// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use crate::internal_prelude::*;

#[macro_export]
macro_rules! define_composite_analyzer {
    (
        type_ident: $type_ident: ident,
        analyzers: {
            $(
                $analyzer_ident: ident: (
                    $analyzer_type: ty,
                    $analyzer_initializer: expr
                    $(,)?
                )
            ),* $(,)?
        } $(,)?
    ) => {
        paste! {
            pub struct [< $type_ident Analyzer >] {
                $(
                    $analyzer_ident: ($analyzer_type, bool),
                )*
            }

            impl $crate::internal_prelude::ManifestStaticAnalyzer
                for [< $type_ident Analyzer >]
            {
                type Initializer = [< $type_ident Initializer >];
                type Output = [< $type_ident StaticOutput >];
                type PermissionState = [< $type_ident StaticPermissionState >];
                type RequirementState = [< $type_ident StaticRequirementState >];

                fn new(
                    initializer: Self::Initializer,
                ) -> (Self, Self::PermissionState, Self::RequirementState) {
                    $(
                        let (
                            [< $analyzer_ident _analyzer >],
                            [< $analyzer_ident _permission_state >],
                            [< $analyzer_ident _requirement_state >],
                        ) = <$analyzer_type as $crate::internal_prelude::ManifestStaticAnalyzer>::new(initializer.$analyzer_ident);
                    )*

                    let analyzers = [< $type_ident Analyzer >] {
                        $(
                            $analyzer_ident: ([< $analyzer_ident _analyzer >], true),
                        )*
                    };
                    let permission_states = [< $type_ident StaticPermissionState >] {
                        $(
                            $analyzer_ident: [< $analyzer_ident _permission_state >],
                        )*
                    };
                    let requirement_states = [< $type_ident StaticRequirementState >] {
                        $(
                            $analyzer_ident: [< $analyzer_ident _requirement_state >],
                        )*
                    };

                    (
                        analyzers,
                        permission_states,
                        requirement_states
                    )
                }

                fn output(self) -> Self::Output {
                    [< $type_ident StaticOutput >] {
                        $(
                            $analyzer_ident: $crate::internal_prelude::ManifestStaticAnalyzer::output(self.$analyzer_ident.0),
                        )*
                    }
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
                    $(
                        if $crate::internal_prelude::ManifestAnalyzerPermissionState::all_instructions_permitted(&permission_state.$analyzer_ident) {
                            $crate::internal_prelude::ManifestStaticAnalyzer::process_permission(
                                &mut self.$analyzer_ident.0,
                                &mut permission_state.$analyzer_ident,
                                named_address_store,
                                instruction,
                                maybe_typed_invocation,
                            );
                        }
                        self.$analyzer_ident.1 = $crate::internal_prelude::ManifestAnalyzerPermissionState::all_instructions_permitted(&permission_state.$analyzer_ident);
                    )*
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
                    $(
                        if self.$analyzer_ident.1 {
                            $crate::internal_prelude::ManifestStaticAnalyzer::process_requirement(
                                &mut self.$analyzer_ident.0,
                                &mut requirement_state.$analyzer_ident,
                                named_address_store,
                                instruction,
                                maybe_typed_invocation,
                            );
                        }
                    )*
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
                    $(
                        if self.$analyzer_ident.1 {
                            $crate::internal_prelude::ManifestStaticAnalyzer::process_instruction(
                                &mut self.$analyzer_ident.0,
                                named_address_store,
                                instruction,
                                maybe_typed_invocation,
                            );
                        }
                    )*
                }
            }

            impl $crate::internal_prelude::ManifestDynamicAnalyzer
                for [< $type_ident Analyzer >]
            {
                type Output = [< $type_ident DynamicOutput >];
                type RequirementState = [< $type_ident DynamicRequirementState >];

                fn new(
                    initializer: Self::Initializer,
                ) -> (
                    Self,
                    <Self as $crate::internal_prelude::ManifestStaticAnalyzer>::PermissionState,
                    <Self as $crate::internal_prelude::ManifestStaticAnalyzer>::RequirementState,
                    <Self as $crate::internal_prelude::ManifestDynamicAnalyzer>::RequirementState,
                )
                {
                    $(
                        let (
                            [< $analyzer_ident _analyzer >],
                            [< $analyzer_ident _static_permission_state >],
                            [< $analyzer_ident _static_requirement_state >],
                            [< $analyzer_ident _dynamic_requirement_state >],
                        ) = <$analyzer_type as $crate::internal_prelude::ManifestDynamicAnalyzer>::new(initializer.$analyzer_ident);
                    )*

                    let analyzers = [< $type_ident Analyzer >] {
                        $(
                            $analyzer_ident: ([< $analyzer_ident _analyzer >], true),
                        )*
                    };
                    let static_permission_states = [< $type_ident StaticPermissionState >] {
                        $(
                            $analyzer_ident: [< $analyzer_ident _static_permission_state >],
                        )*
                    };
                    let static_requirement_states = [< $type_ident StaticRequirementState >] {
                        $(
                            $analyzer_ident: [< $analyzer_ident _static_requirement_state >],
                        )*
                    };
                    let dynamic_requirement_states = [< $type_ident DynamicRequirementState >] {
                        $(
                            $analyzer_ident: [< $analyzer_ident _dynamic_requirement_state >],
                        )*
                    };

                    (
                        analyzers,
                        static_permission_states,
                        static_requirement_states,
                        dynamic_requirement_states
                    )
                }

                fn output(self) -> $crate::internal_prelude::CombinedAnalysisOutput<
                    [< $type_ident StaticOutput >],
                    [< $type_ident DynamicOutput >],
                > {
                    $(
                        let $crate::internal_prelude::CombinedAnalysisOutput {
                            static_analyzer_output: [< $analyzer_ident _static_output >],
                            dynamic_analyzer_output: [< $analyzer_ident _dynamic_output >],
                        } = $crate::internal_prelude::ManifestDynamicAnalyzer::output(self.$analyzer_ident.0);
                    )*

                    $crate::internal_prelude::CombinedAnalysisOutput {
                        static_analyzer_output: [< $type_ident StaticOutput >] {
                            $(
                                $analyzer_ident: [< $analyzer_ident _static_output >],
                            )*
                        },
                        dynamic_analyzer_output: [< $type_ident DynamicOutput >] {
                            $(
                                $analyzer_ident: [< $analyzer_ident _dynamic_output >],
                            )*
                        }
                    }
                }

                fn process_requirement(
                    &mut self,
                    requirement_state: &mut <Self as $crate::internal_prelude::ManifestDynamicAnalyzer>::RequirementState,
                    named_address_store: &NamedAddressStore,
                    instruction: &GroupedInstruction,
                    maybe_typed_invocation: Option<(
                        &ManifestInvocationReceiver,
                        &TypedManifestNativeInvocation,
                    )>,
                ) {
                    $(
                        if self.$analyzer_ident.1 {
                            $crate::internal_prelude::ManifestDynamicAnalyzer::process_requirement(
                                &mut self.$analyzer_ident.0,
                                &mut requirement_state.$analyzer_ident,
                                named_address_store,
                                instruction,
                                maybe_typed_invocation,
                            );
                        }
                    )*
                }

                fn process_instruction(
                    &mut self,
                    named_address_store: &NamedAddressStore,
                    instruction: &GroupedInstruction,
                    invocation_io: &InvocationIo<InvocationIoItems>,
                    maybe_typed_invocation: Option<(
                        &ManifestInvocationReceiver,
                        &TypedManifestNativeInvocation,
                    )>,
                ) {
                    $(
                        if self.$analyzer_ident.1 {
                            $crate::internal_prelude::ManifestDynamicAnalyzer::process_instruction(
                                &mut self.$analyzer_ident.0,
                                named_address_store,
                                instruction,
                                invocation_io,
                                maybe_typed_invocation,
                            );
                        }
                    )*
                }
            }

            pub struct [< $type_ident Initializer >] {
                $(
                    pub $analyzer_ident: <$analyzer_type as $crate::internal_prelude::ManifestStaticAnalyzer>::Initializer,
                )*
            }

            pub struct [< $type_ident StaticOutput >] {
                $(
                    pub $analyzer_ident: <$analyzer_type as $crate::internal_prelude::ManifestStaticAnalyzer>::Output,
                )*
            }

            pub struct [< $type_ident ResolvedStaticOutput >] {
                $(
                    pub $analyzer_ident: Option<<$analyzer_type as $crate::internal_prelude::ManifestStaticAnalyzer>::Output>,
                )*
            }

            pub struct [< $type_ident DynamicOutput >] {
                $(
                    pub $analyzer_ident: <$analyzer_type as $crate::internal_prelude::ManifestDynamicAnalyzer>::Output,
                )*
            }

            pub struct [< $type_ident ResolvedDynamicOutput >] {
                $(
                    pub $analyzer_ident: $crate::internal_prelude::CombinedAnalysisOutput<
                        Option< <$analyzer_type as $crate::internal_prelude::ManifestStaticAnalyzer>::Output >,
                        Option< <$analyzer_type as $crate::internal_prelude::ManifestDynamicAnalyzer>::Output >,
                    >,
                )*
            }

            pub struct [< $type_ident StaticPermissionState >] {
                $(
                    $analyzer_ident: <$analyzer_type as $crate::internal_prelude::ManifestStaticAnalyzer>::PermissionState,
                )*
            }


            impl Default for [< $type_ident Initializer >] {
                fn default() -> Self {
                    [< $type_ident Initializer >] {
                        $(
                            $analyzer_ident: $analyzer_initializer,
                        )*
                    }
                }
            }

            impl [< $type_ident ResolvedStaticOutput >] {
                pub fn new(
                    output: [< $type_ident StaticOutput >],
                    permission_state: &[< $type_ident StaticPermissionState >],
                    requirement_state: &[< $type_ident StaticRequirementState >],
                ) -> Self {
                    Self {
                        $(
                            $analyzer_ident: (
                                $crate::internal_prelude::ManifestAnalyzerPermissionState::all_instructions_permitted(&permission_state.$analyzer_ident)
                                && $crate::internal_prelude::ManifestAnalyzerRequirementState::all_requirements_met(&requirement_state.$analyzer_ident)
                            ).then_some(output.$analyzer_ident),
                        )*
                    }
                }
            }

            impl [< $type_ident ResolvedDynamicOutput >] {
                pub fn new(
                    output: $crate::internal_prelude::CombinedAnalysisOutput<
                        [< $type_ident StaticOutput >],
                        [< $type_ident DynamicOutput >],
                    >,
                    static_permission_state: &[< $type_ident StaticPermissionState >],
                    static_requirement_state: &[< $type_ident StaticRequirementState >],
                    dynamic_requirement_state: &[< $type_ident DynamicRequirementState >],
                ) -> Self {
                    [< $type_ident ResolvedDynamicOutput >] {
                        $(
                            $analyzer_ident: $crate::internal_prelude::CombinedAnalysisOutput {
                                static_analyzer_output: (
                                    $crate::internal_prelude::ManifestAnalyzerPermissionState::all_instructions_permitted(&static_permission_state.$analyzer_ident)
                                    && $crate::internal_prelude::ManifestAnalyzerRequirementState::all_requirements_met(&static_requirement_state.$analyzer_ident)
                                )
                                .then_some(output.static_analyzer_output.$analyzer_ident),
                                dynamic_analyzer_output: (
                                    $crate::internal_prelude::ManifestAnalyzerPermissionState::all_instructions_permitted(&static_permission_state.$analyzer_ident)
                                    && $crate::internal_prelude::ManifestAnalyzerRequirementState::all_requirements_met(&static_requirement_state.$analyzer_ident)
                                    && $crate::internal_prelude::ManifestAnalyzerRequirementState::all_requirements_met(&dynamic_requirement_state.$analyzer_ident)
                                )
                                .then_some(output.dynamic_analyzer_output.$analyzer_ident)
                            },
                        )*
                    }
                }
            }

            impl $crate::internal_prelude::ManifestAnalyzerPermissionState
                for [< $type_ident StaticPermissionState >]
            {
                fn all_instructions_permitted(&self) -> bool {
                    // We always permit instructions
                    true
                }
            }

            pub struct [< $type_ident StaticRequirementState >] {
                $(
                    $analyzer_ident: <$analyzer_type as $crate::internal_prelude::ManifestStaticAnalyzer>::RequirementState,
                )*
            }

            impl $crate::internal_prelude::ManifestAnalyzerRequirementState
                for [< $type_ident StaticRequirementState >]
            {
                fn all_requirements_met(&self) -> bool {
                    // The requirement for this analyzer is always met.
                    true
                }
            }

            pub struct [< $type_ident DynamicRequirementState >] {
                $(
                    $analyzer_ident: <$analyzer_type as $crate::internal_prelude::ManifestDynamicAnalyzer>::RequirementState,
                )*
            }

            impl $crate::internal_prelude::ManifestAnalyzerRequirementState
                for [< $type_ident DynamicRequirementState >]
            {
                fn all_requirements_met(&self) -> bool {
                    // The requirement for this analyzer is always met.
                    true
                }
            }
        }
    };
}
pub use define_composite_analyzer;
