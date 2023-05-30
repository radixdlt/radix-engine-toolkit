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

use super::traits::InvocationHandler;

use crate::error::VisitorError;
use crate::model::constants::RADIX_ENGINE_HASH_LENGTH;
use crate::model::instruction::Instruction;
use crate::model::transaction::{
    InstructionList, TransactionIntent, TransactionIntentConversionError,
};
use crate::visitor::{traverse_instruction, ValueNetworkAggregatorVisitor};
use native_transaction::prelude::{HasIntentHash, TransactionPayload};
use toolkit_derive::serializable;

// =================
// Model Definition
// =================

/// Takes a transaction intent and produces the hash of it. This is the hash that a client needs to
/// sign.
#[serializable]
pub struct Input {
    /// The transaction intent to compile.
    #[serde(flatten)]
    pub transaction_intent: TransactionIntent,
}

/// The response from [`Input`].
#[serializable]
pub struct Output {
    #[schemars(with = "String")]
    #[schemars(regex(pattern = "[0-9a-fA-F]+"))]
    #[serde_as(as = "serde_with::hex::Hex")]
    pub hash: [u8; RADIX_ENGINE_HASH_LENGTH],
}

// ===============
// Implementation
// ===============

pub struct Handler;
impl InvocationHandler<Input, Output> for Handler {
    type Error = Error;

    fn pre_process(mut input: Input) -> Result<Input, Error> {
        // Visitors
        let mut network_aggregator_visitor = ValueNetworkAggregatorVisitor::default();

        // Instructions
        let instructions: &mut [Instruction] = match input.transaction_intent.manifest.instructions
        {
            InstructionList::Parsed(ref mut instructions) => instructions,
            InstructionList::String(..) => &mut [],
        };

        // Traverse instructions with visitors
        instructions
            .iter_mut()
            .map(|instruction| {
                traverse_instruction(instruction, &mut [&mut network_aggregator_visitor], &mut [])
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(Self::Error::PreProcessingError)?;

        // Check for network mismatches
        let expected_network_id = input.transaction_intent.header.network_id;
        if let Some(network_id) = network_aggregator_visitor
            .0
            .iter()
            .find(|network_id| **network_id != expected_network_id)
        {
            return Err(Self::Error::InvalidNetworkIdEncountered {
                found: *network_id,
                expected: expected_network_id,
            });
        }
        Ok(input)
    }

    fn handle(input: &Input) -> Result<Output, Error> {
        Ok(Output {
            hash: input
                .transaction_intent
                .to_native_transaction_intent()?
                .prepare()
                .unwrap()
                .intent_hash()
                .0
                 .0,
        })
    }

    fn post_process(_: &Input, output: Output) -> Result<Output, Error> {
        Ok(output)
    }
}

#[serializable]
#[serde(tag = "type", content = "error")]
pub enum Error {
    /// An error emitted during the pre processing of the invocation
    PreProcessingError(VisitorError),

    /// An error emitted when an address is encountered in the manifest with an invalid network id
    InvalidNetworkIdEncountered { expected: u8, found: u8 },

    /// An error emitted when the compilation of the transaction intent fails
    CompilationError(TransactionIntentConversionError),
}

impl From<TransactionIntentConversionError> for Error {
    fn from(value: TransactionIntentConversionError) -> Self {
        Self::CompilationError(value)
    }
}
