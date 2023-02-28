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

use std::fmt::Display;

use toolkit_derive::serializable;

use crate::model::value::ast::ManifestAstValueKind;

/// The error model used by the Radix Engine Toolkit - Represents the set of all errors which the
/// Radix Engine Toolkit may return for a request.
#[serializable]
#[serde(tag = "error")]
pub enum Error {
    // ======
    // Value
    // ======
    /// An error emitted when the toolkit attempts to decode some string as a hex string and fails
    FailedToDecodeHex {
        message: String,
    },

    /// A generic error where an operation expected something to be of one length but it was found
    /// to be of a different length
    InvalidLength {
        /// The length that the object was expected to be.
        #[serde_as(as = "serde_with::DisplayFromStr")]
        expected: usize,

        /// The length that the object was found to be.
        #[serde_as(as = "serde_with::DisplayFromStr")]
        found: usize,
    },

    /// Represents an address error encountered during the Bech32 encoding or decoding phase
    AddressError {
        message: String,
    },

    /// An error emitted when the passed `Value` is not one of the accepted value types for a given
    /// request or operation.
    InvalidKind {
        /// A set of the expected `ManifestAstValueKind`s for a given request or operation (this
        /// set forms an 'or' and not an 'and').
        expected: Vec<ManifestAstValueKind>,

        /// The `ManifestAstValueKind` that was found.
        found: ManifestAstValueKind,
    },

    /// An error emitted when an invalid enum discriminator is encountered. This typically means
    /// that an enum discriminator was given as a string but there was no known mapping from the
    /// string to a `u8`.
    InvalidEnumDiscriminator {
        discriminator: String,
    },

    // =====
    // SBOR
    // =====
    /// Represents an error when trying to encode some object in SBOR.
    SborEncodeError {
        message: String,
    },

    /// Represents an error when trying to decode some object in SBOR.
    SborDecodeError {
        message: String,
    },

    // ====
    // AST
    // ====
    /// An error emitted when a value of an unexpected kind is encountered while parsing the AST.
    /// As an example, a `Decimal` value is represented as a `Decimal("some number string")`. If
    /// we attempt to parse a `Decimal` and instead of the internals being a string we find some
    /// other type (e.g. `Decimal(Bucket(12)))`, then this error is emitted.
    UnexpectedAstContents {
        parsing: ManifestAstValueKind,
        expected: Vec<ManifestAstValueKind>,
        found: ManifestAstValueKind,
    },

    /// An error emitted when the parsing of a value from string fails.
    ParseError {
        kind: ManifestAstValueKind,
        message: String,
    },

    /// An error emitted when an invalid expression string is encountered.
    InvalidExpressionString {
        found: String,
        excepted: Vec<String>,
    },

    // ============
    // Manifest
    // ============
    /// An error emitted when the transaction compilation fails
    ManifestCompileError {
        message: String,
    },

    /// An error emitted when the transaction decompilation fails
    ManifestDecompileError {
        message: String,
    },

    ManifestGenerationError {
        message: String,
    },

    // ===========
    // Validation
    // ===========
    /// An error emitted when there is a network mismatch between addresses and the header network
    NetworkMismatchError {
        found: u8,
        expected: u8,
    },

    /// An error emitted when a byte array could not be interpreted through any of the intent
    /// formats
    UnrecognizedCompiledIntentFormat,

    /// An error emitted when an address of a unknown format is passed to the Radix Engine Toolkit
    /// for processing.
    UnrecognizedAddressFormat,

    /// An error emitted when the validation of the content of a Scrypto type fails. Currently,
    /// this is emitted if the validation of non-fungible local ids fails.
    ContentValidationError {
        message: String,
    },

    // =========
    // Requests
    // =========
    /// An error emitted when the contents of the request string pointer could not be loaded.
    InvalidRequestString {
        message: String,
    },

    /// An error emitted when an operation was meant to be infallible but has failed. This is used
    /// as a replacement to panics to bubble whatever error to the caller. If you see this error,
    /// please report it as you should never get to see it.
    Infallible {
        message: String,
    },
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

macro_rules! to_debug_string {
    ($expression: expr) => {
        format!("{:?}", $expression)
    };
}

macro_rules! generate_from_error {
    ($error: ty as $variant: ident) => {
        impl From<$error> for Error {
            fn from(message: $error) -> Self {
                Self::$variant {
                    message: to_debug_string!(message),
                }
            }
        }
    };
}

generate_from_error!(hex::FromHexError as FailedToDecodeHex);
generate_from_error!(scrypto::radix_engine_interface::address::AddressError as AddressError);
generate_from_error!(sbor::EncodeError as SborEncodeError);
generate_from_error!(sbor::DecodeError as SborDecodeError);
generate_from_error!(native_transaction::manifest::CompileError as ManifestCompileError);
generate_from_error!(native_transaction::manifest::DecompileError as ManifestDecompileError);
generate_from_error!(std::str::Utf8Error as InvalidRequestString);
generate_from_error!(
    native_transaction::manifest::generator::GeneratorError as ManifestGenerationError
);
generate_from_error!(
    scrypto::radix_engine_interface::blueprints::resource::ContentValidationError
        as ContentValidationError
);

macro_rules! impl_from_parse_error {
    ($($error_type: ty => $kind: ident,)*) => {
        $(
            impl From<$error_type> for Error {
                fn from(error: $error_type) -> Self {
                    Self::ParseError {
                        kind: ManifestAstValueKind::$kind,
                        message: format!("{:?}", error)
                    }
                }
            }
        )*
    };
}

impl_from_parse_error! {
    scrypto::prelude::ParseDecimalError => Decimal,
    scrypto::prelude::ParsePreciseDecimalError => PreciseDecimal,
    scrypto::prelude::ParseNonFungibleLocalIdError => NonFungibleLocalId,
    scrypto::prelude::ParseNonFungibleGlobalIdError => NonFungibleGlobalId,
    native_transaction_data::model::ParseManifestBlobRefError => Blob,
}

/// The result type used by the Radix Engine Toolkit where all errors are of a single type.
pub type Result<T> = std::result::Result<T, Error>;
