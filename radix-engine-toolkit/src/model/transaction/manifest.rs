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

use crate::model::address::Bech32Coder;
use crate::model::transaction::{InstructionKind, InstructionList};
use crate::utils::debug_string;
use native_transaction::manifest::{decompile, DecompileError};
use radix_engine_common::prelude::hash;
use toolkit_derive::serializable;

use super::InstructionListConversionError;

// =================
// Model Definition
// =================

/// A transaction intent consisting of instructions as well as blobs
#[serializable]
#[schemars(example = "crate::example::transaction::transaction_structure::manifest")]
pub struct TransactionManifest {
    /// The transaction manifest instructions to be executed in the transaction.
    pub instructions: InstructionList,

    /// An array of byte arrays which is serialized as an array of hex strings which represents the
    /// blobs included in the transaction.
    #[schemars(with = "Vec<String>")]
    #[serde_as(as = "Vec<serde_with::hex::Hex>")]
    pub blobs: Vec<Vec<u8>>,
}

// ============
// Conversions
// ============

impl TransactionManifest {
    pub fn from_native_manifest(
        native_manifest: &native_transaction::prelude::TransactionManifestV1,
        instructions_kind: InstructionKind,
        bech32_coder: &Bech32Coder,
    ) -> Result<Self, TransactionManifestConversionError> {
        decompile(
            &native_manifest.instructions,
            bech32_coder.network_definition(),
        )
        .map(InstructionList::String)
        .map_err(TransactionManifestConversionError::from)
        .and_then(|instructions| {
            instructions
                .convert_to_manifest_instructions_kind(
                    instructions_kind,
                    bech32_coder,
                    native_manifest.blobs.values().cloned().collect(),
                )
                .map_err(TransactionManifestConversionError::from)
        })
        .map(|instructions| TransactionManifest {
            instructions,
            blobs: native_manifest.blobs.values().cloned().collect(),
        })
    }

    pub fn to_native_manifest(
        &self,
        bech32_coder: &Bech32Coder,
    ) -> Result<
        native_transaction::prelude::TransactionManifestV1,
        TransactionManifestConversionError,
    > {
        self.instructions
            .basic_instructions(bech32_coder, self.blobs.clone())
            .map(
                |basic_instructions| native_transaction::prelude::TransactionManifestV1 {
                    instructions: basic_instructions,
                    blobs: self
                        .blobs
                        .iter()
                        .map(|blob| (hash(blob), blob.clone()))
                        .collect(),
                },
            )
            .map_err(TransactionManifestConversionError::from)
    }
}

/// An error emitted if the conversion between the native and RET representations of the Transaction
/// Manifest fails.
#[serializable]
#[serde(tag = "type")]
pub enum TransactionManifestConversionError {
    InstructionConversionError(InstructionListConversionError),

    /// An error emitted when the decompilation of manifests fail
    ScryptoDecompileError {
        message: String,
    },
}

impl From<DecompileError> for TransactionManifestConversionError {
    fn from(value: DecompileError) -> Self {
        Self::ScryptoDecompileError {
            message: debug_string(value),
        }
    }
}

impl From<InstructionListConversionError> for TransactionManifestConversionError {
    fn from(value: InstructionListConversionError) -> Self {
        Self::InstructionConversionError(value)
    }
}
