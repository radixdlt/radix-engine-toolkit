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

use crate::transaction_types::*;

pub fn hash(manifest: &TransactionManifestV2) -> Result<Hash, EncodeError> {
    to_payload_bytes(manifest).map(scrypto::prelude::hash)
}

pub fn to_payload_bytes(
    manifest: &TransactionManifestV2,
) -> Result<Vec<u8>, EncodeError> {
    manifest_encode(manifest)
}

pub fn from_payload_bytes<T>(
    payload_bytes: T,
) -> Result<TransactionManifestV2, DecodeError>
where
    T: AsRef<[u8]>,
{
    manifest_decode(payload_bytes.as_ref())
}

pub fn statically_validate(
    _manifest: &TransactionManifestV2,
) -> Result<(), TransactionValidationError> {
    todo!()
}

pub fn is_enclosed(_manifest: &TransactionManifestV2) -> bool {
    todo!()
}

pub fn statically_analyze(_manifest: &TransactionManifestV2) -> StaticAnalysis {
    todo!()
}

pub fn dynamically_analyze(
    _manifest: &TransactionManifestV2,
    _receipt: &RuntimeToolkitTransactionReceipt,
) -> Result<DynamicAnalysis, TransactionTypesError> {
    todo!()
}
