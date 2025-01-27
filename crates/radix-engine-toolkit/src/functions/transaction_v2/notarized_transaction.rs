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

use crate::models::transaction_hash::TransactionHash;

pub fn hash(
    notarized_transaction: &NotarizedTransactionV2,
) -> Result<TransactionHash, PrepareError> {
    notarized_transaction
        .prepare(&PreparationSettings::latest())
        .map(|prepared| prepared.signed_transaction_intent_hash())
        .map(|hash| {
            TransactionHash::new(
                hash,
                notarized_transaction
                    .signed_transaction_intent
                    .transaction_intent
                    .root_intent_core
                    .header
                    .network_id,
            )
        })
}

pub fn to_payload_bytes(
    notarized_transaction: &NotarizedTransactionV2,
) -> Result<Vec<u8>, EncodeError> {
    notarized_transaction.to_raw().map(|raw| raw.to_vec())
}

pub fn from_payload_bytes<T>(
    payload_bytes: T,
) -> Result<NotarizedTransactionV2, DecodeError>
where
    T: AsRef<[u8]>,
{
    NotarizedTransactionV2::from_raw(&payload_bytes.as_ref().to_vec().into())
}

pub fn statically_validate(
    notarized_transaction: &NotarizedTransactionV2,
    network_definition: &NetworkDefinition,
) -> Result<(), TransactionValidationError> {
    let validator =
        TransactionValidator::new_with_latest_config(network_definition);
    notarized_transaction
        .prepare(&PreparationSettings::latest())
        .map_err(TransactionValidationError::PrepareError)
        .and_then(|prepared| validator.validate_notarized_v2(prepared))
        .map(|_| ())
}

pub fn extract_signer_public_keys(
    notarized_transaction: &NotarizedTransactionV2,
) -> Result<IndexSet<PublicKey>, TransactionValidationError> {
    let validator =
        TransactionValidator::new_with_latest_config_network_agnostic();
    notarized_transaction
        .prepare(&PreparationSettings::latest())
        .map_err(TransactionValidationError::PrepareError)
        .and_then(|prepared| validator.validate_notarized_v2(prepared))
        .map(|validated_notarized_transaction| {
            validated_notarized_transaction
                .transaction_intent_info
                .signer_keys
                .into_iter()
                .chain(
                    validated_notarized_transaction
                        .non_root_subintents_info
                        .into_iter()
                        .flat_map(|subintent_info| subintent_info.signer_keys),
                )
                .collect()
        })
}
