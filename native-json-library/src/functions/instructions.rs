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

use sbor::prelude::HashSet;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

//===================
// Instructions Hash
//===================

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct InstructionsHashInput {
    pub instructions: SerializableInstructions,
    pub network_id: SerializableU8,
}
pub type InstructionsHashOutput = SerializableHash;

pub struct InstructionsHash;
impl<'a> Function<'a> for InstructionsHash {
    type Input = InstructionsHashInput;
    type Output = InstructionsHashOutput;

    fn handle(
        InstructionsHashInput {
            instructions,
            network_id,
        }: Self::Input,
    ) -> Result<Self::Output, InvocationHandlingError> {
        let instructions = instructions.to_instructions(*network_id)?;

        let hash = radix_engine_toolkit::functions::instructions::hash(&instructions).map_err(
            |error| {
                InvocationHandlingError::EncodeError(
                    debug_string(error),
                    debug_string(instructions),
                )
            },
        )?;

        Ok(hash.into())
    }
}

export_function!(InstructionsHash as instructions_hash);
export_jni_function!(InstructionsHash as instructionsHash);

//======================
// Instructions Convert
//======================

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct InstructionsConvertInput {
    pub instructions: SerializableInstructions,
    pub network_id: SerializableU8,
    pub output_kind: SerializableInstructionsKind,
}
pub type InstructionsConvertOutput = SerializableInstructions;

pub struct InstructionsConvert;
impl<'a> Function<'a> for InstructionsConvert {
    type Input = InstructionsConvertInput;
    type Output = InstructionsConvertOutput;

    fn handle(
        Self::Input {
            mut instructions,
            network_id,
            output_kind,
        }: Self::Input,
    ) -> Result<Self::Output, crate::error::InvocationHandlingError> {
        instructions.convert_serializable_instructions_kind(output_kind, *network_id)?;
        Ok(instructions)
    }
}

export_function!(InstructionsConvert as instructions_convert);
export_jni_function!(InstructionsConvert as instructionsConvert);

//======================
// Instructions Compile
//======================

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct InstructionsCompileInput {
    pub instructions: SerializableInstructions,
    pub network_id: SerializableU8,
}
pub type InstructionsCompileOutput = SerializableBytes;

pub struct InstructionsCompile;
impl<'a> Function<'a> for InstructionsCompile {
    type Input = InstructionsCompileInput;
    type Output = InstructionsCompileOutput;

    fn handle(
        InstructionsCompileInput {
            instructions,
            network_id,
        }: Self::Input,
    ) -> Result<Self::Output, InvocationHandlingError> {
        let instructions = instructions.to_instructions(*network_id)?;

        let compiled = radix_engine_toolkit::functions::instructions::compile(&instructions)
            .map_err(|error| {
                InvocationHandlingError::EncodeError(
                    debug_string(error),
                    debug_string(instructions),
                )
            })?;

        Ok(compiled.into())
    }
}

export_function!(InstructionsCompile as instructions_compile);
export_jni_function!(InstructionsCompile as instructionsCompile);

//========================
// Instructions Decompile
//========================

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct InstructionsDecompileInput {
    pub compiled: SerializableBytes,
    pub network_id: SerializableU8,
    pub instructions_kind: SerializableInstructionsKind,
}
pub type InstructionsDecompileOutput = SerializableInstructions;

pub struct InstructionsDecompile;
impl<'a> Function<'a> for InstructionsDecompile {
    type Input = InstructionsDecompileInput;
    type Output = InstructionsDecompileOutput;

    fn handle(
        InstructionsDecompileInput {
            compiled,
            network_id,
            instructions_kind,
        }: Self::Input,
    ) -> Result<Self::Output, InvocationHandlingError> {
        let instructions = radix_engine_toolkit::functions::instructions::decompile(&**compiled)
            .map_err(|error| {
                InvocationHandlingError::EncodeError(debug_string(error), debug_string(compiled))
            })?;

        let instructions =
            SerializableInstructions::new(&instructions, instructions_kind, *network_id)?;

        Ok(instructions)
    }
}

export_function!(InstructionsDecompile as instructions_decompile);
export_jni_function!(InstructionsDecompile as instructionsDecompile);

//==================================
// Instructions Statically Validate
//==================================

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct InstructionsStaticallyValidateInput {
    pub instructions: SerializableInstructions,
    pub network_id: SerializableU8,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
#[serde(tag = "kind", content = "value")]
pub enum InstructionsStaticallyValidateOutput {
    Valid,
    Invalid(String),
}

pub struct InstructionsStaticallyValidate;
impl<'a> Function<'a> for InstructionsStaticallyValidate {
    type Input = InstructionsStaticallyValidateInput;
    type Output = InstructionsStaticallyValidateOutput;

    fn handle(
        InstructionsStaticallyValidateInput {
            instructions,
            network_id,
        }: Self::Input,
    ) -> Result<Self::Output, InvocationHandlingError> {
        let instructions = instructions.to_instructions(*network_id)?;

        match radix_engine_toolkit::functions::instructions::statically_validate(&instructions) {
            Ok(..) => Ok(Self::Output::Valid),
            Err(error) => Ok(Self::Output::Invalid(debug_string(error))),
        }
    }
}

export_function!(InstructionsStaticallyValidate as instructions_statically_validate);
export_jni_function!(InstructionsStaticallyValidate as instructionsStaticallyValidate);

//================================
// Instructions Extract Addresses
//================================

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct InstructionsExtractAddressesInput {
    pub instructions: SerializableInstructions,
    pub network_id: SerializableU8,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct InstructionsExtractAddressesOutput {
    pub addresses: HashSet<SerializableNodeId>,
    pub named_addresses: HashSet<SerializableU32>,
}

pub struct InstructionsExtractAddresses;
impl<'a> Function<'a> for InstructionsExtractAddresses {
    type Input = InstructionsExtractAddressesInput;
    type Output = InstructionsExtractAddressesOutput;

    fn handle(
        InstructionsExtractAddressesInput {
            instructions,
            network_id,
        }: Self::Input,
    ) -> Result<Self::Output, crate::error::InvocationHandlingError> {
        let instructions = instructions.to_instructions(*network_id)?;

        let (addresses, named_addresses) =
            radix_engine_toolkit::functions::instructions::extract_addresses(&instructions);

        Ok(Self::Output {
            addresses: addresses
                .into_iter()
                .map(|node_id| SerializableNodeId::new(node_id, *network_id))
                .collect(),
            named_addresses: named_addresses.into_iter().map(Into::into).collect(),
        })
    }
}

export_function!(InstructionsExtractAddresses as instructions_extract_addresses);
export_jni_function!(InstructionsExtractAddresses as instructionsExtractAddresses);
