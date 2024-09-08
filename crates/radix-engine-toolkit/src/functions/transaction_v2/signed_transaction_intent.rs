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

use radix_transactions::model::*;
use sbor::*;
use scrypto::prelude::*;

use crate::models::transaction_hash::TransactionHash;

pub fn hash(
    signed_transaction_intent: &SignedTransactionIntentV2,
) -> Result<TransactionHash, PrepareError> {
    signed_transaction_intent
        .prepare()
        .map(|prepared| prepared.signed_transaction_intent_hash())
        .map(|hash| {
            TransactionHash::new(
                hash,
                signed_transaction_intent
                    .root_intent
                    .root_intent_core
                    .header
                    .network_id,
            )
        })
}

pub fn to_payload_bytes(
    signed_transaction_intent: &SignedTransactionIntentV2,
) -> Result<Vec<u8>, EncodeError> {
    signed_transaction_intent.to_payload_bytes()
}

pub fn from_payload_bytes<T>(
    payload_bytes: T,
) -> Result<SignedTransactionIntentV2, DecodeError>
where
    T: AsRef<[u8]>,
{
    SignedTransactionIntentV2::from_payload_bytes(payload_bytes.as_ref())
}
