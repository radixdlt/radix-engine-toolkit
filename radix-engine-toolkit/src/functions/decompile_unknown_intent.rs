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
use super::{
    decompile_notarized_transaction, decompile_signed_transaction_intent,
    decompile_transaction_intent,
};

use crate::error::VisitorError;
use crate::model::transaction::{
    InstructionList, NotarizedTransaction, SignedTransactionIntent, TransactionIntent,
    TransactionManifest,
};
use crate::model::{instruction::Instruction, transaction::InstructionKind};
use crate::visitor::traverse_instruction;
use crate::visitor::ValueAliasingVisitor;

use toolkit_derive::serializable;

// =================
// Model Definition
// =================

/// There are certain cases where we might have some blob which we suspect is a transaction intent
/// but we have no way of verifying whether that is true or not. Looking at the type id byte of the
/// blob does not help either as it's a generic Struct type which is not too telling. For this
/// specific use case, this library provides this function which attempts to decompile a transaction
/// intent of an unknown type.
#[serializable]
pub struct Input {
    /// Defines the output format that we would like the manifest to be in after this function is
    /// executed.
    pub instructions_output_kind: InstructionKind,

    /// A byte array serialized as a hex string which represents what is suspected to be a compiled
    /// intent of an unknown kind.
    #[schemars(with = "String")]
    #[schemars(regex(pattern = "[0-9a-fA-F]+"))]
    #[serde_as(as = "serde_with::hex::Hex")]
    pub compiled_unknown_intent: Vec<u8>,
}

/// The response from [`Input`]. This is an tagged union which
/// can either be a [`Output`],
/// [`Output`], or [`Output`]
/// depending on the passed intent.
#[serializable]
#[serde(tag = "type", content = "value")]
pub enum Output {
    TransactionIntent(decompile_transaction_intent::Output),
    SignedTransactionIntent(decompile_signed_transaction_intent::Output),
    NotarizedTransactionIntent(decompile_notarized_transaction::Output),
}

// ============
// Conversions
// ============

impl From<Input> for decompile_transaction_intent::Input {
    fn from(input: Input) -> Self {
        decompile_transaction_intent::Input {
            compiled_intent: input.compiled_unknown_intent,
            instructions_output_kind: input.instructions_output_kind,
        }
    }
}

impl From<Input> for decompile_signed_transaction_intent::Input {
    fn from(input: Input) -> Self {
        decompile_signed_transaction_intent::Input {
            compiled_signed_intent: input.compiled_unknown_intent,
            instructions_output_kind: input.instructions_output_kind,
        }
    }
}

impl From<Input> for decompile_notarized_transaction::Input {
    fn from(input: Input) -> Self {
        decompile_notarized_transaction::Input {
            compiled_notarized_intent: input.compiled_unknown_intent,
            instructions_output_kind: input.instructions_output_kind,
        }
    }
}

impl From<decompile_transaction_intent::Output> for Output {
    fn from(output: decompile_transaction_intent::Output) -> Self {
        Self::TransactionIntent(output)
    }
}

impl From<decompile_signed_transaction_intent::Output> for Output {
    fn from(output: decompile_signed_transaction_intent::Output) -> Self {
        Self::SignedTransactionIntent(output)
    }
}

impl From<decompile_notarized_transaction::Output> for Output {
    fn from(output: decompile_notarized_transaction::Output) -> Self {
        Self::NotarizedTransactionIntent(output)
    }
}

// ===============
// Implementation
// ===============

pub struct Handler;
impl InvocationHandler<Input, Output> for Handler {
    type Error = Error;

    fn pre_process(input: Input) -> Result<Input, Error> {
        Ok(input)
    }

    fn handle(input: &Input) -> Result<Output, Error> {
        if let Ok(output) = decompile_transaction_intent::Handler::fulfill(input.clone().into()) {
            Ok(output.into())
        } else if let Ok(output) =
            decompile_signed_transaction_intent::Handler::fulfill(input.clone().into())
        {
            Ok(output.into())
        } else if let Ok(output) =
            decompile_notarized_transaction::Handler::fulfill(input.clone().into())
        {
            Ok(output.into())
        } else {
            Err(Error::UnrecognizedTransactionFormat)
        }
    }

    fn post_process(_: &Input, mut output: Output) -> Result<Output, Error> {
        // Visitors
        let mut aliasing_visitor = ValueAliasingVisitor::default();

        // Instructions
        let instructions: &mut [Instruction] = match output {
            Output::NotarizedTransactionIntent(decompile_notarized_transaction::Output {
                notarized_intent:
                    NotarizedTransaction {
                        signed_intent:
                            SignedTransactionIntent {
                                intent:
                                    TransactionIntent {
                                        manifest:
                                            TransactionManifest {
                                                instructions:
                                                    InstructionList::Parsed(ref mut instructions),
                                                ..
                                            },
                                        ..
                                    },
                                ..
                            },
                        ..
                    },
            })
            | Output::SignedTransactionIntent(decompile_signed_transaction_intent::Output {
                signed_intent:
                    SignedTransactionIntent {
                        intent:
                            TransactionIntent {
                                manifest:
                                    TransactionManifest {
                                        instructions: InstructionList::Parsed(ref mut instructions),
                                        ..
                                    },
                                ..
                            },
                        ..
                    },
            })
            | Output::TransactionIntent(decompile_transaction_intent::Output {
                transaction_intent:
                    TransactionIntent {
                        manifest:
                            TransactionManifest {
                                instructions: InstructionList::Parsed(ref mut instructions),
                                ..
                            },
                        ..
                    },
            }) => instructions,
            _ => &mut [],
        };

        // Traverse instructions with visitors
        instructions
            .iter_mut()
            .map(|instruction| {
                traverse_instruction(instruction, &mut [&mut aliasing_visitor], &mut [])
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(Error::PostProcessingError)?;

        // The aliasing visitor performs all of the modifications in place as it meets them. Nothing
        // else needs to be done here.

        Ok(output)
    }
}

#[serializable]
#[serde(tag = "type")]
pub enum Error {
    /// An error emitted if the passed compiled intent is neither an unsigned, signed, or notarized
    /// intent.
    UnrecognizedTransactionFormat,

    /// An error emitted during the post processing of the invocation
    PostProcessingError(VisitorError),
}
