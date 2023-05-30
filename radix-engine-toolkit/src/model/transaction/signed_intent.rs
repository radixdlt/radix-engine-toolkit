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

use crate::model::transaction::{InstructionKind, TransactionIntent};
use crate::traits::CompilableIntent;
use crate::utils::debug_string;
use native_transaction::model as native;
use native_transaction::prelude::{
    IntentSignatureV1, IntentSignaturesV1, SignedIntentV1, TransactionPayload,
};
use sbor::{DecodeError, EncodeError};
use toolkit_derive::serializable;

use super::TransactionIntentConversionError;

// =================
// Model Definition
// =================

/// A signed transaction intent which is made up of the intent as well as the intent signatures.
#[serializable]
#[schemars(example = "crate::example::transaction::transaction_structure::signed_intent")]
pub struct SignedTransactionIntent {
    /// The intent of the transaction.
    pub intent: TransactionIntent,

    /// A vector of transaction intent signatures.
    #[schemars(with = "Vec<crate::model::crypto::SignatureWithPublicKey>")]
    #[serde_as(as = "Vec<serde_with::FromInto<crate::model::crypto::SignatureWithPublicKey>>")]
    pub intent_signatures: Vec<native::SignatureWithPublicKeyV1>,
}

// ===============
// Implementation
// ===============

impl CompilableIntent for SignedTransactionIntent {
    type Error = SignedTransactionIntentConversionError;

    fn compile(&self) -> Result<Vec<u8>, Self::Error> {
        self.to_native_signed_transaction_intent()
            .and_then(|intent| intent.to_payload_bytes().map_err(Self::Error::from))
    }

    fn decompile<T>(data: &T, instructions_kind: InstructionKind) -> Result<Self, Self::Error>
    where
        Self: Sized,
        T: AsRef<[u8]>,
    {
        SignedIntentV1::from_payload_bytes(data.as_ref())
            .map_err(Self::Error::from)
            .and_then(|decoded| {
                Self::from_native_signed_transaction_intent(&decoded, instructions_kind)
            })
    }
}

// ===========
// Conversion
// ===========

impl SignedTransactionIntent {
    pub fn from_native_signed_transaction_intent(
        native_signed_transaction_intent: &native::SignedIntentV1,
        instructions_kind: InstructionKind,
    ) -> Result<Self, SignedTransactionIntentConversionError> {
        TransactionIntent::from_native_transaction_intent(
            &native_signed_transaction_intent.intent,
            instructions_kind,
        )
        .map(|transaction_intent| Self {
            intent: transaction_intent,
            intent_signatures: native_signed_transaction_intent
                .intent_signatures
                .clone()
                .signatures
                .into_iter()
                .map(|sig| sig.0)
                .collect(),
        })
        .map_err(SignedTransactionIntentConversionError::from)
    }

    pub fn to_native_signed_transaction_intent(
        &self,
    ) -> Result<native::SignedIntentV1, SignedTransactionIntentConversionError> {
        self.intent
            .to_native_transaction_intent()
            .map(|transaction_intent| native::SignedIntentV1 {
                intent: transaction_intent,
                intent_signatures: IntentSignaturesV1 {
                    signatures: self
                        .intent_signatures
                        .iter()
                        .map(|sig| IntentSignatureV1(*sig))
                        .collect(),
                },
            })
            .map_err(SignedTransactionIntentConversionError::from)
    }
}

/// An error emitted if the conversion between the native and RET representations of the signed
/// transaction intent fails.
#[serializable]
#[serde(tag = "type")]
pub enum SignedTransactionIntentConversionError {
    /// Emitted when the decoding of the SBOR payload into a transaction intent fails
    DecodeError { message: String },

    /// Emitted when the encoding of the SBOR payload into a transaction intent fails
    EncodeError { message: String },

    /// Emitted when the conversion of the intent fails
    TransactionIntentConversionError(TransactionIntentConversionError),
}

impl From<DecodeError> for SignedTransactionIntentConversionError {
    fn from(value: DecodeError) -> Self {
        Self::DecodeError {
            message: debug_string(value),
        }
    }
}

impl From<EncodeError> for SignedTransactionIntentConversionError {
    fn from(value: EncodeError) -> Self {
        Self::EncodeError {
            message: debug_string(value),
        }
    }
}

impl From<TransactionIntentConversionError> for SignedTransactionIntentConversionError {
    fn from(value: TransactionIntentConversionError) -> Self {
        Self::TransactionIntentConversionError(value)
    }
}
