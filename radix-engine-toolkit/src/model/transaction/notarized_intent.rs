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

use crate::{error::Result, CompilableIntent, Error, InstructionKind};
use native_transaction::model as native;
use native_transaction_data::{manifest_decode, manifest_encode};
use toolkit_derive::serializable;

use crate::SignedTransactionIntent;

// =================
// Model Definition
// =================

/// A notarized transaction intent which is made up of a signed transaction intent and the notary
/// intent on said signed intent.
#[serializable]
pub struct NotarizedTransaction {
    /// The signed transaction intent of the transaction.
    pub signed_intent: SignedTransactionIntent,

    /// The signature of the notary on the signed transaction intent.
    #[schemars(with = "crate::model::crypto::Signature")]
    #[serde_as(as = "serde_with::FromInto<crate::model::crypto::Signature>")]
    pub notary_signature: native::Signature,
}

// ===============
// Implementation
// ===============

impl CompilableIntent for NotarizedTransaction {
    fn compile(&self) -> Result<Vec<u8>> {
        self.to_native_notarized_transaction_intent()
            .and_then(|notarized_transaction| {
                manifest_encode(&notarized_transaction).map_err(Error::from)
            })
    }

    fn decompile<T>(data: &T, instructions_kind: InstructionKind) -> Result<Self>
    where
        Self: Sized,
        T: AsRef<[u8]>,
    {
        manifest_decode(data.as_ref())
            .map_err(Error::from)
            .and_then(|decoded| {
                Self::from_native_notarized_transaction_intent(&decoded, instructions_kind)
            })
    }
}

// ============
// Conversions
// ============

impl NotarizedTransaction {
    pub fn from_native_notarized_transaction_intent(
        native_notarized_transaction_intent: &native::NotarizedTransaction,
        instructions_kind: InstructionKind,
    ) -> Result<Self> {
        SignedTransactionIntent::from_native_signed_transaction_intent(
            &native_notarized_transaction_intent.signed_intent,
            instructions_kind,
        )
        .map(|signed_intent| Self {
            signed_intent,
            notary_signature: native_notarized_transaction_intent.notary_signature,
        })
    }

    pub fn to_native_notarized_transaction_intent(&self) -> Result<native::NotarizedTransaction> {
        self.signed_intent
            .to_native_signed_transaction_intent()
            .map(|signed_intent| native::NotarizedTransaction {
                signed_intent,
                notary_signature: self.notary_signature,
            })
    }
}
