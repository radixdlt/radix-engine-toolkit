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
pub struct PresentedProofsVisitor(PresentedProofsOutput);

impl PresentedProofsVisitor {
    pub fn new() -> Self {
        Default::default()
    }
}

impl ManifestAnalysisVisitor for PresentedProofsVisitor {
    type Output = PresentedProofsOutput;
    type ValidityState = ConstManifestAnalysisVisitorValidityState<true>;

    fn output(self) -> Self::Output {
        self.0
    }

    fn validity_state(&self) -> &Self::ValidityState {
        &ConstManifestAnalysisVisitorValidityState::<true>
    }

    fn on_instruction(
        &mut self,
        _: &NamedAddressStore,
        grouped_instruction: &GroupedInstruction,
        _: &InstructionIndex,
        maybe_typed_invocation: Option<&TypedManifestNativeInvocation>,
        _: Option<&InvocationIo<InvocationIoItems>>,
    ) {
        // INVARIANT(traversal-address-checking): There's an invariant that this
        // method makes use of which is that the address in the invocation will
        // not require checking for the entity type. The traverser has done the
        // steps required to keep track of the allocated addresses and has done
        // the typed invocation interpretation correctly.

        // We only want to act on CallMethod instructions.
        let Some(CallMethod { address, .. }) =
            grouped_instruction.as_call_method()
        else {
            return;
        };

        // Interpret the typed invocation. If the invocation is not a create
        // proof invocation then we skip it since there's nothing that this
        // visitor needs to do.
        let resource_specifier = match maybe_typed_invocation {
            Some(
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
            ) => ManifestResourceSpecifier::Amount(*resource_address, *amount),
            Some(
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
            ) => ManifestResourceSpecifier::Ids(*resource_address, ids.clone()),
            _ => return,
        };

        // Adding the created proof to the visitor state.
        self.0
            .presented_proofs
            .entry(*address)
            .or_default()
            .push(resource_specifier);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct PresentedProofsOutput {
    pub presented_proofs:
        IndexMap<ManifestGlobalAddress, Vec<ManifestResourceSpecifier>>,
}
