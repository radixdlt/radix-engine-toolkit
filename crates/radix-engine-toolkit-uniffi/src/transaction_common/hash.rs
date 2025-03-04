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

#[derive(Clone, Debug, Object)]
pub struct TransactionHash(
    pub(crate) engine::Hash,
    pub(crate) String,
    pub(crate) u8,
);

#[uniffi::export]
impl TransactionHash {
    #[uniffi::constructor]
    pub fn from_str(string: String, network_id: u8) -> Result<Arc<Self>> {
        let network_definition =
            engine::NetworkDefinition::from_network_id(network_id);
        let hash = toolkit::functions::utils::decode_transaction_id(
            &string,
            &network_definition,
        )?;
        Ok(Arc::new(Self(hash, string, network_id)))
    }

    pub fn as_hash(&self) -> Arc<Hash> {
        Arc::new(Hash(self.0))
    }

    pub fn as_str(&self) -> String {
        self.1.to_owned()
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub fn network_id(&self) -> u8 {
        self.2
    }
}

impl TransactionHash {
    pub fn new<T>(hash: &T, network_id: u8) -> Self
    where
        T: engine::IsTransactionHash + engine::IsHash,
    {
        let network_definition =
            engine::NetworkDefinition::from_network_id(network_id);
        let bech32_encoder =
            engine::TransactionHashBech32Encoder::new(&network_definition);
        let encoded = bech32_encoder
            .encode(hash)
            .expect("Bech32m encoding tx hashes cant fail");
        Self(*hash.as_hash(), encoded, network_id)
    }
}
