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

use crate::prelude::*;

#[derive(Clone, Debug, Object)]
pub struct Instructions(pub(crate) Vec<NativeInstruction>, pub(crate) u8);

#[uniffi::export]
impl Instructions {
    #[uniffi::constructor]
    pub fn from_string(string: String, network_id: u8) -> Result<Arc<Self>> {
        let network_definition =
            core_network_definition_from_network_id(network_id);
        let blob_provider = NativeMockBlobProvider::new();
        native_compile(&string, &network_definition, blob_provider)
            .map_err(Into::into)
            .map(|manifest| Arc::new(Self(manifest.instructions, network_id)))
    }

    #[uniffi::constructor]
    pub fn from_instructions(
        instructions: Vec<Instruction>,
        network_id: u8,
    ) -> Result<Arc<Self>> {
        let instructions = instructions
            .into_iter()
            .map(|instruction| instruction.to_native())
            .collect::<Result<_>>()?;
        Ok(Arc::new(Self(instructions, network_id)))
    }

    pub fn as_str(&self) -> Result<String> {
        let network_definition =
            core_network_definition_from_network_id(self.1);
        native_decompile(&self.0, &network_definition).map_err(Into::into)
    }

    pub fn instructions_list(&self) -> Vec<Instruction> {
        self.0
            .iter()
            .map(|instruction| Instruction::from_native(instruction, self.1))
            .collect()
    }

    pub fn network_id(&self) -> u8 {
        self.1
    }
}
