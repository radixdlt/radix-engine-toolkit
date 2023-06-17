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

use native_json_library::prelude::*;
use radix_engine_common::prelude::*;
use transaction::manifest::*;
use walkdir::WalkDir;

use super::traits::HasExamples;

const NUMBER_OF_MANIFESTS: usize = 24;
const NUMBER_OF_MANIFESTS_DOUBLE: usize = NUMBER_OF_MANIFESTS * 2;

impl<'f> HasExamples<'f, NUMBER_OF_MANIFESTS_DOUBLE> for InstructionsHash {
    fn example_inputs() -> [Self::Input; NUMBER_OF_MANIFESTS_DOUBLE] {
        InstructionsConvert::example_inputs().map(
            |InstructionsConvertInput {
                 instructions,
                 network_id,
                 ..
             }| Self::Input {
                instructions,
                network_id,
            },
        )
    }
}

impl<'f> HasExamples<'f, NUMBER_OF_MANIFESTS_DOUBLE> for InstructionsConvert {
    fn example_inputs() -> [Self::Input; NUMBER_OF_MANIFESTS_DOUBLE] {
        let mut inputs = Vec::new();

        let path = "../native-json-library/tests/manifests";
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
            inputs.push(Self::Input {
                instructions: instructions.clone(),
                network_id: 0xf2.into(),
                output_kind: SerializableInstructionsKind::String,
            });

            instructions
                .convert_serializable_instructions_kind(SerializableInstructionsKind::String, 0xf2)
                .unwrap();

            inputs.push(Self::Input {
                instructions: instructions.clone(),
                network_id: 0xf2.into(),
                output_kind: SerializableInstructionsKind::Parsed,
            });
        }

        let err_string = format!("NUMBER_OF_MANIFESTS should be {}", inputs.len() / 2);
        inputs.try_into().expect(&err_string)
    }
}

impl<'f> HasExamples<'f, NUMBER_OF_MANIFESTS_DOUBLE> for InstructionsCompile {
    fn example_inputs() -> [Self::Input; NUMBER_OF_MANIFESTS_DOUBLE] {
        InstructionsConvert::example_inputs().map(
            |InstructionsConvertInput {
                 instructions,
                 network_id,
                 ..
             }| Self::Input {
                instructions,
                network_id,
            },
        )
    }
}

impl<'f> HasExamples<'f, NUMBER_OF_MANIFESTS_DOUBLE> for InstructionsDecompile {
    fn example_inputs() -> [Self::Input; NUMBER_OF_MANIFESTS_DOUBLE] {
        InstructionsCompile::example_outputs().map(|output| InstructionsDecompileInput {
            compiled: output,
            instructions_kind: SerializableInstructionsKind::String,
            network_id: 0xf2.into(),
        })
    }
}

impl<'f> HasExamples<'f, NUMBER_OF_MANIFESTS_DOUBLE> for InstructionsStaticallyValidate {
    fn example_inputs() -> [Self::Input; NUMBER_OF_MANIFESTS_DOUBLE] {
        InstructionsConvert::example_inputs().map(
            |InstructionsConvertInput {
                 instructions,
                 network_id,
                 ..
             }| Self::Input {
                instructions,
                network_id,
            },
        )
    }
}

impl<'f> HasExamples<'f, NUMBER_OF_MANIFESTS_DOUBLE> for InstructionsExtractAddresses {
    fn example_inputs() -> [Self::Input; NUMBER_OF_MANIFESTS_DOUBLE] {
        InstructionsConvert::example_inputs().map(
            |InstructionsConvertInput {
                 instructions,
                 network_id,
                 ..
             }| Self::Input {
                instructions,
                network_id,
            },
        )
    }
}
