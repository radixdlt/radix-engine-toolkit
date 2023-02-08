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

use crate::error::Result;
use crate::request::Handler;
use crate::request::{
    DecompileNotarizedTransactionRequest, DecompileNotarizedTransactionResponse,
    DecompileSignedTransactionIntentRequest, DecompileSignedTransactionIntentResponse,
    DecompileTransactionIntentRequest, DecompileTransactionIntentResponse,
};
use crate::{
    traverse_instruction, DecompileNotarizedTransactionHandler,
    DecompileSignedTransactionIntentHandler, DecompileTransactionIntentHandler, Error, Instruction,
    InstructionKind, NotarizedTransaction, SignedTransactionIntent, TransactionIntent,
    TransactionManifest, ValueAliasingVisitor,
};
use serializable::serializable;

// =================
// Model Definition
// =================

/// There are certain cases where we might have some blob which we suspect is a transaction intent
/// but we have no way of verifying whether that is true or not. Looking at the type id byte of the
/// blob does not help either as it's a generic Struct type which is not too telling. For this
/// specific use case, this library provides this function which attempts to decompile a transaction
/// intent of an unknown type.
#[serializable]
pub struct DecompileUnknownTransactionIntentRequest {
    /// Defines the output format that we would like the manifest to be in after this request is
    /// performed.
    pub instructions_output_kind: InstructionKind,

    /// A byte array serialized as a hex string which represents what is suspected to be a compiled
    /// intent of an unknown kind.
    #[schemars(with = "String")]
    #[schemars(regex(pattern = "[0-9a-fA-F]+"))]
    #[serde_as(as = "serde_with::hex::Hex")]
    pub compiled_unknown_intent: Vec<u8>,
}

/// The response from [`DecompileUnknownTransactionIntentRequest`]. This is an tagged union which
/// can either be a [`DecompileTransactionIntentResponse`],
/// [`DecompileSignedTransactionIntentResponse`], or [`DecompileNotarizedTransactionResponse`]
/// depending on the passed intent.
#[serializable]
#[serde(tag = "type", content = "value")]
pub enum DecompileUnknownTransactionIntentResponse {
    TransactionIntent(DecompileTransactionIntentResponse),
    SignedTransactionIntent(DecompileSignedTransactionIntentResponse),
    NotarizedTransactionIntent(DecompileNotarizedTransactionResponse),
}

impl From<DecompileUnknownTransactionIntentRequest> for DecompileTransactionIntentRequest {
    fn from(request: DecompileUnknownTransactionIntentRequest) -> Self {
        DecompileTransactionIntentRequest {
            compiled_intent: request.compiled_unknown_intent,
            instructions_output_kind: request.instructions_output_kind,
        }
    }
}

// ============
// Conversions
// ============

impl From<DecompileUnknownTransactionIntentRequest> for DecompileSignedTransactionIntentRequest {
    fn from(request: DecompileUnknownTransactionIntentRequest) -> Self {
        DecompileSignedTransactionIntentRequest {
            compiled_signed_intent: request.compiled_unknown_intent,
            instructions_output_kind: request.instructions_output_kind,
        }
    }
}

impl From<DecompileUnknownTransactionIntentRequest> for DecompileNotarizedTransactionRequest {
    fn from(request: DecompileUnknownTransactionIntentRequest) -> Self {
        DecompileNotarizedTransactionRequest {
            compiled_notarized_intent: request.compiled_unknown_intent,
            instructions_output_kind: request.instructions_output_kind,
        }
    }
}

impl From<DecompileTransactionIntentResponse> for DecompileUnknownTransactionIntentResponse {
    fn from(response: DecompileTransactionIntentResponse) -> Self {
        Self::TransactionIntent(response)
    }
}

impl From<DecompileSignedTransactionIntentResponse> for DecompileUnknownTransactionIntentResponse {
    fn from(response: DecompileSignedTransactionIntentResponse) -> Self {
        Self::SignedTransactionIntent(response)
    }
}

impl From<DecompileNotarizedTransactionResponse> for DecompileUnknownTransactionIntentResponse {
    fn from(response: DecompileNotarizedTransactionResponse) -> Self {
        Self::NotarizedTransactionIntent(response)
    }
}

// ===============
// Implementation
// ===============

pub struct DecompileUnknownTransactionIntentHandler;

impl Handler<DecompileUnknownTransactionIntentRequest, DecompileUnknownTransactionIntentResponse>
    for DecompileUnknownTransactionIntentHandler
{
    fn pre_process(
        request: DecompileUnknownTransactionIntentRequest,
    ) -> Result<DecompileUnknownTransactionIntentRequest> {
        Ok(request)
    }

    fn handle(
        request: &DecompileUnknownTransactionIntentRequest,
    ) -> Result<DecompileUnknownTransactionIntentResponse> {
        if let Ok(response) = DecompileTransactionIntentHandler::fulfill(request.clone().into()) {
            Ok(response.into())
        } else if let Ok(response) =
            DecompileSignedTransactionIntentHandler::fulfill(request.clone().into())
        {
            Ok(response.into())
        } else if let Ok(response) =
            DecompileNotarizedTransactionHandler::fulfill(request.clone().into())
        {
            Ok(response.into())
        } else {
            Err(Error::UnrecognizedCompiledIntentFormat)
        }
    }

    fn post_process(
        _: &DecompileUnknownTransactionIntentRequest,
        mut response: DecompileUnknownTransactionIntentResponse,
    ) -> Result<DecompileUnknownTransactionIntentResponse> {
        // Visitors
        let mut aliasing_visitor = ValueAliasingVisitor::default();

        // Instructions
        let instructions: &mut [Instruction] = match response {
            DecompileUnknownTransactionIntentResponse::NotarizedTransactionIntent(
                DecompileNotarizedTransactionResponse {
                    notarized_intent:
                        NotarizedTransaction {
                            signed_intent:
                                SignedTransactionIntent {
                                    intent:
                                        TransactionIntent {
                                            manifest:
                                                TransactionManifest {
                                                    instructions:
                                                        crate::InstructionList::Parsed(
                                                            ref mut instructions,
                                                        ),
                                                    ..
                                                },
                                            ..
                                        },
                                    ..
                                },
                            ..
                        },
                },
            )
            | DecompileUnknownTransactionIntentResponse::SignedTransactionIntent(
                DecompileSignedTransactionIntentResponse {
                    signed_intent:
                        SignedTransactionIntent {
                            intent:
                                TransactionIntent {
                                    manifest:
                                        TransactionManifest {
                                            instructions:
                                                crate::InstructionList::Parsed(ref mut instructions),
                                            ..
                                        },
                                    ..
                                },
                            ..
                        },
                },
            )
            | DecompileUnknownTransactionIntentResponse::TransactionIntent(
                DecompileTransactionIntentResponse {
                    transaction_intent:
                        TransactionIntent {
                            manifest:
                                TransactionManifest {
                                    instructions:
                                        crate::InstructionList::Parsed(ref mut instructions),
                                    ..
                                },
                            ..
                        },
                },
            ) => instructions,
            _ => &mut [],
        };

        // Traverse instructions with visitors
        instructions
            .iter_mut()
            .map(|instruction| {
                traverse_instruction(instruction, &mut [&mut aliasing_visitor], &mut [])
            })
            .collect::<Result<Vec<_>>>()?;

        // The aliasing visitor performs all of the modifications in place as it meets them. Nothing
        // else needs to be done here.

        Ok(response)
    }
}
