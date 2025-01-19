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
    pub transaction_header: TransactionHeaderV2,
    pub root_intent_core: Arc<IntentCoreV2>,
    pub non_root_subintents: Vec<Arc<SubintentV2>>,
}

#[uniffi::export]
impl TransactionIntentV2 {
    #[uniffi::constructor]
    pub fn new(
        transaction_header: TransactionHeaderV2,
        root_intent_core: Arc<IntentCoreV2>,
        non_root_subintents: Vec<Arc<SubintentV2>>,
    ) -> Arc<Self> {
        Arc::new(Self {
            transaction_header,
            root_intent_core,
            non_root_subintents,
        })
    }

    #[uniffi::constructor]
    pub fn from_payload_bytes(compiled_intent: Vec<u8>) -> Result<Arc<Self>> {
        toolkit::functions::transaction_v2::transaction_intent::from_payload_bytes(
            compiled_intent,
        )
        .map_err(RadixEngineToolkitError::from)
        .and_then(|intent| intent.try_into().map(Arc::new))
    }

    pub fn transaction_header(&self) -> TransactionHeaderV2 {
        self.transaction_header.clone()
    }

    pub fn root_intent_core(&self) -> Arc<IntentCoreV2> {
        self.root_intent_core.clone()
    }

    pub fn non_root_subintents(&self) -> Vec<Arc<SubintentV2>> {
        self.non_root_subintents.clone()
    }

    pub fn hash(&self) -> Result<Arc<TransactionHash>> {
        let transaction_intent_hash =
            engine::TransactionIntentV2::try_from(self.clone())?
                .prepare(&engine::PreparationSettings::latest())?
                .transaction_intent_hash();
        Ok(Arc::new(TransactionHash::new(
            &transaction_intent_hash,
            self.root_intent_core.header.network_id,
        )))
    }

    pub fn transaction_intent_hash(&self) -> Result<Arc<TransactionHash>> {
        self.hash()
    }

    pub fn to_payload_bytes(&self) -> Result<Vec<u8>> {
        engine::TransactionIntentV2::try_from(self.clone()).and_then(|intent| {
            toolkit::functions::transaction_v2::transaction_intent::to_payload_bytes(&intent)
                .map_err(Into::into)
        })
    }
}

impl TryFrom<engine::TransactionIntentV2> for TransactionIntentV2 {
    type Error = RadixEngineToolkitError;

    fn try_from(
        engine::TransactionIntentV2 {
            transaction_header,
            root_intent_core,
            non_root_subintents,
        }: engine::TransactionIntentV2,
    ) -> Result<Self> {
        Ok(Self {
            transaction_header: transaction_header.into(),
            root_intent_core: Arc::new(IntentCoreV2::try_from(
                root_intent_core,
            )?),
            non_root_subintents: non_root_subintents
                .0
                .into_iter()
                .map(|value| SubintentV2::try_from(value).map(Arc::new))
                .collect::<Result<_>>()?,
        })
    }
}

impl TryFrom<TransactionIntentV2> for engine::TransactionIntentV2 {
    type Error = RadixEngineToolkitError;

    fn try_from(
        TransactionIntentV2 {
            transaction_header,
            root_intent_core,
            non_root_subintents,
        }: TransactionIntentV2,
    ) -> Result<Self> {
        Ok(Self {
            transaction_header: transaction_header.try_into()?,
            root_intent_core: root_intent_core.as_ref().clone().try_into()?,
            non_root_subintents: engine::NonRootSubintentsV2(
                non_root_subintents
                    .into_iter()
                    .map(|item| item.as_ref().clone())
                    .map(|item| item.try_into())
                    .collect::<Result<_>>()?,
            ),
        })
    }
}
