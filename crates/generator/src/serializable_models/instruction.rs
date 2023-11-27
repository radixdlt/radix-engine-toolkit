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

use super::traits::HasExamples;
use crate::function_examples::notarized_transaction::*;
use radix_engine_toolkit::prelude::*;
use scrypto::api::node_modules::metadata::*;
use scrypto::prelude::*;
use transaction::prelude::*;

impl<'f> HasExamples<'f> for SerializableInstruction {
    fn examples() -> Vec<Self> {
        ExamplesBuilder::new()
            .add_instructions(
                notarized_transactions()
                    .into_iter()
                    .flat_map(|item| {
                        item.signed_intent
                            .intent
                            .manifest
                            .instructions
                            .to_instructions(0xf2)
                            .unwrap()
                    })
                    .collect::<Vec<_>>(),
            )
            .build()
    }
}

#[derive(Default)]
struct ExamplesBuilder(Vec<SerializableInstruction>);
impl ExamplesBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_instructions(self, instructions: Vec<InstructionV1>) -> Self {
        self.add_serializable_instructions(
            instructions
                .into_iter()
                .map(|instruction| {
                    SerializableInstruction::from_instruction(
                        &instruction,
                        0xf2,
                    )
                    .unwrap()
                })
                .collect(),
        )
    }

    pub fn add_serializable_instructions(
        mut self,
        instructions: Vec<SerializableInstruction>,
    ) -> Self {
        self.0.extend(instructions);
        self
    }

    pub fn build(self) -> Vec<SerializableInstruction> {
        self.0
            .into_iter()
            .collect::<sbor::prelude::IndexSet<_>>()
            .into_iter()
            .collect()
    }
}

#[derive(NonFungibleData, ScryptoSbor, ManifestSbor)]
pub struct Human {
    name: String,
    age: u128,
    height: (u8, u8, u8),
}
