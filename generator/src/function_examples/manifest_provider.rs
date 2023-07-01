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

use radix_engine_common::prelude::*;
use radix_engine_toolkit::prelude::*;
use transaction::manifest::*;
use walkdir::WalkDir;

pub const NUMBER_OF_MANIFESTS: usize = 25;
pub const NUMBER_OF_MANIFESTS_DOUBLE: usize = NUMBER_OF_MANIFESTS * 2;

pub fn get_serializable_instructions() -> [SerializableInstructions; NUMBER_OF_MANIFESTS_DOUBLE] {
    let mut output = Vec::new();

    let path = "../radix-engine-toolkit/tests/manifests";
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

        let mut instructions = SerializableInstructions::Parsed(
            to_serializable_instructions(&manifest.instructions, 0xf2).unwrap(),
        );
        output.push(instructions.clone());

        instructions
            .convert_serializable_instructions_kind(SerializableInstructionsKind::String, 0xf2)
            .unwrap();

        output.push(instructions.clone());
    }

    output.try_into().unwrap()
}
