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
use radix_engine::utils::*;
use sbor::*;
use scrypto::api::node_modules::metadata::*;
use scrypto::prelude::*;
use transaction::errors::*;
use transaction::prelude::*;
use transaction::validation::*;

use crate::instruction_visitor::core::error::*;
use crate::instruction_visitor::core::traverser::*;
use crate::instruction_visitor::visitors::account_proofs_visitor::*;
use crate::instruction_visitor::visitors::transaction_type::general_transaction_visitor::*;
use crate::instruction_visitor::visitors::transaction_type::simple_transfer_visitor::*;
use crate::instruction_visitor::visitors::transaction_type::transfer_visitor::*;
use crate::sbor::indexed_manifest_value::*;
use crate::utils;

pub fn hash(instructions: &[InstructionV1]) -> Result<Hash, EncodeError> {
    compile(instructions).map(scrypto::prelude::hash)
}

pub fn compile(instructions: &[InstructionV1]) -> Result<Vec<u8>, EncodeError> {
    manifest_encode(instructions)
}

pub fn decompile<T>(payload_bytes: T) -> Result<Vec<InstructionV1>, DecodeError>
where
    T: AsRef<[u8]>,
{
    manifest_decode(payload_bytes.as_ref())
}

pub fn statically_validate(
    instructions: &[InstructionV1],
) -> Result<(), InstructionValidationError> {
    radix_engine::utils::validate_call_arguments_to_native_components(instructions)
        .map_err(InstructionValidationError::LocatedInstructionSchemaValidationError)?;
    NotarizedTransactionValidator::validate_instructions_v1(instructions)
        .map_err(InstructionValidationError::TransactionValidationError)?;
    Ok(())
}

pub fn extract_addresses(instructions: &[InstructionV1]) -> (HashSet<NodeId>, HashSet<u32>) {
    let indexed_manifest_value = IndexedManifestValue::from_typed(instructions);

    (
        indexed_manifest_value
            .static_addresses()
            .iter()
            .cloned()
            .collect(),
        indexed_manifest_value
            .named_addresses()
            .iter()
            .cloned()
            .collect(),
    )
}

pub fn transaction_type(
    instructions: &[InstructionV1],
    preview_receipt: &TransactionReceipt,
) -> Result<TransactionType, InstructionVisitorError> {
    let mut account_proofs_visitor = AccountProofsVisitor::default();
    let mut simple_transfer_visitor = SimpleTransactionTypeVisitor::default();
    let mut transfer_visitor = TransferTransactionTypeVisitor::default();
    let mut general_transaction_visitor = GeneralTransactionTypeVisitor::new(preview_receipt)?;

    traverse(
        instructions,
        &mut [
            &mut simple_transfer_visitor,
            &mut transfer_visitor,
            &mut account_proofs_visitor,
            &mut general_transaction_visitor,
        ],
    )?;

    if let Some((from_account_address, to_account_address, transfer)) =
        simple_transfer_visitor.output()
    {
        Ok(TransactionType::SimpleTransfer(Box::new(
            SimpleTransferTransactionType {
                from: from_account_address,
                to: to_account_address,
                transferred: transfer,
            },
        )))
    } else if let Some((from_account_address, transfers)) = transfer_visitor.output() {
        Ok(TransactionType::Transfer(Box::new(
            TransferTransactionType {
                from: from_account_address,
                transfers,
            },
        )))
    } else if let Some((account_withdraws, account_deposits)) = general_transaction_visitor.output()
    {
        Ok(TransactionType::GeneralTransaction(Box::new(
            GeneralTransactionType {
                account_proofs: account_proofs_visitor.output(),
                account_withdraws,
                account_deposits,
                addresses_in_manifest: extract_addresses(instructions),
                metadata_of_newly_created_entities: utils::metadata_of_newly_created_entities(
                    preview_receipt,
                )
                .unwrap(),
                data_of_newly_minted_non_fungibles: utils::data_of_newly_minted_non_fungibles(
                    preview_receipt,
                )
                .unwrap(),
            },
        )))
    } else {
        Ok(TransactionType::NonConforming)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TransactionType {
    SimpleTransfer(Box<SimpleTransferTransactionType>),
    Transfer(Box<TransferTransactionType>),
    GeneralTransaction(Box<GeneralTransactionType>),
    NonConforming,
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
pub struct GeneralTransactionType {
    pub account_proofs: HashSet<ResourceAddress>,
    pub account_withdraws: HashMap<ComponentAddress, Vec<ResourceSpecifier>>,
    pub account_deposits: HashMap<ComponentAddress, Vec<Source<ResourceSpecifier>>>,
    pub addresses_in_manifest: (HashSet<NodeId>, HashSet<u32>),
    pub metadata_of_newly_created_entities: HashMap<GlobalAddress, HashMap<String, MetadataValue>>,
    pub data_of_newly_minted_non_fungibles:
        HashMap<ResourceAddress, HashMap<NonFungibleLocalId, ScryptoValue>>,
}

#[derive(Clone, Debug)]
pub enum InstructionValidationError {
    TransactionValidationError(TransactionValidationError),
    LocatedInstructionSchemaValidationError(LocatedInstructionSchemaValidationError),
}
