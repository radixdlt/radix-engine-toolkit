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

use crate::{Error, Result};
use serializable::serializable;
use std::str::FromStr;

// =================
// Model Definition
// =================

#[serializable]
#[serde(untagged)]
#[derive(Clone)]
/// Represents an untagged transient identifier typically used as an identifiers for Scrypto buckets
/// and proofs. Could either be a string or an unsigned 32-bit number (which is serialized as a
/// number and not a string)
pub enum TransientIdentifier {
    String(String),
    U32(u32),
}

#[serializable]
#[derive(Clone)]
/// Represents a BucketId which uses a transient identifier.
pub struct BucketId(pub TransientIdentifier);

#[serializable]
#[derive(Clone)]
/// Represents a ProofId which uses a transient identifier.
pub struct ProofId(pub TransientIdentifier);

// ============
// Conversions
// ============

impl FromStr for TransientIdentifier {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self::String(s.to_owned()))
    }
}

impl From<String> for TransientIdentifier {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<u32> for TransientIdentifier {
    fn from(value: u32) -> Self {
        Self::U32(value)
    }
}

impl From<TransientIdentifier> for BucketId {
    fn from(identifier: TransientIdentifier) -> Self {
        Self(identifier)
    }
}

impl From<BucketId> for TransientIdentifier {
    fn from(bucket_id: BucketId) -> Self {
        bucket_id.0
    }
}

impl From<TransientIdentifier> for ProofId {
    fn from(identifier: TransientIdentifier) -> Self {
        Self(identifier)
    }
}

impl From<ProofId> for TransientIdentifier {
    fn from(proof_id: ProofId) -> Self {
        proof_id.0
    }
}
