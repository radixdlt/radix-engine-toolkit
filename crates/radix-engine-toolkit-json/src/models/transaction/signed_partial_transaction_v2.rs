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
pub struct SerializableSignedPartialTransactionV2 {
    pub partial_transaction: SerializablePartialTransactionV2,
    pub root_subintent_signatures: Vec<SerializableSignatureWithPublicKey>,
    pub non_root_subintent_signatures:
        Vec<Vec<SerializableSignatureWithPublicKey>>,
}

impl FromNative for SerializableSignedPartialTransactionV2 {
    type Native = SignedPartialTransactionV2;
    type Error = SerializableInstructionsError;
    type Context = ();

    fn to_native(&self, network_id: u8) -> Result<Self::Native, Self::Error> {
        let partial_transaction =
            self.partial_transaction.to_native(network_id)?;

        let root_subintent_signatures = IntentSignaturesV2 {
            signatures: self
                .root_subintent_signatures
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

        Ok(SignedPartialTransactionV2 {
            partial_transaction,
            root_subintent_signatures,
            non_root_subintent_signatures,
        })
    }

    fn from_native(
        native: &Self::Native,
        network_id: u8,
        _context: Self::Context,
    ) -> Result<Self, Self::Error> {
        let partial_transaction =
            SerializablePartialTransactionV2::from_native(
                &native.partial_transaction,
                network_id,
                (),
            )?;

        let root_subintent_signatures = native
            .root_subintent_signatures
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
            partial_transaction,
            root_subintent_signatures,
            non_root_subintent_signatures,
        })
    }
}
