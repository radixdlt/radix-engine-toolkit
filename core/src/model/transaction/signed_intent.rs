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

use crate::{error::Result, CompilableIntent, Error};
use native_transaction::model as native;
use scrypto::prelude::{scrypto_decode, scrypto_encode, SignatureWithPublicKey};
use serializable::serializable;

use crate::{InstructionKind, TransactionIntent, ValueRef};

// =================
// Model Definition
// =================

/// A signed transaction intent which is made up of the intent as well as the intent signatures.
#[serializable]
pub struct SignedTransactionIntent {
    /// The intent of the transaction.
    pub intent: TransactionIntent,

    /// A vector of transaction intent signatures.
    #[schemars(with = "Vec<crate::model::crypto::SignatureWithPublicKey>")]
    #[serde_as(as = "Vec<serde_with::FromInto<crate::model::crypto::SignatureWithPublicKey>>")]
    pub intent_signatures: Vec<SignatureWithPublicKey>,
}

// ===============
// Implementation
// ===============

impl ValueRef for SignedTransactionIntent {
    fn borrow_values(&self) -> Vec<&crate::Value> {
        self.intent.borrow_values()
    }

    fn borrow_values_mut(&mut self) -> Vec<&mut crate::Value> {
        self.intent.borrow_values_mut()
    }
}

impl CompilableIntent for SignedTransactionIntent {
    fn compile(&self) -> Result<Vec<u8>> {
        self.to_native_signed_transaction_intent()
            .and_then(|intent| scrypto_encode(&intent).map_err(Error::from))
    }

    fn decompile<T>(data: &T, instructions_kind: InstructionKind) -> Result<Self>
    where
        Self: Sized,
        T: AsRef<[u8]>,
    {
        scrypto_decode(data.as_ref())
            .map_err(Error::from)
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
        native_signed_transaction_intent: &native::SignedTransactionIntent,
        instructions_kind: InstructionKind,
    ) -> Result<Self> {
        TransactionIntent::from_native_transaction_intent(
            &native_signed_transaction_intent.intent,
            instructions_kind,
        )
        .map(|transaction_intent| Self {
            intent: transaction_intent,
            intent_signatures: native_signed_transaction_intent.intent_signatures.clone(),
        })
    }

    pub fn to_native_signed_transaction_intent(&self) -> Result<native::SignedTransactionIntent> {
        self.intent
            .to_native_transaction_intent()
            .map(|transaction_intent| native::SignedTransactionIntent {
                intent: transaction_intent,
                intent_signatures: self.intent_signatures.clone(),
            })
    }
}
