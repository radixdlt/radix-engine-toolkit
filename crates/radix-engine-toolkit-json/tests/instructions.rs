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

#![allow(dead_code)]
#![allow(clippy::expect_fun_call)]

use radix_engine_common::prelude::*;
use radix_engine_toolkit_json::models::transaction::instruction::*;
use transaction::manifest::*;
use walkdir::WalkDir;

#[test]
fn common_manifests_can_all_be_converted_to_serializable() {
    // Arrange
    let path = ".";
    for entry in WalkDir::new(path) {
        let path = entry.unwrap().path().canonicalize().unwrap();

        if path.extension().and_then(|str| str.to_str()) != Some("rtm") {
            continue;
        }

        let manifest_string = std::fs::read_to_string(&path).unwrap();
        let manifest = compile(
            &manifest_string,
            &NetworkDefinition::simulator(),
            MockBlobProvider::new(),
        )
        .unwrap();

        // Act
        let serializable_instructions =
            to_serializable_instructions(&manifest.instructions, 0xf2);

        // Assert
        serializable_instructions.expect(&format!("Failed on: {:?}", path));
    }
}

#[test]
fn common_manifests_can_be_converted_to_serialized_and_back() {
    // Arrange
    let path = ".";
    for entry in WalkDir::new(path) {
        let path = entry.unwrap().path().canonicalize().unwrap();

        if path.extension().and_then(|str| str.to_str()) != Some("rtm") {
            continue;
        }

        let manifest_string = std::fs::read_to_string(&path).unwrap();
        let manifest = compile(
            &manifest_string,
            &NetworkDefinition::simulator(),
            MockBlobProvider::new(),
        )
        .unwrap();

        let serializable_instructions =
            to_serializable_instructions(&manifest.instructions, 0xf2).unwrap();

        // Act
        let instructions = to_native_instructions(&serializable_instructions);

        // Assert
        instructions.expect(&format!("Failed on: {:?}", path));
    }
}

#[test]
#[ignore = "This test fails because of the issues with the IndexMap. Will fix this in the Scrypto repo."]
fn common_manifests_can_be_converted_to_serialized_and_back_and_are_equal() {
    // Arrange
    let path = ".";
    for entry in WalkDir::new(path) {
        let path = entry.unwrap().path().canonicalize().unwrap();

        if path.extension().and_then(|str| str.to_str()) != Some("rtm") {
            continue;
        }

        let manifest_string = std::fs::read_to_string(&path).unwrap();
        let manifest = compile(
            &manifest_string,
            &NetworkDefinition::simulator(),
            MockBlobProvider::new(),
        )
        .unwrap();

        let serializable_instructions =
            to_serializable_instructions(&manifest.instructions, 0xf2).unwrap();

        // Act
        let instructions =
            to_native_instructions(&serializable_instructions).unwrap();

        // Assert
        assert_eq!(instructions, manifest.instructions, "{path:?}")
    }
}
