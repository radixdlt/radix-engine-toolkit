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
pub struct IntentV1 {
    pub header: TransactionHeaderV1,
    pub manifest: Arc<TransactionManifestV1>,
    pub message: MessageV1,
}

#[uniffi::export]
impl IntentV1 {
    #[uniffi::constructor]
    pub fn new(
        header: TransactionHeaderV1,
        manifest: Arc<TransactionManifestV1>,
        message: MessageV1,
    ) -> Arc<Self> {
        Arc::new(Self {
            header,
            manifest,
            message,
        })
    }

    #[uniffi::constructor]
    pub fn from_payload_bytes(compiled_intent: Vec<u8>) -> Result<Arc<Self>> {
        toolkit::functions::transaction_v1::intent::from_payload_bytes(
            compiled_intent,
        )
        .map(|intent| Arc::new(intent.into()))
        .map_err(Into::into)
    }

    pub fn header(&self) -> TransactionHeaderV1 {
        self.header.clone()
    }

    pub fn manifest(&self) -> Arc<TransactionManifestV1> {
        self.manifest.clone()
    }

    pub fn message(&self) -> MessageV1 {
        self.message.clone()
    }

    pub fn hash(&self) -> Result<Arc<TransactionHash>> {
        engine::IntentV1::try_from(self.clone()).and_then(|intent| {
            toolkit::functions::transaction_v1::intent::hash(&intent)
                .map_err(Into::into)
                .map(|hash| {
                    let intent_hash = engine::TransactionIntentHash(hash.hash);
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
        engine::IntentV1::try_from(self.clone()).and_then(|intent| {
            toolkit::functions::transaction_v1::intent::to_payload_bytes(
                &intent,
            )
            .map_err(Into::into)
        })
    }

    pub fn statically_validate(&self, network_id: u8) -> Result<()> {
        toolkit::functions::transaction_v1::intent::statically_validate(
            &self.clone().try_into()?,
            &engine::NetworkDefinition::from_network_id(network_id),
        )
        .map_err(Into::into)
    }
}

impl From<engine::IntentV1> for IntentV1 {
    fn from(
        engine::IntentV1 {
            blobs,
            header,
            instructions,
            message,
        }: engine::IntentV1,
    ) -> Self {
        let blobs = blobs.blobs;
        let instructions = instructions.0;
        let manifest = engine::TransactionManifestV1 {
            instructions,
            blobs: blobs
                .iter()
                .map(|blob| (engine::hash(&blob.0), blob.0.clone()))
                .collect::<IndexMap<_, _>>(),
            object_names: Default::default(),
        };

        Self {
            manifest: Arc::new(TransactionManifestV1::from_native(
                &manifest,
                header.network_id,
            )),
            header: header.into(),
            message: message.into(),
        }
    }
}

impl TryFrom<IntentV1> for engine::IntentV1 {
    type Error = RadixEngineToolkitError;

    fn try_from(value: IntentV1) -> Result<Self> {
        let blobs = engine::BlobsV1 {
            blobs: value
                .manifest
                .blobs
                .iter()
                .cloned()
                .map(engine::BlobV1)
                .collect(),
        };
        let instructions =
            engine::InstructionsV1(value.manifest.instructions.0.clone());
        let header = value.header.try_into()?;
        let message = value.message.try_into()?;

        Ok(Self {
            blobs,
            message,
            header,
            instructions,
        })
    }
}
