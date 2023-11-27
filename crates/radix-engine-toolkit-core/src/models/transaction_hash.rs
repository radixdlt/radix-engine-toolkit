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

use scrypto::prelude::*;
use transaction::prelude::{HashHasHrp, TransactionHashBech32Encoder};

pub struct TransactionHash {
    pub hash: Hash,
    pub id: String,
}

impl TransactionHash {
    pub fn new<H>(transaction_hash: H, network_id: u8) -> Self
    where
        H: HashHasHrp + IsHash,
    {
        let network_definition =
            crate::utils::network_definition_from_network_id(network_id);
        let encoder = TransactionHashBech32Encoder::new(&network_definition);
        let hash = *transaction_hash.as_hash();
        let id = encoder.encode(&transaction_hash).unwrap();
        Self { hash, id }
    }
}
