// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at

//   http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use super::traits::Handler;

use crate::error::VisitorError;
use crate::model::address::Bech32Coder;
use crate::model::instruction::Instruction;
use crate::model::transaction::{
    InstructionKind, InstructionList, InstructionListConversionError, TransactionManifest,
};
use crate::visitor::{traverse_instruction, ValueAliasingVisitor, ValueNetworkAggregatorVisitor};
use toolkit_derive::serializable;

// =================
// Model Definition
// =================

/// Clients have a need to be able to read, parse, understand, and interrogate transaction manifests
/// to get more information on what a transactions might be doing. Transaction manifests have so far
/// existed in one format: as strings. While the string format is very human readable, it is not
/// easily readable by machines as a lexer and parser are needed to make sense of them; thus, it is
/// for clients to programmatically make sense of transactions. As such, there is a need for another
/// transaction manifest format (to supplement, NOT replace) which machines can easily make sense of
/// without the need to implement a lexer and parser.
///
/// Therefore, this library introduces a `Parsed` format for transaction manifests which clients can
/// use when wanting to read and interrogate their transaction manifests in code. The transaction
/// manifest `Parsed` format has a 1:1 mapping to the string format of transaction manifests,
/// meaning that anything which can be done in the string format of transaction manifests, can be
/// done in the `Parsed` format as well. If a JSON interface for the Radix Engine Toolkit is used,
/// then the parsed instructions will be all in JSON.
///
/// This function allows the client the convert their manifest between the two supported manifest
/// types: string and parsed.
#[serializable]
pub struct ConvertManifestRequest {
    /// An unsigned 8 bit integer serialized as a string which represents the ID of the network
    /// that the manifest will be used on. The primary use of this is for any Bech32m encoding
    /// or decoding of addresses
    #[schemars(with = "String")]
    #[schemars(regex(pattern = "[0-9]+"))]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub network_id: u8,

    /// Defines the output format that we would like the manifest to be in after this request is
    /// performed.
    pub instructions_output_kind: InstructionKind,

    /// The manifest to convert to the format described by `instructions_output_kind`
    pub manifest: TransactionManifest,
}

/// The response of the [`ConvertManifestRequest`]
#[serializable]
pub struct ConvertManifestResponse {
    /// The manifest after it has been converted to the instruction kind specified in the request
    #[serde(flatten)]
    pub manifest: TransactionManifest,
}

// ===============
// Implementation
// ===============

pub struct ConvertManifestHandler;

impl Handler<ConvertManifestRequest, ConvertManifestResponse> for ConvertManifestHandler {
    type Error = ConvertManifestError;

    fn pre_process(
        mut request: ConvertManifestRequest,
    ) -> Result<ConvertManifestRequest, ConvertManifestError> {
        // Visitors
        let mut network_aggregator_visitor = ValueNetworkAggregatorVisitor::default();

        // Instructions
        let instructions: &mut [Instruction] = match request.manifest.instructions {
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
            .map_err(ConvertManifestError::PreProcessingError)?;

        // Check for network mismatches
        if let Some(network_id) = network_aggregator_visitor
            .0
            .iter()
            .find(|network_id| **network_id != request.network_id)
        {
            return Err(ConvertManifestError::InvalidNetworkIdEncountered {
                found: *network_id,
                expected: request.network_id,
            });
        }
        Ok(request)
    }

    fn handle(
        request: &ConvertManifestRequest,
    ) -> Result<ConvertManifestResponse, ConvertManifestError> {
        request
            .manifest
            .instructions
            .convert_to_manifest_instructions_kind(
                request.instructions_output_kind,
                &Bech32Coder::new(request.network_id),
                request.manifest.blobs.clone(),
            )
            .map(|instructions| ConvertManifestResponse {
                manifest: TransactionManifest {
                    instructions,
                    blobs: request.manifest.blobs.clone(),
                },
            })
            .map_err(Self::Error::from)
    }

    fn post_process(
        _: &ConvertManifestRequest,
        mut response: ConvertManifestResponse,
    ) -> Result<ConvertManifestResponse, ConvertManifestError> {
        // Visitors
        let mut aliasing_visitor = ValueAliasingVisitor::default();

        // Instructions
        let instructions: &mut [Instruction] = match response.manifest.instructions {
            InstructionList::Parsed(ref mut instructions) => instructions,
            InstructionList::String(..) => &mut [],
        };

        // Traverse instructions with visitors
        instructions
            .iter_mut()
            .map(|instruction| {
                traverse_instruction(instruction, &mut [&mut aliasing_visitor], &mut [])
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(ConvertManifestError::PostProcessingError)?;

        // The aliasing visitor performs all of the modifications in place as it meets them. Nothing
        // else needs to be done here.

        Ok(response)
    }
}

#[serializable]
#[serde(tag = "type")]
pub enum ConvertManifestError {
    /// An error emitted during the pre processing of the invocation
    PreProcessingError(VisitorError),

    /// An error emitted during the post processing of the invocation
    PostProcessingError(VisitorError),

    /// An error emitted when an address is encountered in the manifest with an invalid network id
    InvalidNetworkIdEncountered { expected: u8, found: u8 },

    /// Emitted if the conversion of instructions from one format to another fails
    InstructionsConversionError(InstructionListConversionError),
}

impl From<InstructionListConversionError> for ConvertManifestError {
    fn from(value: InstructionListConversionError) -> Self {
        Self::InstructionsConversionError(value)
    }
}
