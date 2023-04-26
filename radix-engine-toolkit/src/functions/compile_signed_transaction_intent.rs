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
use crate::model::instruction::Instruction;
use crate::model::transaction::{
    InstructionList, SignedTransactionIntent, SignedTransactionIntentConversionError,
};
use crate::traits::CompilableIntent;
use crate::visitor::{traverse_instruction, ValueNetworkAggregatorVisitor};
use toolkit_derive::serializable;

// =================
// Model Definition
// =================

/// Takes in a raw transaction intent as well as its signatures and compiles it. This is useful when
/// a notary wishes to notarize a signed transaction intent.
#[serializable]
pub struct Input {
    /// The signed transaction intent to compile
    #[serde(flatten)]
    pub signed_intent: SignedTransactionIntent,
}

/// The response from [`Input`].
#[serializable]
pub struct Output {
    /// A byte array serialized as a hex string which represents the compiled signed transaction
    /// intent.
    #[schemars(with = "String")]
    #[schemars(regex(pattern = "[0-9a-fA-F]+"))]
    #[serde_as(as = "serde_with::hex::Hex")]
    pub compiled_intent: Vec<u8>,
}

// ===============
// Implementation
// ===============

pub struct Handler;
impl InvocationHandler<Input, Output> for Handler {
    type Error = Error;

    fn pre_process(mut request: Input) -> Result<Input, Error> {
        // Visitors
        let mut network_aggregator_visitor = ValueNetworkAggregatorVisitor::default();

        // Instructions
        let instructions: &mut [Instruction] =
            match request.signed_intent.intent.manifest.instructions {
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
        let expected_network_id = request.signed_intent.intent.header.network_id;
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
        Ok(request)
    }

    fn handle(request: &Input) -> Result<Output, Error> {
        request
            .signed_intent
            .compile()
            .map(|compiled_intent| Output { compiled_intent })
            .map_err(Self::Error::from)
    }

    fn post_process(_: &Input, response: Output) -> Result<Output, Error> {
        Ok(response)
    }
}

#[serializable]
#[serde(tag = "type")]
pub enum Error {
    /// An error emitted during the pre processing of the invocation
    PreProcessingError(VisitorError),

    /// An error emitted when an address is encountered in the manifest with an invalid network id
    InvalidNetworkIdEncountered { expected: u8, found: u8 },

    /// An error emitted when the compilation of the transaction intent fails
    CompilationError(SignedTransactionIntentConversionError),
}

impl From<SignedTransactionIntentConversionError> for Error {
    fn from(value: SignedTransactionIntentConversionError) -> Self {
        Self::CompilationError(value)
    }
}
