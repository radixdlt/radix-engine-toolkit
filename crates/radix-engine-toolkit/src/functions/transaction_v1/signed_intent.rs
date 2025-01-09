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

use radix_transactions::errors::*;
use radix_transactions::model::*;
use radix_transactions::validation::*;
use sbor::*;
use scrypto::prelude::*;

use crate::types::*;

pub fn hash(
    signed_intent: &SignedIntentV1,
) -> Result<TransactionHash, PrepareError> {
    signed_intent
        .prepare(&PreparationSettings::latest())
        .map(|prepared| prepared.signed_transaction_intent_hash())
        .map(|hash| {
            TransactionHash::new(hash, signed_intent.intent.header.network_id)
        })
}

pub fn to_payload_bytes(
    signed_intent: &SignedIntentV1,
) -> Result<Vec<u8>, EncodeError> {
    signed_intent.to_raw().map(|raw| raw.to_vec())
}

pub fn from_payload_bytes<T>(
    payload_bytes: T,
) -> Result<SignedIntentV1, DecodeError>
where
    T: AsRef<[u8]>,
{
    SignedIntentV1::from_raw(&payload_bytes.as_ref().to_vec().into())
}

pub fn statically_validate(
    signed_intent: &SignedIntentV1,
    network_definition: &NetworkDefinition,
) -> Result<(), TransactionValidationError> {
    let validator =
        TransactionValidator::new_with_latest_config(network_definition);
    signed_intent
        .prepare(&PreparationSettings::latest())
        .map_err(TransactionValidationError::PrepareError)
        .and_then(|prepared| {
            validator
                .validate_intent_v1(&prepared.intent)
                .map_err(|error| {
                    TransactionValidationError::IntentValidationError(
                        TransactionValidationErrorLocation::Unlocatable,
                        error,
                    )
                })
        })
        .map(|_| ())
}
