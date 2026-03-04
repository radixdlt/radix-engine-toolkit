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

use crate::prelude::*;

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct SerializablePreviewTransactionV2 {
    pub transaction_intent: SerializableTransactionIntentV2,
    pub root_signer_public_keys: Vec<SerializablePublicKey>,
    pub non_root_subintent_signer_public_keys: Vec<Vec<SerializablePublicKey>>,
}

impl FromNative for SerializablePreviewTransactionV2 {
    type Native = PreviewTransactionV2;
    type Error = SerializableInstructionsError;
    type Context = ();

    fn to_native(&self, network_id: u8) -> Result<Self::Native, Self::Error> {
        let transaction_intent =
            self.transaction_intent.to_native(network_id)?;
        let root_signer_public_keys = self
            .root_signer_public_keys
            .iter()
            .cloned()
            .map(Into::into)
            .collect();
        let non_root_subintent_signer_public_keys = self
            .non_root_subintent_signer_public_keys
            .iter()
            .map(|public_keys| {
                public_keys
                    .iter()
                    .cloned()
                    .map(Into::into)
                    .collect::<Vec<_>>()
            })
            .collect();

        Ok(PreviewTransactionV2 {
            transaction_intent,
            root_signer_public_keys,
            non_root_subintent_signer_public_keys,
        })
    }

    fn from_native(
        native: &Self::Native,
        network_id: u8,
        _context: Self::Context,
    ) -> Result<Self, Self::Error> {
        let transaction_intent = SerializableTransactionIntentV2::from_native(
            &native.transaction_intent,
            network_id,
            (),
        )?;
        let root_signer_public_keys = native
            .root_signer_public_keys
            .iter()
            .cloned()
            .map(Into::into)
            .collect();
        let non_root_subintent_signer_public_keys = native
            .non_root_subintent_signer_public_keys
            .iter()
            .map(|public_keys| {
                public_keys
                    .iter()
                    .cloned()
                    .map(Into::into)
                    .collect::<Vec<_>>()
            })
            .collect();

        Ok(Self {
            transaction_intent,
            root_signer_public_keys,
            non_root_subintent_signer_public_keys,
        })
    }
}
