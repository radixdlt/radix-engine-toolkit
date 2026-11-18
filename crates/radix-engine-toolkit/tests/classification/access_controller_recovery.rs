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
                }.into(),
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
                }.into(),
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
        AccessControllerRecoveryOutput { recovery_proposals },
    )) = detailed_manifest_classification.last()
    else {
        panic!("Not an access controller recovery transaction")
    };

    assert_eq!(
        recovery_proposals
            .iter()
            .map(|proposal| proposal.access_controller)
            .collect::<Vec<_>>(),
        vec![access_controller]
    );
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
                }.into(),
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
                }.into(),
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
        AccessControllerRecoveryOutput { recovery_proposals },
    )) = detailed_manifest_classification.last()
    else {
        panic!("Not an access controller recovery transaction")
    };

    assert_eq!(
        recovery_proposals
            .iter()
            .map(|proposal| proposal.access_controller)
            .collect::<Vec<_>>(),
        vec![access_controller]
    );
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
                }
                .into(),
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
        AccessControllerRecoveryOutput { recovery_proposals },
    )) = detailed_manifest_classification.last()
    else {
        panic!("Not an entity securify transaction")
    };

    assert_eq!(
        recovery_proposals
            .iter()
            .map(|proposal| proposal.access_controller)
            .collect::<Vec<_>>(),
        vec![access_controller]
    );
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
                }
                .into(),
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
        AccessControllerRecoveryOutput { recovery_proposals },
    )) = detailed_manifest_classification.last()
    else {
        panic!("Not an entity securify transaction")
    };

    assert_eq!(
        recovery_proposals
            .iter()
            .map(|proposal| proposal.access_controller)
            .collect::<Vec<_>>(),
        vec![access_controller]
    );
}

#[test]
fn stop_timed_recovery_and_initiate_recovery_is_classified_as_recovery() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (pk, _, account) = ledger.new_account(true);
    let access_controller =
        ledger.new_allow_all_access_controller_for_account((pk, account));

    // First initiate a timed recovery to have something to stop
    let initiate_timed_recovery_manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .call_method(
            access_controller,
            ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_RECOVERY_IDENT,
            AccessControllerInitiateRecoveryAsPrimaryManifestInput {
                rule_set: RuleSet {
                    primary_role: rule!(allow_all),
                    recovery_role: rule!(allow_all),
                    confirmation_role: rule!(allow_all),
                }
                .into(),
                timed_recovery_delay_in_minutes: Some(1),
            },
        )
        .build();

    ledger
        .execute_manifest(
            initiate_timed_recovery_manifest,
            vec![NonFungibleGlobalId::from_public_key(&pk)],
        )
        .expect_commit_success();

    // Now build a manifest that stops timed recovery and initiates a new recovery
    let builder = ManifestBuilder::new()
        .lock_fee(account, 10)
        .call_method(
            access_controller,
            ACCESS_CONTROLLER_STOP_TIMED_RECOVERY_IDENT,
            AccessControllerStopTimedRecoveryManifestInput {
                rule_set: RuleSet {
                    primary_role: rule!(allow_all),
                    recovery_role: rule!(allow_all),
                    confirmation_role: rule!(allow_all),
                }
                .into(),
                timed_recovery_delay_in_minutes: Some(1),
            },
        )
        .call_method(
            access_controller,
            ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_PRIMARY_IDENT,
            AccessControllerInitiateRecoveryAsPrimaryManifestInput {
                rule_set: RuleSet {
                    primary_role: rule!(allow_all),
                    recovery_role: rule!(allow_all),
                    confirmation_role: rule!(allow_all),
                }
                .into(),
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

    // Assert - should be classified as Recovery, not StopTimedRecovery
    assert!(manifest_classification
        .contains(&ManifestClassification::AccessControllerRecovery));

    let Some(DetailedManifestClassification::AccessControllerRecovery(
        AccessControllerRecoveryOutput { recovery_proposals },
    )) = detailed_manifest_classification.last()
    else {
        panic!("Not an access controller recovery transaction")
    };

    assert_eq!(
        recovery_proposals
            .iter()
            .map(|proposal| proposal.access_controller)
            .collect::<Vec<_>>(),
        vec![access_controller]
    );
}

fn allow_all_rule_set() -> RuleSet {
    RuleSet {
        primary_role: rule!(allow_all),
        recovery_role: rule!(allow_all),
        confirmation_role: rule!(allow_all),
    }
}

fn initiate_recovery_input(
) -> AccessControllerInitiateRecoveryAsRecoveryManifestInput {
    AccessControllerInitiateRecoveryAsRecoveryManifestInput {
        rule_set: allow_all_rule_set().into(),
        timed_recovery_delay_in_minutes: None,
    }
}

fn is_classified_as_recovery(manifest: &TransactionManifestV1) -> bool {
    let StaticAnalysis {
        manifest_classification,
        ..
    } = statically_analyze(manifest).unwrap();
    manifest_classification
        .contains(&ManifestClassification::AccessControllerRecovery)
}

#[test]
fn recovery_output_contains_the_proposed_rule_set() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (account, access_controller) = ledger.new_allow_all_access_controller();
    let rule_set = RuleSet {
        primary_role: rule!(deny_all),
        recovery_role: rule!(allow_all),
        confirmation_role: rule!(allow_all),
    };

    let manifest = ManifestBuilder::new()
        .lock_fee(account, 10)
        .call_method(
            access_controller,
            ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_RECOVERY_IDENT,
            AccessControllerInitiateRecoveryAsRecoveryManifestInput {
                rule_set: rule_set.clone().into(),
                timed_recovery_delay_in_minutes: Some(10),
            },
        )
        .build();

    // Act
    let (
        _,
        DynamicAnalysis {
            detailed_manifest_classification,
            ..
        },
    ) = ledger.analyze(manifest);

    // Assert
    let Some(DetailedManifestClassification::AccessControllerRecovery(
        AccessControllerRecoveryOutput { recovery_proposals },
    )) = detailed_manifest_classification.last()
    else {
        panic!("Not an access controller recovery transaction")
    };
    assert_eq!(
        recovery_proposals,
        &vec![AccessControllerRecoveryProposal {
            access_controller,
            proposer: Proposer::Recovery,
            rule_set,
            timed_recovery_delay_in_minutes: Some(10),
        }]
    );
}

#[test]
fn confirming_another_access_controller_is_not_recovery() {
    // Arrange
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let access_controller1 = allocator.new_access_controller_address();
    let access_controller2 = allocator.new_access_controller_address();
    let manifest = ManifestBuilder::new()
        .lock_fee(account, 10)
        .call_method(
            access_controller1,
            ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_RECOVERY_IDENT,
            initiate_recovery_input(),
        )
        .call_method(
            access_controller2,
            ACCESS_CONTROLLER_QUICK_CONFIRM_RECOVERY_ROLE_RECOVERY_PROPOSAL_IDENT,
            AccessControllerQuickConfirmRecoveryRoleRecoveryProposalManifestInput {
                rule_set: allow_all_rule_set().into(),
                timed_recovery_delay_in_minutes: None,
            },
        )
        .build();

    // Act & Assert
    assert!(!is_classified_as_recovery(&manifest));
}

#[test]
fn stopping_another_access_controller_is_not_recovery() {
    // Arrange
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let access_controller1 = allocator.new_access_controller_address();
    let access_controller2 = allocator.new_access_controller_address();
    let manifest = ManifestBuilder::new()
        .lock_fee(account, 10)
        .call_method(
            access_controller2,
            ACCESS_CONTROLLER_STOP_TIMED_RECOVERY_IDENT,
            AccessControllerStopTimedRecoveryManifestInput {
                rule_set: allow_all_rule_set().into(),
                timed_recovery_delay_in_minutes: None,
            },
        )
        .call_method(
            access_controller1,
            ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_RECOVERY_IDENT,
            initiate_recovery_input(),
        )
        .build();

    // Act & Assert
    assert!(!is_classified_as_recovery(&manifest));
}

#[test]
fn locking_contingent_fee_is_permitted_in_recovery() {
    // Arrange
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let access_controller = allocator.new_access_controller_address();
    let manifest = ManifestBuilder::new()
        .lock_contingent_fee(account, 10)
        .call_method(
            access_controller,
            ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_RECOVERY_IDENT,
            initiate_recovery_input(),
        )
        .build();

    // Act & Assert
    assert!(is_classified_as_recovery(&manifest));
}

/// A recovery that also withdraws XRD from an account and contributes it to the
/// recovery fee vault of either the recovered access controller or another one.
fn recovery_with_xrd_top_up(
    top_up_recovered_controller: bool,
) -> TransactionManifestV1 {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let access_controller = allocator.new_access_controller_address();
    let other_access_controller = allocator.new_access_controller_address();
    let top_up_target = if top_up_recovered_controller {
        access_controller
    } else {
        other_access_controller
    };

    ManifestBuilder::new()
        .call_method(
            access_controller,
            ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_RECOVERY_IDENT,
            initiate_recovery_input(),
        )
        .withdraw_from_account(account, XRD, 100)
        .take_from_worktop(XRD, 100, "xrd")
        .then(|builder| {
            let bucket = builder.bucket("xrd");
            builder.call_method(
                top_up_target,
                ACCESS_CONTROLLER_CONTRIBUTE_RECOVERY_FEE_IDENT,
                AccessControllerContributeRecoveryFeeManifestInput { bucket },
            )
        })
        .build()
}

#[test]
fn xrd_top_up_of_recovered_access_controller_is_permitted_in_recovery() {
    assert!(is_classified_as_recovery(&recovery_with_xrd_top_up(true)));
}

#[test]
fn xrd_top_up_of_another_access_controller_is_not_recovery() {
    assert!(!is_classified_as_recovery(&recovery_with_xrd_top_up(false)));
}

/// A recovery of a fresh access controller, with extra instructions added by
/// `extra`.
fn recovery_with(
    extra: impl FnOnce(ManifestBuilder, ComponentAddress) -> ManifestBuilder,
) -> TransactionManifestV1 {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let access_controller = allocator.new_access_controller_address();
    let builder = ManifestBuilder::new().call_method(
        access_controller,
        ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_RECOVERY_IDENT,
        initiate_recovery_input(),
    );
    extra(builder, account).build()
}

#[test]
fn xrd_withdrawal_without_top_up_is_not_recovery() {
    let manifest = recovery_with(|builder, account| {
        builder
            .withdraw_from_account(account, XRD, 100)
            .try_deposit_entire_worktop_or_abort(account, None)
    });
    assert!(!is_classified_as_recovery(&manifest));
}

#[test]
fn non_xrd_withdrawal_is_not_recovery() {
    let manifest = recovery_with(|builder, account| {
        builder
            .withdraw_from_account(account, ACCOUNT_OWNER_BADGE, 1)
            .try_deposit_entire_worktop_or_abort(account, None)
    });
    assert!(!is_classified_as_recovery(&manifest));
}

#[test]
fn setting_account_metadata_is_permitted_in_recovery() {
    let manifest = recovery_with(|builder, account| {
        builder.set_metadata(account, "owner_keys", "not important")
    });
    assert!(is_classified_as_recovery(&manifest));
}

#[test]
fn removing_account_metadata_is_not_recovery() {
    let manifest = recovery_with(|builder, account| {
        builder.call_metadata_method(
            account,
            METADATA_REMOVE_IDENT,
            MetadataRemoveInput {
                key: "owner_keys".into(),
            },
        )
    });
    assert!(!is_classified_as_recovery(&manifest));
}

#[test]
fn setting_resource_metadata_is_not_recovery() {
    let manifest = recovery_with(|builder, _| {
        builder.set_metadata(XRD, "name", "not important")
    });
    assert!(!is_classified_as_recovery(&manifest));
}
