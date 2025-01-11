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
macro_rules! define_composite_visitor {
    (
        type_ident: $composite_visitor_ident: ident,
        visitors: {
            $(
                $visitor_ident: ident: (
                    $visitor_type: ty,
                    $visitor_constructor: expr
                    $(,)?
                )
            ),* $(,)?
        } $(,)?
    ) => {
        paste! {
            #[derive(Default)]
            pub struct [< $composite_visitor_ident Visitor >] {
                visitors: [< $composite_visitor_ident Visitors >],
                validity_state: [< $composite_visitor_ident ManifestAnalysisVisitorValidityState >]
            }

            impl [< $composite_visitor_ident Visitor >] {
                pub fn new() -> Self {
                    Default::default()
                }
            }

            impl $crate::internal_prelude::ManifestAnalysisVisitor
                for [< $composite_visitor_ident Visitor >]
            {
                type Output = [< $composite_visitor_ident VisitorOutput >];
                type ValidityState = [< $composite_visitor_ident ManifestAnalysisVisitorValidityState >];

                fn output(self) -> Self::Output {
                    [< $composite_visitor_ident VisitorOutput >] {
                        $(
                            $visitor_ident: $crate::internal_prelude::ManifestAnalysisVisitor::output(self.visitors.$visitor_ident)
                        )*
                    }
                }

                fn validity_state(&self) -> &Self::ValidityState {
                    &self.validity_state
                }

                fn on_instruction(
                    &mut self,
                    named_address_store: &NamedAddressStore,
                    grouped_instruction: &GroupedInstruction,
                    instruction_index: &InstructionIndex,
                    maybe_typed_invocation: Option<&TypedManifestNativeInvocation>,
                    maybe_invocation_io: Option<&InvocationIo<InvocationIoItems>>,
                ) {
                    $(
                        if self.validity_state.$visitor_ident {
                           self
                                .visitors
                                .$visitor_ident
                                .on_instruction(
                                    named_address_store,
                                    grouped_instruction,
                                    instruction_index,
                                    maybe_typed_invocation,
                                    maybe_invocation_io
                                );
                            self.validity_state.$visitor_ident &= self
                                .visitors
                                .$visitor_ident
                                .validity_state()
                                .is_visitor_accepting_instructions();
                        }
                    )*
                }
            }

            struct [< $composite_visitor_ident Visitors >] {
                $(
                    $visitor_ident: $visitor_type,
                )*
            }

            impl Default for [< $composite_visitor_ident Visitors >] {
                fn default() -> Self {
                    Self {
                        $(
                            $visitor_ident: $visitor_constructor,
                        )*
                    }
                }
            }

            pub struct [< $composite_visitor_ident ManifestAnalysisVisitorValidityState >] {
                $(
                    $visitor_ident: bool,
                )*
            }

            impl Default for [< $composite_visitor_ident ManifestAnalysisVisitorValidityState >] {
                fn default() -> Self {
                    Self {
                        $(
                            $visitor_ident: true,
                        )*
                    }
                }
            }

            impl $crate::internal_prelude::ManifestAnalysisVisitorValidityState for
                [< $composite_visitor_ident ManifestAnalysisVisitorValidityState >]
            {
                fn is_visitor_accepting_instructions(&self) -> bool {
                    $(
                        self.$visitor_ident
                    ) || *
                }
            }

            pub struct [< $composite_visitor_ident VisitorOutput >] {
                $(
                    pub $visitor_ident: <$visitor_type as $crate::internal_prelude::ManifestAnalysisVisitor>::Output,
                )*
            }
        }
    };
}
pub use define_composite_visitor;
