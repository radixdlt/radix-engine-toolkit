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
pub struct SerializableNotarizedTransactionV2 {
    pub signed_transaction_intent: SerializableSignedTransactionIntentV2,
    pub notary_signature: SerializableSignature,
}

impl FromNative for SerializableNotarizedTransactionV2 {
    type Native = NotarizedTransactionV2;
    type Error = SerializableInstructionsError;
    type Context = ();

    fn to_native(&self, network_id: u8) -> Result<Self::Native, Self::Error> {
        let signed_transaction_intent =
            self.signed_transaction_intent.to_native(network_id)?;
        let notary_signature =
            NotarySignatureV2(self.notary_signature.clone().into());

        Ok(NotarizedTransactionV2 {
            signed_transaction_intent,
            notary_signature,
        })
    }

    fn from_native(
        native: &Self::Native,
        network_id: u8,
        _context: Self::Context,
    ) -> Result<Self, Self::Error> {
        let signed_transaction_intent =
            SerializableSignedTransactionIntentV2::from_native(
                &native.signed_transaction_intent,
                network_id,
                (),
            )?;
        let notary_signature = native.notary_signature.0.into();

        Ok(Self {
            signed_transaction_intent,
            notary_signature,
        })
    }
}
