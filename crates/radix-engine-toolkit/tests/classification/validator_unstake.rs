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
fn single_validator_unstake_classifies_as_validator_unstake_transaction() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (pk, _, account) = ledger.new_account(true);
    let (validator, lsu, claim_nft) = ledger.new_validator(pk, account);

    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .withdraw_from_account(account, XRD, 10)
        .take_all_from_worktop(XRD, "bucket")
        .stake_validator(validator, "bucket")
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();
    ledger
        .execute_manifest(
            manifest,
            vec![NonFungibleGlobalId::from_public_key(&pk)],
        )
        .expect_commit_success();

    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account, lsu, 5)
        .take_all_from_worktop(lsu, "bucket")
        .unstake_validator(validator, "bucket")
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
        .contains(&ManifestClassification::ValidatorUnstake));
    let Some(DetailedManifestClassification::ValidatorUnstake(
        ValidatorUnstakingOutput { unstake_operations },
    )) = detailed_manifest_classification.first()
    else {
        panic!("Not a validator unstake transaction")
    };
    assert_eq!(unstake_operations.len(), 1);

    let unstake_operation = unstake_operations.first().unwrap();
    assert_eq!(unstake_operation.liquid_stake_unit_amount, dec!(5));
    assert_eq!(unstake_operation.liquid_stake_unit_address, lsu);
    assert_eq!(unstake_operation.validator_address, validator);
    assert_eq!(unstake_operation.claim_nft_address, claim_nft);
    assert_eq!(unstake_operation.claim_nfts.len(), 1);
}

#[test]
fn single_validator_unstake_with_lock_fee_and_withdraw_classifies_as_validator_unstake_transaction(
) {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (pk, _, account) = ledger.new_account(true);
    let (validator, lsu, claim_nft) = ledger.new_validator(pk, account);

    let manifest = ManifestBuilder::new()
        .lock_fee_and_withdraw(account, 10, XRD, 10)
        .take_all_from_worktop(XRD, "bucket")
        .stake_validator(validator, "bucket")
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();
    ledger
        .execute_manifest(
            manifest,
            vec![NonFungibleGlobalId::from_public_key(&pk)],
        )
        .expect_commit_success();

    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account, lsu, 5)
        .take_all_from_worktop(lsu, "bucket")
        .unstake_validator(validator, "bucket")
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
        .contains(&ManifestClassification::ValidatorUnstake));
    let Some(DetailedManifestClassification::ValidatorUnstake(
        ValidatorUnstakingOutput { unstake_operations },
    )) = detailed_manifest_classification.first()
    else {
        panic!("Not a validator unstake transaction")
    };
    assert_eq!(unstake_operations.len(), 1);

    let unstake_operation = unstake_operations.first().unwrap();
    assert_eq!(unstake_operation.liquid_stake_unit_amount, dec!(5));
    assert_eq!(unstake_operation.liquid_stake_unit_address, lsu);
    assert_eq!(unstake_operation.validator_address, validator);
    assert_eq!(unstake_operation.claim_nft_address, claim_nft);
    assert_eq!(unstake_operation.claim_nfts.len(), 1);
}

#[test]
fn multiple_validator_unstake_classifies_as_validator_unstake_transaction() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (pk, _, account) = ledger.new_account(true);
    let (validator, lsu, claim_nft) = ledger.new_validator(pk, account);

    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .withdraw_from_account(account, XRD, 10)
        .take_all_from_worktop(XRD, "bucket")
        .stake_validator(validator, "bucket")
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();
    ledger
        .execute_manifest(
            manifest,
            vec![NonFungibleGlobalId::from_public_key(&pk)],
        )
        .expect_commit_success();

    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account, lsu, 5)
        .take_all_from_worktop(lsu, "bucket")
        .unstake_validator(validator, "bucket")
        .withdraw_from_account(account, lsu, 5)
        .take_all_from_worktop(lsu, "bucket1")
        .unstake_validator(validator, "bucket1")
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
        .contains(&ManifestClassification::ValidatorUnstake));
    let Some(DetailedManifestClassification::ValidatorUnstake(
        ValidatorUnstakingOutput { unstake_operations },
    )) = detailed_manifest_classification.first()
    else {
        panic!("Not a validator unstake transaction")
    };
    assert_eq!(unstake_operations.len(), 2);

    let unstake_operation = unstake_operations.first().unwrap();
    assert_eq!(unstake_operation.liquid_stake_unit_amount, dec!(5));
    assert_eq!(unstake_operation.liquid_stake_unit_address, lsu);
    assert_eq!(unstake_operation.validator_address, validator);
    assert_eq!(unstake_operation.claim_nft_address, claim_nft);
    assert_eq!(unstake_operation.claim_nfts.len(), 1);

    let unstake_operation = unstake_operations.get(1).unwrap();
    assert_eq!(unstake_operation.liquid_stake_unit_amount, dec!(5));
    assert_eq!(unstake_operation.liquid_stake_unit_address, lsu);
    assert_eq!(unstake_operation.validator_address, validator);
    assert_eq!(unstake_operation.claim_nft_address, claim_nft);
    assert_eq!(unstake_operation.claim_nfts.len(), 1);
}

#[test]
fn transfer_in_unstake_transaction_disqualifies_classification() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (pk, _, account) = ledger.new_account(true);
    let (validator, lsu, _) = ledger.new_validator(pk, account);

    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .withdraw_from_account(account, XRD, 10)
        .take_all_from_worktop(XRD, "bucket")
        .stake_validator(validator, "bucket")
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();
    ledger
        .execute_manifest(
            manifest,
            vec![NonFungibleGlobalId::from_public_key(&pk)],
        )
        .expect_commit_success();

    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account, lsu, 7)
        .take_from_worktop(lsu, 2, "transfer")
        .try_deposit_or_abort(account, None, "transfer")
        .take_all_from_worktop(lsu, "bucket")
        .unstake_validator(validator, "bucket")
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
    assert!(!manifest_classification
        .contains(&ManifestClassification::ValidatorUnstake));
    assert!(!detailed_manifest_classification.iter().any(
        |classification| matches!(
            classification,
            DetailedManifestClassification::ValidatorUnstake(..)
        )
    ));
}
