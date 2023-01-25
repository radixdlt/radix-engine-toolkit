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

use crate::error::{Error, Result};
use native_transaction::manifest::KNOWN_ENUM_DISCRIMINATORS;
use serializable::serializable;

// =================
// Model Definition
// =================

/// A union of the types of discriminators that enums may have. This may either be a string or an
/// 8-bit unsigned number.
#[serializable]
#[serde(tag = "type")]
#[derive(PartialEq, Eq, Hash)]
pub enum EnumDiscriminator {
    String {
        /// A string discriminator of the fully qualified well-known enum name
        discriminator: String,
    },
    U8 {
        /// An 8-bit unsigned integer serialized as a string.
        #[schemars(regex(pattern = "[0-9]+"))]
        #[schemars(with = "String")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        discriminator: u8,
    },
}

// ============
// Conversions
// ============

impl EnumDiscriminator {
    /// Resolves the enum discriminator to a [`u8`] discriminator.
    pub fn resolve_discriminator(&self) -> Result<u8> {
        match self {
            Self::U8 { discriminator } => Ok(*discriminator),
            Self::String { discriminator } => KNOWN_ENUM_DISCRIMINATORS
                .get(discriminator.as_str())
                .copied()
                .ok_or(Error::InvalidEnumDiscriminator {
                    discriminator: discriminator.clone(),
                }),
        }
    }
}
