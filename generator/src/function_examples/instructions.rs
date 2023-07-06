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

use super::manifest_provider::*;
use super::traits::HasExamples;

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
        get_serializable_instructions().map(|instructions| {
            let other_kind = match instructions {
                SerializableInstructions::Parsed(..) => SerializableInstructionsKind::String,
                SerializableInstructions::String(..) => SerializableInstructionsKind::Parsed,
            };

            Self::Input {
                instructions,
                network_id: 0xf2.into(),
                instructions_kind: other_kind,
            }
        })
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
