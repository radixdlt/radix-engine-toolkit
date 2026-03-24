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
pub struct SerializableSignedTransactionIntentV2 {
    pub transaction_intent: SerializableTransactionIntentV2,
    pub transaction_intent_signatures: Vec<SerializableSignatureWithPublicKey>,
    pub non_root_subintent_signatures:
        Vec<Vec<SerializableSignatureWithPublicKey>>,
}

impl FromNative for SerializableSignedTransactionIntentV2 {
    type Native = SignedTransactionIntentV2;
    type Error = SerializableInstructionsError;
    type Context = ();

    fn to_native(&self, network_id: u8) -> Result<Self::Native, Self::Error> {
        let transaction_intent =
            self.transaction_intent.to_native(network_id)?;

        let transaction_intent_signatures = IntentSignaturesV2 {
            signatures: self
                .transaction_intent_signatures
                .iter()
                .map(|s| IntentSignatureV1(s.clone().into()))
                .collect(),
        };

        let non_root_subintent_signatures = NonRootSubintentSignaturesV2 {
            by_subintent: self
                .non_root_subintent_signatures
                .iter()
                .map(|sigs| IntentSignaturesV2 {
                    signatures: sigs
                        .iter()
                        .map(|s| IntentSignatureV1(s.clone().into()))
                        .collect(),
                })
                .collect(),
        };

        Ok(SignedTransactionIntentV2 {
            transaction_intent,
            transaction_intent_signatures,
            non_root_subintent_signatures,
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

        let transaction_intent_signatures = native
            .transaction_intent_signatures
            .signatures
            .iter()
            .map(|s| s.0.into())
            .collect();

        let non_root_subintent_signatures = native
            .non_root_subintent_signatures
            .by_subintent
            .iter()
            .map(|sigs| sigs.signatures.iter().map(|s| s.0.into()).collect())
            .collect();

        Ok(Self {
            transaction_intent,
            transaction_intent_signatures,
            non_root_subintent_signatures,
        })
    }
}
