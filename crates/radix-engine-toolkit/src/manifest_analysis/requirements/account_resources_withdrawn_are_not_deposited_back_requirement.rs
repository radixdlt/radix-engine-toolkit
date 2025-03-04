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

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AccountResourcesWithdrawnAreNotDepositedBackRequirement {
    resources_withdrawn: HashSet<ManifestResourceAddress>,
    resources_deposited: HashSet<ManifestResourceAddress>,
}

impl ManifestAnalyzerRequirementState
    for AccountResourcesWithdrawnAreNotDepositedBackRequirement
{
    fn requirement_state(&self) -> RequirementState {
        if self
            .resources_deposited
            .is_disjoint(&self.resources_withdrawn)
        {
            RequirementState::Fulfilled
        } else {
            RequirementState::CurrentlyUnfulfilled
        }
    }

    fn process_instruction(&mut self, context: InstructionContext<'_>) {
        if let InstructionContext::InvocationInstruction {
            typed_native_invocation:
                Some(TypedNativeInvocation {
                    invocation:
                        TypedManifestNativeInvocation::AccountBlueprintInvocation(
                            AccountBlueprintInvocation::Method(
                                AccountBlueprintMethod::Withdraw(
                                    AccountWithdrawManifestInput {
                                        resource_address, ..
                                    },
                                )
                                | AccountBlueprintMethod::LockFeeAndWithdraw(
                                    AccountLockFeeAndWithdrawManifestInput {
                                        resource_address,
                                        ..
                                    },
                                )
                                | AccountBlueprintMethod::WithdrawNonFungibles(
                                    AccountWithdrawNonFungiblesManifestInput {
                                        resource_address,
                                        ..
                                    },
                                )
                                | AccountBlueprintMethod::LockFeeAndWithdrawNonFungibles(
                                    AccountLockFeeAndWithdrawNonFungiblesManifestInput {
                                        resource_address,
                                        ..
                                    },
                                ),
                            ),
                        ),
                    ..
                }),
            ..
        } = context
        {
            self.resources_withdrawn.insert(*resource_address);
        }

        if let InstructionContext::InvocationInstruction {
            typed_native_invocation:
                Some(TypedNativeInvocation {
                    invocation:
                        TypedManifestNativeInvocation::AccountBlueprintInvocation(
                            AccountBlueprintInvocation::Method(
                                AccountBlueprintMethod::Deposit(..)
                                | AccountBlueprintMethod::DepositBatch(..)
                                | AccountBlueprintMethod::TryDepositOrAbort(..)
                                | AccountBlueprintMethod::TryDepositBatchOrAbort(
                                    ..,
                                ),
                            ),
                        ),
                    ..
                }),
            static_analysis_invocation_io,
            ..
        } = context
        {
            self.resources_deposited.extend(
                static_analysis_invocation_io
                    .input
                    .specified_resources()
                    .iter()
                    .filter_map(|(resource_address, resource)| {
                        let bounds = resource.bounds();

                        // There are two cases that we ignore: the first is when
                        // the bounds inform us that none of the resource is on
                        // the worktop and the second is when the bounds give us
                        // completely no information at all about the resource
                        // meaning that the analyzer can't either tell if they
                        // are on the worktop or not. This manifests itself with
                        // a lower bound of zero and an upper bound that's
                        // unbounded.
                        let is_zero = bounds.is_zero();
                        let is_completely_unknown = bounds.lower_bound()
                            == LowerBound::Inclusive(dec!(0))
                            && bounds.upper_bound() == UpperBound::unbounded();
                        if is_zero || is_completely_unknown {
                            None
                        } else {
                            Some(*resource_address)
                        }
                    })
                    .map(ManifestResourceAddress::Static),
            );
        }
    }
}
