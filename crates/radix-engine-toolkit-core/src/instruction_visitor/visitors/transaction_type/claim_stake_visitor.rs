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

use crate::instruction_visitor::core::error::*;
use crate::instruction_visitor::core::traits::*;
use crate::utils::*;
use radix_engine::system::system_modules::execution_trace::*;
use radix_engine::transaction::*;
use radix_engine_common::prelude::*;
use radix_engine_interface::blueprints::account::*;
use radix_engine_interface::blueprints::consensus_manager::*;
use transaction::prelude::*;

pub struct ClaimStakeVisitor<'r> {
    /// The execution trace of the transaction
    execution_trace: &'r TransactionExecutionTrace,

    /// The finite state machine used by the visitor to determine if this
    /// transaction is an claim transaction or not.
    finite_state_machine: ClaimStakeFiniteStateMachine,

    /// The index of the current instruction
    instruction_index: usize,

    /// The claims made in the transaction - the non-fungible data is put as
    /// unit since they will be resolved by the visitor as finalization.
    claims: Vec<ClaimStakeInformation>,
}

impl<'r> ClaimStakeVisitor<'r> {
    pub fn new(execution_trace: &'r TransactionExecutionTrace) -> Self {
        Self {
            execution_trace,
            finite_state_machine: Default::default(),
            instruction_index: Default::default(),
            claims: Default::default(),
        }
    }

    pub fn output(self) -> Option<Vec<ClaimStakeInformation>> {
        if !matches!(
            self.finite_state_machine,
            ClaimStakeFiniteStateMachine::NotAnClaimTransaction(..)
        ) {
            Some(self.claims)
        } else {
            None
        }
    }
}

impl<'r> InstructionVisitor for ClaimStakeVisitor<'r> {
    fn visit_instruction(
        &mut self,
        instruction: &InstructionV1,
    ) -> Result<(), InstructionVisitorError> {
        self.finite_state_machine.transition(
            instruction.clone(),
            self.execution_trace,
            self.instruction_index,
        );

        if let ClaimStakeFiniteStateMachine::Claimed {
            account_withdrawn_from,
            validator_address,
            claim_nft_resource,
            claim_nft_local_ids,
            claimed_xrd,
        } = &self.finite_state_machine
        {
            let stake_info = ClaimStakeInformation {
                from_account: *account_withdrawn_from,
                validator_address: *validator_address,
                claim_nft_resource: *claim_nft_resource,
                claim_nft_local_ids: claim_nft_local_ids.clone(),
                claimed_xrd: *claimed_xrd,
            };
            self.claims.push(stake_info)
        }

        Ok(())
    }

    fn post_visit(&mut self) -> Result<(), InstructionVisitorError> {
        self.instruction_index += 1;
        Ok(())
    }

    fn is_enabled(&self) -> bool {
        !matches!(
            self.finite_state_machine,
            ClaimStakeFiniteStateMachine::NotAnClaimTransaction(true)
        )
    }
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct ClaimStakeInformation {
    pub from_account: ComponentAddress,
    pub validator_address: ComponentAddress,
    pub claim_nft_resource: ResourceAddress,
    pub claim_nft_local_ids: BTreeSet<NonFungibleLocalId>,
    pub claimed_xrd: Decimal,
}

#[derive(Clone)]
pub enum ClaimStakeFiniteStateMachine {
    /// The transaction has been marked as being not an claim transaction. The
    /// boolean this enum holds is whether this decision is final or not.
    /// As an example, the initial state of the FSM
    /// is [`ClaimStakeFiniteStateMachine::NotAnClaimTransaction(false)`]
    /// indicating that we have not yet concluded whether it's an claim
    /// transaction or not but we're still open to it.
    NotAnClaimTransaction(bool),

    /// The Claim NFTs have been withdrawn from an account and are currently in
    /// the worktop. This keeps track of the address of what has been
    /// withdrawn, the amount that has been withdrawn, and what account it
    /// has been withdrawn from.
    ClaimNftWithdrawn {
        /// The address of the account that the Claim NFTs were withdrawn from.
        account_withdrawn_from: ComponentAddress,
        /// The address of the resource that was withdrawn - it is checked that
        /// this is a fungible resource address.
        claim_nft_resource: ResourceAddress,
        /// The ids of the claim NFTs withdrawn from the account.
        claim_nft_local_ids: BTreeSet<NonFungibleLocalId>,
    },

    /// The LSUs have been taken from the worktop and put into a
    /// [`ManifestBucket`].
    ClaimNftInBucket {
        /// The address of the account that the Claim NFTs were withdrawn from.
        account_withdrawn_from: ComponentAddress,
        /// The address of the resource that was withdrawn - it is checked that
        /// this is a fungible resource address.
        claim_nft_resource: ResourceAddress,
        /// The ids of the claim NFTs withdrawn from the account.
        claim_nft_local_ids: BTreeSet<NonFungibleLocalId>,
    },

    /// The claim NFT has been given to the validator and it has given back
    /// XRD.
    Claimed {
        /// The address of the account that the Claim NFTs were withdrawn from.
        account_withdrawn_from: ComponentAddress,
        /// The address of the validator that the XRD was staked to.
        validator_address: ComponentAddress,
        /// The resource address of the claim NFT.
        claim_nft_resource: ResourceAddress,
        /// The ids of the claim non-fungibles used to claim the stake.
        claim_nft_local_ids: BTreeSet<NonFungibleLocalId>,
        /// The XRD returned as a result of the claim.
        claimed_xrd: Decimal,
    },

    /// The claim NFT is in a bucket and ready to be deposited.
    XrdInBucket {
        /// The address of the account that the Claim NFTs were withdrawn from
        /// - we would like to make sure that the LSUs are deposited
        /// back into the same account.
        account_withdrawn_from: ComponentAddress,
    },

    /// The XRD has been deposited back into the account.
    DepositedIntoAccount,
}

impl Default for ClaimStakeFiniteStateMachine {
    fn default() -> Self {
        Self::NotAnClaimTransaction(false)
    }
}

impl ClaimStakeFiniteStateMachine {
    /// Transitions the FSM from one state to the next. If any errors occur in
    /// the transition then the FSM transitions into
    /// [`ClaimStakeFiniteStateMachine::NotAnClaimTransaction(true)`]
    /// meaning that the FSM has concluded that this is not a stake transaction
    /// and that this conclusion is final.
    pub fn transition(
        &mut self,
        instruction: InstructionV1,
        execution_trace: &TransactionExecutionTrace,
        instruction_index: usize,
    ) {
        match (&self, instruction) {
            // We can transition to [`ClaimNftWithdrawn`] if we're in the
            // initial state or if we are
            // in the [`DepositedIntoAccount`] state and we get a CallMethod to
            // account withdraw.
            (
                Self::NotAnClaimTransaction(false) | Self::DepositedIntoAccount,
                InstructionV1::CallMethod {
                    address: DynamicGlobalAddress::Static(address),
                    method_name,
                    args,
                },
            ) if is_account(&address)
                && method_name == ACCOUNT_WITHDRAW_NON_FUNGIBLES_IDENT =>
            {
                let account_address =
                    ComponentAddress::try_from(address).expect("Must succeed");

                let (resource_address, ids) = match manifest_encode(&args)
                    .ok()
                    .and_then(|encoded| manifest_decode(&encoded).ok())
                {
                    Some(AccountWithdrawNonFungiblesInput {
                        resource_address,
                        ids,
                    }) if !resource_address.is_fungible() => {
                        (resource_address, ids)
                    }
                    _ => {
                        *self = Self::NotAnClaimTransaction(true);
                        return;
                    }
                };

                *self = Self::ClaimNftWithdrawn {
                    account_withdrawn_from: account_address,
                    claim_nft_resource: resource_address,
                    claim_nft_local_ids: ids.into_iter().collect(),
                }
            }
            // Transition from ClaimNftWithdrawn -> ClaimNftInBucket. We can
            // make this transition with any of the take from
            // worktop instructions.
            (
                Self::ClaimNftWithdrawn {
                    account_withdrawn_from,
                    claim_nft_resource,
                    claim_nft_local_ids,
                },
                InstructionV1::TakeFromWorktop {
                    resource_address,
                    amount,
                },
            ) if amount == Decimal::from(claim_nft_local_ids.len())
                && resource_address == *claim_nft_resource =>
            {
                *self = Self::ClaimNftInBucket {
                    account_withdrawn_from: *account_withdrawn_from,
                    claim_nft_resource: *claim_nft_resource,
                    claim_nft_local_ids: claim_nft_local_ids.clone(),
                }
            }
            (
                Self::ClaimNftWithdrawn {
                    account_withdrawn_from,
                    claim_nft_resource,
                    claim_nft_local_ids,
                },
                InstructionV1::TakeNonFungiblesFromWorktop {
                    resource_address,
                    ids,
                },
            ) if &ids.iter().cloned().collect::<BTreeSet<_>>()
                == claim_nft_local_ids
                && resource_address == *claim_nft_resource =>
            {
                *self = Self::ClaimNftInBucket {
                    account_withdrawn_from: *account_withdrawn_from,
                    claim_nft_resource: *claim_nft_resource,
                    claim_nft_local_ids: claim_nft_local_ids.clone(),
                }
            }
            (
                Self::ClaimNftWithdrawn {
                    account_withdrawn_from,
                    claim_nft_resource,
                    claim_nft_local_ids,
                },
                InstructionV1::TakeAllFromWorktop { resource_address },
            ) if resource_address == *claim_nft_resource => {
                *self = Self::ClaimNftInBucket {
                    account_withdrawn_from: *account_withdrawn_from,
                    claim_nft_resource: *claim_nft_resource,
                    claim_nft_local_ids: claim_nft_local_ids.clone(),
                }
            }
            // Transition from ClaimNftInBucket -> Claimed
            (
                Self::ClaimNftInBucket {
                    account_withdrawn_from,
                    claim_nft_resource,
                    claim_nft_local_ids,
                },
                InstructionV1::CallMethod {
                    address: DynamicGlobalAddress::Static(address),
                    method_name,
                    ..
                },
            ) if is_validator(&address)
                && method_name == VALIDATOR_CLAIM_XRD_IDENT =>
            {
                let validator_address =
                    ComponentAddress::try_from(address).expect("Must succeed");

                let Some(claimed_xrd) =
                    get_returned_xrd_amount(execution_trace, instruction_index)
                else {
                    *self = Self::NotAnClaimTransaction(true);
                    return;
                };

                *self = Self::Claimed {
                    account_withdrawn_from: *account_withdrawn_from,
                    validator_address,
                    claim_nft_resource: *claim_nft_resource,
                    claim_nft_local_ids: claim_nft_local_ids.clone(),
                    claimed_xrd,
                }
            }
            // Claimed -> DepositedIntoAccount with CallMethod input
            (
                Self::Claimed {
                    account_withdrawn_from,
                    ..
                },
                InstructionV1::CallMethod {
                    address: DynamicGlobalAddress::Static(address),
                    method_name,
                    ..
                },
            ) if is_account(&address)
                && method_name == ACCOUNT_DEPOSIT_BATCH_IDENT
                && account_withdrawn_from.as_node_id()
                    == address.as_node_id() =>
            {
                *self = Self::DepositedIntoAccount
            }
            // Claimed -> XrdInBucket with TakeFromWorktop inputs
            (
                Self::Claimed {
                    account_withdrawn_from,
                    ..
                },
                InstructionV1::TakeAllFromWorktop {
                    resource_address: XRD,
                },
            ) => {
                *self = Self::XrdInBucket {
                    account_withdrawn_from: *account_withdrawn_from,
                }
            }
            // XrdInBucket -> DepositedIntoAccount with CallMethod input
            (
                Self::XrdInBucket {
                    account_withdrawn_from,
                },
                InstructionV1::CallMethod {
                    address: DynamicGlobalAddress::Static(address),
                    method_name,
                    ..
                },
            ) if is_account(&address)
                && (method_name == ACCOUNT_DEPOSIT_IDENT
                    || method_name == ACCOUNT_DEPOSIT_BATCH_IDENT)
                && account_withdrawn_from.as_node_id()
                    == address.as_node_id() =>
            {
                *self = Self::DepositedIntoAccount
            }
            // For anything else transition into a final state of
            // [`NotAnClaimTransaction()`]
            _ => *self = Self::NotAnClaimTransaction(true),
        }
    }
}

fn get_returned_xrd_amount(
    execution_trace: &TransactionExecutionTrace,
    instruction_index: usize,
) -> Option<Decimal> {
    match execution_trace
        .worktop_changes()
        .get(&instruction_index)
        .map(|rtn| rtn.as_slice())
    {
        Some([WorktopChange::Put(ResourceSpecifier::Amount(XRD, amount))]) => {
            Some(*amount)
        }
        None | Some([..]) => None,
    }
}
