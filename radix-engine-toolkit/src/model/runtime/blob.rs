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

use native_transaction::data::ManifestBlobRef;
use toolkit_derive::serializable;

use crate::model::constants::RADIX_ENGINE_HASH_LENGTH;

#[serializable]
/// Represents the hash of a blob provided as part of a transaction manifest. This is represented as
/// a byte array of 32 bytes which is serialized as a hex string.
pub struct Blob(
    #[schemars(with = "String")]
    #[schemars(length(equal = 64))]
    #[schemars(regex(pattern = "[0-9a-fA-F]+"))]
    #[serde_as(as = "serde_with::hex::Hex")]
    [u8; RADIX_ENGINE_HASH_LENGTH],
);

impl From<ManifestBlobRef> for Blob {
    fn from(value: ManifestBlobRef) -> Self {
        Self(value.0)
    }
}

impl From<Blob> for ManifestBlobRef {
    fn from(value: Blob) -> Self {
        Self(value.0)
    }
}
