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
fn non_simple_transfer_manifest_classifies_as_transfer_and_general() {
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
