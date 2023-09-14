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
use transaction::model::*;
use transaction::prelude::*;

pub struct StakeVisitor<'r> {
    /// The execution trace from the staking transaction preview.
    execution_trace: &'r TransactionExecutionTrace,

    /// The finite state machine used by the visitor to determine if this transaction is a stake
    /// transaction or not.
    finite_state_machine: StakeFiniteStateMachine,

    /// The index of the current instruction
    instruction_index: usize,

    /// The stakes that have been made in this transaction
    stakes: Vec<StakeInformation>,
}

impl<'r> StakeVisitor<'r> {
    pub fn new(execution_trace: &'r TransactionExecutionTrace) -> Self {
        Self {
            execution_trace,
            finite_state_machine: Default::default(),
            instruction_index: Default::default(),
            stakes: Default::default(),
        }
    }

    pub fn output(self) -> Option<Vec<StakeInformation>> {
        if !matches!(
            self.finite_state_machine,
            StakeFiniteStateMachine::NotAStakeTransaction(..)
        ) {
            Some(self.stakes)
        } else {
            None
        }
    }
}

impl<'r> InstructionVisitor for StakeVisitor<'r> {
    fn post_visit(&mut self) -> Result<(), InstructionVisitorError> {
        self.instruction_index += 1;
        Ok(())
    }

    fn is_enabled(&self) -> bool {
        !matches!(
            self.finite_state_machine,
            StakeFiniteStateMachine::NotAStakeTransaction(true)
        )
    }

    fn visit_instruction(
        &mut self,
        instruction: &InstructionV1,
    ) -> Result<(), InstructionVisitorError> {
        self.finite_state_machine.transition(
            instruction.clone(),
            self.execution_trace,
            self.instruction_index,
        );

        if let StakeFiniteStateMachine::Staked {
            account_withdrawn_from,
            validator_address,
            stake_unit_address,
            stake_unit_amount,
            xrd_staked,
            ..
        } = self.finite_state_machine
        {
            let stake_info = StakeInformation {
                from_account: account_withdrawn_from,
                validator_address,
                stake_unit_resource: stake_unit_address,
                stake_unit_amount,
                staked_xrd: xrd_staked,
            };
            self.stakes.push(stake_info)
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct StakeInformation {
    pub from_account: ComponentAddress,
    pub validator_address: ComponentAddress,
    pub stake_unit_resource: ResourceAddress,
    pub stake_unit_amount: Decimal,
    pub staked_xrd: Decimal,
}

#[derive(Clone, Copy)]
pub enum StakeFiniteStateMachine {
    /// The transaction has been marked as being not a staking transaction. The boolean this enum
    /// holds is whether this decision is final or not. As an example, when the FSM first beings
    /// we start in [`StakeFiniteStateMachine::NotAStakeTransaction`] with the boolean set to
    /// [`false`] meaning that we can possibly advance the FSM.
    NotAStakeTransaction(bool),

    /// The resources have been withdrawn from an account and are currently in the worktop. This
    /// keeps track of how much were withdrawn and from which account. This does not keep track of
    /// any [`ResourceAddress`]es since only withdraws of XRD are permitted. Any other withdraw
    /// will put the FSM in a [`StakeFiniteStateMachine::NotAStakeTransaction(true)`] state.
    XrdWithdrawn {
        /// The address of the account that the XRD was withdrawn from.
        account_withdrawn_from: ComponentAddress,
        /// The amount of XRD that was withdrawn from the account.
        withdrawn_amount: Decimal,
    },

    /// The XRD has been taken from the worktop and is now in a [`ManifestBucket`]. This state is
    /// tracking everything from the previous states as well as the amount of XRD used up so far
    /// and the amount of XRD in the bucket.  
    XrdInBucket {
        /// The address of the account that the XRD was withdrawn from.
        account_withdrawn_from: ComponentAddress,
        /// The amount of XRD that was withdrawn from the account.
        withdrawn_amount: Decimal,
        /// The amount of XRD that has been used up thus far.
        xrd_used_up_so_far: Decimal,
        /// The amount of XRD that is currently in the bucket.
        amount_in_bucket: Decimal,
    },

    /// XRD has been staked to a validator and they have given us back some stake units of some
    /// amount.
    Staked {
        /// The address of the account that the XRD was withdrawn from.
        account_withdrawn_from: ComponentAddress,
        /// The amount of XRD that was withdrawn from the account.
        withdrawn_amount: Decimal,
        /// The amount of XRD that has been used up thus far.
        xrd_used_up_so_far: Decimal,
        /// The address of the validator that the XRD was staked to.
        validator_address: ComponentAddress,
        /// The address of the stake units that the validator has given back in return.
        stake_unit_address: ResourceAddress,
        /// The amount of stake units that the validator has given back in return.
        stake_unit_amount: Decimal,
        /// The amount of XRD that has been staked.
        xrd_staked: Decimal,
    },

    /// The liquid stake units are put in a bucket. To get to this state we first ensure that all
    /// of the XRD withdrawn has been used up.
    LiquidStakeUnitsInBucket {
        /// The address of the account that the XRD was withdrawn from - we would like to make sure
        /// that the LSUs are deposited back into the same account.
        account_withdrawn_from: ComponentAddress,
    },

    /// The stake units have been deposited into an account. It is checked that the account that
    /// they are deposited into is the same account that they were withdrawn from.
    DepositedIntoAccount,
}

impl Default for StakeFiniteStateMachine {
    fn default() -> Self {
        Self::NotAStakeTransaction(false)
    }
}

impl StakeFiniteStateMachine {
    /// Transitions the FSM from one state to the next. If any errors occur in the transition then
    /// the FSM transitions into [`StakeFiniteStateMachine::NotAStakeTransaction(true)`] meaning
    /// that the FSM has concluded that this is not a stake transaction and that this conclusion is
    /// final.
    pub fn transition(
        &mut self,
        instruction: InstructionV1,
        execution_trace: &TransactionExecutionTrace,
        instruction_index: usize,
    ) {
        match (&self, instruction) {
            // We can transition to [`XrdWithdrawn`] if we're in the initial state or if we are in
            // the [`DepositedIntoAccount`] state and we get a CallMethod to account withdraw.
            (
                Self::NotAStakeTransaction(false) | Self::DepositedIntoAccount,
                InstructionV1::CallMethod {
                    address: DynamicGlobalAddress::Static(address),
                    method_name,
                    args,
                },
            ) if is_account(&address) && method_name == ACCOUNT_WITHDRAW_IDENT => {
                let account_address = ComponentAddress::try_from(address).expect("Must succeed");

                let Some(AccountWithdrawInput {
                    resource_address: XRD,
                    amount,
                }) = manifest_encode(&args)
                    .ok()
                    .and_then(|encoded| manifest_decode(&encoded).ok())
                else {
                    *self = Self::NotAStakeTransaction(true);
                    return;
                };

                *self = Self::XrdWithdrawn {
                    account_withdrawn_from: account_address,
                    withdrawn_amount: amount,
                };
            }
            // The transition from XrdWithdrawn to XrdInBucket. This happens when we're in the
            // XrdWithdrawn state and we get a TakeFromWorktopByAmount.
            (
                Self::XrdWithdrawn {
                    account_withdrawn_from,
                    withdrawn_amount,
                },
                InstructionV1::TakeFromWorktop {
                    resource_address: XRD,
                    amount,
                },
            ) => {
                *self = Self::XrdInBucket {
                    account_withdrawn_from: *account_withdrawn_from,
                    withdrawn_amount: *withdrawn_amount,
                    xrd_used_up_so_far: Decimal::ZERO,
                    amount_in_bucket: amount,
                }
            }
            // The transition from XrdWithdrawn to XrdInBucket. This happens when we're in the
            // XrdWithdrawn state and we get a TakeAllFromWorktop.
            (
                Self::XrdWithdrawn {
                    account_withdrawn_from,
                    withdrawn_amount,
                },
                InstructionV1::TakeAllFromWorktop {
                    resource_address: XRD,
                },
            ) => {
                *self = Self::XrdInBucket {
                    account_withdrawn_from: *account_withdrawn_from,
                    withdrawn_amount: *withdrawn_amount,
                    xrd_used_up_so_far: Decimal::ZERO,
                    amount_in_bucket: *withdrawn_amount,
                }
            }
            // The transition from XrdInBucket to Staked. This happens when we do a CallMethod to a
            // validator component
            (
                Self::XrdInBucket {
                    account_withdrawn_from,
                    withdrawn_amount,
                    xrd_used_up_so_far,
                    amount_in_bucket,
                },
                InstructionV1::CallMethod {
                    address: DynamicGlobalAddress::Static(address),
                    method_name,
                    ..
                },
            ) if is_validator(&address) && method_name == VALIDATOR_STAKE_IDENT => {
                let validator_address = ComponentAddress::try_from(address).expect("Must succeed");

                let Some((stake_unit_address, stake_unit_amount)) =
                    get_returned_stake_units(execution_trace, instruction_index)
                else {
                    *self = Self::NotAStakeTransaction(true);
                    return;
                };

                *self = Self::Staked {
                    account_withdrawn_from: *account_withdrawn_from,
                    withdrawn_amount: *withdrawn_amount,
                    xrd_used_up_so_far: *xrd_used_up_so_far + *amount_in_bucket, /* TODO: Safe Math & Check doesn't exceed withdrawn_amount */
                    validator_address,
                    stake_unit_address,
                    stake_unit_amount,
                    xrd_staked: *amount_in_bucket,
                }
            }
            // The transition from Staked back to XrdInBucket - This happens when we take from
            // worktop by amount of XRD again after staking (typically to stake again)
            (
                Self::Staked {
                    account_withdrawn_from,
                    withdrawn_amount,
                    xrd_used_up_so_far,
                    ..
                },
                InstructionV1::TakeFromWorktop {
                    resource_address: XRD,
                    amount,
                },
            ) => {
                *self = Self::XrdInBucket {
                    account_withdrawn_from: *account_withdrawn_from,
                    withdrawn_amount: *withdrawn_amount,
                    xrd_used_up_so_far: *xrd_used_up_so_far,
                    amount_in_bucket: amount,
                }
            }
            // The transition from Staked back to XrdInBucket - This happens when we take from
            // worktop of all of XRD again after staking (typically to stake again)
            (
                Self::Staked {
                    account_withdrawn_from,
                    withdrawn_amount,
                    xrd_used_up_so_far,
                    ..
                },
                InstructionV1::TakeAllFromWorktop {
                    resource_address: XRD,
                },
            ) => {
                *self = Self::XrdInBucket {
                    account_withdrawn_from: *account_withdrawn_from,
                    withdrawn_amount: *withdrawn_amount,
                    xrd_used_up_so_far: *xrd_used_up_so_far,
                    amount_in_bucket: *withdrawn_amount - *xrd_used_up_so_far, /* TODO: Check if
                                                                                * negative */
                }
            }
            // Transition from Staked to DepositedIntoAccount - This happens when the deposit batch
            // method is called on an account while we're in the staked state. This transition fails
            // if the XRD used up so far doesn't equal that which has been withdrawn.
            (
                Self::Staked {
                    account_withdrawn_from,
                    withdrawn_amount,
                    xrd_used_up_so_far,
                    ..
                },
                InstructionV1::CallMethod {
                    address: DynamicGlobalAddress::Static(address),
                    method_name,
                    ..
                },
            ) if is_account(&address)
                && method_name == ACCOUNT_DEPOSIT_BATCH_IDENT
                && xrd_used_up_so_far == withdrawn_amount
                && account_withdrawn_from.as_node_id() == address.as_node_id() =>
            {
                *self = Self::DepositedIntoAccount
            }

            // Transition from Staked to LiquidStakeUnitsInBucket - This happens when we're in the
            // Staked state and we get a TakeAllFromWorktop instruction of the LSU's resource
            // address
            (
                Self::Staked {
                    account_withdrawn_from,
                    stake_unit_address,
                    ..
                },
                InstructionV1::TakeAllFromWorktop { resource_address },
            ) if resource_address == *stake_unit_address => {
                *self = Self::LiquidStakeUnitsInBucket {
                    account_withdrawn_from: *account_withdrawn_from,
                }
            }
            (
                Self::LiquidStakeUnitsInBucket {
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
                && account_withdrawn_from.as_node_id() == address.as_node_id() =>
            {
                *self = Self::DepositedIntoAccount
            }
            // For anything else transition into a final state of [`NotAStakeTransaction`]
            _ => *self = Self::NotAStakeTransaction(true),
        }
    }
}

fn get_returned_stake_units(
    execution_trace: &TransactionExecutionTrace,
    instruction_index: usize,
) -> Option<(ResourceAddress, Decimal)> {
    match execution_trace
        .worktop_changes()
        .get(&instruction_index)
        .map(|rtn| rtn.as_slice())
    {
        Some([WorktopChange::Put(ResourceSpecifier::Amount(resource_address, amount))]) => {
            Some((*resource_address, *amount))
        }
        None | Some([..]) => None,
    }
}
