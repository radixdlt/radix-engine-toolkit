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
use std::str::FromStr;

use scrypto_utils::copy_u8_array;
use serializable::serializable;

use crate::constants::RADIX_ENGINE_NODE_IDENTIFIER_LENGTH;
use crate::error::{Error, Result};

// =================
// Model Definition
// =================

#[serializable]
/// Represents a Radix Engine persistent node identifier which is 36 bytes long and serialized as a
/// hexadecimal string of length 72 (since hex encoding doubles the number of bytes needed.)
pub struct NodeIdentifier(
    #[serde_as(as = "serde_with::hex::Hex")]
    #[schemars(length(equal = 72))]
    #[schemars(regex(pattern = "[0-9a-fA-F]+"))]
    pub [u8; RADIX_ENGINE_NODE_IDENTIFIER_LENGTH],
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
        if bytes.len() != RADIX_ENGINE_NODE_IDENTIFIER_LENGTH {
            Err(Error::InvalidLength {
                expected: RADIX_ENGINE_NODE_IDENTIFIER_LENGTH,
                found: bytes.len(),
            })
        } else {
            Ok(NodeIdentifier(copy_u8_array(&bytes)))
        }
    }
}
