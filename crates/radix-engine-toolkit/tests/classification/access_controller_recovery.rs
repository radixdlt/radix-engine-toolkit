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
fn initiate_recovery_as_primary_and_quick_confirm_is_classified_as_recovery() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (account, access_controller) = ledger.new_allow_all_access_controller();

    let builder = ManifestBuilder::new()
        .lock_fee(account, 10)
        .set_metadata(account, "owner_keys", "not important")
        .call_method(
            access_controller,
            ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_PRIMARY_IDENT,
            AccessControllerInitiateRecoveryAsPrimaryManifestInput {
                rule_set: RuleSet {
                    primary_role: rule!(allow_all),
                    recovery_role: rule!(allow_all),
                    confirmation_role: rule!(allow_all),
                },
                timed_recovery_delay_in_minutes: None,
            }
        )
        .call_method(
            access_controller,
            ACCESS_CONTROLLER_QUICK_CONFIRM_PRIMARY_ROLE_RECOVERY_PROPOSAL_IDENT,
            AccessControllerQuickConfirmPrimaryRoleRecoveryProposalManifestInput {
               rule_set: RuleSet {
                    primary_role: rule!(allow_all),
                    recovery_role: rule!(allow_all),
                    confirmation_role: rule!(allow_all),
                },
                timed_recovery_delay_in_minutes: None,
            }
        );

    let manifest = builder.build();

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

    assert!(manifest_classification
        .contains(&ManifestClassification::AccessControllerRecovery));

    let Some(DetailedManifestClassification::AccessControllerRecovery(
        AccessControllerRecoveryOutput { access_controllers },
    )) = detailed_manifest_classification.last()
    else {
        panic!("Not an access controller recovery transaction")
    };

    assert_eq!(access_controllers.clone(), vec![access_controller]);
}

#[test]
fn initiate_recovery_as_recovery_and_quick_confirm_is_classified_as_recovery() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (account, access_controller) = ledger.new_allow_all_access_controller();

    let builder = ManifestBuilder::new()
        .lock_fee(account, 10)
        .set_metadata(account, "owner_keys", "not important")
        .call_method(
            access_controller,
            ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_RECOVERY_IDENT,
            AccessControllerInitiateRecoveryAsPrimaryManifestInput {
                rule_set: RuleSet {
                    primary_role: rule!(allow_all),
                    recovery_role: rule!(allow_all),
                    confirmation_role: rule!(allow_all),
                },
                timed_recovery_delay_in_minutes: None,
            }
        )
        .call_method(
            access_controller,
            ACCESS_CONTROLLER_QUICK_CONFIRM_RECOVERY_ROLE_RECOVERY_PROPOSAL_IDENT,
            AccessControllerQuickConfirmPrimaryRoleRecoveryProposalManifestInput {
               rule_set: RuleSet {
                    primary_role: rule!(allow_all),
                    recovery_role: rule!(allow_all),
                    confirmation_role: rule!(allow_all),
                },
                timed_recovery_delay_in_minutes: None,
            }
        );

    let manifest = builder.build();

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

    assert!(manifest_classification
        .contains(&ManifestClassification::AccessControllerRecovery));

    let Some(DetailedManifestClassification::AccessControllerRecovery(
        AccessControllerRecoveryOutput { access_controllers },
    )) = detailed_manifest_classification.last()
    else {
        panic!("Not an access controller recovery transaction")
    };

    assert_eq!(access_controllers.clone(), vec![access_controller]);
}

#[test]
fn initiate_recovery_as_recovery_timed_confirmation_is_classified_as_recovery()
{
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (account, access_controller) = ledger.new_allow_all_access_controller();

    let builder = ManifestBuilder::new()
        .lock_fee(account, 10)
        .set_metadata(account, "owner_keys", "not important")
        .call_method(
            access_controller,
            ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_RECOVERY_IDENT,
            AccessControllerInitiateRecoveryAsPrimaryManifestInput {
                rule_set: RuleSet {
                    primary_role: rule!(allow_all),
                    recovery_role: rule!(allow_all),
                    confirmation_role: rule!(allow_all),
                },
                timed_recovery_delay_in_minutes: None,
            },
        );

    let manifest = builder.build();

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

    assert!(manifest_classification
        .contains(&ManifestClassification::AccessControllerRecovery));

    let Some(DetailedManifestClassification::AccessControllerRecovery(
        AccessControllerRecoveryOutput { access_controllers },
    )) = detailed_manifest_classification.last()
    else {
        panic!("Not an entity securify transaction")
    };

    assert_eq!(access_controllers.clone(), vec![access_controller]);
}

#[test]
fn initiate_recovery_with_lock_fee_on_access_controller() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (account, access_controller) = ledger.new_allow_all_access_controller();

    let builder = ManifestBuilder::new()
        .call_method(
            access_controller,
            ACCESS_CONTROLLER_LOCK_RECOVERY_FEE_IDENT,
            AccessControllerLockRecoveryFeeManifestInput {
                amount: Decimal::one(),
            },
        )
        .lock_fee(account, 10)
        .set_metadata(account, "owner_keys", "not important")
        .call_method(
            access_controller,
            ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_RECOVERY_IDENT,
            AccessControllerInitiateRecoveryAsPrimaryManifestInput {
                rule_set: RuleSet {
                    primary_role: rule!(allow_all),
                    recovery_role: rule!(allow_all),
                    confirmation_role: rule!(allow_all),
                },
                timed_recovery_delay_in_minutes: None,
            },
        );

    let manifest = builder.build();

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

    assert!(manifest_classification
        .contains(&ManifestClassification::AccessControllerRecovery));

    let Some(DetailedManifestClassification::AccessControllerRecovery(
        AccessControllerRecoveryOutput { access_controllers },
    )) = detailed_manifest_classification.last()
    else {
        panic!("Not an entity securify transaction")
    };

    assert_eq!(access_controllers.clone(), vec![access_controller]);
}
