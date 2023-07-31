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

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

//===============
// Manifest Hash
//===============

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct ManifestHashInput {
    pub manifest: SerializableTransactionManifest,
    pub network_id: SerializableU8,
}
#[typeshare::typeshare]
pub type ManifestHashOutput = SerializableHash;

pub struct ManifestHash;
impl<'f> Function<'f> for ManifestHash {
    type Input = ManifestHashInput;
    type Output = ManifestHashOutput;

    fn handle(
        ManifestHashInput {
            manifest,
            network_id,
        }: Self::Input,
    ) -> Result<Self::Output, crate::error::InvocationHandlingError> {
        let manifest = manifest.to_native(*network_id)?;
        let hash =
            radix_engine_toolkit_core::functions::manifest::hash(&manifest).map_err(|error| {
                InvocationHandlingError::EncodeError(debug_string(error), debug_string(manifest))
            })?;
        Ok(hash.into())
    }
}

export_function!(ManifestHash as manifest_hash);
export_jni_function!(ManifestHash as manifestHash);

//==================
// Manifest Compile
//==================

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct ManifestCompileInput {
    pub manifest: SerializableTransactionManifest,
    pub network_id: SerializableU8,
}
#[typeshare::typeshare]
pub type ManifestCompileOutput = SerializableBytes;

pub struct ManifestCompile;
impl<'f> Function<'f> for ManifestCompile {
    type Input = ManifestCompileInput;
    type Output = ManifestCompileOutput;

    fn handle(
        ManifestCompileInput {
            manifest,
            network_id,
        }: Self::Input,
    ) -> Result<Self::Output, crate::error::InvocationHandlingError> {
        let manifest = manifest.to_native(*network_id)?;
        let compile = radix_engine_toolkit_core::functions::manifest::compile(&manifest).map_err(
            |error| {
                InvocationHandlingError::EncodeError(debug_string(error), debug_string(manifest))
            },
        )?;
        Ok(compile.into())
    }
}

export_function!(ManifestCompile as manifest_compile);
export_jni_function!(ManifestCompile as manifestCompile);

//====================
// Manifest Decompile
//====================

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct ManifestDecompileInput {
    pub compiled: SerializableBytes,
    pub network_id: SerializableU8,
    pub instructions_kind: SerializableInstructionsKind,
}
#[typeshare::typeshare]
pub type ManifestDecompileOutput = SerializableTransactionManifest;

pub struct ManifestDecompile;
impl<'a> Function<'a> for ManifestDecompile {
    type Input = ManifestDecompileInput;
    type Output = ManifestDecompileOutput;

    fn handle(
        ManifestDecompileInput {
            compiled,
            network_id,
            instructions_kind,
        }: Self::Input,
    ) -> Result<Self::Output, InvocationHandlingError> {
        let manifest = radix_engine_toolkit_core::functions::manifest::decompile(&**compiled)
            .map_err(|error| {
                InvocationHandlingError::EncodeError(debug_string(error), debug_string(compiled))
            })?;

        let manifest = SerializableTransactionManifest::from_native(
            &manifest,
            *network_id,
            instructions_kind,
        )?;

        Ok(manifest)
    }
}

export_function!(ManifestDecompile as manifest_decompile);
export_jni_function!(ManifestDecompile as manifestDecompile);

//==============================
// Manifest Statically Validate
//==============================

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct ManifestStaticallyValidateInput {
    pub manifest: SerializableTransactionManifest,
    pub network_id: SerializableU8,
}

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
#[serde(tag = "kind", content = "value")]
pub enum ManifestStaticallyValidateOutput {
    Valid,
    Invalid(String),
}

pub struct ManifestStaticallyValidate;
impl<'a> Function<'a> for ManifestStaticallyValidate {
    type Input = ManifestStaticallyValidateInput;
    type Output = ManifestStaticallyValidateOutput;

    fn handle(
        ManifestStaticallyValidateInput {
            manifest,
            network_id,
        }: Self::Input,
    ) -> Result<Self::Output, InvocationHandlingError> {
        let manifest = manifest.to_native(*network_id)?;

        match radix_engine_toolkit_core::functions::manifest::statically_validate(&manifest) {
            Ok(..) => Ok(Self::Output::Valid),
            Err(error) => Ok(Self::Output::Invalid(debug_string(error))),
        }
    }
}

export_function!(ManifestStaticallyValidate as manifest_statically_validate);
export_jni_function!(ManifestStaticallyValidate as manifestStaticallyValidate);
