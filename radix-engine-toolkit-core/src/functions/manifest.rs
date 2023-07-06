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

use sbor::*;
use scrypto::prelude::*;
use transaction::errors::*;
use transaction::prelude::*;
use transaction::validation::*;

pub fn hash(manifest: &TransactionManifestV1) -> Result<Hash, EncodeError> {
    compile(manifest).map(scrypto::prelude::hash)
}

pub fn compile(manifest: &TransactionManifestV1) -> Result<Vec<u8>, EncodeError> {
    manifest_encode(manifest)
}

pub fn decompile<T>(payload_bytes: T) -> Result<TransactionManifestV1, DecodeError>
where
    T: AsRef<[u8]>,
{
    manifest_decode(payload_bytes.as_ref())
}

pub fn statically_validate(
    manifest: &TransactionManifestV1,
) -> Result<(), TransactionValidationError> {
    NotarizedTransactionValidator::validate_instructions_v1(&manifest.instructions)
}
