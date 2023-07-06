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

use crate::prelude::*;

#[derive(Clone, Debug, Object)]
pub struct NotarizedTransaction {
    pub signed_intent: Arc<SignedIntent>,
    pub notary_signature: Signature,
}

#[uniffi::export]
impl NotarizedTransaction {
    #[uniffi::constructor]
    pub fn new(signed_intent: Arc<SignedIntent>, notary_signature: Signature) -> Arc<Self> {
        Arc::new(Self {
            signed_intent,
            notary_signature,
        })
    }

    #[uniffi::constructor]
    pub fn decompile(compiled_notarized_transaction: Vec<u8>) -> Result<Arc<Self>> {
        core_notarized_transaction_decompile(compiled_notarized_transaction)
            .map(|notarized_transaction| Arc::new(notarized_transaction.into()))
            .map_err(Into::into)
    }

    pub fn signed_intent(&self) -> Arc<SignedIntent> {
        self.signed_intent.clone()
    }

    pub fn notary_signature(&self) -> Signature {
        self.notary_signature.clone()
    }

    pub fn hash(&self) -> Result<Arc<TransactionHash>> {
        NativeNotarizedTransaction::try_from(self.clone()).and_then(|notarized_transaction| {
            core_notarized_transaction_hash(&notarized_transaction)
                .map_err(Into::into)
                .map(|hash| {
                    let notarized_transaction_hash = NativeNotarizedTransactionHash(hash);
                    Arc::new(TransactionHash::new(
                        &notarized_transaction_hash,
                        self.signed_intent.intent.header.network_id,
                    ))
                })
        })
    }

    pub fn notarized_transaction_hash(&self) -> Result<Arc<TransactionHash>> {
        self.hash()
    }

    pub fn signed_intent_hash(&self) -> Result<Arc<TransactionHash>> {
        self.signed_intent.hash()
    }

    pub fn intent_hash(&self) -> Result<Arc<TransactionHash>> {
        self.signed_intent.intent.hash()
    }

    pub fn compile(&self) -> Result<Vec<u8>> {
        NativeNotarizedTransaction::try_from(self.clone()).and_then(|notarized_transaction| {
            core_notarized_transaction_compile(&notarized_transaction).map_err(Into::into)
        })
    }

    pub fn statically_validate(&self, validation_config: Arc<ValidationConfig>) -> Result<()> {
        core_notarized_transaction_statically_validate(
            &self.clone().try_into()?,
            validation_config.as_ref().clone().into(),
        )
        .map_err(Into::into)
    }
}

impl From<NativeNotarizedTransaction> for NotarizedTransaction {
    fn from(
        NativeNotarizedTransaction {
            notary_signature,
            signed_intent,
        }: NativeNotarizedTransaction,
    ) -> Self {
        let signed_intent = SignedIntent::from(signed_intent);
        let notary_signature = Signature::from(notary_signature.0);

        Self {
            signed_intent: Arc::new(signed_intent),
            notary_signature,
        }
    }
}

impl TryFrom<NotarizedTransaction> for NativeNotarizedTransaction {
    type Error = RadixEngineToolkitError;

    fn try_from(value: NotarizedTransaction) -> Result<Self> {
        let signed_intent = NativeSignedIntent::try_from(value.signed_intent.as_ref().clone())?;
        let notary_signature = NativeSignature::try_from(value.notary_signature)?;

        Ok(Self {
            signed_intent,
            notary_signature: NativeNotarySignature(notary_signature),
        })
    }
}
