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
pub struct Intent {
    pub header: TransactionHeader,
    pub manifest: Arc<TransactionManifest>,
}

#[uniffi::export]
impl Intent {
    #[uniffi::constructor]
    pub fn new(header: TransactionHeader, manifest: Arc<TransactionManifest>) -> Arc<Self> {
        Arc::new(Self { header, manifest })
    }

    #[uniffi::constructor]
    pub fn decompile(compiled_intent: Vec<u8>) -> Result<Arc<Self>> {
        core_intent_decompile(compiled_intent)
            .map(|intent| Arc::new(intent.into()))
            .map_err(Into::into)
    }

    pub fn header(&self) -> TransactionHeader {
        self.header.clone()
    }

    pub fn manifest(&self) -> Arc<TransactionManifest> {
        self.manifest.clone()
    }

    pub fn hash(&self) -> Result<Arc<Hash>> {
        NativeIntent::try_from(self.clone()).and_then(|intent| {
            core_intent_hash(&intent)
                .map_err(Into::into)
                .map(|hash| Arc::new(Hash(hash)))
        })
    }

    pub fn intent_hash(&self) -> Result<Arc<Hash>> {
        self.hash()
    }

    pub fn compile(&self) -> Result<Vec<u8>> {
        NativeIntent::try_from(self.clone())
            .and_then(|intent| core_intent_compile(&intent).map_err(Into::into))
    }

    pub fn statically_validate(&self, validation_config: Arc<ValidationConfig>) -> Result<()> {
        core_intent_statically_validate(
            &self.clone().try_into()?,
            validation_config.as_ref().clone().into(),
        )
        .map_err(Into::into)
    }
}

impl From<NativeIntent> for Intent {
    fn from(
        NativeIntent {
            blobs,
            header,
            instructions,
            ..
        }: NativeIntent,
    ) -> Self {
        let blobs = blobs.blobs;
        let instructions = instructions.0;
        let manifest = NativeTransactionManifest {
            instructions,
            blobs: blobs
                .iter()
                .map(|blob| (native_hash(&blob.0), blob.0.clone()))
                .collect::<BTreeMap<_, _>>(),
        };

        Self {
            manifest: Arc::new(TransactionManifest::from_native(
                &manifest,
                header.network_id,
            )),
            header: header.into(),
        }
    }
}

impl TryFrom<Intent> for NativeIntent {
    type Error = RadixEngineToolkitError;

    fn try_from(value: Intent) -> Result<Self> {
        let blobs = NativeBlobs {
            blobs: value
                .manifest
                .blobs
                .iter()
                .cloned()
                .map(NativeBlob)
                .collect(),
        };
        let instructions = NativeInstructions(value.manifest.instructions.0.clone());
        let header = value.header.try_into()?;

        Ok(Self {
            blobs,
            header,
            instructions,
            attachments: transaction::prelude::AttachmentsV1 {},
        })
    }
}
