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

use toolkit_derive::serializable;

use crate::model::value::ast::{ManifestAstValueConversionError, ManifestAstValueKind};

/// Errors emitted during the conversion of instructions form the RET format to the native format
/// used in Scrypto.
#[serializable]
#[serde(tag = "type")]
pub enum InstructionConversionError {
    ManifestAstValueConversionError(ManifestAstValueConversionError),
    PackageSchemaResolutionError(PackageSchemaResolutionError),
    TupleConversionError { content: String },
}

impl From<PackageSchemaResolutionError> for InstructionConversionError {
    fn from(value: PackageSchemaResolutionError) -> Self {
        Self::PackageSchemaResolutionError(value)
    }
}

impl From<ManifestAstValueConversionError> for InstructionConversionError {
    fn from(value: ManifestAstValueConversionError) -> Self {
        Self::ManifestAstValueConversionError(value)
    }
}

#[serializable]
#[serde(tag = "type")]
pub enum PackageSchemaResolutionError {
    InvalidValueKind {
        expected: ManifestAstValueKind,
        actual: ManifestAstValueKind,
    },

    FailedToSborDecode,
    FailedToSborEncode,
    FailedToGenerateValue,
}
