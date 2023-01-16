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

use scrypto::prelude::{scrypto_decode, scrypto_encode};
use serializable::serializable;

use crate::address::Bech32Coder;
use crate::error::Result;
use crate::model::transaction::{TransactionHeader, TransactionManifest};
use crate::traits::ValueRef;
use crate::{CompilableIntent, Error, InstructionKind};
use native_transaction::model as native;

// =================
// Model Definition
// =================

/// A transaction intent which is made of the header containing the transaction metadata and a
/// manifest consisting of the instructions and blobs.
#[serializable]
pub struct TransactionIntent {
    /// A transaction header of the transaction metadata.
    pub header: TransactionHeader,

    /// A transaction manifest of the transaction instructions and blobs.
    pub manifest: TransactionManifest,
}

// ===============
// Implementation
// ===============

impl ValueRef for TransactionIntent {
    fn borrow_values(&self) -> Vec<&crate::Value> {
        self.manifest.borrow_values()
    }

    fn borrow_values_mut(&mut self) -> Vec<&mut crate::Value> {
        self.manifest.borrow_values_mut()
    }
}

impl CompilableIntent for TransactionIntent {
    fn compile(&self) -> Result<Vec<u8>> {
        self.to_native_transaction_intent()
            .and_then(|intent| scrypto_encode(&intent).map_err(Error::from))
    }

    fn decompile<T>(data: &T, instructions_kind: InstructionKind) -> Result<Self>
    where
        Self: Sized,
        T: AsRef<[u8]>,
    {
        scrypto_decode(data.as_ref())
            .map_err(Error::from)
            .and_then(|decoded| Self::from_native_transaction_intent(&decoded, instructions_kind))
    }
}

// ============
// Conversions
// ============

impl TransactionIntent {
    pub fn from_native_transaction_intent(
        native_transaction_intent: &native::TransactionIntent,
        instructions_kind: InstructionKind,
    ) -> Result<Self> {
        let bech32_coder = Bech32Coder::new(native_transaction_intent.header.network_id);

        TransactionManifest::from_native_manifest(
            &native_transaction_intent.manifest,
            instructions_kind,
            &bech32_coder,
        )
        .map(|transaction_manifest| Self {
            manifest: transaction_manifest,
            header: native_transaction_intent.header.clone().into(),
        })
    }

    pub fn to_native_transaction_intent(&self) -> Result<native::TransactionIntent> {
        let bech32_coder = Bech32Coder::new(self.header.network_id);

        self.manifest
            .to_native_manifest(&bech32_coder)
            .map(|transaction_manifest| native::TransactionIntent {
                manifest: transaction_manifest,
                header: self.header.clone().into(),
            })
    }
}
