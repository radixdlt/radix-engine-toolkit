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

use crate::{impl_from_parse_error, utils::debug_string};
use scrypto::runtime::ContentValidationError;
use toolkit_derive::serializable;

/// An error emitted if the conversion between the native and RET representations of the Manifest
/// Sbor fails.
#[serializable]
#[serde(tag = "type")]
pub enum ManifestSborValueConversionError {
    /// An error emitted when trying to parse a string to a type fails.
    ParseError { parsing: String, message: String },

    /// An error emitted when the length of the byte array encountered is unexpected.
    InvalidLength { expected: usize, actual: usize },

    /// An error emitted when invalid non-fungible local ids are provided.
    ScryptoContentValidationError { message: String },
}

impl_from_parse_error! {
    ManifestSborValueConversionError,
    scrypto::prelude::ParseDecimalError => Decimal
}
impl_from_parse_error! {
    ManifestSborValueConversionError,
    scrypto::prelude::ParsePreciseDecimalError => PreciseDecimal
}
impl_from_parse_error! {
    ManifestSborValueConversionError,
    scrypto::prelude::ParseNonFungibleLocalIdError => NonFungibleLocalId
}
impl_from_parse_error! {
    ManifestSborValueConversionError,
    scrypto::prelude::ParseNonFungibleGlobalIdError => NonFungibleGlobalId
}
impl_from_parse_error! {
    ManifestSborValueConversionError,
    scrypto::prelude::ParseManifestBlobRefError => Blob
}

impl From<ContentValidationError> for ManifestSborValueConversionError {
    fn from(value: ContentValidationError) -> Self {
        Self::ScryptoContentValidationError {
            message: debug_string(value),
        }
    }
}
