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

use radix_transaction::model::NotarizedTransaction as NativeNotarizedTransaction;
use radix_transaction::validation::{
    NotarizedTransactionValidator, TestIntentHashManager, TransactionValidator,
};
use scrypto::prelude::{scrypto_decode, scrypto_encode, Signature};

use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::model::transaction::SignedTransactionIntent;
use crate::model::ManifestInstructionsKind;
use crate::traits::{CompilableIntent, TryIntoWithContext, Validate, ValidateWithContext};
use crate::utils::validation_config_from_header;

// =================
// Model Definition
// =================

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NotarizedTransaction {
    pub signed_intent: SignedTransactionIntent,
    pub notary_signature: Signature,
}

// ============
// Conversions
// ============

impl TryInto<NativeNotarizedTransaction> for NotarizedTransaction {
    type Error = Error;

    fn try_into(self) -> Result<NativeNotarizedTransaction, Self::Error> {
        let notarized_transaction = NativeNotarizedTransaction {
            signed_intent: self.signed_intent.try_into()?,
            notary_signature: self.notary_signature,
        };
        Ok(notarized_transaction)
    }
}

impl TryIntoWithContext<NotarizedTransaction, ManifestInstructionsKind>
    for NativeNotarizedTransaction
{
    type Error = Error;

    fn try_into_with_context(
        self,
        manifest_output_format: ManifestInstructionsKind,
    ) -> Result<NotarizedTransaction, Self::Error> {
        let notarized_transaction = NotarizedTransaction {
            signed_intent: self
                .signed_intent
                .try_into_with_context(manifest_output_format)?,
            notary_signature: self.notary_signature,
        };
        Ok(notarized_transaction)
    }
}

// ==============================
// Compilation and Decompilation
// ==============================

impl CompilableIntent for NotarizedTransaction {
    fn compile(&self) -> Result<Vec<u8>, Error> {
        // Convert the notarized transaction intent into a native notarized transaction intent.
        let notarized_transaction: NativeNotarizedTransaction = self.clone().try_into()?;

        // Compile the native notarized transaction intent
        Ok(scrypto_encode(&notarized_transaction).expect("Failed to encode trusted payload"))
    }

    fn decompile<T>(
        data: &T,
        output_manifest_format: ManifestInstructionsKind,
    ) -> Result<Self, Error>
    where
        Self: Sized,
        T: AsRef<[u8]>,
    {
        // Decompile to a native notarized transaction intent
        let data = data.as_ref();
        let notarized_transaction = scrypto_decode::<NativeNotarizedTransaction>(data)?;

        // Convert to this type
        notarized_transaction.try_into_with_context(output_manifest_format)
    }
}

// ===========
// Validation
// ===========

impl Validate for NotarizedTransaction {
    fn validate(&self) -> Result<(), Error> {
        self.signed_intent.intent.header.validate()?;
        self.signed_intent
            .intent
            .manifest
            .validate(self.signed_intent.intent.header.network_id)?;
        NotarizedTransactionValidator::new(validation_config_from_header(
            &self.signed_intent.intent.header,
        ))
        .validate(&self.clone().try_into()?, &TestIntentHashManager::new())?;

        Ok(())
    }
}
