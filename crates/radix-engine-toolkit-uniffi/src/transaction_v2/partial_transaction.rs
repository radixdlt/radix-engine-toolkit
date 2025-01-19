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
pub struct PartialTransactionV2 {
    pub root_subintent: Arc<SubintentV2>,
    pub non_root_subintents: Vec<Arc<SubintentV2>>,
}

#[uniffi::export]
impl PartialTransactionV2 {
    #[uniffi::constructor]
    pub fn new(
        root_subintent: Arc<SubintentV2>,
        non_root_subintents: Vec<Arc<SubintentV2>>,
    ) -> Arc<Self> {
        Arc::new(Self {
            root_subintent,
            non_root_subintents,
        })
    }

    #[uniffi::constructor]
    pub fn from_payload_bytes(compiled_intent: Vec<u8>) -> Result<Arc<Self>> {
        toolkit::functions::transaction_v2::partial_transaction::from_payload_bytes(
            compiled_intent,
        )
        .map_err(RadixEngineToolkitError::from)
        .and_then(|partial_transaction| {
            partial_transaction.try_into().map(Arc::new)
        })
    }

    pub fn root_subintent(&self) -> Arc<SubintentV2> {
        self.root_subintent.clone()
    }

    pub fn non_root_subintents(&self) -> Vec<Arc<SubintentV2>> {
        self.non_root_subintents.clone()
    }

    pub fn root_subintent_hash(&self) -> Result<Arc<TransactionHash>> {
        self.root_subintent.subintent_hash()
    }

    pub fn to_payload_bytes(&self) -> Result<Vec<u8>> {
        engine::PartialTransactionV2::try_from(self.clone()).and_then(|intent| {
            toolkit::functions::transaction_v2::partial_transaction::to_payload_bytes(&intent)
                .map_err(Into::into)
        })
    }
}

impl TryFrom<engine::PartialTransactionV2> for PartialTransactionV2 {
    type Error = RadixEngineToolkitError;

    fn try_from(
        engine::PartialTransactionV2 {
            root_subintent,
            non_root_subintents,
        }: engine::PartialTransactionV2,
    ) -> Result<Self> {
        Ok(Self {
            root_subintent: Arc::new(SubintentV2::try_from(root_subintent)?),
            non_root_subintents: non_root_subintents
                .0
                .into_iter()
                .map(|value| SubintentV2::try_from(value).map(Arc::new))
                .collect::<Result<_>>()?,
        })
    }
}

impl TryFrom<PartialTransactionV2> for engine::PartialTransactionV2 {
    type Error = RadixEngineToolkitError;

    fn try_from(
        PartialTransactionV2 {
            root_subintent,
            non_root_subintents,
        }: PartialTransactionV2,
    ) -> Result<Self> {
        Ok(Self {
            root_subintent: root_subintent.as_ref().clone().try_into()?,
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
