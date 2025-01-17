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

#[test]
fn single_validator_claim_is_classified_as_validator_claim_transaction() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (pk, _, account) = ledger.new_account(true);
    let (validator, lsu, claim_nft) = ledger.new_validator(pk, account);

    let manifest1 = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .withdraw_from_account(account, XRD, 10)
        .take_all_from_worktop(XRD, "bucket")
        .stake_validator(validator, "bucket")
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();
    let manifest2 = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .withdraw_from_account(account, lsu, 5)
        .take_all_from_worktop(lsu, "bucket")
        .unstake_validator(validator, "bucket")
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();

    [manifest1, manifest2].into_iter().for_each(|manifest| {
        ledger
            .execute_manifest(
                manifest,
                vec![NonFungibleGlobalId::from_public_key(&pk)],
            )
            .expect_commit_success();
    });

    ledger.set_current_epoch(Epoch::of(10_000));

    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account, claim_nft, 1)
        .take_all_from_worktop(claim_nft, "bucket")
        .claim_xrd(validator, "bucket")
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();

    // Act
    let (
        StaticAnalysis {
            manifest_classification,
            ..
        },
        DynamicAnalysis {
            detailed_manifest_classification,
            ..
        },
    ) = ledger.analyze(manifest);

    // Assert
    assert!(manifest_classification
        .contains(&ManifestClassification::ValidatorClaimXrd));
    let Some(DetailedManifestClassification::ValidatorClaimXrd(
        ValidatorClaimingXrdOutput { claim_operations },
    )) = detailed_manifest_classification.last()
    else {
        panic!("Not a validator claim transaction")
    };
    assert_eq!(claim_operations.len(), 1);

    let claim_operation = claim_operations.first().unwrap();
    assert_eq!(claim_operation.validator_address, validator);
    assert_eq!(claim_operation.claim_nft_address, claim_nft);
    assert_eq!(claim_operation.xrd_amount, dec!(5));
}

#[test]
fn single_validator_claim_with_withdraw_of_non_fungibles_is_classified_as_validator_claim_transaction(
) {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (pk, _, account) = ledger.new_account(true);
    let (validator, lsu, claim_nft) = ledger.new_validator(pk, account);

    let manifest1 = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .withdraw_from_account(account, XRD, 10)
        .take_all_from_worktop(XRD, "bucket")
        .stake_validator(validator, "bucket")
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();
    let manifest2 = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .withdraw_from_account(account, lsu, 5)
        .take_all_from_worktop(lsu, "bucket")
        .unstake_validator(validator, "bucket")
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();

    [manifest1, manifest2].into_iter().for_each(|manifest| {
        ledger
            .execute_manifest(
                manifest,
                vec![NonFungibleGlobalId::from_public_key(&pk)],
            )
            .expect_commit_success();
    });

    ledger.set_current_epoch(Epoch::of(10_000));
    let claim_nft_id = ledger
        .get_component_vaults(account, claim_nft)
        .first()
        .copied()
        .map(|address| {
            ledger
                .inspect_non_fungible_vault(address)
                .unwrap()
                .1
                .next()
                .unwrap()
        })
        .unwrap();

    let manifest = ManifestBuilder::new()
        .withdraw_non_fungibles_from_account(account, claim_nft, [claim_nft_id])
        .take_all_from_worktop(claim_nft, "bucket")
        .claim_xrd(validator, "bucket")
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();

    // Act
    let (
        StaticAnalysis {
            manifest_classification,
            ..
        },
        DynamicAnalysis {
            detailed_manifest_classification,
            ..
        },
    ) = ledger.analyze(manifest);

    // Assert
    assert!(manifest_classification
        .contains(&ManifestClassification::ValidatorClaimXrd));
    let Some(DetailedManifestClassification::ValidatorClaimXrd(
        ValidatorClaimingXrdOutput { claim_operations },
    )) = detailed_manifest_classification.last()
    else {
        panic!("Not a validator claim transaction")
    };
    assert_eq!(claim_operations.len(), 1);

    let claim_operation = claim_operations.first().unwrap();
    assert_eq!(claim_operation.validator_address, validator);
    assert_eq!(claim_operation.claim_nft_address, claim_nft);
    assert_eq!(claim_operation.xrd_amount, dec!(5));
}

#[test]
fn transfer_in_claim_transaction_qualifies_for_classification_but_not_detailed_classification(
) {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (pk, _, account) = ledger.new_account(true);
    let (validator, lsu, claim_nft) = ledger.new_validator(pk, account);

    let manifest1 = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .withdraw_from_account(account, XRD, 10)
        .take_all_from_worktop(XRD, "bucket")
        .stake_validator(validator, "bucket")
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();
    let manifest2 = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .withdraw_from_account(account, lsu, 5)
        .take_all_from_worktop(lsu, "bucket")
        .unstake_validator(validator, "bucket")
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();

    [manifest1, manifest2].into_iter().for_each(|manifest| {
        ledger
            .execute_manifest(
                manifest,
                vec![NonFungibleGlobalId::from_public_key(&pk)],
            )
            .expect_commit_success();
    });

    ledger.set_current_epoch(Epoch::of(10_000));

    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account, XRD, 1)
        .take_all_from_worktop(XRD, "xrd")
        .try_deposit_or_abort(account, None, "xrd")
        .withdraw_from_account(account, claim_nft, 1)
        .take_all_from_worktop(claim_nft, "bucket")
        .claim_xrd(validator, "bucket")
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();

    // Act
    let (
        StaticAnalysis {
            manifest_classification,
            ..
        },
        DynamicAnalysis {
            detailed_manifest_classification,
            ..
        },
    ) = ledger.analyze(manifest);

    // Assert
    assert!(manifest_classification
        .contains(&ManifestClassification::ValidatorClaimXrd));
    assert!(!detailed_manifest_classification.iter().any(
        |classification| matches!(
            classification,
            DetailedManifestClassification::ValidatorClaimXrd(..)
        )
    ));
}
