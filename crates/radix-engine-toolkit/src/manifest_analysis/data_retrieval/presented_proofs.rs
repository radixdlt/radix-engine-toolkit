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
pub struct PresentedProofsAnalyzer(PresentedProofsOutput);

impl ManifestStaticAnalyzer for PresentedProofsAnalyzer {
    type Initializer = ();
    type Output = PresentedProofsOutput;
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

    fn process_permission(
        &mut self,
        _: &mut Self::PermissionState,
        _: &NamedAddressStore,
        _: &GroupedInstruction,
        _: Option<&TypedNativeInvocation>,
    ) {
    }

    fn process_requirement(
        &mut self,
        _: &mut Self::RequirementState,
        _: &NamedAddressStore,
        _: &GroupedInstruction,
        _: Option<&TypedNativeInvocation>,
    ) {
    }

    fn process_instruction(
        &mut self,
        _: &NamedAddressStore,
        _: &GroupedInstruction,
        typed_native_invocation: Option<&TypedNativeInvocation>,
    ) {
        // Interpreting the typed invocation and converting it into a resource
        // specifier of the created proof.
        let Some(typed_native_invocation) = typed_native_invocation else {
            return;
        };
        let (account, proof_specifier) = match typed_native_invocation {
            TypedNativeInvocation {
                receiver: ManifestInvocationReceiver::GlobalMethod(receiver),
                invocation:
                    TypedManifestNativeInvocation::AccountBlueprintInvocation(
                        AccountBlueprintInvocation::Method(
                            AccountBlueprintMethod::CreateProofOfAmount(
                                AccountCreateProofOfAmountManifestInput {
                                    resource_address,
                                    amount,
                                },
                            ),
                        ),
                    ),
            } => (
                receiver,
                ManifestResourceSpecifier::Amount(*resource_address, *amount),
            ),
            TypedNativeInvocation {
                receiver: ManifestInvocationReceiver::GlobalMethod(receiver),
                invocation:
                    TypedManifestNativeInvocation::AccountBlueprintInvocation(
                        AccountBlueprintInvocation::Method(
                            AccountBlueprintMethod::CreateProofOfNonFungibles(
                                AccountCreateProofOfNonFungiblesManifestInput {
                                    resource_address,
                                    ids,
                                },
                            ),
                        ),
                    ),
            } => (
                receiver,
                ManifestResourceSpecifier::Ids(*resource_address, ids.clone()),
            ),
            _ => return,
        };

        // Adding the created proof to the output.
        self.0
            .created_proofs
            .entry(account.into())
            .or_default()
            .push(proof_specifier);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct PresentedProofsOutput {
    pub created_proofs:
        IndexMap<ManifestGlobalAddress, Vec<ManifestResourceSpecifier>>,
}
