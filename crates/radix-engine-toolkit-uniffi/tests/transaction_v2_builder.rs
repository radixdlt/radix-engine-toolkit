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

use std::sync::*;

use radix_engine_toolkit_uniffi::builder::preview_transaction_v2_builder::PreviewTransactionV2Builder;
use radix_engine_toolkit_uniffi::prelude::*;
use scrypto_test::prelude::RawPreviewTransaction;

#[test]
fn subintent_transaction_hash_is_derived_correctly() -> Result<()> {
    // Arrange
    let [signer_private_key] = private_keys();

    let partial_transaction = SignedPartialTransactionV2Builder::new()
        .intent_header(IntentHeaderV2 {
            network_id: 0x01,
            start_epoch_inclusive: 1,
            end_epoch_exclusive: 10,
            min_proposer_timestamp_inclusive: None,
            max_proposer_timestamp_exclusive: None,
            intent_discriminator: 100,
        })
        .manifest(
            ManifestV2Builder::new(1)
                .drop_all_proofs()?
                .drop_auth_zone_proofs()?
                .drop_auth_zone_signature_proofs()?
                .yield_to_parent(vec![])?
                .build(),
        )
        .prepare_for_signing()?
        .sign_with_private_key(signer_private_key.clone())
        .build();

    // Act
    let subintent_hash = partial_transaction
        .partial_transaction()
        .root_subintent
        .subintent_hash()?
        .as_str();

    // Assert
    assert!(subintent_hash.starts_with("subtxid_"));

    Ok(())
}

#[test]
fn partial_transaction_builder_produces_valid_partial_transactions(
) -> Result<()> {
    // Arrange
    let [signer_private_key] = private_keys();

    // Act
    let partial_transaction = SignedPartialTransactionV2Builder::new()
        .intent_header(IntentHeaderV2 {
            network_id: 0x01,
            start_epoch_inclusive: 1,
            end_epoch_exclusive: 10,
            min_proposer_timestamp_inclusive: None,
            max_proposer_timestamp_exclusive: None,
            intent_discriminator: 100,
        })
        .manifest(
            ManifestV2Builder::new(1)
                .drop_all_proofs()?
                .drop_auth_zone_proofs()?
                .drop_auth_zone_signature_proofs()?
                .yield_to_parent(vec![])?
                .build(),
        )
        .prepare_for_signing()?
        .sign_with_private_key(signer_private_key.clone())
        .build();

    // Assert
    let partial_transaction = engine::SignedPartialTransactionV2::try_from(
        partial_transaction.as_ref().clone(),
    )?;
    let validator = engine::TransactionValidator::new_with_latest_config(
        &engine::NetworkDefinition::mainnet(),
    );
    validator
        .validate_signed_partial_transaction_v2(
            partial_transaction
                .prepare(&engine::PreparationSettings::latest())?,
        )
        .expect("Validation error");

    Ok(())
}

#[test]
fn partial_transaction_builder_produces_valid_partial_transactions_with_child_subintents(
) -> Result<()> {
    // Arrange
    let [signer_private_key] = private_keys();
    let child_partial_transaction = SignedPartialTransactionV2Builder::new()
        .intent_header(IntentHeaderV2 {
            network_id: 0x01,
            start_epoch_inclusive: 1,
            end_epoch_exclusive: 10,
            min_proposer_timestamp_inclusive: None,
            max_proposer_timestamp_exclusive: None,
            intent_discriminator: 100,
        })
        .manifest(
            ManifestV2Builder::new(1)
                .drop_all_proofs()?
                .drop_auth_zone_proofs()?
                .drop_auth_zone_signature_proofs()?
                .yield_to_parent(vec![])?
                .build(),
        )
        .prepare_for_signing()?
        .sign_with_private_key(signer_private_key.clone())
        .build();

    // Act
    let partial_transaction = SignedPartialTransactionV2Builder::new()
        .intent_header(IntentHeaderV2 {
            network_id: 0x01,
            start_epoch_inclusive: 1,
            end_epoch_exclusive: 10,
            min_proposer_timestamp_inclusive: None,
            max_proposer_timestamp_exclusive: None,
            intent_discriminator: 100,
        })
        .add_child(child_partial_transaction.clone())
        .manifest(
            ManifestV2Builder::new(1)
                .use_child(
                    child_partial_transaction.root_subintent_hash()?,
                    "subintent".into(),
                )?
                .yield_to_child("subintent".into(), vec![])?
                .yield_to_parent(vec![])?
                .build(),
        )
        .prepare_for_signing()?
        .sign_with_private_key(signer_private_key.clone())
        .build();

    // Assert
    let partial_transaction = engine::SignedPartialTransactionV2::try_from(
        partial_transaction.as_ref().clone(),
    )?;
    let validator = engine::TransactionValidator::new_with_latest_config(
        &engine::NetworkDefinition::mainnet(),
    );
    validator
        .validate_signed_partial_transaction_v2(
            partial_transaction
                .prepare(&engine::PreparationSettings::latest())?,
        )
        .expect("Validation error");

    Ok(())
}

#[test]
fn partial_transaction_builder_produces_valid_partial_transactions_with_multiple_layers_of_child_subintents(
) -> Result<()> {
    // Arrange
    let [signer_private_key] = private_keys();
    let child_child_partial_transaction =
        SignedPartialTransactionV2Builder::new()
            .intent_header(IntentHeaderV2 {
                network_id: 0x01,
                start_epoch_inclusive: 1,
                end_epoch_exclusive: 10,
                min_proposer_timestamp_inclusive: None,
                max_proposer_timestamp_exclusive: None,
                intent_discriminator: 100,
            })
            .manifest(
                ManifestV2Builder::new(1)
                    .drop_all_proofs()?
                    .drop_auth_zone_proofs()?
                    .drop_auth_zone_signature_proofs()?
                    .yield_to_parent(vec![])?
                    .build(),
            )
            .prepare_for_signing()?
            .sign_with_private_key(signer_private_key.clone())
            .build();
    let child_partial_transaction = SignedPartialTransactionV2Builder::new()
        .intent_header(IntentHeaderV2 {
            network_id: 0x01,
            start_epoch_inclusive: 1,
            end_epoch_exclusive: 10,
            min_proposer_timestamp_inclusive: None,
            max_proposer_timestamp_exclusive: None,
            intent_discriminator: 100,
        })
        .manifest(
            ManifestV2Builder::new(1)
                .use_child(
                    child_child_partial_transaction
                        .partial_transaction()
                        .root_subintent()
                        .subintent_hash()?,
                    "subintent".into(),
                )?
                .yield_to_child("subintent".into(), vec![])?
                .yield_to_parent(vec![])?
                .build(),
        )
        .add_child(child_child_partial_transaction)
        .prepare_for_signing()?
        .sign_with_private_key(signer_private_key.clone())
        .build();

    // Act
    let partial_transaction = SignedPartialTransactionV2Builder::new()
        .intent_header(IntentHeaderV2 {
            network_id: 0x01,
            start_epoch_inclusive: 1,
            end_epoch_exclusive: 10,
            min_proposer_timestamp_inclusive: None,
            max_proposer_timestamp_exclusive: None,
            intent_discriminator: 100,
        })
        .add_child(child_partial_transaction.clone())
        .manifest(
            ManifestV2Builder::new(1)
                .use_child(
                    child_partial_transaction.root_subintent_hash()?,
                    "subintent".into(),
                )?
                .yield_to_child("subintent".into(), vec![])?
                .yield_to_parent(vec![])?
                .build(),
        )
        .prepare_for_signing()?
        .sign_with_private_key(signer_private_key.clone())
        .build();

    // Assert
    let partial_transaction = engine::SignedPartialTransactionV2::try_from(
        partial_transaction.as_ref().clone(),
    )?;
    let validator = engine::TransactionValidator::new_with_latest_config(
        &engine::NetworkDefinition::mainnet(),
    );
    validator
        .validate_signed_partial_transaction_v2(
            partial_transaction
                .prepare(&engine::PreparationSettings::latest())?,
        )
        .expect("Validation error");

    Ok(())
}

#[test]
fn transaction_builder_v2_produces_statically_valid_transactions() -> Result<()>
{
    // Arrange
    let [notary_private_key, signer_private_key] = private_keys();

    // Act
    let transaction = TransactionV2Builder::new()
        .transaction_header(TransactionHeaderV2 {
            notary_public_key: notary_private_key.public_key(),
            notary_is_signatory: true,
            tip_basis_points: 0,
        })
        .intent_header(IntentHeaderV2 {
            network_id: 0x01,
            start_epoch_inclusive: 1,
            end_epoch_exclusive: 10,
            min_proposer_timestamp_inclusive: None,
            max_proposer_timestamp_exclusive: None,
            intent_discriminator: 100,
        })
        .manifest(
            ManifestV2Builder::new(1)
                .drop_all_proofs()?
                .drop_auth_zone_proofs()?
                .drop_auth_zone_signature_proofs()?
                .build(),
        )
        .prepare_for_signing()?
        .sign_with_private_key(signer_private_key.clone())
        .notarize_with_private_key(notary_private_key)?;

    // Assert
    let notarized_transaction =
        engine::NotarizedTransactionV2::try_from(transaction.as_ref().clone())?;
    let validator = engine::TransactionValidator::new_with_latest_config(
        &engine::NetworkDefinition::mainnet(),
    );
    validator
        .validate_notarized_v2(
            notarized_transaction
                .prepare(&engine::PreparationSettings::latest())?,
        )
        .expect("Validation failed");

    Ok(())
}

#[test]
fn transaction_builder_v2_produces_statically_valid_transactions_with_multiple_layers_of_child_subintents(
) -> Result<()> {
    // Arrange
    let [notary_private_key, signer_private_key] = private_keys();
    let child_child_partial_transaction =
        SignedPartialTransactionV2Builder::new()
            .intent_header(IntentHeaderV2 {
                network_id: 0x01,
                start_epoch_inclusive: 1,
                end_epoch_exclusive: 10,
                min_proposer_timestamp_inclusive: None,
                max_proposer_timestamp_exclusive: None,
                intent_discriminator: 100,
            })
            .manifest(
                ManifestV2Builder::new(1)
                    .drop_all_proofs()?
                    .drop_auth_zone_proofs()?
                    .drop_auth_zone_signature_proofs()?
                    .yield_to_parent(vec![])?
                    .build(),
            )
            .prepare_for_signing()?
            .sign_with_private_key(signer_private_key.clone())
            .build();
    let child_partial_transaction = SignedPartialTransactionV2Builder::new()
        .intent_header(IntentHeaderV2 {
            network_id: 0x01,
            start_epoch_inclusive: 1,
            end_epoch_exclusive: 10,
            min_proposer_timestamp_inclusive: None,
            max_proposer_timestamp_exclusive: None,
            intent_discriminator: 100,
        })
        .add_child(child_child_partial_transaction.clone())
        .manifest(
            ManifestV2Builder::new(1)
                .use_child(
                    child_child_partial_transaction.root_subintent_hash()?,
                    "subintent".into(),
                )?
                .yield_to_child("subintent".into(), vec![])?
                .yield_to_parent(vec![])?
                .build(),
        )
        .prepare_for_signing()?
        .sign_with_private_key(signer_private_key.clone())
        .build();

    // Act
    let transaction = TransactionV2Builder::new()
        .transaction_header(TransactionHeaderV2 {
            notary_public_key: notary_private_key.public_key(),
            notary_is_signatory: true,
            tip_basis_points: 0,
        })
        .intent_header(IntentHeaderV2 {
            network_id: 0x01,
            start_epoch_inclusive: 1,
            end_epoch_exclusive: 10,
            min_proposer_timestamp_inclusive: None,
            max_proposer_timestamp_exclusive: None,
            intent_discriminator: 100,
        })
        .add_child(child_partial_transaction.clone())
        .manifest(
            ManifestV2Builder::new(1)
                .use_child(
                    child_partial_transaction.root_subintent_hash()?,
                    "subintent".into(),
                )?
                .yield_to_child("subintent".into(), vec![])?
                .build(),
        )
        .prepare_for_signing()?
        .sign_with_private_key(signer_private_key.clone())
        .notarize_with_private_key(notary_private_key)?;

    // Assert
    let notarized_transaction =
        engine::NotarizedTransactionV2::try_from(transaction.as_ref().clone())?;
    let validator = engine::TransactionValidator::new_with_latest_config(
        &engine::NetworkDefinition::mainnet(),
    );
    validator
        .validate_notarized_v2(
            notarized_transaction
                .prepare(&engine::PreparationSettings::latest())?,
        )
        .expect("Validation failed");

    Ok(())
}

#[test]
fn preview_v2_builder_produces_valid_preview_transactions() -> Result<()> {
    // Arrange
    let [notary_private_key, signer_private_key] = private_keys();

    // Act
    let preview_transaction = PreviewTransactionV2Builder::new()
        .transaction_header(TransactionHeaderV2 {
            notary_public_key: notary_private_key.public_key(),
            notary_is_signatory: true,
            tip_basis_points: 0,
        })
        .intent_header(IntentHeaderV2 {
            network_id: 0x01,
            start_epoch_inclusive: 1,
            end_epoch_exclusive: 10,
            min_proposer_timestamp_inclusive: None,
            max_proposer_timestamp_exclusive: None,
            intent_discriminator: 100,
        })
        .manifest(
            ManifestV2Builder::new(1)
                .drop_all_proofs()?
                .drop_auth_zone_proofs()?
                .drop_auth_zone_signature_proofs()?
                .build(),
        )
        .add_root_intent_signer(signer_private_key.public_key())
        .build()?;

    // Assert
    let preview_transaction = engine::PreviewTransactionV2::from_raw(
        &RawPreviewTransaction::from_vec(preview_transaction),
    )
    .expect("Failed");
    let transaction_validator =
        engine::TransactionValidator::new_with_latest_config(
            &engine::NetworkDefinition::mainnet(),
        );
    preview_transaction
        .prepare_and_validate(&transaction_validator)
        .expect("Validation failed!");

    Ok(())
}

fn private_keys<const N: usize>() -> [Arc<PrivateKey>; N] {
    std::array::from_fn(|i| i + 1)
        .map(|value| value as u64)
        .map(|value| engine::Ed25519PrivateKey::from_u64(value).unwrap())
        .map(|value| PrivateKey(engine::PrivateKey::Ed25519(value)))
        .map(Arc::new)
}
