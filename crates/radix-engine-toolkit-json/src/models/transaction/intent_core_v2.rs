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

use radix_transactions::manifest::{compile_manifest, MockBlobProvider};
use radix_transactions::prelude::*;
use sbor_json::utils::network_definition_from_network_id;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct SerializableIntentCoreV2 {
    pub header: SerializableIntentHeaderV2,
    pub instructions: String,
    pub blobs: Vec<SerializableBytes>,
    pub message: SerializableMessageV2,
    pub children: Vec<SerializableHash>,
}

impl FromNative for SerializableIntentCoreV2 {
    type Native = IntentCoreV2;
    type Error = SerializableInstructionsError;
    type Context = ();

    fn to_native(&self, network_id: u8) -> Result<Self::Native, Self::Error> {
        let header: IntentHeaderV2 = self.header.clone().into();
        let network_definition = network_definition_from_network_id(network_id);

        let manifest = compile_manifest::<TransactionManifestV2>(
            &self.instructions,
            &network_definition,
            MockBlobProvider::new(),
        )?;

        let blobs = BlobsV1 {
            blobs: self.blobs.iter().map(|b| BlobV1((**b).clone())).collect(),
        };

        let children = ChildSubintentSpecifiersV2 {
            children: self
                .children
                .iter()
                .map(|h| ChildSubintentSpecifier {
                    hash: SubintentHash((**h).into()),
                })
                .collect(),
        };

        let message: MessageV2 = self.message.clone().into();

        Ok(IntentCoreV2 {
            header,
            instructions: InstructionsV2(manifest.instructions),
            blobs,
            message,
            children,
        })
    }

    fn from_native(
        native: &Self::Native,
        network_id: u8,
        _context: Self::Context,
    ) -> Result<Self, Self::Error> {
        let header: SerializableIntentHeaderV2 = native.header.clone().into();

        let network_definition = network_definition_from_network_id(network_id);
        let instructions = radix_transactions::manifest::decompile(
            &TransactionManifestV2 {
                instructions: native.instructions.0.clone(),
                blobs: Default::default(),
                object_names: Default::default(),
                children: Default::default(),
            },
            &network_definition,
        )?;

        let blobs = native
            .blobs
            .blobs
            .iter()
            .map(|b| b.0.clone().into())
            .collect();

        let children = native
            .children
            .children
            .iter()
            .map(|c| c.hash.0.into())
            .collect();

        let message: SerializableMessageV2 = native.message.clone().into();

        Ok(Self {
            header,
            instructions,
            blobs,
            message,
            children,
        })
    }
}
