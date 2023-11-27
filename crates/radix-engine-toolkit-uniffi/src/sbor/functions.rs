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

use sbor::WellKnownTypeId;

use crate::prelude::*;

#[uniffi::export]
pub fn sbor_decode_to_string_representation(
    bytes: Vec<u8>,
    representation: SerializationMode,
    network_id: u8,
    schema: Option<Schema>,
) -> Result<String> {
    match bytes.first().copied() {
        Some(NATIVE_SCRYPTO_SBOR_V1_PAYLOAD_PREFIX) => {
            scrypto_sbor_decode_to_string_representation(
                bytes,
                representation,
                network_id,
                schema,
            )
        }
        Some(NATIVE_MANIFEST_SBOR_V1_PAYLOAD_PREFIX) => {
            manifest_sbor_decode_to_string_representation(
                bytes,
                ManifestSborStringRepresentation::JSON {
                    value: representation,
                },
                network_id,
                schema,
            )
        }
        _ => Err(RadixEngineToolkitError::DecodeError {
            error: "Invalid Sbor payload prefix".into(),
        }),
    }
}

#[uniffi::export]
pub fn scrypto_sbor_decode_to_string_representation(
    bytes: Vec<u8>,
    representation: SerializationMode,
    network_id: u8,
    schema: Option<Schema>,
) -> Result<String> {
    let network_definition =
        core_network_definition_from_network_id(network_id);
    let bech32_encoder = NativeAddressBech32Encoder::new(&network_definition);
    let string = core_scrypto_decode_to_string_representation(
        bytes,
        representation.into(),
        &bech32_encoder,
        if let Some(schema) = schema {
            Some(schema.try_into()?)
        } else {
            None
        },
    )?;
    Ok(string)
}

#[uniffi::export]
pub fn scrypto_sbor_encode_string_representation(
    representation: ScryptoSborString,
) -> Result<Vec<u8>> {
    core_scrypto_encode_string_representation(representation.into())
        .map_err(Into::into)
}

#[uniffi::export]
pub fn manifest_sbor_decode_to_string_representation(
    bytes: Vec<u8>,
    representation: ManifestSborStringRepresentation,
    network_id: u8,
    schema: Option<Schema>,
) -> Result<String> {
    let network_definition =
        core_network_definition_from_network_id(network_id);
    let bech32_encoder = NativeAddressBech32Encoder::new(&network_definition);
    let string = core_manifest_decode_to_string_representation(
        bytes,
        representation.into(),
        &bech32_encoder,
        if let Some(schema) = schema {
            Some(schema.try_into()?)
        } else {
            None
        },
    )?;
    Ok(string)
}

#[derive(Clone, Debug, Enum)]
pub enum ManifestSborStringRepresentation {
    ManifestString,
    JSON { value: SerializationMode },
}

#[derive(Clone, Debug, Enum)]
pub enum SerializationMode {
    Programmatic,
    Natural,
}

#[derive(Clone, Debug, Enum)]
pub enum LocalTypeId {
    WellKnown { value: u8 },
    SchemaLocalIndex { value: u64 },
}

#[derive(Clone, Debug, Record)]
pub struct Schema {
    pub local_type_id: LocalTypeId,
    pub schema: Vec<u8>,
}

#[derive(Clone, Debug, Enum)]
pub enum ScryptoSborString {
    ProgrammaticJson { value: String },
}

impl From<ManifestSborStringRepresentation>
    for CoreManifestSborStringRepresentation
{
    fn from(value: ManifestSborStringRepresentation) -> Self {
        match value {
            ManifestSborStringRepresentation::ManifestString => {
                Self::ManifestString
            }
            ManifestSborStringRepresentation::JSON { value } => {
                Self::JSON(value.into())
            }
        }
    }
}

impl From<SerializationMode> for NativeSerializationMode {
    fn from(value: SerializationMode) -> Self {
        match value {
            SerializationMode::Natural => Self::Natural,
            SerializationMode::Programmatic => Self::Programmatic,
        }
    }
}

impl From<LocalTypeId> for NativeLocalTypeId {
    fn from(value: LocalTypeId) -> Self {
        match value {
            LocalTypeId::WellKnown { value } => {
                Self::WellKnown(WellKnownTypeId::of(value))
            }
            LocalTypeId::SchemaLocalIndex { value } => {
                Self::SchemaLocalIndex(value as usize)
            }
        }
    }
}

impl TryFrom<Schema>
    for (NativeLocalTypeId, NativeSchema<NativeScryptoCustomSchema>)
{
    type Error = RadixEngineToolkitError;

    fn try_from(
        Schema {
            local_type_id,
            schema,
        }: Schema,
    ) -> Result<Self> {
        let local_type_id = local_type_id.into();
        let schema = native_scrypto_decode(&schema)?;
        Ok((local_type_id, schema))
    }
}

impl From<ScryptoSborString> for CoreScryptoStringRepresentation {
    fn from(value: ScryptoSborString) -> Self {
        match value {
            ScryptoSborString::ProgrammaticJson { value } => {
                CoreScryptoStringRepresentation::ProgrammaticJson(value)
            }
        }
    }
}
