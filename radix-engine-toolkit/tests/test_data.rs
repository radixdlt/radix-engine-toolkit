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

#![allow(unused)]

use radix_engine_toolkit::functions::*;
use scrypto::prelude::*;
use transaction::builder::*;
use transaction::model::*;
use transaction::prelude::*;
use transaction::validation::*;

pub fn notarized_transaction() -> NotarizedTransactionV1 {
    let account1 = derive::virtual_account_address_from_public_key(&private_key1().public_key());
    let account2 = derive::virtual_account_address_from_public_key(&private_key2().public_key());

    TransactionBuilder::new()
        .manifest(
            ManifestBuilder::new()
                .withdraw_from_account(account1, RADIX_TOKEN, 10.into())
                .try_deposit_batch_or_abort(account2)
                .build(),
        )
        .header(TransactionHeaderV1 {
            network_id: 0x01,
            start_epoch_inclusive: Epoch::of(100),
            end_epoch_exclusive: Epoch::of(200),
            nonce: 100,
            notary_public_key: private_key1().public_key().into(),
            notary_is_signatory: true,
            tip_percentage: 0,
        })
        .sign(&private_key2())
        .sign(&private_key3())
        .sign(&private_key4())
        .notarize(&private_key1())
        .build()
}

pub fn signed_intent() -> SignedIntentV1 {
    notarized_transaction().signed_intent
}

pub fn intent() -> IntentV1 {
    signed_intent().intent
}

pub fn manifest() -> TransactionManifestV1 {
    radix_engine_toolkit::utils::manifest_from_intent(&intent())
}

pub fn private_key1() -> Secp256k1PrivateKey {
    Secp256k1PrivateKey::from_u64(1).unwrap()
}

pub fn private_key2() -> Ed25519PrivateKey {
    Ed25519PrivateKey::from_u64(1).unwrap()
}

pub fn private_key3() -> Secp256k1PrivateKey {
    Secp256k1PrivateKey::from_u64(2).unwrap()
}

pub fn private_key4() -> Ed25519PrivateKey {
    Ed25519PrivateKey::from_u64(2).unwrap()
}
