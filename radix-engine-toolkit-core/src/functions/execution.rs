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

use radix_engine::system::system_modules::execution_trace::*;
use radix_engine::transaction::*;
use radix_engine_interface::blueprints::account::DefaultDepositRule;
use scrypto::api::node_modules::metadata::*;
use scrypto::prelude::*;
use transaction::prelude::*;

use crate::instruction_visitor::core::error::*;
use crate::instruction_visitor::core::traverser::*;
use crate::instruction_visitor::visitors::account_proofs_visitor::*;
use crate::instruction_visitor::visitors::transaction_type::account_deposit_settings_visitor::*;
use crate::instruction_visitor::visitors::transaction_type::general_transaction_visitor::*;
use crate::instruction_visitor::visitors::transaction_type::simple_transfer_visitor::*;
use crate::instruction_visitor::visitors::transaction_type::transfer_visitor::*;
use crate::utils;

pub fn analyze(
    instructions: &[InstructionV1],
    preview_receipt: &TransactionReceipt,
) -> Result<ExecutionAnalysis, ExecutionModuleError> {
    let (execution_trace, fee_summary) = match preview_receipt.transaction_result {
        TransactionResult::Commit(CommitResult {
            outcome: TransactionOutcome::Success(..),
            ref execution_trace,
            ref fee_summary,
            ..
        }) => Ok((execution_trace, fee_summary)),
        _ => Err(ExecutionModuleError::IsNotCommitSuccess(
            preview_receipt.clone(),
        )),
    }?;

    let mut account_proofs_visitor = AccountProofsVisitor::default();
    let mut simple_transfer_visitor = SimpleTransactionTypeVisitor::default();
    let mut transfer_visitor = TransferTransactionTypeVisitor::default();
    let mut account_deposit_settings_visitor = AccountDepositSettingsVisitor::default();
    let mut general_transaction_visitor = GeneralTransactionTypeVisitor::new(execution_trace);

    traverse(
        instructions,
        &mut [
            &mut simple_transfer_visitor,
            &mut transfer_visitor,
            &mut account_proofs_visitor,
            &mut account_deposit_settings_visitor,
            &mut general_transaction_visitor,
        ],
    )?;

    let mut transaction_types = vec![];
    if let Some((from_account_address, to_account_address, transfer)) =
        simple_transfer_visitor.output()
    {
        transaction_types.push(TransactionType::SimpleTransfer(Box::new(
            SimpleTransferTransactionType {
                from: from_account_address,
                to: to_account_address,
                transferred: transfer,
            },
        )))
    }
    if let Some((from_account_address, transfers)) = transfer_visitor.output() {
        transaction_types.push(TransactionType::Transfer(Box::new(
            TransferTransactionType {
                from: from_account_address,
                transfers,
            },
        )))
    }
    if let Some((
        resource_preference_changes,
        default_deposit_rule_changes,
        authorized_depositors_changes,
    )) = account_deposit_settings_visitor.output()
    {
        transaction_types.push(TransactionType::AccountDepositSettings(Box::new(
            AccountDepositSettingsTransactionType {
                resource_preference_changes,
                default_deposit_rule_changes,
                authorized_depositors_changes,
            },
        )))
    }
    if let Some((account_withdraws, account_deposits)) = general_transaction_visitor.output() {
        transaction_types.push(TransactionType::GeneralTransaction(Box::new(
            GeneralTransactionType {
                account_proofs: account_proofs_visitor.output(),
                account_withdraws,
                account_deposits,
                addresses_in_manifest: crate::functions::instructions::extract_addresses(
                    instructions,
                ),
                metadata_of_newly_created_entities: utils::metadata_of_newly_created_entities(
                    preview_receipt,
                )
                .unwrap(),
                data_of_newly_minted_non_fungibles: utils::data_of_newly_minted_non_fungibles(
                    preview_receipt,
                )
                .unwrap(),
                addresses_of_newly_created_entities: utils::addresses_of_newly_created_entities(
                    preview_receipt,
                )
                .unwrap(),
            },
        )))
    };

    let mut fee_locks = FeeLocks::default();
    for (_, amount, is_contingent) in fee_summary.locked_fees.iter() {
        let amount = amount.amount();
        if *is_contingent {
            fee_locks.contingent_lock += amount;
        } else {
            fee_locks.lock += amount;
        }
    }

    let fee_summary = {
        // Previews sometimes reports a cost unit price of zero. So, we will:
        // * Calculate all fees in XRD from the cost units.
        // * Calculate all fees based on the current cost unit price.
        let mut network_fees = 0u128;
        for value in fee_summary.execution_cost_breakdown.values() {
            network_fees += *value as u128;
        }
        let network_fee = cost_units_to_xrd(network_fees);

        let royalty_fee = fee_summary
            .royalty_cost_breakdown
            .values()
            .map(|(_, v)| *v)
            .sum();

        FeeSummary {
            network_fee,
            royalty_fee,
        }
    };

    Ok(ExecutionAnalysis {
        fee_locks,
        fee_summary,
        transaction_types,
    })
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExecutionAnalysis {
    pub fee_locks: FeeLocks,
    pub fee_summary: FeeSummary,
    pub transaction_types: Vec<TransactionType>,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct FeeSummary {
    pub network_fee: Decimal,
    pub royalty_fee: Decimal,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct FeeLocks {
    pub lock: Decimal,
    pub contingent_lock: Decimal,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TransactionType {
    SimpleTransfer(Box<SimpleTransferTransactionType>),
    Transfer(Box<TransferTransactionType>),
    AccountDepositSettings(Box<AccountDepositSettingsTransactionType>),
    GeneralTransaction(Box<GeneralTransactionType>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SimpleTransferTransactionType {
    pub from: ComponentAddress,
    pub to: ComponentAddress,
    pub transferred: ResourceSpecifier,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransferTransactionType {
    pub from: ComponentAddress,
    pub transfers: HashMap<ComponentAddress, HashMap<ResourceAddress, Resources>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AccountDepositSettingsTransactionType {
    pub resource_preference_changes:
        HashMap<ComponentAddress, HashMap<ResourceAddress, ResourcePreferenceAction>>,
    pub default_deposit_rule_changes: HashMap<ComponentAddress, DefaultDepositRule>,
    pub authorized_depositors_changes: HashMap<ComponentAddress, AuthorizedDepositorsChanges>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GeneralTransactionType {
    pub account_proofs: HashSet<ResourceAddress>,
    pub account_withdraws: HashMap<ComponentAddress, Vec<ResourceTracker>>,
    pub account_deposits: HashMap<ComponentAddress, Vec<ResourceTracker>>,
    pub addresses_in_manifest: (HashSet<NodeId>, HashSet<u32>),
    pub addresses_of_newly_created_entities: HashSet<NodeId>,
    pub metadata_of_newly_created_entities:
        HashMap<GlobalAddress, HashMap<String, Option<MetadataValue>>>,
    pub data_of_newly_minted_non_fungibles:
        HashMap<ResourceAddress, HashMap<NonFungibleLocalId, ScryptoValue>>,
}

#[derive(Clone, Debug)]
pub enum ExecutionModuleError {
    IsNotCommitSuccess(TransactionReceipt),
    InstructionVisitorError(InstructionVisitorError),
    LocatedGeneralTransactionTypeError(LocatedGeneralTransactionTypeError),
}

impl From<InstructionVisitorError> for ExecutionModuleError {
    fn from(value: InstructionVisitorError) -> Self {
        Self::InstructionVisitorError(value)
    }
}

fn cost_units_to_xrd(cost_units: u128) -> Decimal {
    Decimal::from_str(COST_UNIT_PRICE_IN_XRD).unwrap() * cost_units
}
