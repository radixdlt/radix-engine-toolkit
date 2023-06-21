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

use std::ops::Deref;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use transaction::prelude::TransactionManifestV1;

use crate::prelude::*;

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct SerializableTransactionManifest {
    pub instructions: SerializableInstructions,
    pub blobs: Vec<SerializableBytes>,
}

impl NativeConvertible for SerializableTransactionManifest {
    type Native = TransactionManifestV1;
    type Error = SerializableInstructionsError;
    type Context = SerializableInstructionsKind;

    fn to_native(&self, network_id: u8) -> Result<Self::Native, Self::Error> {
        let instructions = self.instructions.to_instructions(network_id)?;
        let blobs = self
            .blobs
            .iter()
            .map(|value| (scrypto::prelude::hash(&**value), value.deref().clone()))
            .collect();

        Ok(TransactionManifestV1 {
            instructions,
            blobs,
        })
    }

    fn from_native(
        native: &Self::Native,
        network_id: u8,
        output_kind: Self::Context,
    ) -> Result<Self, Self::Error> {
        let instructions =
            SerializableInstructions::from_native(&native.instructions, network_id, output_kind)?;
        let blobs = native.blobs.values().cloned().map(Into::into).collect();

        Ok(Self {
            instructions,
            blobs,
        })
    }
}
