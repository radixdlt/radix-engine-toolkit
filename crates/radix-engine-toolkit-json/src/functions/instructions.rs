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

use radix_engine_common::types::EntityType;
use radix_engine_toolkit::models::node_id::TypedNodeId;
use sbor::prelude::{HashMap, HashSet};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

//===================
// Instructions Hash
//===================

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct InstructionsHashInput {
    pub instructions: SerializableInstructions,
    pub network_id: SerializableU8,
}
#[typeshare::typeshare]
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

        let hash =
            radix_engine_toolkit::functions::instructions::hash(&instructions)
                .map_err(|error| {
                    InvocationHandlingError::EncodeError(
                        debug_string(error),
                        debug_string(instructions),
                    )
                })?;

        Ok(hash.into())
    }
}

export_function!(InstructionsHash as instructions_hash);
export_jni_function!(InstructionsHash as instructionsHash);

//======================
// Instructions Convert
//======================

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct InstructionsConvertInput {
    pub instructions: SerializableInstructions,
    pub network_id: SerializableU8,
    pub instructions_kind: SerializableInstructionsKind,
}
#[typeshare::typeshare]
pub type InstructionsConvertOutput = SerializableInstructions;

pub struct InstructionsConvert;
impl<'a> Function<'a> for InstructionsConvert {
    type Input = InstructionsConvertInput;
    type Output = InstructionsConvertOutput;

    fn handle(
        Self::Input {
            mut instructions,
            network_id,
            instructions_kind,
        }: Self::Input,
    ) -> Result<Self::Output, crate::error::InvocationHandlingError> {
        instructions.convert_serializable_instructions_kind(
            instructions_kind,
            *network_id,
        )?;
        Ok(instructions)
    }
}

export_function!(InstructionsConvert as instructions_convert);
export_jni_function!(InstructionsConvert as instructionsConvert);

//======================
// Instructions Compile
//======================

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct InstructionsCompileInput {
    pub instructions: SerializableInstructions,
    pub network_id: SerializableU8,
}
#[typeshare::typeshare]
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

        let compiled = radix_engine_toolkit::functions::instructions::compile(
            &instructions,
        )
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

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct InstructionsDecompileInput {
    pub compiled: SerializableBytes,
    pub network_id: SerializableU8,
    pub instructions_kind: SerializableInstructionsKind,
}
#[typeshare::typeshare]
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
        let instructions =
            radix_engine_toolkit::functions::instructions::decompile(
                &**compiled,
            )
            .map_err(|error| {
                InvocationHandlingError::EncodeError(
                    debug_string(error),
                    debug_string(compiled),
                )
            })?;

        let instructions = SerializableInstructions::new(
            &instructions,
            instructions_kind,
            *network_id,
        )?;

        Ok(instructions)
    }
}

export_function!(InstructionsDecompile as instructions_decompile);
export_jni_function!(InstructionsDecompile as instructionsDecompile);

//==================================
// Instructions Statically Validate
//==================================

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct InstructionsStaticallyValidateInput {
    pub instructions: SerializableInstructions,
    pub network_id: SerializableU8,
}

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
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

        match radix_engine_toolkit::functions::instructions::statically_validate(
            &instructions,
        ) {
            Ok(..) => Ok(Self::Output::Valid),
            Err(error) => Ok(Self::Output::Invalid(debug_string(error))),
        }
    }
}

export_function!(
    InstructionsStaticallyValidate as instructions_statically_validate
);
export_jni_function!(
    InstructionsStaticallyValidate as instructionsStaticallyValidate
);

//================================
// Instructions Extract Addresses
//================================

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct InstructionsExtractAddressesInput {
    pub instructions: SerializableInstructions,
    pub network_id: SerializableU8,
}

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct InstructionsExtractAddressesOutput {
    pub addresses: HashMap<SerializableEntityType, Vec<SerializableNodeId>>,
    #[typeshare(serialized_as = "Vec<SerializableU32>")]
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
            radix_engine_toolkit::functions::instructions::extract_addresses(
                &instructions,
            );

        let addresses = transform_addresses_set_to_map(addresses, *network_id);

        Ok(Self::Output {
            addresses,
            named_addresses: named_addresses
                .into_iter()
                .map(Into::into)
                .collect(),
        })
    }
}

export_function!(
    InstructionsExtractAddresses as instructions_extract_addresses
);
export_jni_function!(
    InstructionsExtractAddresses as instructionsExtractAddresses
);

#[typeshare::typeshare]
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    JsonSchema,
)]
pub enum SerializableEntityType {
    GlobalPackage,
    GlobalConsensusManager,
    GlobalValidator,
    GlobalTransactionTracker,
    GlobalGenericComponent,
    GlobalAccount,
    GlobalIdentity,
    GlobalAccessController,
    GlobalOneResourcePool,
    GlobalTwoResourcePool,
    GlobalMultiResourcePool,
    GlobalVirtualSecp256k1Account,
    GlobalVirtualSecp256k1Identity,
    GlobalVirtualEd25519Account,
    GlobalVirtualEd25519Identity,
    GlobalFungibleResourceManager,
    InternalFungibleVault,
    GlobalNonFungibleResourceManager,
    InternalNonFungibleVault,
    InternalGenericComponent,
    InternalKeyValueStore,
}

impl SerializableEntityType {
    pub fn all() -> Vec<SerializableEntityType> {
        vec![
            Self::GlobalPackage,
            Self::GlobalConsensusManager,
            Self::GlobalValidator,
            Self::GlobalTransactionTracker,
            Self::GlobalGenericComponent,
            Self::GlobalAccount,
            Self::GlobalIdentity,
            Self::GlobalAccessController,
            Self::GlobalOneResourcePool,
            Self::GlobalTwoResourcePool,
            Self::GlobalMultiResourcePool,
            Self::GlobalVirtualSecp256k1Account,
            Self::GlobalVirtualSecp256k1Identity,
            Self::GlobalVirtualEd25519Account,
            Self::GlobalVirtualEd25519Identity,
            Self::GlobalFungibleResourceManager,
            Self::InternalFungibleVault,
            Self::GlobalNonFungibleResourceManager,
            Self::InternalNonFungibleVault,
            Self::InternalGenericComponent,
            Self::InternalKeyValueStore,
        ]
    }
}

impl From<EntityType> for SerializableEntityType {
    fn from(value: EntityType) -> Self {
        match value {
            EntityType::GlobalPackage => Self::GlobalPackage,
            EntityType::GlobalConsensusManager => Self::GlobalConsensusManager,
            EntityType::GlobalValidator => Self::GlobalValidator,
            EntityType::GlobalTransactionTracker => {
                Self::GlobalTransactionTracker
            }
            EntityType::GlobalGenericComponent => Self::GlobalGenericComponent,
            EntityType::GlobalAccount => Self::GlobalAccount,
            EntityType::GlobalIdentity => Self::GlobalIdentity,
            EntityType::GlobalAccessController => Self::GlobalAccessController,
            EntityType::GlobalOneResourcePool => Self::GlobalOneResourcePool,
            EntityType::GlobalTwoResourcePool => Self::GlobalTwoResourcePool,
            EntityType::GlobalMultiResourcePool => {
                Self::GlobalMultiResourcePool
            }
            EntityType::GlobalVirtualSecp256k1Account => {
                Self::GlobalVirtualSecp256k1Account
            }
            EntityType::GlobalVirtualSecp256k1Identity => {
                Self::GlobalVirtualSecp256k1Identity
            }
            EntityType::GlobalVirtualEd25519Account => {
                Self::GlobalVirtualEd25519Account
            }
            EntityType::GlobalVirtualEd25519Identity => {
                Self::GlobalVirtualEd25519Identity
            }
            EntityType::GlobalFungibleResourceManager => {
                Self::GlobalFungibleResourceManager
            }
            EntityType::InternalFungibleVault => Self::InternalFungibleVault,
            EntityType::GlobalNonFungibleResourceManager => {
                Self::GlobalNonFungibleResourceManager
            }
            EntityType::InternalNonFungibleVault => {
                Self::InternalNonFungibleVault
            }
            EntityType::InternalGenericComponent => {
                Self::InternalGenericComponent
            }
            EntityType::InternalKeyValueStore => Self::InternalKeyValueStore,
        }
    }
}

impl From<SerializableEntityType> for EntityType {
    fn from(value: SerializableEntityType) -> Self {
        match value {
            SerializableEntityType::GlobalPackage => Self::GlobalPackage,
            SerializableEntityType::GlobalConsensusManager => {
                Self::GlobalConsensusManager
            }
            SerializableEntityType::GlobalValidator => Self::GlobalValidator,
            SerializableEntityType::GlobalTransactionTracker => {
                Self::GlobalTransactionTracker
            }
            SerializableEntityType::GlobalGenericComponent => {
                Self::GlobalGenericComponent
            }
            SerializableEntityType::GlobalAccount => Self::GlobalAccount,
            SerializableEntityType::GlobalIdentity => Self::GlobalIdentity,
            SerializableEntityType::GlobalAccessController => {
                Self::GlobalAccessController
            }
            SerializableEntityType::GlobalOneResourcePool => {
                Self::GlobalOneResourcePool
            }
            SerializableEntityType::GlobalTwoResourcePool => {
                Self::GlobalTwoResourcePool
            }
            SerializableEntityType::GlobalMultiResourcePool => {
                Self::GlobalMultiResourcePool
            }
            SerializableEntityType::GlobalVirtualSecp256k1Account => {
                Self::GlobalVirtualSecp256k1Account
            }
            SerializableEntityType::GlobalVirtualSecp256k1Identity => {
                Self::GlobalVirtualSecp256k1Identity
            }
            SerializableEntityType::GlobalVirtualEd25519Account => {
                Self::GlobalVirtualEd25519Account
            }
            SerializableEntityType::GlobalVirtualEd25519Identity => {
                Self::GlobalVirtualEd25519Identity
            }
            SerializableEntityType::GlobalFungibleResourceManager => {
                Self::GlobalFungibleResourceManager
            }
            SerializableEntityType::InternalFungibleVault => {
                Self::InternalFungibleVault
            }
            SerializableEntityType::GlobalNonFungibleResourceManager => {
                Self::GlobalNonFungibleResourceManager
            }
            SerializableEntityType::InternalNonFungibleVault => {
                Self::InternalNonFungibleVault
            }
            SerializableEntityType::InternalGenericComponent => {
                Self::InternalGenericComponent
            }
            SerializableEntityType::InternalKeyValueStore => {
                Self::InternalKeyValueStore
            }
        }
    }
}

pub(crate) fn transform_addresses_set_to_map(
    addresses: HashSet<TypedNodeId>,
    network_id: u8,
) -> HashMap<SerializableEntityType, Vec<SerializableNodeId>> {
    let mut addresses_map =
        HashMap::<SerializableEntityType, Vec<SerializableNodeId>>::new();
    for node_id in addresses.into_iter() {
        addresses_map
            .entry(node_id.entity_type().into())
            .or_default()
            .push(SerializableNodeId::new(*node_id.as_node_id(), network_id))
    }
    for entity_type in SerializableEntityType::all() {
        addresses_map.entry(entity_type).or_default();
    }
    addresses_map
}
