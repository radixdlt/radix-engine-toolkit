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

use super::access_controller_utils::*;
use crate::prelude::*;

fn rule_set() -> ManifestRuleSet {
    RuleSet {
        primary_role: rule!(allow_all),
        recovery_role: rule!(allow_all),
        confirmation_role: rule!(allow_all),
    }
    .into()
}

fn is_access_controller_class(class: &ManifestClassification) -> bool {
    ACCESS_CONTROLLER_CLASSES.contains(class)
}

fn classify(manifest: &SubintentManifestV2) -> Vec<ManifestClassification> {
    let StaticAnalysis {
        manifest_classification,
        ..
    } = statically_analyze(manifest).unwrap();
    manifest_classification.into_iter().collect()
}

#[test]
fn recovery_subintent_is_classified_as_recovery() {
    // Arrange
    let mut allocator = TestAddressAllocator::new();
    let access_controller = allocator.new_access_controller_address();
    let manifest = ManifestBuilder::new_subintent_v2()
        .call_method(
            access_controller,
            ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_RECOVERY_IDENT,
            AccessControllerInitiateRecoveryAsRecoveryManifestInput {
                rule_set: rule_set(),
                timed_recovery_delay_in_minutes: None,
            },
        )
        .call_method(
            access_controller,
            ACCESS_CONTROLLER_QUICK_CONFIRM_RECOVERY_ROLE_RECOVERY_PROPOSAL_IDENT,
            AccessControllerQuickConfirmRecoveryRoleRecoveryProposalManifestInput {
                rule_set: rule_set(),
                timed_recovery_delay_in_minutes: None,
            },
        )
        .yield_to_parent(())
        .build();

    // Act
    let classification = classify(&manifest);

    // Assert
    assert!(classification
        .contains(&ManifestClassification::AccessControllerRecovery));
}

#[test]
fn timed_recovery_subintent_with_verify_parent_is_classified_as_timed_recovery()
{
    // Arrange
    let mut allocator = TestAddressAllocator::new();
    let access_controller = allocator.new_access_controller_address();
    let manifest = ManifestBuilder::new_subintent_v2()
        .verify_parent(rule!(allow_all))
        .call_method(
            access_controller,
            ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_RECOVERY_IDENT,
            AccessControllerInitiateRecoveryAsRecoveryManifestInput {
                rule_set: rule_set(),
                timed_recovery_delay_in_minutes: Some(10),
            },
        )
        .yield_to_parent(())
        .build();

    // Act
    let classification = classify(&manifest);

    // Assert
    assert!(classification
        .contains(&ManifestClassification::AccessControllerTimedRecovery));
}

#[test]
fn recovery_subintent_with_yield_to_child_is_not_classified_as_recovery() {
    // Arrange
    let mut allocator = TestAddressAllocator::new();
    let access_controller = allocator.new_access_controller_address();
    let manifest = ManifestBuilder::new_subintent_v2()
        .use_child("child", SubintentHash(Hash([0; 32])))
        .call_method(
            access_controller,
            ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_RECOVERY_IDENT,
            AccessControllerInitiateRecoveryAsRecoveryManifestInput {
                rule_set: rule_set(),
                timed_recovery_delay_in_minutes: None,
            },
        )
        .yield_to_child("child", ())
        .yield_to_parent(())
        .build();

    // Act
    let classification = classify(&manifest);

    // Assert
    assert!(!classification.iter().any(is_access_controller_class));
}

#[test]
fn stop_timed_recovery_subintent_is_classified_as_cancel_recovery() {
    // Arrange
    let mut allocator = TestAddressAllocator::new();
    let access_controller = allocator.new_access_controller_address();
    let manifest = ManifestBuilder::new_subintent_v2()
        .call_method(
            access_controller,
            ACCESS_CONTROLLER_STOP_TIMED_RECOVERY_IDENT,
            AccessControllerStopTimedRecoveryManifestInput {
                rule_set: rule_set(),
                timed_recovery_delay_in_minutes: None,
            },
        )
        .call_method(
            access_controller,
            ACCESS_CONTROLLER_CANCEL_RECOVERY_ROLE_RECOVERY_PROPOSAL_IDENT,
            AccessControllerCancelRecoveryRoleRecoveryProposalManifestInput {},
        )
        .yield_to_parent(())
        .build();

    // Act
    let classification = classify(&manifest);

    // Assert
    assert!(classification
        .contains(&ManifestClassification::AccessControllerCancelRecovery));
}

#[test]
fn confirm_timed_recovery_subintent_is_classified_as_confirm_recovery() {
    // Arrange
    let mut allocator = TestAddressAllocator::new();
    let access_controller = allocator.new_access_controller_address();
    let manifest = ManifestBuilder::new_subintent_v2()
        .call_method(
            access_controller,
            ACCESS_CONTROLLER_TIMED_CONFIRM_RECOVERY_IDENT,
            AccessControllerTimedConfirmRecoveryManifestInput {
                rule_set: rule_set(),
                timed_recovery_delay_in_minutes: Some(10),
            },
        )
        .yield_to_parent(())
        .build();

    // Act
    let classification = classify(&manifest);

    // Assert
    assert!(classification
        .contains(&ManifestClassification::AccessControllerConfirmRecovery));
}

#[test]
fn recovery_subintent_yielding_resources_to_parent_is_not_classified_as_recovery(
) {
    // Arrange
    let mut allocator = TestAddressAllocator::new();
    let access_controller = allocator.new_access_controller_address();
    let manifest = ManifestBuilder::new_subintent_v2()
        .call_method(
            access_controller,
            ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_RECOVERY_IDENT,
            AccessControllerInitiateRecoveryAsRecoveryManifestInput {
                rule_set: rule_set(),
                timed_recovery_delay_in_minutes: None,
            },
        )
        .yield_to_parent((ManifestExpression::EntireWorktop,))
        .build();

    // Act
    let classification = classify(&manifest);

    // Assert
    assert!(!classification.iter().any(is_access_controller_class));
}

/// Asserts that an enclosed subintent (one that starts by asserting the
/// worktop is empty) gets the expected classification, both as a subintent and
/// as the transaction manifest that `as_enclosed` turns it into.
fn assert_enclosed_classification(
    builder: ManifestBuilder<SubintentManifestV2>,
    expected: ManifestClassification,
) {
    let manifest = builder.yield_to_parent(()).build();
    assert!(
        classify(&manifest).contains(&expected),
        "subintent not classified as {expected:?}"
    );

    let enclosed = radix_engine_toolkit::functions::transaction_v2::subintent_manifest::as_enclosed(&manifest)
        .expect("Must be an enclosed subintent");
    let StaticAnalysis {
        manifest_classification,
        ..
    } = statically_analyze(&enclosed).unwrap();
    assert!(
        manifest_classification.contains(&expected),
        "enclosed manifest not classified as {expected:?}"
    );
}

#[test]
fn enclosed_recovery_subintent_is_classified_as_recovery() {
    let mut allocator = TestAddressAllocator::new();
    let access_controller = allocator.new_access_controller_address();
    assert_enclosed_classification(
        ManifestBuilder::new_subintent_v2()
            .assert_worktop_is_empty()
            .call_method(
                access_controller,
                ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_RECOVERY_IDENT,
                AccessControllerInitiateRecoveryAsRecoveryManifestInput {
                    rule_set: rule_set(),
                    timed_recovery_delay_in_minutes: None,
                },
            )
            .call_method(
                access_controller,
                ACCESS_CONTROLLER_QUICK_CONFIRM_RECOVERY_ROLE_RECOVERY_PROPOSAL_IDENT,
                AccessControllerQuickConfirmRecoveryRoleRecoveryProposalManifestInput {
                    rule_set: rule_set(),
                    timed_recovery_delay_in_minutes: None,
                },
            ),
        ManifestClassification::AccessControllerRecovery,
    );
}

#[test]
fn enclosed_stop_timed_recovery_subintent_is_classified_as_cancel_recovery() {
    let mut allocator = TestAddressAllocator::new();
    let access_controller = allocator.new_access_controller_address();
    assert_enclosed_classification(
        ManifestBuilder::new_subintent_v2()
            .assert_worktop_is_empty()
            .call_method(
                access_controller,
                ACCESS_CONTROLLER_STOP_TIMED_RECOVERY_IDENT,
                AccessControllerStopTimedRecoveryManifestInput {
                    rule_set: rule_set(),
                    timed_recovery_delay_in_minutes: None,
                },
            )
            .call_method(
                access_controller,
                ACCESS_CONTROLLER_CANCEL_RECOVERY_ROLE_RECOVERY_PROPOSAL_IDENT,
                AccessControllerCancelRecoveryRoleRecoveryProposalManifestInput {},
            ),
        ManifestClassification::AccessControllerCancelRecovery,
    );
}

#[test]
fn enclosed_confirm_timed_recovery_subintent_is_classified_as_confirm_recovery()
{
    let mut allocator = TestAddressAllocator::new();
    let access_controller = allocator.new_access_controller_address();
    assert_enclosed_classification(
        ManifestBuilder::new_subintent_v2()
            .assert_worktop_is_empty()
            .call_method(
                access_controller,
                ACCESS_CONTROLLER_TIMED_CONFIRM_RECOVERY_IDENT,
                AccessControllerTimedConfirmRecoveryManifestInput {
                    rule_set: rule_set(),
                    timed_recovery_delay_in_minutes: Some(10),
                },
            ),
        ManifestClassification::AccessControllerConfirmRecovery,
    );
}

#[test]
fn asserting_worktop_contains_only_some_resources_is_not_recovery() {
    let mut allocator = TestAddressAllocator::new();
    let access_controller = allocator.new_access_controller_address();
    let manifest = ManifestBuilder::new_subintent_v2()
        .assert_worktop_resources_only(
            ManifestResourceConstraints::new().with_exact_amount(XRD, 0),
        )
        .call_method(
            access_controller,
            ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_RECOVERY_IDENT,
            AccessControllerInitiateRecoveryAsRecoveryManifestInput {
                rule_set: rule_set(),
                timed_recovery_delay_in_minutes: None,
            },
        )
        .yield_to_parent(())
        .build();

    assert!(!classify(&manifest).iter().any(is_access_controller_class));
}
