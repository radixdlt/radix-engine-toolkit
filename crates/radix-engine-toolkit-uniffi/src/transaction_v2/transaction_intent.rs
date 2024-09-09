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
pub struct TransactionIntentV2 {
    pub root_header: TransactionHeaderV2,
    pub root_intent_core: Arc<IntentCoreV2>,
    pub subintents: Vec<Arc<IntentCoreV2>>,
}

#[uniffi::export]
impl TransactionIntentV2 {
    #[uniffi::constructor]
    pub fn new(
        root_header: TransactionHeaderV2,
        root_intent_core: Arc<IntentCoreV2>,
        subintents: Vec<Arc<IntentCoreV2>>,
    ) -> Arc<Self> {
        Arc::new(Self {
            root_header,
            root_intent_core,
            subintents,
        })
    }

    #[uniffi::constructor]
    pub fn from_payload_bytes(compiled_intent: Vec<u8>) -> Result<Arc<Self>> {
        core_transaction_v2_transaction_intent_from_payload_bytes(
            compiled_intent,
        )
        .map_err(RadixEngineToolkitError::from)
        .and_then(|intent| intent.try_into().map(Arc::new))
    }

    pub fn root_header(&self) -> TransactionHeaderV2 {
        self.root_header.clone()
    }

    pub fn root_intent_core(&self) -> Arc<IntentCoreV2> {
        self.root_intent_core.clone()
    }

    pub fn subintents(&self) -> Vec<Arc<IntentCoreV2>> {
        self.subintents.clone()
    }

    pub fn hash(&self) -> Result<Arc<TransactionHash>> {
        NativeTransactionIntentV2::try_from(self.clone()).and_then(|intent| {
            core_transaction_v2_transaction_intent_hash(&intent)
                .map_err(Into::into)
                .map(|hash| {
                    let intent_hash = NativeIntentHash(hash.hash);
                    Arc::new(TransactionHash::new(
                        &intent_hash,
                        self.root_intent_core.header.network_id,
                    ))
                })
        })
    }

    pub fn intent_hash(&self) -> Result<Arc<TransactionHash>> {
        self.hash()
    }

    pub fn to_payload_bytes(&self) -> Result<Vec<u8>> {
        NativeTransactionIntentV2::try_from(self.clone()).and_then(|intent| {
            core_transaction_v2_transaction_intent_to_payload_bytes(&intent)
                .map_err(Into::into)
        })
    }
}

impl TryFrom<NativeTransactionIntentV2> for TransactionIntentV2 {
    type Error = RadixEngineToolkitError;

    fn try_from(
        NativeTransactionIntentV2 {
            root_header,
            root_intent_core,
            subintents,
        }: NativeTransactionIntentV2,
    ) -> Result<Self> {
        Ok(Self {
            root_header: root_header.into(),
            root_intent_core: Arc::new(IntentCoreV2::try_from(
                root_intent_core,
            )?),
            subintents: subintents
                .0
                .into_iter()
                .map(|value| {
                    IntentCoreV2::try_from(value.intent_core).map(Arc::new)
                })
                .collect::<Result<_>>()?,
        })
    }
}

impl TryFrom<TransactionIntentV2> for NativeTransactionIntentV2 {
    type Error = RadixEngineToolkitError;

    fn try_from(
        TransactionIntentV2 {
            root_header,
            root_intent_core,
            subintents,
        }: TransactionIntentV2,
    ) -> Result<Self> {
        Ok(Self {
            root_header: root_header.try_into()?,
            root_intent_core: root_intent_core.as_ref().clone().try_into()?,
            subintents: NativeSubintentsV2(
                subintents
                    .into_iter()
                    .map(|item| item.as_ref().clone())
                    .map(|item| {
                        item.try_into()
                            .map(|item| NativeSubintentV2 { intent_core: item })
                    })
                    .collect::<Result<_>>()?,
            ),
        })
    }
}
