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

use crate::{
    impl_display_as_debug, impl_from_parse_error, model::address::AddressError, utils::debug_string,
};
use toolkit_derive::serializable;

use super::ManifestAstValueKind;

/// An error emitted if the conversion between the native and RET representations of the Manifest
/// AST fails.
#[serializable]
#[serde(tag = "type")]
pub enum ManifestAstValueConversionError {
    /// Enum discriminators could either be a `u8` or a `String` with a well-known mapping back to
    /// the numeric discriminator. This error is emitted when the mapping of a string variant name
    /// to `u8` discriminator fails.
    FailedToResolveEnumDiscriminator { variant_name: String },

    /// This error is emitted during the conversion from the native AST models to the RET AST
    /// models if the number of elements in a map is odd (thus it's not a valid key value map).
    MapHasOddNumberOfElements,

    /// An error emitted when unexpected contents of an element is encountered while performing the
    /// conversion.
    UnexpectedContents {
        parsing: ManifestAstValueKind,
        expected: Vec<ManifestAstValueKind>,
        actual: ManifestAstValueKind,
    },

    /// Emitted when an unexpected expression string is encountered which can not be mapped to the
    /// expression enum.
    InvalidExpressionString {
        expected: Vec<String>,
        actual: String,
    },

    /// An error emitted when a value of an expected kind is encountered. Typically, this error is
    /// emitted when converting a ManifestAstValue to its underlying type.
    InvalidKind {
        expected: Vec<ManifestAstValueKind>,
        actual: ManifestAstValueKind,
    },

    /// An error emitted when trying to parse a string to a type fails.
    ParseError { parsing: String, message: String },

    /// An error emitted when address encoding or decoding fails.
    AddressError(AddressError),

    /// An error emitted when the decoding of hex strings fail
    HexDecodeError { message: String },
}

impl_display_as_debug!(ManifestAstValueConversionError);

impl_from_parse_error! {
    ManifestAstValueConversionError,
    scrypto::prelude::ParseDecimalError => Decimal
}
impl_from_parse_error! {
    ManifestAstValueConversionError,
    scrypto::prelude::ParsePreciseDecimalError => PreciseDecimal
}
impl_from_parse_error! {
    ManifestAstValueConversionError,
    scrypto::prelude::ParseNonFungibleLocalIdError => NonFungibleLocalId
}
impl_from_parse_error! {
    ManifestAstValueConversionError,
    scrypto::prelude::ParseNonFungibleGlobalIdError => NonFungibleGlobalId
}
impl_from_parse_error! {
    ManifestAstValueConversionError,
    scrypto::prelude::ParseManifestBlobRefError => Blob
}

impl From<AddressError> for ManifestAstValueConversionError {
    fn from(value: AddressError) -> Self {
        Self::AddressError(value)
    }
}

impl From<hex::FromHexError> for ManifestAstValueConversionError {
    fn from(value: hex::FromHexError) -> Self {
        Self::HexDecodeError {
            message: debug_string(value),
        }
    }
}
