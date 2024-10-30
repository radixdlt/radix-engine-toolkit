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
pub struct NotarizedTransactionV2 {
    pub signed_transaction_intent: Arc<SignedTransactionIntentV2>,
    pub notary_signature: SignatureV1,
}

#[uniffi::export]
impl NotarizedTransactionV2 {
    #[uniffi::constructor]
    pub fn new(
        signed_transaction_intent: Arc<SignedTransactionIntentV2>,
        notary_signature: SignatureV1,
    ) -> Arc<Self> {
        Arc::new(Self {
            signed_transaction_intent,
            notary_signature,
        })
    }

    #[uniffi::constructor]
    pub fn from_payload_bytes(
        compiled_notarized_transaction: Vec<u8>,
    ) -> Result<Arc<Self>> {
        core_transaction_v2_notarized_transaction_from_payload_bytes(
            compiled_notarized_transaction,
        )
        .map_err(RadixEngineToolkitError::from)
        .and_then(|intent| intent.try_into().map(Arc::new))
    }

    pub fn signed_transaction_intent(&self) -> Arc<SignedTransactionIntentV2> {
        self.signed_transaction_intent.clone()
    }

    pub fn notary_signature(&self) -> SignatureV1 {
        self.notary_signature.clone()
    }

    pub fn hash(&self) -> Result<Arc<TransactionHash>> {
        NativeNotarizedTransactionV2::try_from(self.clone()).and_then(
            |notarized_transaction| {
                core_transaction_v2_notarized_transaction_hash(
                    &notarized_transaction,
                )
                .map_err(Into::into)
                .map(|hash| {
                    let notarized_transaction_hash =
                        NativeNotarizedTransactionHash(hash.hash);
                    Arc::new(TransactionHash::new(
                        &notarized_transaction_hash,
                        self.signed_transaction_intent
                            .transaction_intent
                            .root_intent_core
                            .header
                            .network_id,
                    ))
                })
            },
        )
    }

    pub fn notarized_transaction_hash(&self) -> Result<Arc<TransactionHash>> {
        self.hash()
    }

    pub fn signed_transaction_intent_hash(
        &self,
    ) -> Result<Arc<TransactionHash>> {
        self.signed_transaction_intent.hash()
    }

    pub fn intent_hash(&self) -> Result<Arc<TransactionHash>> {
        self.signed_transaction_intent.transaction_intent.hash()
    }

    pub fn to_payload_bytes(&self) -> Result<Vec<u8>> {
        NativeNotarizedTransactionV2::try_from(self.clone()).and_then(
            |notarized_transaction| {
                core_transaction_v2_notarized_transaction_to_payload_bytes(
                    &notarized_transaction,
                )
                .map_err(Into::into)
            },
        )
    }

    pub fn statically_validate(&self, network_id: u8) -> Result<()> {
        core_transaction_v2_notarized_transaction_statically_validate(
            &self.clone().try_into()?,
            &core_network_definition_from_network_id(network_id),
        )
        .map_err(Into::into)
    }
}

impl TryFrom<NativeNotarizedTransactionV2> for NotarizedTransactionV2 {
    type Error = RadixEngineToolkitError;

    fn try_from(
        NativeNotarizedTransactionV2 {
            notary_signature,
            signed_transaction_intent,
        }: NativeNotarizedTransactionV2,
    ) -> Result<Self> {
        Ok(Self {
            signed_transaction_intent: Arc::new(
                signed_transaction_intent.try_into()?,
            ),
            notary_signature: notary_signature.0.into(),
        })
    }
}

impl TryFrom<NotarizedTransactionV2> for NativeNotarizedTransactionV2 {
    type Error = RadixEngineToolkitError;

    fn try_from(value: NotarizedTransactionV2) -> Result<Self> {
        Ok(Self {
            signed_transaction_intent: value
                .signed_transaction_intent
                .as_ref()
                .clone()
                .try_into()?,
            notary_signature: NativeNotarySignatureV2(
                value.notary_signature.try_into()?,
            ),
        })
    }
}
