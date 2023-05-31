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

use transaction::validation::ValidationConfig;
mod test_data;

#[test]
fn signed_intent_hash_can_be_obtained() {
    // Arrange
    let signed_intent = test_data::signed_intent();

    // Act
    let hash = radix_engine_toolkit::functions::signed_intent::hash(&signed_intent);

    // Assert
    assert!(hash.is_ok())
}

#[test]
fn signed_intent_can_be_compiled() {
    // Arrange
    let signed_intent = test_data::signed_intent();

    // Act
    let compiled = radix_engine_toolkit::functions::signed_intent::compile(&signed_intent);

    // Assert
    assert!(compiled.is_ok())
}

#[test]
fn signed_intent_can_be_compiled_and_later_decompiled() {
    // Arrange
    let signed_intent = test_data::signed_intent();
    let compiled = radix_engine_toolkit::functions::signed_intent::compile(&signed_intent).unwrap();

    // Act
    let decompiled = radix_engine_toolkit::functions::signed_intent::decompile(compiled);

    // Assert
    assert!(decompiled.is_ok());
    assert_eq!(decompiled, Ok(signed_intent))
}

#[test]
fn signed_intent_can_be_statically_validated() {
    // Arrange
    let signed_intent = test_data::signed_intent();
    let validation_config = ValidationConfig::default(0x01);

    // Act
    let validation_result = radix_engine_toolkit::functions::signed_intent::statically_validate(
        &signed_intent,
        validation_config,
    );

    // Assert
    assert!(validation_result.is_ok())
}
