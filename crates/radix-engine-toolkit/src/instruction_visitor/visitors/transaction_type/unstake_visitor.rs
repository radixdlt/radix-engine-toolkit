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

use crate::functions::execution::ExecutionAnalysisTransactionReceipt;
use crate::instruction_visitor::core::error::InstructionVisitorError;
use crate::instruction_visitor::core::traits::InstructionVisitor;
use crate::utils::*;
use radix_engine::blueprints::consensus_manager::UnstakeData;
use radix_engine::system::system_modules::execution_trace::{
    ResourceSpecifier, WorktopChange,
};
use radix_engine::transaction::*;
use radix_engine_common::prelude::*;
use radix_engine_interface::blueprints::account::*;
use radix_engine_interface::blueprints::consensus_manager::*;
use transaction::prelude::*;

pub struct UnstakeVisitor<'r> {
    /// The receipt of the transaction
    transaction_receipt: &'r ExecutionAnalysisTransactionReceipt<'r>,

    /// The finite state machine used by the visitor to determine if this
    /// transaction is an ustake transaction or not.
    finite_state_machine: UnstakeFiniteStateMachine,

    /// The index of the current instruction
    instruction_index: usize,

    /// The unstakes made in the transaction - the non-fungible data is put as
    /// unit since they will be resolved by the visitor as finalization.
    unstakes: Vec<UnstakeInformation<()>>,
}

impl<'r> UnstakeVisitor<'r> {
    pub fn new(
        transaction_receipt: &'r ExecutionAnalysisTransactionReceipt,
    ) -> Self {
        Self {
            transaction_receipt,
            finite_state_machine: Default::default(),
            instruction_index: Default::default(),
            unstakes: Default::default(),
        }
    }

    pub fn output(self) -> Option<Vec<UnstakeInformation<UnstakeData>>> {
        if matches!(
            self.finite_state_machine,
            UnstakeFiniteStateMachine::NotAnUnstakeTransaction(..)
        ) {
            return None;
        }

        let non_fungible_data =
            data_of_newly_minted_non_fungibles(self.transaction_receipt);
        self.unstakes
            .into_iter()
            .map(
                |UnstakeInformation {
                     from_account,
                     stake_unit_address,
                     stake_unit_amount,
                     validator_address,
                     claim_nft_resource,
                     claim_nft_local_id,
                     ..
                 }| {
                    non_fungible_data
                        .get(&claim_nft_resource)
                        .and_then(|map| map.get(&claim_nft_local_id))
                        .and_then(|data| scrypto_encode(&data).ok())
                        .and_then(|data| scrypto_decode(&data).ok())
                        .map(|claim_nft_data| UnstakeInformation {
                            from_account,
                            stake_unit_address,
                            stake_unit_amount,
                            validator_address,
                            claim_nft_resource,
                            claim_nft_local_id,
                            claim_nft_data,
                        })
                },
            )
            .collect::<Option<Vec<_>>>()
    }
}

impl<'r> InstructionVisitor for UnstakeVisitor<'r> {
    fn visit_instruction(
        &mut self,
        instruction: &InstructionV1,
    ) -> Result<(), InstructionVisitorError> {
        self.finite_state_machine.transition(
            instruction.clone(),
            self.transaction_receipt.execution_trace(),
            self.instruction_index,
        );

        if let UnstakeFiniteStateMachine::Unstaked {
            account_withdrawn_from,
            stake_unit_address,
            stake_unit_amount,
            validator_address,
            claim_nft_resource,
            claim_nft_local_id,
        } = &self.finite_state_machine
        {
            let stake_info = UnstakeInformation {
                from_account: *account_withdrawn_from,
                stake_unit_address: *stake_unit_address,
                stake_unit_amount: *stake_unit_amount,
                validator_address: *validator_address,
                claim_nft_resource: *claim_nft_resource,
                claim_nft_local_id: claim_nft_local_id.clone(),
                claim_nft_data: (),
            };
            self.unstakes.push(stake_info)
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
            UnstakeFiniteStateMachine::NotAnUnstakeTransaction(true)
        )
    }
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct UnstakeInformation<T> {
    pub from_account: ComponentAddress,
    pub stake_unit_address: ResourceAddress,
    pub stake_unit_amount: Decimal,
    pub validator_address: ComponentAddress,
    pub claim_nft_resource: ResourceAddress,
    pub claim_nft_local_id: NonFungibleLocalId,
    pub claim_nft_data: T,
}

#[derive(Clone)]
pub enum UnstakeFiniteStateMachine {
    /// The transaction has been marked as being not an unstaking transaction.
    /// The boolean this enum holds is whether this decision is final or
    /// not. As an example, the initial state of the FSM is
    /// [`UnstakeFiniteStateMachine::NotAnUnstakeTransaction(false)`]
    /// indicating that we have not yet concluded whether it's an unstake
    /// transaction or not but we're still open to it.
    NotAnUnstakeTransaction(bool),

    /// The LSUs have been withdrawn from an account and are currently in the
    /// worktop. This keeps track of the address of what has been
    /// withdrawn, the amount that has been withdrawn, and what account it
    /// has been withdrawn from.
    LiquidStakeUnitsWithdrawn {
        /// The address of the account that the XRD was withdrawn from.
        account_withdrawn_from: ComponentAddress,
        /// The address of the resource that was withdrawn - it is checked that
        /// this is a fungible resource address.
        stake_unit_address: ResourceAddress,
        /// The amount of LSUs that was withdrawn from the account.
        stake_unit_amount: Decimal,
    },

    /// The LSUs have been taken from the worktop and put into a
    /// [`ManifestBucket`]. This state is tracking everything from the
    /// previous states as well as the amount of XRD used up so far and the
    /// amount of XRD in the bucket.
    LiquidStakeUnitsInBucket {
        /// The address of the account that the XRD was withdrawn from.
        account_withdrawn_from: ComponentAddress,
        /// The address of the resource that was withdrawn - it is checked that
        /// this is a fungible resource address.
        stake_unit_address: ResourceAddress,
        /// The amount of LSUs that was withdrawn from the account.
        stake_unit_amount: Decimal,
    },

    /// LSUs have been sent to a validator for unstaking and the validator has
    /// returned a claim NFT back.
    Unstaked {
        /// The address of the account that the XRD was withdrawn from.
        account_withdrawn_from: ComponentAddress,
        /// The address of the resource that was withdrawn - it is checked that
        /// this is a fungible resource address.
        stake_unit_address: ResourceAddress,
        /// The amount of LSUs that was withdrawn from the account.
        stake_unit_amount: Decimal,
        /// The address of the validator that the XRD was staked to.
        validator_address: ComponentAddress,
        /// The resource address of the claim NFT.
        claim_nft_resource: ResourceAddress,
        /// The non-fungible local id of the claim NFT given back.
        claim_nft_local_id: NonFungibleLocalId,
    },

    /// The claim NFT is in a bucket and ready to be deposited.
    ClaimNftInBucket {
        /// The address of the account that the XRD was withdrawn from - we
        /// would like to make sure that the LSUs are deposited back
        /// into the same account.
        account_withdrawn_from: ComponentAddress,
    },

    /// The claim NFT has been deposited into an account.
    DepositedIntoAccount,
}

impl Default for UnstakeFiniteStateMachine {
    fn default() -> Self {
        Self::NotAnUnstakeTransaction(false)
    }
}

impl UnstakeFiniteStateMachine {
    /// Transitions the FSM from one state to the next. If any errors occur in
    /// the transition then the FSM transitions into
    /// [`UnstakeFiniteStateMachine::NotAnUnstakeTransaction(true)`] meaning
    /// that the FSM has concluded that this is not a stake transaction and that
    /// this conclusion is final.
    pub fn transition(
        &mut self,
        instruction: InstructionV1,
        execution_trace: &TransactionExecutionTrace,
        instruction_index: usize,
    ) {
        match (&self, instruction) {
            // We can transition to [`LiquidStakeUnitsWithdrawn`] if we're in
            // the initial state or if we are in the
            // [`DepositedIntoAccount`] state and we get a CallMethod to account
            // withdraw.
            (
                Self::NotAnUnstakeTransaction(false)
                | Self::DepositedIntoAccount,
                InstructionV1::CallMethod {
                    address: DynamicGlobalAddress::Static(address),
                    method_name,
                    args,
                },
            ) if is_account(&address)
                && method_name == ACCOUNT_WITHDRAW_IDENT =>
            {
                let account_address =
                    ComponentAddress::try_from(address).expect("Must succeed");

                let (resource_address, amount) = match manifest_encode(&args)
                    .ok()
                    .and_then(|encoded| manifest_decode(&encoded).ok())
                {
                    Some(AccountWithdrawInput {
                        resource_address,
                        amount,
                    }) if resource_address.is_fungible() => {
                        (resource_address, amount)
                    }
                    _ => {
                        *self = Self::NotAnUnstakeTransaction(true);
                        return;
                    }
                };

                *self = Self::LiquidStakeUnitsWithdrawn {
                    account_withdrawn_from: account_address,
                    stake_unit_address: resource_address,
                    stake_unit_amount: amount,
                }
            }
            // Transition from LiquidStakeUnitsWithdrawn ->
            // LiquidStakeUnitsInBucket. This happens when we get a
            // TakeFromWorktop or a TakeAllFromWorktop
            (
                Self::LiquidStakeUnitsWithdrawn {
                    account_withdrawn_from,
                    stake_unit_address,
                    stake_unit_amount,
                },
                InstructionV1::TakeFromWorktop {
                    resource_address,
                    amount,
                },
            ) if amount == *stake_unit_amount
                && resource_address == *stake_unit_address =>
            {
                *self = Self::LiquidStakeUnitsInBucket {
                    account_withdrawn_from: *account_withdrawn_from,
                    stake_unit_address: *stake_unit_address,
                    stake_unit_amount: *stake_unit_amount,
                }
            }
            (
                Self::LiquidStakeUnitsWithdrawn {
                    account_withdrawn_from,
                    stake_unit_address,
                    stake_unit_amount,
                },
                InstructionV1::TakeAllFromWorktop { resource_address },
            ) if resource_address == *stake_unit_address => {
                *self = Self::LiquidStakeUnitsInBucket {
                    account_withdrawn_from: *account_withdrawn_from,
                    stake_unit_address: *stake_unit_address,
                    stake_unit_amount: *stake_unit_amount,
                }
            }
            // Transition from LiquidStakeUnitsInBucket -> Unstaked
            (
                Self::LiquidStakeUnitsInBucket {
                    account_withdrawn_from,
                    stake_unit_address,
                    stake_unit_amount,
                },
                InstructionV1::CallMethod {
                    address: DynamicGlobalAddress::Static(address),
                    method_name,
                    ..
                },
            ) if is_validator(&address)
                && method_name == VALIDATOR_UNSTAKE_IDENT =>
            {
                let validator_address =
                    ComponentAddress::try_from(address).expect("Must succeed");

                let Some((claim_nft_resource, claim_nft_local_id)) =
                    get_returned_claim_nft(execution_trace, instruction_index)
                else {
                    *self = Self::NotAnUnstakeTransaction(true);
                    return;
                };

                *self = Self::Unstaked {
                    account_withdrawn_from: *account_withdrawn_from,
                    stake_unit_address: *stake_unit_address,
                    stake_unit_amount: *stake_unit_amount,
                    validator_address,
                    claim_nft_resource,
                    claim_nft_local_id,
                }
            }
            // Unstaked -> DepositedIntoAccount with CallMethod input
            (
                Self::Unstaked {
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
            // Unstaked -> ClaimNftInBucket with TakeFromWorktop inputs
            (
                Self::Unstaked {
                    account_withdrawn_from,
                    claim_nft_resource,
                    ..
                },
                InstructionV1::TakeFromWorktop {
                    resource_address,
                    amount,
                },
            ) if resource_address == *claim_nft_resource
                && amount == Decimal::ONE =>
            {
                *self = Self::ClaimNftInBucket {
                    account_withdrawn_from: *account_withdrawn_from,
                }
            }
            (
                Self::Unstaked {
                    account_withdrawn_from,
                    claim_nft_resource,
                    ..
                },
                InstructionV1::TakeAllFromWorktop { resource_address },
            ) if resource_address == *claim_nft_resource => {
                *self = Self::ClaimNftInBucket {
                    account_withdrawn_from: *account_withdrawn_from,
                }
            }
            // ClaimNftInBucket -> DepositedIntoAccount with CallMethod input
            (
                Self::ClaimNftInBucket {
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
            // [`NotAnUnstakeTransaction()`]
            _ => *self = Self::NotAnUnstakeTransaction(true),
        }
    }
}

fn get_returned_claim_nft(
    execution_trace: &TransactionExecutionTrace,
    instruction_index: usize,
) -> Option<(ResourceAddress, NonFungibleLocalId)> {
    match execution_trace
        .worktop_changes()
        .get(&instruction_index)
        .map(|rtn| rtn.as_slice())
    {
        Some(
            [WorktopChange::Put(ResourceSpecifier::Ids(resource_address, ids))],
        ) => ids
            .iter()
            .next()
            .map(|local_id| (*resource_address, local_id.clone())),
        None | Some([..]) => None,
    }
}
