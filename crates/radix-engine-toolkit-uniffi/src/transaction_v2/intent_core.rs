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
pub struct IntentCoreV2 {
    pub header: IntentHeaderV2,
    pub blobs: Vec<Vec<u8>>,
    pub message: MessageV2,
    pub children: Vec<Arc<Hash>>,
    pub instructions: Arc<InstructionsV2>,
}

#[uniffi::export]
impl IntentCoreV2 {
    #[uniffi::constructor]
    pub fn new(
        header: IntentHeaderV2,
        blobs: Vec<Vec<u8>>,
        message: MessageV2,
        children: Vec<Arc<Hash>>,
        instructions: Arc<InstructionsV2>,
    ) -> Arc<Self> {
        Arc::new(Self {
            header,
            blobs,
            message,
            children,
            instructions,
        })
    }

    #[uniffi::constructor]
    pub fn from_payload_bytes(compiled_intent: Vec<u8>) -> Result<Arc<Self>> {
        core_transaction_v2_intent_core_from_payload_bytes(compiled_intent)
            .map_err(RadixEngineToolkitError::from)
            .and_then(|intent| intent.try_into().map(Arc::new))
    }

    pub fn header(&self) -> IntentHeaderV2 {
        self.header.clone()
    }

    pub fn blobs(&self) -> Vec<Vec<u8>> {
        self.blobs.clone()
    }

    pub fn message(&self) -> MessageV2 {
        self.message.clone()
    }

    pub fn children(&self) -> Vec<Arc<Hash>> {
        self.children.clone()
    }

    pub fn instructions(&self) -> Arc<InstructionsV2> {
        self.instructions.clone()
    }

    pub fn hash(&self) -> Result<Arc<TransactionHash>> {
        NativeIntentCoreV2::try_from(self.clone()).and_then(|intent| {
            core_transaction_v2_intent_core_hash(&intent)
                .map_err(Into::into)
                .map(|hash| {
                    let intent_hash = NativeIntentHash(hash);
                    Arc::new(TransactionHash::new(
                        &intent_hash,
                        self.header.network_id,
                    ))
                })
        })
    }

    pub fn intent_hash(&self) -> Result<Arc<TransactionHash>> {
        self.hash()
    }

    pub fn to_payload_bytes(&self) -> Result<Vec<u8>> {
        NativeIntentCoreV2::try_from(self.clone()).and_then(|intent| {
            core_transaction_v2_intent_core_to_payload_bytes(&intent)
                .map_err(Into::into)
        })
    }
}

impl TryFrom<NativeIntentCoreV2> for IntentCoreV2 {
    type Error = RadixEngineToolkitError;

    fn try_from(
        NativeIntentCoreV2 {
            blobs,
            header,
            instructions,
            message,
            children,
        }: NativeIntentCoreV2,
    ) -> Result<Self> {
        Ok(Self {
            instructions: Arc::new(InstructionsV2(
                instructions.0.as_ref().clone(),
                header.network_id,
            )),
            header: IntentHeaderV2::from(header),
            blobs: blobs.blobs.into_iter().map(|value| value.0).collect(),
            message: MessageV2::from(message),
            children: children
                .children
                .into_iter()
                .map(|value| Arc::new(Hash(value.hash.0)))
                .collect(),
        })
    }
}

impl TryFrom<IntentCoreV2> for NativeIntentCoreV2 {
    type Error = RadixEngineToolkitError;

    fn try_from(
        IntentCoreV2 {
            header,
            blobs,
            message,
            children,
            instructions,
        }: IntentCoreV2,
    ) -> Result<Self> {
        Ok(Self {
            header: header.try_into()?,
            blobs: NativeBlobsV1 {
                blobs: blobs.into_iter().map(NativeBlobV1).collect(),
            },
            message: NativeMessageV2::try_from(message)?,
            children: NativeChildIntentsV2 {
                children: children
                    .into_iter()
                    .map(|value| NativeSubintentHash(value.0))
                    .map(|hash| NativeChildSubintent { hash })
                    .collect(),
            },
            instructions: NativeInstructionsV2(std::rc::Rc::new(
                instructions.as_ref().0.to_vec(),
            )),
        })
    }
}
