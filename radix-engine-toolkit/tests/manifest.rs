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

mod test_data;

#[test]
fn manifest_hash_can_be_obtained() {
    // Arrange
    let manifest = test_data::manifest();

    // Act
    let hash = radix_engine_toolkit::functions::manifest::hash(&manifest);

    // Assert
    assert!(hash.is_ok())
}

#[test]
fn manifest_can_be_compiled() {
    // Arrange
    let manifest = test_data::manifest();

    // Act
    let compiled = radix_engine_toolkit::functions::manifest::compile(&manifest);

    // Assert
    assert!(compiled.is_ok())
}

#[test]
fn manifest_can_be_compiled_and_later_decompiled() {
    // Arrange
    let manifest = test_data::manifest();
    let compiled = radix_engine_toolkit::functions::manifest::compile(&manifest).unwrap();

    // Act
    let decompiled = radix_engine_toolkit::functions::manifest::decompile(compiled);

    // Assert
    assert!(decompiled.is_ok());
    assert_eq!(decompiled, Ok(manifest))
}

#[test]
fn manifest_can_be_statically_validated() {
    // Arrange
    let manifest = test_data::manifest();

    // Act
    let validation_result =
        radix_engine_toolkit::functions::manifest::statically_validate(&manifest);

    // Assert
    assert!(validation_result.is_ok())
}
