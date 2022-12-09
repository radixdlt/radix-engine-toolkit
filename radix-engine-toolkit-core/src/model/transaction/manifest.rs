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

use radix_transaction::manifest::decompile;
use radix_transaction::manifest::generator::generate_manifest;
use radix_transaction::model::TransactionManifest as NativeTransactionManifest;

use scrypto::prelude::hash;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::error::Error;
use crate::model::address::Bech32Coder;
use crate::model::{ManifestInstructions, ManifestInstructionsKind};
use crate::traits::{TryIntoWithContext, ValidateWithContext};

// =================
// Model Definition
// =================

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct TransactionManifest {
    pub instructions: ManifestInstructions,
    #[serde_as(as = "Vec<serde_with::hex::Hex>")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub blobs: Vec<Vec<u8>>,
}

// ============
// Conversions
// ============

impl<T> TryIntoWithContext<NativeTransactionManifest, T> for TransactionManifest
where
    T: AsRef<Bech32Coder>,
{
    type Error = Error;

    fn try_into_with_context(
        self,
        bech32_coder: T,
    ) -> Result<NativeTransactionManifest, Self::Error> {
        let bech32_coder = bech32_coder.as_ref();

        let transaction_manifest = NativeTransactionManifest {
            instructions: self
                .instructions
                .transaction_instructions(bech32_coder, self.blobs.clone())?,
            blobs: self.blobs,
        };
        Ok(transaction_manifest)
    }
}

impl<T> TryIntoWithContext<TransactionManifest, (ManifestInstructionsKind, T)>
    for NativeTransactionManifest
where
    T: AsRef<Bech32Coder>,
{
    type Error = Error;

    fn try_into_with_context(
        self,
        (manifest_instruction_kind, bech32_coder): (ManifestInstructionsKind, T),
    ) -> Result<TransactionManifest, Self::Error> {
        let bech32_coder = bech32_coder.as_ref();

        // Converting a TransactionInstruction to a string is rather easy to do, so we will convert
        // them into a String, and then we will convert the entire manifest instructions into the
        // requested format provided in the context.
        let manifest_instructions = ManifestInstructions::String(decompile(
            &self.instructions,
            &bech32_coder.network_definition,
        )?);

        // Converting the manifest instructions according to the requested manifest instructions
        // kind.
        let manifest_instructions = manifest_instructions.convert_to_manifest_instructions_kind(
            manifest_instruction_kind,
            bech32_coder,
            self.blobs.clone(),
        )?;

        let transaction_manifest = TransactionManifest {
            instructions: manifest_instructions,
            blobs: self.blobs,
        };
        Ok(transaction_manifest)
    }
}

// ===========
// Validation
// ===========

impl ValidateWithContext<u8> for TransactionManifest {
    fn validate(&self, network_id: u8) -> Result<(), Error> {
        let bech32_coder = Bech32Coder::new(network_id);

        self.instructions
            .instructions(&bech32_coder)?
            .iter()
            .map(|instruction| instruction.validate(network_id))
            .collect::<Result<Vec<_>, _>>()?;
        generate_manifest(
            &self.instructions.ast_instructions(&bech32_coder)?,
            &bech32_coder.decoder,
            self.blobs.iter().map(|x| (hash(x), x.clone())).collect(),
        )?;
        Ok(())
    }
}
