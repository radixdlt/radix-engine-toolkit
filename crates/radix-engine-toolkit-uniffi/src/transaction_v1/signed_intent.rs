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
pub struct SignedTransactionIntentV1 {
    pub intent: Arc<IntentV1>,
    pub intent_signatures: Vec<SignatureWithPublicKeyV1>,
}

#[uniffi::export]
impl SignedTransactionIntentV1 {
    #[uniffi::constructor]
    pub fn new(
        intent: Arc<IntentV1>,
        intent_signatures: Vec<SignatureWithPublicKeyV1>,
    ) -> Arc<Self> {
        Arc::new(Self {
            intent,
            intent_signatures,
        })
    }

    #[uniffi::constructor]
    pub fn decompile(compiled_signed_intent: Vec<u8>) -> Result<Arc<Self>> {
        core_transaction_v1_signed_intent_from_payload_bytes(
            compiled_signed_intent,
        )
        .map(|signed_intent| Arc::new(signed_intent.into()))
        .map_err(Into::into)
    }

    pub fn intent(&self) -> Arc<IntentV1> {
        self.intent.clone()
    }

    pub fn intent_signatures(&self) -> Vec<SignatureWithPublicKeyV1> {
        self.intent_signatures.clone()
    }

    pub fn hash(&self) -> Result<Arc<TransactionHash>> {
        NativeSignedIntentV1::try_from(self.clone()).and_then(|signed_intent| {
            core_transaction_v1_signed_intent_hash(&signed_intent)
                .map_err(Into::into)
                .map(|hash| {
                    let signed_intent_hash =
                        NativeSignedTransactionIntentHash(hash.hash);
                    Arc::new(TransactionHash::new(
                        &signed_intent_hash,
                        self.intent.header.network_id,
                    ))
                })
        })
    }

    pub fn signed_intent_hash(&self) -> Result<Arc<TransactionHash>> {
        self.hash()
    }

    pub fn intent_hash(&self) -> Result<Arc<TransactionHash>> {
        self.intent.hash()
    }

    pub fn compile(&self) -> Result<Vec<u8>> {
        NativeSignedIntentV1::try_from(self.clone()).and_then(|signed_intent| {
            core_transaction_v1_signed_intent_to_payload_bytes(&signed_intent)
                .map_err(Into::into)
        })
    }

    pub fn statically_validate(
        &self,
        validation_config: Arc<ValidationConfig>,
    ) -> Result<()> {
        core_transaction_v1_signed_intent_statically_validate(
            &self.clone().try_into()?,
            validation_config.as_ref().clone().into(),
        )
        .map_err(Into::into)
    }
}

impl From<NativeSignedIntentV1> for SignedTransactionIntentV1 {
    fn from(
        NativeSignedIntentV1 {
            intent,
            intent_signatures,
        }: NativeSignedIntentV1,
    ) -> Self {
        let intent = IntentV1::from(intent);
        let intent_signatures = intent_signatures
            .signatures
            .into_iter()
            .map(|signature| SignatureWithPublicKeyV1::from(signature.0))
            .collect::<Vec<_>>();

        Self {
            intent: Arc::new(intent),
            intent_signatures,
        }
    }
}

impl TryFrom<SignedTransactionIntentV1> for NativeSignedIntentV1 {
    type Error = RadixEngineToolkitError;

    fn try_from(value: SignedTransactionIntentV1) -> Result<Self> {
        let intent = NativeIntentV1::try_from(value.intent.as_ref().clone())?;
        let intent_signatures = value
            .intent_signatures
            .into_iter()
            .map(|signature| signature.try_into().map(NativeIntentSignature))
            .collect::<Result<Vec<_>>>()?;

        Ok(Self {
            intent,
            intent_signatures: NativeIntentSignatures {
                signatures: intent_signatures,
            },
        })
    }
}
