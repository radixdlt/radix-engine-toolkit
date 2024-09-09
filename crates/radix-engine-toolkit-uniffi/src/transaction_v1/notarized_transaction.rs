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
pub struct NotarizedTransactionV1 {
    pub signed_intent: Arc<SignedTransactionIntentV1>,
    pub notary_signature: SignatureV1,
}

#[uniffi::export]
impl NotarizedTransactionV1 {
    #[uniffi::constructor]
    pub fn new(
        signed_intent: Arc<SignedTransactionIntentV1>,
        notary_signature: SignatureV1,
    ) -> Arc<Self> {
        Arc::new(Self {
            signed_intent,
            notary_signature,
        })
    }

    #[uniffi::constructor]
    pub fn decompile(
        compiled_notarized_transaction: Vec<u8>,
    ) -> Result<Arc<Self>> {
        core_transaction_v1_notarized_transaction_from_payload_bytes(
            compiled_notarized_transaction,
        )
        .map(|notarized_transaction| Arc::new(notarized_transaction.into()))
        .map_err(Into::into)
    }

    pub fn signed_intent(&self) -> Arc<SignedTransactionIntentV1> {
        self.signed_intent.clone()
    }

    pub fn notary_signature(&self) -> SignatureV1 {
        self.notary_signature.clone()
    }

    pub fn hash(&self) -> Result<Arc<TransactionHash>> {
        NativeNotarizedTransactionV1::try_from(self.clone()).and_then(
            |notarized_transaction| {
                core_transaction_v1_notarized_transaction_hash(
                    &notarized_transaction,
                )
                .map_err(Into::into)
                .map(|hash| {
                    let notarized_transaction_hash =
                        NativeNotarizedTransactionHash(hash.hash);
                    Arc::new(TransactionHash::new(
                        &notarized_transaction_hash,
                        self.signed_intent.intent.header.network_id,
                    ))
                })
            },
        )
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
        NativeNotarizedTransactionV1::try_from(self.clone()).and_then(
            |notarized_transaction| {
                core_transaction_v1_notarized_transaction_to_payload_bytes(
                    &notarized_transaction,
                )
                .map_err(Into::into)
            },
        )
    }

    pub fn statically_validate(
        &self,
        validation_config: Arc<ValidationConfig>,
    ) -> Result<()> {
        core_transaction_v1_notarized_transaction_statically_validate(
            &self.clone().try_into()?,
            validation_config.as_ref().clone().into(),
        )
        .map_err(Into::into)
    }
}

impl From<NativeNotarizedTransactionV1> for NotarizedTransactionV1 {
    fn from(
        NativeNotarizedTransactionV1 {
            notary_signature,
            signed_intent,
        }: NativeNotarizedTransactionV1,
    ) -> Self {
        let signed_intent = SignedTransactionIntentV1::from(signed_intent);
        let notary_signature = SignatureV1::from(notary_signature.0);

        Self {
            signed_intent: Arc::new(signed_intent),
            notary_signature,
        }
    }
}

impl TryFrom<NotarizedTransactionV1> for NativeNotarizedTransactionV1 {
    type Error = RadixEngineToolkitError;

    fn try_from(value: NotarizedTransactionV1) -> Result<Self> {
        let signed_intent = NativeSignedIntentV1::try_from(
            value.signed_intent.as_ref().clone(),
        )?;
        let notary_signature =
            NativeSignatureV1::try_from(value.notary_signature)?;

        Ok(Self {
            signed_intent,
            notary_signature: NativeNotarySignature(notary_signature),
        })
    }
}
