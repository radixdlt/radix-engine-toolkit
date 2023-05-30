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

use super::header::header1;
use crate::model::transaction::{
    InstructionKind, InstructionList, NotarizedTransaction, SignedTransactionIntent,
    TransactionIntent, TransactionManifest,
};
use native_transaction::{
    builder::TransactionBuilder, ecdsa_secp256k1::EcdsaSecp256k1PrivateKey,
    eddsa_ed25519::EddsaEd25519PrivateKey, prelude::TransactionManifestV1,
};
use radix_engine_common::prelude::hash;

pub fn manifest() -> TransactionManifest {
    TransactionManifest {
        instructions: InstructionList::String("DROP_ALL_PROOFS;".to_owned()),
        blobs: vec![],
    }
}

pub fn intent() -> TransactionIntent {
    TransactionIntent {
        header: header1(),
        manifest: manifest(),
    }
}

pub fn signed_intent() -> SignedTransactionIntent {
    let notary_private_key = EcdsaSecp256k1PrivateKey::from_u64(1).unwrap();
    let intent = intent();

    let native_intent = intent.to_native_transaction_intent().unwrap();
    let transaction = TransactionBuilder::new()
        .header(native_intent.header)
        .manifest(TransactionManifestV1 {
            instructions: native_intent.instructions.0,
            blobs: native_intent
                .blobs
                .blobs
                .iter()
                .map(|blob| (hash(&blob.0), blob.0.clone()))
                .collect(),
        })
        .sign(&EcdsaSecp256k1PrivateKey::from_u64(2).unwrap())
        .sign(&EcdsaSecp256k1PrivateKey::from_u64(3).unwrap())
        .sign(&EcdsaSecp256k1PrivateKey::from_u64(4).unwrap())
        .sign(&EcdsaSecp256k1PrivateKey::from_u64(5).unwrap())
        .sign(&EddsaEd25519PrivateKey::from_u64(2).unwrap())
        .sign(&EddsaEd25519PrivateKey::from_u64(3).unwrap())
        .sign(&EddsaEd25519PrivateKey::from_u64(4).unwrap())
        .sign(&EddsaEd25519PrivateKey::from_u64(5).unwrap())
        .notarize(&notary_private_key)
        .build();
    SignedTransactionIntent::from_native_signed_transaction_intent(
        &transaction.signed_intent,
        InstructionKind::Parsed,
    )
    .unwrap()
}

pub fn notarized_intent() -> NotarizedTransaction {
    let notary_private_key = EcdsaSecp256k1PrivateKey::from_u64(1).unwrap();
    let intent = intent();

    let native_intent = intent.to_native_transaction_intent().unwrap();
    let transaction = TransactionBuilder::new()
        .header(native_intent.header)
        .manifest(TransactionManifestV1 {
            instructions: native_intent.instructions.0,
            blobs: native_intent
                .blobs
                .blobs
                .iter()
                .map(|blob| (hash(&blob.0), blob.0.clone()))
                .collect(),
        })
        .sign(&EcdsaSecp256k1PrivateKey::from_u64(2).unwrap())
        .sign(&EcdsaSecp256k1PrivateKey::from_u64(3).unwrap())
        .sign(&EcdsaSecp256k1PrivateKey::from_u64(4).unwrap())
        .sign(&EcdsaSecp256k1PrivateKey::from_u64(5).unwrap())
        .sign(&EddsaEd25519PrivateKey::from_u64(2).unwrap())
        .sign(&EddsaEd25519PrivateKey::from_u64(3).unwrap())
        .sign(&EddsaEd25519PrivateKey::from_u64(4).unwrap())
        .sign(&EddsaEd25519PrivateKey::from_u64(5).unwrap())
        .notarize(&notary_private_key)
        .build();
    NotarizedTransaction::from_native_notarized_transaction_intent(
        &transaction,
        InstructionKind::Parsed,
    )
    .unwrap()
}
