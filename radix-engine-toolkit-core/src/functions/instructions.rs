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

use radix_engine::utils::*;
use sbor::*;
use scrypto::prelude::*;
use transaction::errors::*;
use transaction::prelude::*;
use transaction::validation::*;

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

#[derive(Clone, Debug)]
pub enum InstructionValidationError {
    TransactionValidationError(TransactionValidationError),
    LocatedInstructionSchemaValidationError(LocatedInstructionSchemaValidationError),
}
