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

use serializable::serializable;

use crate::ValueKind;

#[serializable]
#[serde(tag = "error")]
/// The error model used by the Radix Engine Toolkit - Represents the set of all errors which the
/// Radix Engine Toolkit may return for a request.
pub enum Error {
    /// An error emitted when the toolkit attempts to decode some string as a hex string and fails
    FailedToDecodeHex { value: String },

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
    AddressError { value: String },

    /// An error emitted when the passed `Value` is not one of the accepted value types for a given
    /// request or operation.
    InvalidKind {
        /// A set of the expected `ValueKind`s for a given request or operation (this set forms an
        /// 'or' and not an 'and').
        expected: Vec<ValueKind>,

        /// The `ValueKind` that was found.
        found: ValueKind,
    },

    // =====
    // SBOR
    // =====
    /// An error emitted when some object of some value kind can not be encoded in SBOR without
    /// additional context. This error is typically seen in situations when trying to encode either
    /// a `Bucket("some_string")` or a `Proof("some_string")` as buckets or proofs with String
    /// identifiers can not be encoded in SBOR without an ID Allocator.
    BucketOrProofSBORError { value_kind: ValueKind },

    /// Represents an error when trying to encode some object in SBOR.
    SborEncodeError { value: String },

    /// Represents an error when trying to decode some object in SBOR.
    SborDecodeError { value: String },

    // ====
    // AST
    // ====
    /// An error emitted when a value of an unexpected kind is encountered while parsing the AST.
    /// As an example, a `Decimal` value is represented as a `Decimal("some number string")`. If
    /// we attempt to parse a `Decimal` and instead of the internals being a string we find some
    /// other type (e.g. `Decimal(Bucket(12)))`, then this error is emitted.
    UnexpectedAstContents {
        parsing: ValueKind,
        expected: Vec<ValueKind>,
        found: ValueKind,
    },

    /// An error emitted when the parsing of a value from string fails.
    ParseError { kind: ValueKind, message: String },

    /// An error emitted when an invalid expression string is encountered.
    InvalidExpressionString {
        found: String,
        excepted: Vec<String>,
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
            fn from(value: $error) -> Self {
                Self::$variant {
                    value: to_debug_string!(value),
                }
            }
        }
    };
}

generate_from_error!(hex::FromHexError as FailedToDecodeHex);
generate_from_error!(scrypto::radix_engine_interface::address::AddressError as AddressError);
generate_from_error!(sbor::EncodeError as SborEncodeError);
generate_from_error!(sbor::DecodeError as SborDecodeError);

macro_rules! impl_from_parse_error {
    ($($error_type: ty => $kind: ident,)*) => {
        $(
            impl From<$error_type> for Error {
                fn from(error: $error_type) -> Self {
                    Self::ParseError {
                        kind: ValueKind::$kind,
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
    scrypto::prelude::ParseHashError => Hash,
    scrypto::prelude::ParseNonFungibleIdError => NonFungibleId,
    scrypto::prelude::ParseNonFungibleAddressError => NonFungibleAddress,
    scrypto::prelude::ParseManifestBlobRefError => Blob,
    scrypto::prelude::ParseEcdsaSecp256k1PublicKeyError => EcdsaSecp256k1PublicKey,
    scrypto::prelude::ParseEcdsaSecp256k1SignatureError => EcdsaSecp256k1Signature,
    scrypto::prelude::ParseEddsaEd25519PublicKeyError => EddsaEd25519PublicKey,
    scrypto::prelude::ParseEddsaEd25519SignatureError => EddsaEd25519Signature,
}

/// The result type used by the Radix Engine Toolkit where all errors are of a single type.
pub type Result<T> = std::result::Result<T, Error>;
