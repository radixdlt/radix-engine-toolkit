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
pub struct InstructionsV1(pub(crate) Vec<NativeInstructionV1>, pub(crate) u8);

#[uniffi::export]
impl InstructionsV1 {
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
        instructions: Vec<InstructionV1>,
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
        native_decompile(
            &NativeTransactionManifestV1 {
                instructions: self.0.clone(),
                blobs: Default::default(),
                object_names: Default::default(),
            },
            &network_definition,
        )
        .map_err(Into::into)
    }

    pub fn instructions_list(&self) -> Vec<InstructionV1> {
        self.0
            .iter()
            .map(|instruction| InstructionV1::from_native(instruction, self.1))
            .collect()
    }

    pub fn network_id(&self) -> u8 {
        self.1
    }
}
