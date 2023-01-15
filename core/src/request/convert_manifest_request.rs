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

//! Defines the request and response models for the convert manifest request. This request is made
//! when the client has a manifest in one format (JSON as an example) and they wish to convert
//! the manifest to another format (String as an example). The conversion between the supported
//! formats is dependent on two main factors: the transaction version, and the network id.

use crate::error::Result;
use crate::model::address::Bech32Coder;
use crate::model::instruction_list::InstructionKind;
use crate::model::TransactionManifest;
use crate::{Handler, ValueRef};
use serializable::serializable;

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
    /// The version of the passed transaction manifest. Used to determine how the manifest is
    /// interpreted by the library.
    #[schemars(with = "String")]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub transaction_version: u8,

    /// The network id of the network that this transaction manifest is meant for. This is used for
    /// the Bech32 address encoding and decoding.
    #[schemars(with = "String")]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub network_id: u8,

    /// Defines the output format that we would like the manifest to be in after this request is
    /// performed.
    pub instructions_output_format: InstructionKind,

    /// The manifest to convert to the format described by `instructions_output_format`
    pub manifest: TransactionManifest,
}

/// The response of the [`ConvertManifestRequest`]
#[serializable]
pub struct ConvertManifestResponse {
    /// The converted manifest
    #[serde(flatten)]
    pub manifest: TransactionManifest,
}

// ===============
// Implementation
// ===============

struct ConvertManifestHandler;

impl Handler<ConvertManifestRequest, ConvertManifestResponse> for ConvertManifestHandler {
    fn pre_process(request: ConvertManifestRequest) -> Result<ConvertManifestRequest> {
        // Validate all `Value`s in the request. Ensure that:
        //     1. All addresses are of the network provided in the request.
        //     2. All single-type collections are of a single kind.
        request
            .borrow_values()
            .iter()
            .map(|value| value.validate(Some(request.network_id)))
            .collect::<Result<Vec<_>>>()?;
        Ok(request)
    }

    fn handle(request: &ConvertManifestRequest) -> Result<ConvertManifestResponse> {
        request
            .manifest
            .instructions
            .convert_to_manifest_instructions_kind(
                request.instructions_output_format,
                &Bech32Coder::new(request.network_id),
                request.manifest.blobs.clone(),
            )
            .map(|instructions| ConvertManifestResponse {
                manifest: TransactionManifest {
                    instructions,
                    blobs: request.manifest.blobs.clone(),
                },
            })
    }

    fn post_process(
        _: &ConvertManifestRequest,
        mut response: ConvertManifestResponse,
    ) -> ConvertManifestResponse {
        for value in response.borrow_values_mut().iter_mut() {
            value.alias();
        }
        response
    }
}

impl ValueRef for ConvertManifestRequest {
    fn borrow_values(&self) -> Vec<&crate::Value> {
        self.manifest.borrow_values()
    }

    fn borrow_values_mut(&mut self) -> Vec<&mut crate::Value> {
        self.manifest.borrow_values_mut()
    }
}

impl ValueRef for ConvertManifestResponse {
    fn borrow_values(&self) -> Vec<&crate::Value> {
        self.manifest.borrow_values()
    }

    fn borrow_values_mut(&mut self) -> Vec<&mut crate::Value> {
        self.manifest.borrow_values_mut()
    }
}
