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

use crate::prelude::*;

#[derive(Clone, Debug, Object, Copy)]
pub struct Hash(pub(crate) engine::Hash);

#[uniffi::export]
impl Hash {
    #[uniffi::constructor]
    pub fn new(hash: Vec<u8>) -> Result<Arc<Self>> {
        hash.try_into()
            .map(|value| Arc::new(Self(engine::Hash(value))))
            .map_err(|value| RadixEngineToolkitError::InvalidLength {
                expected: engine::Hash::LENGTH as u64,
                actual: value.len() as u64,
                data: value,
            })
    }

    #[uniffi::constructor]
    pub fn from_hex_string(hash: String) -> Result<Arc<Self>> {
        hash.parse()
            .map(|value| Arc::new(Self(value)))
            .map_err(Into::into)
    }

    #[uniffi::constructor]
    pub fn from_unhashed_bytes(bytes: Vec<u8>) -> Arc<Self> {
        Arc::new(Self(engine::hash(bytes)))
    }

    #[uniffi::constructor]
    pub fn sbor_decode(bytes: Vec<u8>) -> Result<Arc<Self>> {
        let native = match bytes.first().copied() {
            Some(engine::SCRYPTO_SBOR_V1_PAYLOAD_PREFIX) => {
                engine::scrypto_decode::<engine::Hash>(&bytes).map_err(Into::into)
            }
            Some(engine::MANIFEST_SBOR_V1_PAYLOAD_PREFIX) => {
                engine::manifest_decode::<engine::Hash>(&bytes).map_err(Into::into)
            }
            v => Err(RadixEngineToolkitError::DecodeError {
                error: format!("Invalid index byte: {v:?}"),
            }),
        }?;
        Ok(Arc::new(Self::from(native)))
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub fn as_str(&self) -> String {
        self.0.to_string()
    }
}

impl From<engine::Hash> for Hash {
    fn from(value: engine::Hash) -> Self {
        Self(value)
    }
}

impl From<Hash> for engine::Hash {
    fn from(value: Hash) -> Self {
        value.0
    }
}
