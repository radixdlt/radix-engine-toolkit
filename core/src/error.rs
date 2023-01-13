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

use hex::FromHexError;
use scrypto::radix_engine_interface::address::AddressError;
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
    ($error: ident as $variant: ident) => {
        impl From<$error> for Error {
            fn from(value: $error) -> Self {
                Self::$variant {
                    value: to_debug_string!(value),
                }
            }
        }
    };
}

generate_from_error!(FromHexError as FailedToDecodeHex);
generate_from_error!(AddressError as AddressError);

/// The result type used by the Radix Engine Toolkit where all errors are of a single type.
pub type Result<T> = std::result::Result<T, Error>;
