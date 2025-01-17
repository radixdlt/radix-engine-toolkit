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
fn withdraws_without_a_deposit_is_not_a_transfer_transaction() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = ledger.new_account(false);

    let manifest = ManifestBuilder::new()
        .lock_fee_and_withdraw(account, 1, XRD, 0)
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
    assert!(
        !manifest_classification.contains(&ManifestClassification::Transfer)
    );
    assert!(!detailed_manifest_classification.contains(
        &DetailedManifestClassification::Transfer {
            is_one_to_one_transfer: true
        }
    ));
    assert!(!detailed_manifest_classification.contains(
        &DetailedManifestClassification::Transfer {
            is_one_to_one_transfer: false
        }
    ));
}

#[test]
fn deposit_without_a_withdraw_is_not_a_transfer_transaction() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = ledger.new_account(false);

    let manifest = ManifestBuilder::new()
        .take_all_from_worktop(XRD, "bucket")
        .try_deposit_or_abort(account, None, "bucket")
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
    assert!(
        !manifest_classification.contains(&ManifestClassification::Transfer)
    );
    assert!(!detailed_manifest_classification.contains(
        &DetailedManifestClassification::Transfer {
            is_one_to_one_transfer: true
        }
    ));
    assert!(!detailed_manifest_classification.contains(
        &DetailedManifestClassification::Transfer {
            is_one_to_one_transfer: false
        }
    ));
}

#[test]
fn transfer_manifest_classifies_as_transfer_and_general() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();

    let (_, _, account1) = ledger.new_account(false);
    let (_, _, account2) = ledger.new_account(false);

    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account1, XRD, 10)
        .take_from_worktop(XRD, 10, "xrd")
        .try_deposit_or_abort(account2, None, "xrd")
        .build();

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert!(!static_analysis
        .reserved_instructions_summary
        .is_any_reserved_instruction_present());
    assert!(static_analysis.manifest_classification.iter().any(
        |classification| matches!(
            classification,
            ManifestClassification::Transfer
        )
    ));
    assert!(static_analysis.manifest_classification.iter().any(
        |classification| matches!(
            classification,
            ManifestClassification::General
        )
    ));
    assert!(dynamic_analysis
        .detailed_manifest_classification
        .iter()
        .any(|classification| matches!(
            classification,
            DetailedManifestClassification::Transfer {
                is_one_to_one_transfer: true
            }
        )));
    assert!(dynamic_analysis
        .detailed_manifest_classification
        .iter()
        .any(|classification| matches!(
            classification,
            DetailedManifestClassification::General
        )));
}

#[test]
fn transfer_manifest_with_access_controller_proof_classifies_as_transfer() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();

    let (account1, access_controller) =
        ledger.new_allow_all_access_controller();
    let (_, _, account2) = ledger.new_account(false);

    let manifest = ManifestBuilder::new()
        .call_method(access_controller, "create_proof", ())
        .lock_fee_and_withdraw(account1, 1, XRD, 10)
        .take_from_worktop(XRD, 10, "xrd")
        .try_deposit_or_abort(account2, None, "xrd")
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
    assert!(manifest_classification.contains(&ManifestClassification::Transfer));
    assert!(detailed_manifest_classification.contains(
        &DetailedManifestClassification::Transfer {
            is_one_to_one_transfer: true
        }
    ));
}

#[test]
fn transfer_manifest_with_entire_worktop_expression_classifies_as_transfer() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();

    let (_, _, account1) = ledger.new_account(false);
    let (_, _, account2) = ledger.new_account(false);

    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account1, XRD, 10)
        .try_deposit_entire_worktop_or_abort(account2, None)
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
    assert!(manifest_classification.contains(&ManifestClassification::Transfer));
    assert!(detailed_manifest_classification.contains(
        &DetailedManifestClassification::Transfer {
            is_one_to_one_transfer: false
        }
    ));
}

#[test]
fn transfer_with_lock_fee_manifest_classifies_as_transfer_but_has_reserved_instruction(
) {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();

    let (_, _, account1) = ledger.new_account(false);
    let (_, _, account2) = ledger.new_account(false);

    let manifest = ManifestBuilder::new()
        .lock_fee(account1, 10)
        .withdraw_from_account(account1, XRD, 10)
        .take_from_worktop(XRD, 10, "xrd")
        .try_deposit_or_abort(account2, None, "xrd")
        .build();

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert!(static_analysis
        .reserved_instructions_summary
        .has_account_lock_fee_invocations());
    assert!(static_analysis.manifest_classification.iter().any(
        |classification| matches!(
            classification,
            ManifestClassification::Transfer
        )
    ));
    assert!(dynamic_analysis
        .detailed_manifest_classification
        .iter()
        .any(|classification| matches!(
            classification,
            DetailedManifestClassification::Transfer { .. }
        )));
}

#[test]
fn multi_asset_transfer_manifest_classifies_as_transfer_and_general() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();

    let (_, _, account1) = ledger.new_account(false);
    let (_, _, account2) = ledger.new_account(false);

    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account1, XRD, 10)
        .take_from_worktop(XRD, 10, "xrd")
        .try_deposit_or_abort(account2, None, "xrd")
        .withdraw_from_account(account1, XRD, 10)
        .take_from_worktop(XRD, 10, "xrd1")
        .try_deposit_or_abort(account2, None, "xrd1")
        .build();

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert!(!static_analysis
        .reserved_instructions_summary
        .is_any_reserved_instruction_present());
    assert!(static_analysis.manifest_classification.iter().any(
        |classification| matches!(
            classification,
            ManifestClassification::Transfer
        )
    ));
    assert!(static_analysis.manifest_classification.iter().any(
        |classification| matches!(
            classification,
            ManifestClassification::General
        )
    ));
    assert!(dynamic_analysis
        .detailed_manifest_classification
        .iter()
        .any(|classification| matches!(
            classification,
            DetailedManifestClassification::Transfer {
                is_one_to_one_transfer: false
            }
        )));
    assert!(dynamic_analysis
        .detailed_manifest_classification
        .iter()
        .any(|classification| matches!(
            classification,
            DetailedManifestClassification::General
        )));
}

#[test]
fn transfer_manifest_with_or_refund_doesnt_classify_as_transfer_and_general() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();

    let (_, _, account1) = ledger.new_account(false);
    let (_, _, account2) = ledger.new_account(false);

    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account1, XRD, 10)
        .take_from_worktop(XRD, 10, "xrd")
        .try_deposit_or_refund(account2, None, "xrd")
        .build();

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(static_analysis.manifest_classification.len(), 0);
    assert_eq!(dynamic_analysis.detailed_manifest_classification.len(), 0);
}

#[test]
fn all_possible_combinations_of_simple_transfer_fsm_classify_as_simple_transfer(
) {
    use SimpleTransferAction::*;
    let paths: &[&[SimpleTransferAction]] = &[
        &[
            CreateAccessControllerProof,
            LockFee,
            Withdraw,
            TakeFromWorktop,
            Deposit,
        ],
        &[
            CreateAccessControllerProof,
            LockFee,
            Withdraw,
            TakeFromWorktop,
            TryDepositOrAbort,
        ],
        &[
            CreateAccessControllerProof,
            LockFee,
            WithdrawNonFungibles,
            TakeFromWorktop,
            Deposit,
        ],
        &[
            CreateAccessControllerProof,
            LockFee,
            WithdrawNonFungibles,
            TakeFromWorktop,
            TryDepositOrAbort,
        ],
        &[LockFee, Withdraw, TakeFromWorktop, Deposit],
        &[LockFee, Withdraw, TakeFromWorktop, TryDepositOrAbort],
        &[LockFee, WithdrawNonFungibles, TakeFromWorktop, Deposit],
        &[
            LockFee,
            WithdrawNonFungibles,
            TakeFromWorktop,
            TryDepositOrAbort,
        ],
        &[
            CreateAccessControllerProof,
            LockFeeAndWithdraw,
            TakeFromWorktop,
            Deposit,
        ],
        &[
            CreateAccessControllerProof,
            LockFeeAndWithdraw,
            TakeFromWorktop,
            TryDepositOrAbort,
        ],
        &[
            CreateAccessControllerProof,
            LockFeeAndWithdrawNonFungibles,
            TakeFromWorktop,
            Deposit,
        ],
        &[
            CreateAccessControllerProof,
            LockFeeAndWithdrawNonFungibles,
            TakeFromWorktop,
            TryDepositOrAbort,
        ],
        &[Withdraw, TakeFromWorktop, Deposit],
        &[Withdraw, TakeFromWorktop, TryDepositOrAbort],
        &[WithdrawNonFungibles, TakeFromWorktop, Deposit],
        &[WithdrawNonFungibles, TakeFromWorktop, TryDepositOrAbort],
        &[LockFeeAndWithdraw, TakeFromWorktop, Deposit],
        &[LockFeeAndWithdraw, TakeFromWorktop, TryDepositOrAbort],
        &[LockFeeAndWithdrawNonFungibles, TakeFromWorktop, Deposit],
        &[
            LockFeeAndWithdrawNonFungibles,
            TakeFromWorktop,
            TryDepositOrAbort,
        ],
    ];
    for path in paths {
        assert_simple_transfer_path_is_valid(path);
    }
}

fn assert_simple_transfer_path_is_valid(path: &[SimpleTransferAction]) {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (source_account, access_controller) =
        ledger.new_allow_all_access_controller();
    let (_, _, destination_account) = ledger.new_account(true);
    let resource_address = ledger.create_non_fungible_resource(source_account);
    let ids = (1..=3)
        .map(NonFungibleLocalId::integer)
        .collect::<IndexSet<_>>();

    let manifest = path
        .iter()
        .fold(ManifestBuilder::new(), |builder, action| match action {
            SimpleTransferAction::CreateAccessControllerProof => {
                builder.call_method(access_controller, "create_proof", ())
            }
            SimpleTransferAction::LockFee => {
                builder.lock_fee(source_account, 10)
            }
            SimpleTransferAction::Withdraw => builder.withdraw_from_account(
                source_account,
                resource_address,
                3,
            ),
            SimpleTransferAction::WithdrawNonFungibles => builder
                .withdraw_non_fungibles_from_account(
                    source_account,
                    resource_address,
                    ids.clone(),
                ),
            SimpleTransferAction::LockFeeAndWithdraw => builder
                .lock_fee_and_withdraw(source_account, 10, resource_address, 3),
            SimpleTransferAction::LockFeeAndWithdrawNonFungibles => builder
                .lock_fee_and_withdraw_non_fungibles(
                    source_account,
                    10,
                    resource_address,
                    ids.clone(),
                ),
            SimpleTransferAction::TakeFromWorktop => {
                builder.take_from_worktop(resource_address, 3, "bucket")
            }
            SimpleTransferAction::Deposit => {
                builder.deposit(destination_account, "bucket")
            }
            SimpleTransferAction::TryDepositOrAbort => builder
                .try_deposit_or_abort(destination_account, None, "bucket"),
        })
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
    assert!(manifest_classification.contains(&ManifestClassification::Transfer));
    assert!(detailed_manifest_classification.contains(
        &DetailedManifestClassification::Transfer {
            is_one_to_one_transfer: true
        }
    ));
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SimpleTransferAction {
    CreateAccessControllerProof,
    LockFee,
    LockFeeAndWithdraw,
    LockFeeAndWithdrawNonFungibles,
    Withdraw,
    WithdrawNonFungibles,
    TakeFromWorktop,
    Deposit,
    TryDepositOrAbort,
}
