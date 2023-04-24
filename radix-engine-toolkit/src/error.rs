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

use crate::model::address::NonFungibleLocalIdConversionError;
use crate::model::value::ast::ManifestAstValueConversionError;
use crate::model::value::manifest_sbor::ManifestSborValueConversionError;
use crate::model::value::scrypto_sbor::ScryptoSborValueConversionError;
use crate::visitor::AccountDepositsVisitorError;

/// The error type that's returned by the Radix Engine Toolkit when an error takes place. This type
/// is made up of a number of other more granular types which describe the error in full.
#[serializable]
#[serde(tag = "type")]
pub enum RETError {
    InvocationHandlingError(InvocationHandlingError),
}

/// Errors pertaining to invocations handling. This set of errors are returned when an invocation is
/// of a correct structure, but the handling of the invocation failed (e.g., due to validation).
#[serializable]
#[serde(tag = "type")]
pub enum InvocationHandlingError {}

/// Errors emitted when the conversion between the native Scrypto models and the RET models fails.
#[serializable]
#[serde(tag = "type")]
pub enum ConversionError {
    /// An error emitted if the conversion between the native and RET representations of non
    /// fungible local ids fails.
    NonFungibleLocalId(NonFungibleLocalIdConversionError),

    /// An error emitted if the conversion between the native and RET representations of the
    /// Manifest AST fails.
    ManifestAstValueConversionError(ManifestAstValueConversionError),

    /// An error emitted if the conversion between the native and RET representations of the
    /// Manifest Sbor fails.
    ManifestSborValueConversionError(ManifestSborValueConversionError),

    /// An error emitted if the conversion between the native and RET representations of the
    /// Scrypto Sbor fails.
    ScryptoSborValueConversionError(ScryptoSborValueConversionError),
}

/// Errors emitted by the various visitors used in the Radix Engine Toolkit.
#[serializable]
#[serde(tag = "type")]
pub enum VisitorError {
    /// An error emitted when a ManifestAstValue visitor fails or reports an error.
    ValueVisitorError(ValueVisitorError),

    /// An error emitted when an Instruction visitor fails or reports an error.
    InstructionVisitorError(InstructionVisitorError),
}

#[serializable]
#[serde(tag = "type")]
pub enum ValueVisitorError {}

#[serializable]
#[serde(tag = "type")]
pub enum InstructionVisitorError {
    /// Errors emitted by the account deposits visitor.
    AccountDepositsVisitorError(AccountDepositsVisitorError),
}
