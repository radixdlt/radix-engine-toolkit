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

use radix_transactions::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

use crate::prelude::*;

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct SerializableIntent {
    pub header: SerializableTransactionHeader,
    pub manifest: SerializableTransactionManifest,
    pub message: SerializableMessage,
}

impl FromNative for SerializableIntent {
    type Native = IntentV1;
    type Error = SerializableInstructionsError;
    type Context = SerializableInstructionsKind;

    fn to_native(&self, network_id: u8) -> Result<Self::Native, Self::Error> {
        let header = self.header.clone().into();
        let TransactionManifestV1 {
            instructions,
            blobs,
        } = self.manifest.to_native(network_id)?;
        let message = self.message.clone().into();

        Ok(IntentV1 {
            header,
            instructions: InstructionsV1(Rc::new(instructions)),
            blobs: BlobsV1 {
                blobs: blobs.into_values().map(BlobV1).collect(),
            },
            message,
        })
    }

    fn from_native(
        native: &Self::Native,
        network_id: u8,
        context: Self::Context,
    ) -> Result<Self, Self::Error> {
        let instructions = SerializableInstructions::from_native(
            &native.instructions.0,
            network_id,
            context,
        )?;
        let blobs = native
            .blobs
            .blobs
            .clone()
            .into_iter()
            .map(|blob| blob.0.into())
            .collect();
        let message = native.message.clone().into();

        let manifest = SerializableTransactionManifest {
            instructions,
            blobs,
        };
        let header = native.header.clone().into();

        Ok(Self {
            manifest,
            header,
            message,
        })
    }
}
