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

use sbor::*;
use scrypto::prelude::*;
use transaction::errors::*;
use transaction::prelude::*;
use transaction::validation::*;

use crate::instruction_visitor::core::traverser::traverse;
use crate::instruction_visitor::visitors::account_interactions_visitor::*;
use crate::instruction_visitor::visitors::identity_interactions_visitor::IdentityInteractionsVisitor;
use crate::sbor::indexed_manifest_value::*;

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
    NotarizedTransactionValidator::validate_instructions_v1(instructions)
        .map_err(InstructionValidationError::TransactionValidationError)?;
    Ok(())
}

pub fn extract_addresses(instructions: &[InstructionV1]) -> HashSet<NodeId> {
    let indexed_manifest_value = IndexedManifestValue::from_typed(instructions);

    indexed_manifest_value.addresses().iter().cloned().collect()
}

pub fn identities_requiring_auth(instructions: &[InstructionV1]) -> HashSet<ComponentAddress> {
    let mut visitor = IdentityInteractionsVisitor::default();
    traverse(instructions, &mut [&mut visitor]).expect("This visitor can't fail");
    visitor.output()
}

pub fn accounts_requiring_auth(instructions: &[InstructionV1]) -> HashSet<ComponentAddress> {
    let mut visitor = AccountInteractionsVisitor::default();
    traverse(instructions, &mut [&mut visitor]).expect("This visitor can't fail");
    let (accounts_requiring_auth, _, _) = visitor.output();
    accounts_requiring_auth
}

pub fn accounts_withdrawn_from(instructions: &[InstructionV1]) -> HashSet<ComponentAddress> {
    let mut visitor = AccountInteractionsVisitor::default();
    traverse(instructions, &mut [&mut visitor]).expect("This visitor can't fail");
    let (_, accounts_withdrawn_from, _) = visitor.output();
    accounts_withdrawn_from
}

pub fn accounts_deposited_into(instructions: &[InstructionV1]) -> HashSet<ComponentAddress> {
    let mut visitor = AccountInteractionsVisitor::default();
    traverse(instructions, &mut [&mut visitor]).expect("This visitor can't fail");
    let (_, _, accounts_deposited_into) = visitor.output();
    accounts_deposited_into
}

#[derive(Clone, Debug)]
pub enum InstructionValidationError {
    TransactionValidationError(TransactionValidationError),
}
