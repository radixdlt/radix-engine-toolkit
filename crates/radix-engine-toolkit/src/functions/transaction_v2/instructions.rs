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
use radix_transactions::errors::*;
use radix_transactions::prelude::*;
use sbor::*;
use scrypto::prelude::*;

use crate::models::node_id::TypedNodeId;
use crate::sbor::indexed_manifest_value::*;

pub fn hash(instructions: &[InstructionV2]) -> Result<Hash, EncodeError> {
    to_payload_bytes(instructions).map(scrypto::prelude::hash)
}

pub fn to_payload_bytes(
    instructions: &[InstructionV2],
) -> Result<Vec<u8>, EncodeError> {
    manifest_encode(instructions)
}

pub fn from_payload_bytes<T>(
    payload_bytes: T,
) -> Result<Vec<InstructionV2>, DecodeError>
where
    T: AsRef<[u8]>,
{
    manifest_decode(payload_bytes.as_ref())
}

pub fn statically_validate(
    _instructions: &[InstructionV2],
) -> Result<(), InstructionValidationError> {
    todo!("")
}

pub fn extract_addresses(
    instructions: &[InstructionV2],
) -> (HashSet<TypedNodeId>, HashSet<u32>) {
    let indexed_manifest_value = IndexedManifestValue::from_typed(instructions);
    let static_addresses = indexed_manifest_value
        .static_addresses()
        .into_iter()
        .collect();
    let named_addresses = indexed_manifest_value
        .named_addresses()
        .iter()
        .cloned()
        .collect();

    (static_addresses, named_addresses)
}

#[derive(Clone, Debug)]
pub enum InstructionValidationError {
    TransactionValidationError(TransactionValidationError),
    LocatedInstructionSchemaValidationError(
        LocatedInstructionSchemaValidationError,
    ),
}
