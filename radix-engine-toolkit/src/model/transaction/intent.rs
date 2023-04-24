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

use sbor::{DecodeError, EncodeError};
use scrypto::prelude::{manifest_decode, manifest_encode};
use toolkit_derive::serializable;

use crate::model::address::Bech32Coder;
use crate::model::transaction::InstructionKind;
use crate::model::transaction::{TransactionHeader, TransactionManifest};
use crate::traits::CompilableIntent;
use crate::utils::debug_string;
use native_transaction::model as native;

use super::TransactionManifestConversionError;

// =================
// Model Definition
// =================

/// A transaction intent which is made of the header containing the transaction metadata and a
/// manifest consisting of the instructions and blobs.
#[serializable]
#[schemars(example = "crate::example::transaction::transaction_structure::intent")]
pub struct TransactionIntent {
    /// A transaction header of the transaction metadata.
    pub header: TransactionHeader,

    /// A transaction manifest of the transaction instructions and blobs.
    pub manifest: TransactionManifest,
}

// ===============
// Implementation
// ===============

impl CompilableIntent for TransactionIntent {
    type Error = TransactionIntentConversionError;

    fn compile(&self) -> Result<Vec<u8>, Self::Error> {
        self.to_native_transaction_intent()
            .and_then(|intent| manifest_encode(&intent).map_err(Self::Error::from))
    }

    fn decompile<T>(data: &T, instructions_kind: InstructionKind) -> Result<Self, Self::Error>
    where
        Self: Sized,
        T: AsRef<[u8]>,
    {
        manifest_decode(data.as_ref())
            .map_err(Self::Error::from)
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
    ) -> Result<Self, TransactionIntentConversionError> {
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
        .map_err(TransactionIntentConversionError::from)
    }

    pub fn to_native_transaction_intent(
        &self,
    ) -> Result<native::TransactionIntent, TransactionIntentConversionError> {
        let bech32_coder = Bech32Coder::new(self.header.network_id);

        self.manifest
            .to_native_manifest(&bech32_coder)
            .map(|transaction_manifest| native::TransactionIntent {
                manifest: transaction_manifest,
                header: self.header.clone().into(),
            })
            .map_err(TransactionIntentConversionError::from)
    }
}

/// An error emitted if the conversion between the native and RET representations of the Transaction
/// Intent fails.
#[serializable]
#[serde(tag = "type")]
pub enum TransactionIntentConversionError {
    /// Emitted when the decoding of the SBOR payload into a transaction intent fails
    DecodeError { message: String },

    /// Emitted when the encoding of the SBOR payload into a transaction intent fails
    EncodeError { message: String },

    /// Emitted when the conversion of manifests fail
    ManifestConversionError(TransactionManifestConversionError),
}

impl From<DecodeError> for TransactionIntentConversionError {
    fn from(value: DecodeError) -> Self {
        Self::DecodeError {
            message: debug_string(value),
        }
    }
}

impl From<EncodeError> for TransactionIntentConversionError {
    fn from(value: EncodeError) -> Self {
        Self::EncodeError {
            message: debug_string(value),
        }
    }
}

impl From<TransactionManifestConversionError> for TransactionIntentConversionError {
    fn from(value: TransactionManifestConversionError) -> Self {
        Self::ManifestConversionError(value)
    }
}
