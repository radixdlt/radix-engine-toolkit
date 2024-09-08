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

use radix_transactions::validation::ValidationConfig;
mod test_data;

#[test]
fn notarized_transaction_hash_can_be_obtained() {
    // Arrange
    let transaction = test_data::notarized_transaction();

    // Act
    let hash =
        radix_engine_toolkit::functions::transaction_v1::notarized_transaction::hash(
            &transaction,
        );

    // Assert
    assert!(hash.is_ok())
}

#[test]
fn notarized_transaction_can_be_compiled() {
    // Arrange
    let transaction = test_data::notarized_transaction();

    // Act
    let compiled =
        radix_engine_toolkit::functions::transaction_v1::notarized_transaction::to_payload_bytes(
            &transaction,
        );

    // Assert
    assert!(compiled.is_ok())
}

#[test]
fn notarized_transaction_can_be_compiled_and_later_decompiled() {
    // Arrange
    let transaction = test_data::notarized_transaction();
    let compiled =
        radix_engine_toolkit::functions::transaction_v1::notarized_transaction::to_payload_bytes(
            &transaction,
        )
        .unwrap();

    // Act
    let decompiled =
        radix_engine_toolkit::functions::transaction_v1::notarized_transaction::from_payload_bytes(
            compiled,
        );

    // Assert
    assert!(decompiled.is_ok());
    assert_eq!(decompiled, Ok(transaction))
}

#[test]
fn notarized_transaction_can_be_statically_validated() {
    // Arrange
    let transaction = test_data::notarized_transaction();
    let validation_config = ValidationConfig::default(0x01);

    // Act
    let validation_result =
        radix_engine_toolkit::functions::transaction_v1::notarized_transaction::statically_validate(
            &transaction,
            validation_config,
        );

    // Assert
    assert!(validation_result.is_ok())
}
