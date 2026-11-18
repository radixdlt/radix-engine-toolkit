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

fn rule_set() -> ManifestRuleSet {
    RuleSet {
        primary_role: rule!(allow_all),
        recovery_role: rule!(allow_all),
        confirmation_role: rule!(allow_all),
    }
    .into()
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
fn recovery_subintent_with_verify_parent_is_classified_as_recovery() {
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
        .contains(&ManifestClassification::AccessControllerRecovery));
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
    assert!(!classification
        .contains(&ManifestClassification::AccessControllerRecovery));
}

#[test]
fn stop_timed_recovery_subintent_is_classified_as_stop_timed_recovery() {
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
        .contains(&ManifestClassification::AccessControllerStopTimedRecovery));
}

#[test]
fn confirm_timed_recovery_subintent_is_classified_as_confirm_timed_recovery() {
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
    assert!(classification.contains(
        &ManifestClassification::AccessControllerConfirmTimedRecovery
    ));
}
