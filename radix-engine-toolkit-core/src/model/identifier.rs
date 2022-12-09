// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at

//   http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use std::{str::FromStr, convert::Infallible};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone, PartialOrd, Ord)]
#[serde(untagged)]
pub enum Identifier {
    String(String),
    U32(u32),
}

impl From<String> for Identifier {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<u32> for Identifier {
    fn from(value: u32) -> Self {
        Self::U32(value)
    }
}

impl FromStr for Identifier {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::String(s.into()))
    }
}

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub struct BucketId(pub Identifier);

impl From<Identifier> for BucketId {
    fn from(identifier: Identifier) -> Self {
        Self(identifier)
    }
}

impl From<BucketId> for Identifier {
    fn from(bucket_id: BucketId) -> Self {
        bucket_id.0
    }
}

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub struct ProofId(pub Identifier);

impl From<Identifier> for ProofId {
    fn from(identifier: Identifier) -> Self {
        Self(identifier)
    }
}

impl From<ProofId> for Identifier {
    fn from(proof_id: ProofId) -> Self {
        proof_id.0
    }
}
