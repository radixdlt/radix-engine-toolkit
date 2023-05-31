#![allow(unused)]

use radix_engine_toolkit::functions::*;
use scrypto::prelude::*;
use transaction::builder::*;
use transaction::model::*;
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
            start_epoch_inclusive: 100,
            end_epoch_exclusive: 200,
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
    let IntentV1 {
        instructions,
        blobs,
        ..
    } = intent();
    TransactionManifestV1 {
        instructions: instructions.0,
        blobs: blobs
            .blobs
            .into_iter()
            .map(|blob| (hash(&blob.0), blob.0))
            .collect(),
    }
}

pub fn private_key1() -> EcdsaSecp256k1PrivateKey {
    EcdsaSecp256k1PrivateKey::from_u64(1).unwrap()
}

pub fn private_key2() -> EddsaEd25519PrivateKey {
    EddsaEd25519PrivateKey::from_u64(1).unwrap()
}

pub fn private_key3() -> EcdsaSecp256k1PrivateKey {
    EcdsaSecp256k1PrivateKey::from_u64(2).unwrap()
}

pub fn private_key4() -> EddsaEd25519PrivateKey {
    EddsaEd25519PrivateKey::from_u64(2).unwrap()
}
