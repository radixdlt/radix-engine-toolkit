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
pub struct SignedTransactionIntentV2 {
    pub transaction_intent: Arc<TransactionIntentV2>,
    pub transaction_intent_signatures: Vec<SignatureWithPublicKeyV1>,
    pub non_root_subintent_signatures: Vec<Vec<SignatureWithPublicKeyV1>>,
}

#[uniffi::export]
impl SignedTransactionIntentV2 {
    #[uniffi::constructor]
    pub fn new(
        transaction_intent: Arc<TransactionIntentV2>,
        transaction_intent_signatures: Vec<SignatureWithPublicKeyV1>,
        non_root_subintent_signatures: Vec<Vec<SignatureWithPublicKeyV1>>,
    ) -> Arc<Self> {
        Arc::new(Self {
            transaction_intent,
            transaction_intent_signatures,
            non_root_subintent_signatures,
        })
    }

    #[uniffi::constructor]
    pub fn from_payload_bytes(
        compiled_signed_intent: Vec<u8>,
    ) -> Result<Arc<Self>> {
        toolkit::functions::transaction_v2::signed_transaction_intent::from_payload_bytes(
            compiled_signed_intent,
        )
        .map_err(RadixEngineToolkitError::from)
        .and_then(|transaction_intent| transaction_intent.try_into().map(Arc::new))
    }

    pub fn transaction_intent(&self) -> Arc<TransactionIntentV2> {
        self.transaction_intent.clone()
    }

    pub fn transaction_intent_signatures(
        &self,
    ) -> Vec<SignatureWithPublicKeyV1> {
        self.transaction_intent_signatures.clone()
    }

    pub fn hash(&self) -> Result<Arc<TransactionHash>> {
        engine::SignedTransactionIntentV2::try_from(self.clone()).and_then(
            |signed_intent| {
                toolkit::functions::transaction_v2::signed_transaction_intent::hash(
                    &signed_intent,
                )
                .map_err(Into::into)
                .map(|hash| {
                    let signed_intent_hash =
                        engine::SignedTransactionIntentHash(hash.hash);
                    Arc::new(TransactionHash::new(
                        &signed_intent_hash,
                        self.transaction_intent.root_intent_core.header.network_id,
                    ))
                })
            },
        )
    }

    pub fn signed_intent_hash(&self) -> Result<Arc<TransactionHash>> {
        self.hash()
    }

    pub fn intent_hash(&self) -> Result<Arc<TransactionHash>> {
        self.transaction_intent.hash()
    }

    pub fn to_payload_bytes(&self) -> Result<Vec<u8>> {
        engine::SignedTransactionIntentV2::try_from(self.clone()).and_then(|signed_intent| {
            toolkit::functions::transaction_v2::signed_transaction_intent::to_payload_bytes(
                &signed_intent,
            )
            .map_err(Into::into)
        })
    }
}

impl TryFrom<engine::SignedTransactionIntentV2> for SignedTransactionIntentV2 {
    type Error = RadixEngineToolkitError;

    fn try_from(
        engine::SignedTransactionIntentV2 {
            transaction_intent,
            transaction_intent_signatures,
            non_root_subintent_signatures,
        }: engine::SignedTransactionIntentV2,
    ) -> Result<Self> {
        Ok(Self {
            transaction_intent: transaction_intent.try_into().map(Arc::new)?,
            transaction_intent_signatures: transaction_intent_signatures
                .signatures
                .into_iter()
                .map(|value| value.0)
                .map(SignatureWithPublicKeyV1::from)
                .collect(),
            non_root_subintent_signatures: non_root_subintent_signatures
                .by_subintent
                .into_iter()
                .map(|value| {
                    value
                        .signatures
                        .into_iter()
                        .map(|value| value.0)
                        .map(SignatureWithPublicKeyV1::from)
                        .collect()
                })
                .collect(),
        })
    }
}

impl TryFrom<SignedTransactionIntentV2> for engine::SignedTransactionIntentV2 {
    type Error = RadixEngineToolkitError;

    fn try_from(
        SignedTransactionIntentV2 {
            transaction_intent,
            transaction_intent_signatures,
            non_root_subintent_signatures,
        }: SignedTransactionIntentV2,
    ) -> Result<Self> {
        Ok(Self {
            transaction_intent: engine::TransactionIntentV2::try_from(
                transaction_intent.as_ref().clone(),
            )?,
            transaction_intent_signatures: transaction_intent_signatures
                .into_iter()
                .map(|value| {
                    engine::SignatureWithPublicKeyV1::try_from(value)
                        .map(engine::IntentSignatureV1)
                })
                .collect::<Result<_>>()
                .map(|value| engine::IntentSignaturesV2 {
                    signatures: value,
                })?,
            non_root_subintent_signatures: non_root_subintent_signatures
                .into_iter()
                .map(|value| {
                    value
                        .into_iter()
                        .map(|value| {
                            engine::SignatureWithPublicKeyV1::try_from(value)
                                .map(engine::IntentSignatureV1)
                        })
                        .collect::<Result<_>>()
                        .map(|value| engine::IntentSignaturesV2 {
                            signatures: value,
                        })
                })
                .collect::<Result<_>>()
                .map(|value| engine::NonRootSubintentSignaturesV2 {
                    by_subintent: value,
                })?,
        })
    }
}
