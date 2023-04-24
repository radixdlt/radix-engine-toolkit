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
use crate::request::*;
use crate::utils::debug_string;
#[cfg(feature = "radix-engine")]
use crate::visitor::AccountDepositsVisitorError;

/// The error type that's returned by the Radix Engine Toolkit when an error takes place. This type
/// is made up of a number of other more granular types which describe the error in full.
#[serializable]
#[serde(tag = "type")]
pub enum RETError {
    InvocationHandlingError(InvocationHandlingError),
    InvocationInterpretationError(InvocationInterpretationError),
}

/// Errors emitted when the invocation could not be interpreted.
#[serializable]
#[serde(tag = "type")]
pub enum InvocationInterpretationError {
    /// An error emitted when the serialized invocation string contains characters that are not
    /// valid UTF-8
    Utf8Error { message: String },

    /// An error emitted when the deserialization of the invocation fails.
    DeserializationError { message: String },

    /// An error emitted when the serialization of an invocation response fails
    SerializationError { message: String },

    /// An error emitted when the allocation of strings through the JNI environment fails
    JniStringAllocationFailed,

    /// An error emitted when a string could not be read through the JNI environment
    JniStringReadFailed,
}

impl From<std::str::Utf8Error> for InvocationInterpretationError {
    fn from(value: std::str::Utf8Error) -> Self {
        Self::Utf8Error {
            message: debug_string(value),
        }
    }
}

/// Errors pertaining to invocations handling. This set of errors are returned when an invocation is
/// of a correct structure, but the handling of the invocation failed (e.g., due to validation).
#[serializable]
#[serde(tag = "type")]
pub enum InvocationHandlingError {
    #[cfg(feature = "radix-engine")]
    AnalyzeManifestWithPreviewContextError(AnalyzeManifestWithPreviewContextError),
    InformationError(InformationError),
    ConvertManifestError(ConvertManifestError),
    CompileTransactionIntentError(CompileTransactionIntentError),
    DecompileTransactionIntentError(DecompileTransactionIntentError),
    CompileSignedTransactionIntentError(CompileSignedTransactionIntentError),
    DecompileSignedTransactionIntentError(DecompileSignedTransactionIntentError),
    CompileNotarizedTransactionError(CompileNotarizedTransactionError),
    DecompileNotarizedTransactionError(DecompileNotarizedTransactionError),
    DecompileUnknownTransactionIntentError(DecompileUnknownTransactionIntentError),
    DecodeAddressError(DecodeAddressError),
    EncodeAddressError(EncodeAddressError),
    SborDecodeError(SborDecodeError),
    SborEncodeError(SborEncodeError),
    DeriveBabylonAddressFromOlympiaAddressError(DeriveBabylonAddressFromOlympiaAddressError),
    DeriveOlympiaAddressFromPublicKeyError(DeriveOlympiaAddressFromPublicKeyError),
    DeriveVirtualAccountAddressError(DeriveVirtualAccountAddressError),
    DeriveVirtualIdentityAddressError(DeriveVirtualIdentityAddressError),
    AnalyzeManifestError(AnalyzeManifestError),
    KnownEntityAddressesError(KnownEntityAddressesError),
    StaticallyValidateTransactionError(StaticallyValidateTransactionError),
    HashError(HashError),
}

macro_rules! impl_from {
    ($type: ident for $for: ident) => {
        impl From<$type> for $for {
            fn from(value: $type) -> Self {
                Self::$type(value)
            }
        }
    };
}

#[cfg(feature = "radix-engine")]
impl_from! { AnalyzeManifestWithPreviewContextError for InvocationHandlingError }
impl_from! { InformationError for InvocationHandlingError }
impl_from! { ConvertManifestError for InvocationHandlingError }
impl_from! { CompileTransactionIntentError for InvocationHandlingError }
impl_from! { DecompileTransactionIntentError for InvocationHandlingError }
impl_from! { CompileSignedTransactionIntentError for InvocationHandlingError }
impl_from! { DecompileSignedTransactionIntentError for InvocationHandlingError }
impl_from! { CompileNotarizedTransactionError for InvocationHandlingError }
impl_from! { DecompileNotarizedTransactionError for InvocationHandlingError }
impl_from! { DecompileUnknownTransactionIntentError for InvocationHandlingError }
impl_from! { DecodeAddressError for InvocationHandlingError }
impl_from! { EncodeAddressError for InvocationHandlingError }
impl_from! { SborDecodeError for InvocationHandlingError }
impl_from! { SborEncodeError for InvocationHandlingError }
impl_from! { DeriveBabylonAddressFromOlympiaAddressError for InvocationHandlingError }
impl_from! { DeriveOlympiaAddressFromPublicKeyError for InvocationHandlingError }
impl_from! { DeriveVirtualAccountAddressError for InvocationHandlingError }
impl_from! { DeriveVirtualIdentityAddressError for InvocationHandlingError }
impl_from! { AnalyzeManifestError for InvocationHandlingError }
impl_from! { KnownEntityAddressesError for InvocationHandlingError }
impl_from! { StaticallyValidateTransactionError for InvocationHandlingError }
impl_from! { HashError for InvocationHandlingError }

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
    #[cfg(feature = "radix-engine")]
    /// Errors emitted by the account deposits visitor.
    AccountDepositsVisitorError(AccountDepositsVisitorError),
}
