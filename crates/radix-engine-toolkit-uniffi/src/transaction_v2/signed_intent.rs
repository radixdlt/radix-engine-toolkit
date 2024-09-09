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
    pub intent: Arc<TransactionIntentV2>,
    pub intent_signatures: Vec<SignatureWithPublicKeyV1>,
    pub subintent_signatures: Vec<Vec<SignatureWithPublicKeyV1>>,
}

#[uniffi::export]
impl SignedTransactionIntentV2 {
    #[uniffi::constructor]
    pub fn new(
        intent: Arc<TransactionIntentV2>,
        intent_signatures: Vec<SignatureWithPublicKeyV1>,
        subintent_signatures: Vec<Vec<SignatureWithPublicKeyV1>>,
    ) -> Arc<Self> {
        Arc::new(Self {
            intent,
            intent_signatures,
            subintent_signatures,
        })
    }

    #[uniffi::constructor]
    pub fn from_payload_bytes(
        compiled_signed_intent: Vec<u8>,
    ) -> Result<Arc<Self>> {
        core_transaction_v2_signed_transaction_intent_from_payload_bytes(
            compiled_signed_intent,
        )
        .map_err(RadixEngineToolkitError::from)
        .and_then(|intent| intent.try_into().map(Arc::new))
    }

    pub fn intent(&self) -> Arc<TransactionIntentV2> {
        self.intent.clone()
    }

    pub fn intent_signatures(&self) -> Vec<SignatureWithPublicKeyV1> {
        self.intent_signatures.clone()
    }

    pub fn hash(&self) -> Result<Arc<TransactionHash>> {
        NativeSignedTransactionIntentV2::try_from(self.clone()).and_then(
            |signed_intent| {
                core_transaction_v2_signed_transaction_intent_hash(
                    &signed_intent,
                )
                .map_err(Into::into)
                .map(|hash| {
                    let signed_intent_hash =
                        NativeSignedTransactionIntentHash(hash.hash);
                    Arc::new(TransactionHash::new(
                        &signed_intent_hash,
                        self.intent.root_intent_core.header.network_id,
                    ))
                })
            },
        )
    }

    pub fn signed_intent_hash(&self) -> Result<Arc<TransactionHash>> {
        self.hash()
    }

    pub fn intent_hash(&self) -> Result<Arc<TransactionHash>> {
        self.intent.hash()
    }

    pub fn to_payload_bytes(&self) -> Result<Vec<u8>> {
        NativeSignedTransactionIntentV2::try_from(self.clone()).and_then(
            |signed_intent| {
                core_transaction_v2_signed_transaction_intent_to_payload_bytes(
                    &signed_intent,
                )
                .map_err(Into::into)
            },
        )
    }
}

impl TryFrom<NativeSignedTransactionIntentV2> for SignedTransactionIntentV2 {
    type Error = RadixEngineToolkitError;

    fn try_from(
        NativeSignedTransactionIntentV2 {
            root_intent,
            root_intent_signatures,
            subintent_signatures,
        }: NativeSignedTransactionIntentV2,
    ) -> Result<Self> {
        Ok(Self {
            intent: root_intent.try_into().map(Arc::new)?,
            intent_signatures: root_intent_signatures
                .signatures
                .into_iter()
                .map(|value| value.0)
                .map(SignatureWithPublicKeyV1::from)
                .collect(),
            subintent_signatures: subintent_signatures
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

impl TryFrom<SignedTransactionIntentV2> for NativeSignedTransactionIntentV2 {
    type Error = RadixEngineToolkitError;

    fn try_from(
        SignedTransactionIntentV2 {
            intent,
            intent_signatures,
            subintent_signatures,
        }: SignedTransactionIntentV2,
    ) -> Result<Self> {
        Ok(Self {
            root_intent: NativeTransactionIntentV2::try_from(
                intent.as_ref().clone(),
            )?,
            root_intent_signatures: intent_signatures
                .into_iter()
                .map(|value| {
                    NativeSignatureWithPublicKeyV1::try_from(value)
                        .map(NativeIntentSignature)
                })
                .collect::<Result<_>>()
                .map(|value| NativeIntentSignaturesV1 { signatures: value })?,
            subintent_signatures: subintent_signatures
                .into_iter()
                .map(|value| {
                    value
                        .into_iter()
                        .map(|value| {
                            NativeSignatureWithPublicKeyV1::try_from(value)
                                .map(NativeIntentSignature)
                        })
                        .collect::<Result<_>>()
                        .map(|value| NativeIntentSignaturesV2 {
                            signatures: value,
                        })
                })
                .collect::<Result<_>>()
                .map(|value| NativeMultipleIntentSignaturesV2 {
                    by_subintent: value,
                })?,
        })
    }
}
