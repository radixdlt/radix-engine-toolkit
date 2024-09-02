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

use radix_common::prelude::*;
use radix_engine_toolkit_common::receipt::RuntimeToolkitTransactionReceipt;
use radix_transactions::errors::*;
use radix_transactions::prelude::*;
use radix_transactions::validation::*;

use crate::transaction_types::*;

pub fn hash(manifest: &TransactionManifestV1) -> Result<Hash, EncodeError> {
    to_payload_bytes(manifest).map(scrypto::prelude::hash)
}

pub fn to_payload_bytes(
    manifest: &TransactionManifestV1,
) -> Result<Vec<u8>, EncodeError> {
    manifest_encode(manifest)
}

pub fn from_payload_bytes<T>(
    payload_bytes: T,
) -> Result<TransactionManifestV1, DecodeError>
where
    T: AsRef<[u8]>,
{
    manifest_decode(payload_bytes.as_ref())
}

pub fn statically_validate(
    manifest: &TransactionManifestV1,
) -> Result<(), TransactionValidationError> {
    NotarizedTransactionValidatorV1::validate_instructions_v1(
        &manifest.instructions,
    )
}

pub fn summary(manifest: &TransactionManifestV1) -> ManifestSummary {
    crate::transaction_types::summary(manifest)
}

pub fn execution_summary(
    manifest: &TransactionManifestV1,
    receipt: &RuntimeToolkitTransactionReceipt,
) -> Result<ExecutionSummary, TransactionTypesError> {
    crate::transaction_types::execution_summary(manifest, receipt)
}
