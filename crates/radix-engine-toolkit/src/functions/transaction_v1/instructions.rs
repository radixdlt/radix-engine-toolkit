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
use radix_transactions::validation::*;
use scrypto::prelude::*;

use crate::types::*;

pub fn statically_validate(
    instructions: &[InstructionV1],
    blobs: &IndexMap<Hash, Vec<u8>>,
    network_definition: &NetworkDefinition,
) -> Result<(), InstructionValidationError> {
    radix_engine::utils::validate_call_arguments_to_native_components(
        &TransactionManifestV1 {
            instructions: instructions.to_vec(),
            blobs: Default::default(),
            object_names: Default::default(),
        },
    )
    .map_err(
        InstructionValidationError::LocatedInstructionSchemaValidationError,
    )?;
    TransactionValidator::new_with_latest_config(network_definition)
        .validate_instructions_v1(instructions, blobs)
        .map_err(InstructionValidationError::IntentValidationError)?;
    Ok(())
}

pub fn extract_addresses(
    instructions: &[InstructionV1],
) -> (HashSet<TypedNodeId>, HashSet<ManifestNamedAddress>) {
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
    IntentValidationError(IntentValidationError),
    TransactionValidationError(TransactionValidationError),
    LocatedInstructionSchemaValidationError(
        LocatedInstructionSchemaValidationError,
    ),
}
