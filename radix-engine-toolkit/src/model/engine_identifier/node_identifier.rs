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

use radix_engine_common::data::scrypto::model::OBJECT_ID_LENGTH;
use std::fmt::Display;
use std::str::FromStr;

use toolkit_derive::serializable;

use crate::error::{Error, Result};
use crate::utils::checked_copy_u8_slice;

// =================
// Model Definition
// =================

#[serializable]
/// Represents a Radix Engine persistent node identifier which is 36 bytes long and serialized as a
/// hexadecimal string of length 31 (since hex encoding doubles the number of bytes needed.)
#[derive(PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct NodeIdentifier(
    #[schemars(length(equal = 31))]
    #[schemars(regex(pattern = "[0-9a-fA-F]+"))]
    #[schemars(with = "String")]
    #[serde_as(as = "serde_with::hex::Hex")]
    pub [u8; OBJECT_ID_LENGTH],
);

// =====
// Text
// =====

impl Display for NodeIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}

impl FromStr for NodeIdentifier {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        // Attempt the decode the string as a hex-string
        let bytes = hex::decode(s)?;

        // Check that the decoded bytes are of the expected length - error out if they're not
        if bytes.len() != OBJECT_ID_LENGTH {
            Err(Error::InvalidLength {
                expected: OBJECT_ID_LENGTH,
                found: bytes.len(),
            })
        } else {
            Ok(NodeIdentifier(checked_copy_u8_slice(&bytes)?))
        }
    }
}
