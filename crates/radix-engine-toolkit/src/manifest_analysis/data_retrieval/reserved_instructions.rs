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

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct ReservedInstructionsAnalyzer(ReservedInstructionsOutput);

impl ManifestStaticAnalyzer for ReservedInstructionsAnalyzer {
    type Initializer = ();
    type Output = ReservedInstructionsOutput;
    type PermissionState = ConstState<true>;
    type RequirementState = ConstState<true>;

    fn new(
        _: Self::Initializer,
    ) -> (Self, Self::PermissionState, Self::RequirementState) {
        Default::default()
    }

    fn output(self) -> Self::Output {
        self.0
    }

    fn process_requirement(
        &self,
        _: &mut Self::RequirementState,
        _: AnalysisContext<'_>,
    ) {
    }

    fn process_instruction(&mut self, context: AnalysisContext<'_>) {
        let AnalysisContext::InvocationInstruction {
            typed_native_invocation: Some(typed_native_invocation),
            ..
        } = context
        else {
            return;
        };
        match typed_native_invocation {
            // Account
            TypedNativeInvocation {
                receiver: ManifestInvocationReceiver::GlobalMethod(receiver),
                invocation:
                    TypedManifestNativeInvocation::AccountBlueprintInvocation(
                        AccountBlueprintInvocation::Method(
                            AccountBlueprintMethod::Securify(..),
                        ),
                    ),
            } => {
                self.0.account_securify_invocations.insert(receiver.into());
            }
            TypedNativeInvocation {
                receiver: ManifestInvocationReceiver::GlobalMethod(receiver),
                invocation:
                    TypedManifestNativeInvocation::AccountBlueprintInvocation(
                        AccountBlueprintInvocation::Method(
                            AccountBlueprintMethod::LockFee(..)
                            | AccountBlueprintMethod::LockContingentFee(..)
                            | AccountBlueprintMethod::LockFeeAndWithdraw(..)
                            | AccountBlueprintMethod::LockFeeAndWithdrawNonFungibles(..),
                        ),
                    ),
            } => {
                self.0.account_lock_fee_invocations.insert(receiver.into());
            }
            TypedNativeInvocation {
                receiver: ManifestInvocationReceiver::GlobalMethod(receiver),
                invocation:
                    TypedManifestNativeInvocation::MetadataBlueprintInvocation(
                        MetadataBlueprintInvocation::Method(
                            MetadataBlueprintMethod::Set(MetadataSetInput {
                                key, ..
                            }),
                        ),
                    ),
            } if key == OWNER_KEYS_METADATA_KEY && receiver.is_account() => {
                self.0
                    .account_update_owner_keys_metadata_field_invocations
                    .insert(receiver.into());
            }
            TypedNativeInvocation {
                receiver: ManifestInvocationReceiver::GlobalMethod(receiver),
                invocation:
                    TypedManifestNativeInvocation::MetadataBlueprintInvocation(
                        MetadataBlueprintInvocation::Method(
                            MetadataBlueprintMethod::Lock(MetadataLockInput {
                                key, ..
                            }),
                        ),
                    ),
            } if key == OWNER_KEYS_METADATA_KEY && receiver.is_account() => {
                self.0
                    .account_lock_owner_keys_metadata_field_invocations
                    .insert(receiver.into());
            }
            // Identity
            TypedNativeInvocation {
                receiver: ManifestInvocationReceiver::GlobalMethod(receiver),
                invocation:
                    TypedManifestNativeInvocation::IdentityBlueprintInvocation(
                        IdentityBlueprintInvocation::Method(
                            IdentityBlueprintMethod::Securify(..),
                        ),
                    ),
            } => {
                self.0.identity_securify_invocations.insert(receiver.into());
            }
            TypedNativeInvocation {
                receiver: ManifestInvocationReceiver::GlobalMethod(receiver),
                invocation:
                    TypedManifestNativeInvocation::MetadataBlueprintInvocation(
                        MetadataBlueprintInvocation::Method(
                            MetadataBlueprintMethod::Set(MetadataSetInput {
                                key, ..
                            }),
                        ),
                    ),
            } if key == OWNER_KEYS_METADATA_KEY && receiver.is_identity() => {
                self.0
                    .identity_update_owner_keys_metadata_field_invocations
                    .insert(receiver.into());
            }
            TypedNativeInvocation {
                receiver: ManifestInvocationReceiver::GlobalMethod(receiver),
                invocation:
                    TypedManifestNativeInvocation::MetadataBlueprintInvocation(
                        MetadataBlueprintInvocation::Method(
                            MetadataBlueprintMethod::Lock(MetadataLockInput {
                                key, ..
                            }),
                        ),
                    ),
            } if key == OWNER_KEYS_METADATA_KEY && receiver.is_identity() => {
                self.0
                    .identity_lock_owner_keys_metadata_field_invocations
                    .insert(receiver.into());
            }
            // Access Controller
            TypedNativeInvocation {
                receiver: ManifestInvocationReceiver::GlobalMethod(receiver),
                invocation:
                    TypedManifestNativeInvocation::AccessControllerBlueprintInvocation(
                        AccessControllerBlueprintInvocation::Method(..),
                    ),
            } => {
                self.0.access_controller_invocations.insert(receiver.into());
            }
            _ => {}
        }
    }
}

macro_rules! define_output_struct {
    (
        $vis: vis $ident: ident => [$($field_ident: ident),* $(,)?]
    ) => {
        paste! {
            #[derive(Clone, Debug, PartialEq, Eq, Default)]
            $vis struct $ident {
                $(
                    pub $field_ident: IndexSet<ManifestGlobalAddress>,
                )*
            }

            impl $ident {
                pub fn is_any_reserved_instruction_present(&self) -> bool {
                    $(
                        self.[< has_ $field_ident >]()
                    ) || *
                }

                $(
                    pub fn [< has_ $field_ident >](&self) -> bool {
                        !self.$field_ident.is_empty()
                    }
                )*
            }
        }
    };
}
define_output_struct! {
    pub ReservedInstructionsOutput => [
        account_lock_fee_invocations,
        account_securify_invocations,
        account_lock_owner_keys_metadata_field_invocations,
        account_update_owner_keys_metadata_field_invocations,
        identity_securify_invocations,
        identity_lock_owner_keys_metadata_field_invocations,
        identity_update_owner_keys_metadata_field_invocations,
        access_controller_invocations,
    ]
}
