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

use scrypto::address::{DecodeBech32AddressError, EncodeBech32AddressError};
use toolkit_derive::serializable;

use crate::{impl_display_as_debug, utils::debug_string};

#[serializable]
#[serde(tag = "type")]
pub enum AddressError {
    /// Attempted to perform a Regex capture of the network specifier in the HRP. However, a
    /// network specifier could not be found for the given HRP.
    NoNetworkSpecifierMatchesFoundInHrp { hrp: String },

    /// While attempting to extract the network id from the HRP we failed to extract the network
    /// id. This could mean that the logic itself is flawed, the network id could not be
    /// decoded to a u8 or due to other issues.
    FailedToExtractNetworkId {
        hrp: String,
        network_specifier: String,
        network_id_string: Option<String>,
    },

    /// Represents an error emitted when the Bech32 decoding fails and an error is returned by the
    /// Bech32 library used.
    Bech32DecodeError { address: String },

    /// A wrapper around the decode errors defined in Scrypto.
    ScryptoBech32DecodeError { message: String },

    /// A wrapper around the encode errors defined in Scrypto.
    ScryptoBech32EncodeError { message: String },

    /// This error is emitted when constructing a new address if the data section of the address is
    /// not of the length expected for addresses.
    InvalidDataLength { expected: usize, actual: usize },
}

impl_display_as_debug!(AddressError);

impl From<EncodeBech32AddressError> for AddressError {
    fn from(value: EncodeBech32AddressError) -> Self {
        Self::ScryptoBech32EncodeError {
            message: debug_string(value),
        }
    }
}

impl From<DecodeBech32AddressError> for AddressError {
    fn from(value: DecodeBech32AddressError) -> Self {
        Self::ScryptoBech32DecodeError {
            message: debug_string(value),
        }
    }
}
