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
use crate::{Bech32Coder, Error, InstructionKind, InstructionList};
use native_transaction::manifest::decompile;
use native_transaction::model as native;
use serializable::serializable;

// =================
// Model Definition
// =================

/// A transaction intent consisting of instructions as well as blobs
#[serializable]
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
        native_manifest: &native::TransactionManifest,
        instructions_kind: InstructionKind,
        bech32_coder: &Bech32Coder,
    ) -> Result<Self> {
        decompile(
            &native_manifest.instructions,
            bech32_coder.network_definition(),
        )
        .map(InstructionList::String)
        .map_err(Error::from)
        .and_then(|instructions| {
            instructions.convert_to_manifest_instructions_kind(
                instructions_kind,
                bech32_coder,
                native_manifest.blobs.clone(),
            )
        })
        .map(|instructions| TransactionManifest {
            instructions,
            blobs: native_manifest.blobs.clone(),
        })
    }

    pub fn to_native_manifest(
        &self,
        bech32_coder: &Bech32Coder,
    ) -> Result<native::TransactionManifest> {
        self.instructions
            .basic_instructions(bech32_coder, self.blobs.clone())
            .map(|basic_instructions| native::TransactionManifest {
                instructions: basic_instructions,
                blobs: self.blobs.clone(),
            })
    }
}
