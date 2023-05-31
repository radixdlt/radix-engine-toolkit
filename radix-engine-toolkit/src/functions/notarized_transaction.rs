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
use transaction::model::*;
use transaction::validation::*;

type NotarizedTransactionPayload =
    FixedEnumVariant<{ TransactionDiscriminator::V1Notarized as u8 }, NotarizedTransactionV1>;

pub fn hash(
    notarized_transaction: &NotarizedTransactionV1,
) -> Result<Hash, ConvertToPreparedError> {
    notarized_transaction
        .prepare()
        .map(|prepared| Hash(prepared.notarized_transaction_hash().0))
}

pub fn compile(notarized_transaction: &NotarizedTransactionV1) -> Result<Vec<u8>, EncodeError> {
    notarized_transaction.to_payload_bytes()
}

pub fn decompile<T>(payload_bytes: T) -> Result<NotarizedTransactionV1, DecodeError>
where
    T: AsRef<[u8]>,
{
    manifest_decode::<NotarizedTransactionPayload>(payload_bytes.as_ref())
        .map(|decompiled| decompiled.fields)
}

pub fn statically_validate(
    notarized_transaction: &NotarizedTransactionV1,
    validation_config: ValidationConfig,
) -> Result<(), TransactionValidationError> {
    let validator = NotarizedTransactionValidator::new(validation_config);
    let prepared = notarized_transaction
        .prepare()
        .map_err(|error| match error {
            ConvertToPreparedError::EncodeError(error) => {
                TransactionValidationError::EncodeError(error)
            }
            ConvertToPreparedError::PrepareError(error) => {
                TransactionValidationError::PrepareError(error)
            }
        })?;
    validator.validate(prepared).map(|_| ())
}
