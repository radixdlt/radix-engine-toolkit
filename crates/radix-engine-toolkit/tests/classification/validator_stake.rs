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
fn single_validator_stake_classifies_as_validator_stake_transaction() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (pk, _, account) = ledger.new_account(true);
    let (validator, lsu, _) = ledger.new_validator(pk, account);

    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account, XRD, 10)
        .take_all_from_worktop(XRD, "bucket")
        .stake_validator(validator, "bucket")
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
        .contains(&ManifestClassification::ValidatorStake));
    let Some(DetailedManifestClassification::ValidatorStake(
        ValidatorStakingOutput { stake_operations },
    )) = detailed_manifest_classification.last()
    else {
        panic!("Not a validator stake transaction")
    };
    assert_eq!(stake_operations.len(), 1);
    assert_eq!(
        stake_operations.first(),
        Some(&ValidatorStakeOperation {
            validator_address: validator,
            staked_xrd_amount: 10.into(),
            liquid_stake_unit_resource_address: lsu,
            liquid_stake_unit_amount: 10.into()
        })
    )
}

#[test]
fn withdraw_of_non_xrd_fungible_resource_kicks_manifest_out_of_validator_stake()
{
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (pk, _, account) = ledger.new_account(true);
    let (validator, _, _) = ledger.new_validator(pk, account);
    let resource_address =
        ledger.create_fungible_resource(10000.into(), 18, account);

    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account, resource_address, 0)
        .withdraw_from_account(account, XRD, 10)
        .take_all_from_worktop(XRD, "bucket")
        .stake_validator(validator, "bucket")
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
        .contains(&ManifestClassification::ValidatorStake));
    assert!(!detailed_manifest_classification.iter().any(
        |detailed_classification| matches!(
            detailed_classification,
            DetailedManifestClassification::ValidatorStake(..)
        )
    ));
}

#[test]
fn withdraw_of_non_xrd_non_fungible_resource_kicks_manifest_out_of_validator_stake(
) {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (pk, _, account) = ledger.new_account(true);
    let (validator, _, _) = ledger.new_validator(pk, account);
    let resource_address = ledger.create_non_fungible_resource(account);

    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account, resource_address, 0)
        .withdraw_from_account(account, XRD, 10)
        .take_all_from_worktop(XRD, "bucket")
        .stake_validator(validator, "bucket")
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
        .contains(&ManifestClassification::ValidatorStake));
    assert!(!detailed_manifest_classification.iter().any(
        |detailed_classification| matches!(
            detailed_classification,
            DetailedManifestClassification::ValidatorStake(..)
        )
    ));
}

#[test]
fn transfer_in_stake_transaction_disqualifies_classification() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (pk, _, account) = ledger.new_account(true);
    let (validator, _, _) = ledger.new_validator(pk, account);

    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account, XRD, 10)
        .take_all_from_worktop(XRD, "bucket")
        .stake_validator(validator, "bucket")
        .withdraw_from_account(account, XRD, 10)
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
        .contains(&ManifestClassification::ValidatorStake));
    assert!(!detailed_manifest_classification.iter().any(
        |detailed_classification| matches!(
            detailed_classification,
            DetailedManifestClassification::ValidatorStake(..)
        )
    ));
}

#[test]
fn multiple_validator_stakes_classifies_as_validator_stake_transaction() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (pk, _, account) = ledger.new_account(true);
    let (validator1, lsu1, _) = ledger.new_validator(pk, account);
    let (validator2, lsu2, _) = ledger.new_validator(pk, account);

    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account, XRD, 10)
        .take_all_from_worktop(XRD, "bucket1")
        .stake_validator(validator1, "bucket1")
        .withdraw_from_account(account, XRD, 20)
        .take_all_from_worktop(XRD, "bucket2")
        .stake_validator(validator2, "bucket2")
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
        .contains(&ManifestClassification::ValidatorStake));
    let Some(DetailedManifestClassification::ValidatorStake(
        ValidatorStakingOutput {
            mut stake_operations,
        },
    )) = detailed_manifest_classification.last().cloned()
    else {
        panic!("Not a validator stake transaction")
    };
    assert_eq!(stake_operations.len(), 2);
    assert_eq!(
        stake_operations.remove(0),
        ValidatorStakeOperation {
            validator_address: validator1,
            staked_xrd_amount: 10.into(),
            liquid_stake_unit_resource_address: lsu1,
            liquid_stake_unit_amount: 10.into()
        }
    );
    assert_eq!(
        stake_operations.remove(0),
        ValidatorStakeOperation {
            validator_address: validator2,
            staked_xrd_amount: 20.into(),
            liquid_stake_unit_resource_address: lsu2,
            liquid_stake_unit_amount: 20.into()
        }
    )
}
