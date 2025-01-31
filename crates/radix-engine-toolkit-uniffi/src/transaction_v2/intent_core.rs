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

    pub fn into_subintent(self: Arc<Self>) -> Arc<SubintentV2> {
        SubintentV2::new(self)
    }
}

impl TryFrom<engine::IntentCoreV2> for IntentCoreV2 {
    type Error = RadixEngineToolkitError;

    fn try_from(
        engine::IntentCoreV2 {
            blobs,
            header,
            instructions,
            message,
            children,
        }: engine::IntentCoreV2,
    ) -> Result<Self> {
        Ok(Self {
            instructions: Arc::new(InstructionsV2(
                instructions.0,
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

impl TryFrom<IntentCoreV2> for engine::IntentCoreV2 {
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
            blobs: engine::BlobsV1 {
                blobs: blobs.into_iter().map(engine::BlobV1).collect(),
            },
            message: engine::MessageV2::try_from(message)?,
            children: engine::ChildSubintentSpecifiersV2 {
                children: children
                    .into_iter()
                    .map(|value| engine::SubintentHash(value.0))
                    .map(|hash| engine::ChildSubintentSpecifier { hash })
                    .collect(),
            },
            instructions: engine::InstructionsV2(
                instructions.as_ref().0.to_vec(),
            ),
        })
    }
}
