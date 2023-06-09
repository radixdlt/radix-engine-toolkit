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

use native_transaction::{
    ecdsa_secp256k1::EcdsaSecp256k1PrivateKey, eddsa_ed25519::EddsaEd25519PrivateKey,
};

use crate::model::transaction::TransactionHeader;

pub fn header1() -> TransactionHeader {
    TransactionHeader {
        version: 0x01,
        network_id: 0x01,
        start_epoch_inclusive: 100,
        end_epoch_exclusive: 105,
        nonce: 5144,
        notary_public_key: EcdsaSecp256k1PrivateKey::from_u64(1)
            .unwrap()
            .public_key()
            .into(),
        notary_as_signatory: false,
        cost_unit_limit: 100_000_000,
        tip_percentage: 12,
    }
}

pub fn header2() -> TransactionHeader {
    TransactionHeader {
        version: 0x01,
        network_id: 0x01,
        start_epoch_inclusive: 100,
        end_epoch_exclusive: 105,
        nonce: 5144,
        notary_public_key: EddsaEd25519PrivateKey::from_u64(1)
            .unwrap()
            .public_key()
            .into(),
        notary_as_signatory: false,
        cost_unit_limit: 100_000_000,
        tip_percentage: 12,
    }
}