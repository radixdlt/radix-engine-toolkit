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

use crate::functions::*;
use crate::model::address::NonFungibleLocalIdConversionError;
use crate::model::value::ast::ManifestAstValueConversionError;
use crate::model::value::manifest_sbor::ManifestSborValueConversionError;
use crate::model::value::scrypto_sbor::ScryptoSborValueConversionError;
use crate::utils::debug_string;
#[cfg(feature = "radix-engine")]
use crate::visitor::AccountDepositsVisitorError;

/// The error type that's returned by the Radix Engine Toolkit when an error takes place. This type
/// is made up of a number of other more granular types which describe the error in full.
#[serializable]
#[serde(tag = "type", content = "error")]
pub enum RETError {
    InvocationHandlingError(InvocationHandlingError),
    InvocationInterpretationError(InvocationInterpretationError),
}

/// Errors emitted when the invocation could not be interpreted.
#[serializable]
#[serde(tag = "type", content = "error")]
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

/// Errors pertaining to functions handling. This set of errors are returned when an invocation is
/// of a correct structure, but the handling of the invocation failed (e.g., due to validation).
#[serializable]
#[serde(tag = "type", content = "error")]
pub enum InvocationHandlingError {
    #[cfg(feature = "radix-engine")]
    AnalyzeTransactionExecutionError(analyze_transaction_execution::Error),
    InformationError(information::Error),
    ConvertManifestError(convert_manifest::Error),
    CompileTransactionIntentError(compile_transaction_intent::Error),
    DecompileTransactionIntentError(decompile_transaction_intent::Error),
    CompileSignedTransactionIntentError(compile_signed_transaction_intent::Error),
    DecompileSignedTransactionIntentError(decompile_signed_transaction_intent::Error),
    CompileNotarizedTransactionError(compile_notarized_transaction::Error),
    DecompileNotarizedTransactionError(decompile_notarized_transaction::Error),
    DecompileUnknownTransactionIntentError(decompile_unknown_intent::Error),
    DecodeAddressError(decode_address::Error),
    EncodeAddressError(encode_address::Error),
    SborDecodeError(sbor_decode::Error),
    SborEncodeError(sbor_encode::Error),
    DeriveBabylonAddressFromOlympiaAddressError(derive_babylon_address_from_olympia_address::Error),
    DeriveOlympiaAddressFromPublicKeyError(derive_olympia_address_from_public_key::Error),
    DeriveVirtualAccountAddressError(derive_virtual_account_address::Error),
    DeriveVirtualIdentityAddressError(derive_virtual_identity_address::Error),
    ExtractAddressesFromManifestError(extract_addresses_from_manifest::Error),
    KnownEntityAddressesError(known_entity_addresses::Error),
    StaticallyValidateTransactionError(statically_validate_transaction::Error),
    HashTransactionIntentError(hash_transaction_intent::Error),
    HashSignedTransactionIntentError(hash_signed_transaction_intent::Error),
    HashNotarizedTransactionError(hash_notarized_transaction::Error),
    HashError(hash::Error),
}

macro_rules! impl_from {
    ($type: ty => $for: ident as $as: ident) => {
        impl From<$type> for $for {
            fn from(value: $type) -> Self {
                Self::$as(value)
            }
        }
    };
}

#[cfg(feature = "radix-engine")]
impl_from! { analyze_transaction_execution::Error => InvocationHandlingError as AnalyzeTransactionExecutionError }
impl_from! { information::Error => InvocationHandlingError as InformationError }
impl_from! { convert_manifest::Error => InvocationHandlingError as ConvertManifestError }
impl_from! { compile_transaction_intent::Error => InvocationHandlingError as CompileTransactionIntentError }
impl_from! { decompile_transaction_intent::Error => InvocationHandlingError as DecompileTransactionIntentError }
impl_from! { compile_signed_transaction_intent::Error => InvocationHandlingError as CompileSignedTransactionIntentError }
impl_from! { decompile_signed_transaction_intent::Error => InvocationHandlingError as DecompileSignedTransactionIntentError }
impl_from! { compile_notarized_transaction::Error => InvocationHandlingError as CompileNotarizedTransactionError }
impl_from! { decompile_notarized_transaction::Error => InvocationHandlingError as DecompileNotarizedTransactionError }
impl_from! { decompile_unknown_intent::Error => InvocationHandlingError as DecompileUnknownTransactionIntentError }
impl_from! { decode_address::Error => InvocationHandlingError as DecodeAddressError }
impl_from! { encode_address::Error => InvocationHandlingError as EncodeAddressError }
impl_from! { sbor_decode::Error => InvocationHandlingError as SborDecodeError }
impl_from! { sbor_encode::Error => InvocationHandlingError as SborEncodeError }
impl_from! { derive_babylon_address_from_olympia_address::Error => InvocationHandlingError as DeriveBabylonAddressFromOlympiaAddressError }
impl_from! { derive_olympia_address_from_public_key::Error => InvocationHandlingError as DeriveOlympiaAddressFromPublicKeyError }
impl_from! { derive_virtual_account_address::Error => InvocationHandlingError as DeriveVirtualAccountAddressError }
impl_from! { derive_virtual_identity_address::Error => InvocationHandlingError as DeriveVirtualIdentityAddressError }
impl_from! { extract_addresses_from_manifest::Error => InvocationHandlingError as ExtractAddressesFromManifestError }
impl_from! { known_entity_addresses::Error => InvocationHandlingError as KnownEntityAddressesError }
impl_from! { statically_validate_transaction::Error => InvocationHandlingError as StaticallyValidateTransactionError }
impl_from! { hash_transaction_intent::Error => InvocationHandlingError as HashTransactionIntentError }
impl_from! { hash_signed_transaction_intent::Error => InvocationHandlingError as HashSignedTransactionIntentError }
impl_from! { hash_notarized_transaction::Error => InvocationHandlingError as HashNotarizedTransactionError }
impl_from! { hash::Error => InvocationHandlingError as HashError }

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
