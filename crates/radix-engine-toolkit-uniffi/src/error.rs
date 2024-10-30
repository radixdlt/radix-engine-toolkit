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

pub type Result<T> = std::result::Result<T, RadixEngineToolkitError>;

#[derive(Clone, Debug, Error, ThisError)]
pub enum RadixEngineToolkitError {
    #[error("Length check failed. Expected {expected}, actual {actual}.")]
    InvalidLength {
        expected: u64,
        actual: u64,
        data: Vec<u8>,
    },

    #[error("Failed to derive network id from address string {address}")]
    FailedToExtractNetwork { address: String },

    #[error("Failed to Bech32m decode the address: {error}")]
    Bech32DecodeError { error: String },

    #[error(
        "Failed to parse a string into a typed object {type_name}: {error}"
    )]
    ParseError { type_name: String, error: String },

    #[error(
        "Failed to validate content during non-fungible local id conversion: {error}"
    )]
    NonFungibleContentValidationError { error: String },

    #[error("Entity type {actual:?} did not match expected {expected:?}")]
    EntityTypeMismatchError {
        expected: Vec<EntityType>,
        actual: EntityType,
    },

    #[error("Failed to perform a derivation: {error}")]
    DerivationError { error: String },

    #[error("Public key is not valid for a given operation")]
    InvalidPublicKey,

    #[error("Failed to add instruction: {error}")]
    InstructionAddError { error: String },

    #[error("Manifest compilation errored out: {error}")]
    CompileError { error: String },

    #[error("Manifest decompilation errored out: {error}")]
    DecompileError { error: String },

    #[error("Failed while trying to prepare transaction part: {error}")]
    PrepareError { error: String },

    #[error("Failed to SBOR encode some data: {error}")]
    EncodeError { error: String },

    #[error("Failed to SBOR decode some payload: {error}")]
    DecodeError { error: String },

    #[error("Static validation of transaction part has failed: {error}")]
    TransactionValidationFailed { error: String },

    #[error("Execution analysis failed: {error}")]
    ExecutionModuleError { error: String },

    #[error(
        "An error occurred during doing a Manifest SBOR encode/decode: {error}"
    )]
    ManifestSborError { error: String },

    #[error(
        "An error occurred during doing a Scrypto SBOR encode/decode: {error}"
    )]
    ScryptoSborError { error: String },

    #[error(
        "An error occurred when trying to convert native event data to typed: {error}"
    )]
    TypedNativeEventError { error: String },

    #[error(
        "An error occurred when trying to decode the transaction hash: {error:?}"
    )]
    FailedToDecodeTransactionHash { error: String },

    #[error(
        "An error ocurred when building the manifest due to the naming of objects: {error:?}"
    )]
    ManifestBuilderNameRecordError { error: NameRecordError },

    #[error("The node id has no valid entity type: {error}")]
    InvalidEntityTypeIdError { error: String },

    #[error(
        "An error encountered when doing decimal arithmetic pertaining to overflow and underflow"
    )]
    DecimalError,

    #[error("An error that occurred in the signer logic: {error}")]
    SignerError { error: String },

    #[error("The receipt is not a valid preview receipt")]
    InvalidReceipt,

    #[error("The static analysis of the manifest failed: {error}")]
    StaticAnalysisFailed { error: String },

    #[error("Not all of the builder items were specified, progressing failed.")]
    NotAllBuilderItemsWereSpecified,

    #[error("Validation of manifest failed: {error}")]
    ManifestValidationError { error: String },
}

macro_rules! dbg_str {
    ($expr: expr) => {
        format!("{:?}", $expr)
    };
}

macro_rules! impl_parse_error {
    (
        $type: ty,
        $error: ty
    ) => {
        impl From<$error> for RadixEngineToolkitError {
            fn from(value: $error) -> Self {
                Self::ParseError {
                    type_name: stringify!($type).to_owned(),
                    error: dbg_str!(value),
                }
            }
        }
    };
}

macro_rules! impl_dbg_str_from {
    ($error: ty, $variant: ident) => {
        impl From<$error> for RadixEngineToolkitError {
            fn from(value: $error) -> Self {
                Self::$variant {
                    error: dbg_str!(value),
                }
            }
        }
    };
}

impl_parse_error! { scrypto::prelude::Decimal, scrypto::prelude::ParseDecimalError }
impl_parse_error! { scrypto::prelude::PreciseDecimal, scrypto::prelude::ParsePreciseDecimalError }
impl_parse_error! { scrypto::prelude::NonFungibleGlobalId, scrypto::prelude::ParseNonFungibleGlobalIdError }
impl_parse_error! { scrypto::prelude::NonFungibleLocalId, scrypto::prelude::ParseNonFungibleLocalIdError }
impl_parse_error! { scrypto::prelude::Hash, scrypto::prelude::ParseHashError }
impl_parse_error! { u128, std::num::ParseIntError } // TODO: can we continue making the u128 assumption?
impl_parse_error! { scrypto::prelude::ResourceAddress, scrypto::prelude::ParseResourceAddressError }
impl_parse_error! { scrypto::prelude::ComponentAddress, scrypto::prelude::ParseComponentAddressError }
impl_parse_error! { scrypto::prelude::PackageAddress, scrypto::prelude::ParsePackageAddressError }
impl_parse_error! { scrypto::prelude::GlobalAddress, scrypto::prelude::ParseGlobalAddressError }
impl_parse_error! { scrypto::prelude::InternalAddress, scrypto::prelude::ParseInternalAddressError }

impl_dbg_str_from! { NativeContentValidationError, NonFungibleContentValidationError }
impl_dbg_str_from! { CoreDerivationError, DerivationError }
impl_dbg_str_from! { NativeCompileError, CompileError }
impl_dbg_str_from! { NativeDecompileError, DecompileError }
impl_dbg_str_from! { NativePrepareError, PrepareError }
impl_dbg_str_from! { NativeEncodeError, EncodeError }
impl_dbg_str_from! { NativeDecodeError, DecodeError }
impl_dbg_str_from! { NativeTransactionHashBech32DecodeError, FailedToDecodeTransactionHash }
impl_dbg_str_from! { NativeTransactionValidationError, TransactionValidationFailed }
impl_dbg_str_from! { CoreInstructionValidationError, TransactionValidationFailed }
impl_dbg_str_from! { CoreManifestSborError, ManifestSborError }
impl_dbg_str_from! { CoreScryptoSborError, ScryptoSborError }
impl_dbg_str_from! { NativeTypedNativeEventError, TypedNativeEventError }
impl_dbg_str_from! { CoreInvalidEntityTypeIdError, InvalidEntityTypeIdError }
impl_dbg_str_from! { NativeManifestValidationError, ManifestValidationError }
impl_dbg_str_from! { NativeStaticResourceMovementsError, StaticAnalysisFailed }

impl From<NameRecordError> for RadixEngineToolkitError {
    fn from(value: NameRecordError) -> Self {
        Self::ManifestBuilderNameRecordError { error: value }
    }
}
