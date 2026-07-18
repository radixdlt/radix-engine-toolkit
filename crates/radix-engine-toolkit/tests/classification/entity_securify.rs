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
fn securify_account_classifies_as_entity_securify() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = ledger.new_account(true);

    let builder = ManifestBuilder::new()
        .call_method(
            account,
            ACCOUNT_SECURIFY_IDENT,
            AccountSecurifyManifestInput {},
        )
        .take_from_worktop(ACCOUNT_OWNER_BADGE, 1, "bucket")
        .create_proof_from_bucket_of_all("bucket", "proof")
        .push_to_auth_zone("proof")
        .set_metadata(account, "owner_keys", "not important")
        .drop_auth_zone_proofs()
        .allocate_global_address(
            ACCESS_CONTROLLER_PACKAGE,
            ACCESS_CONTROLLER_BLUEPRINT,
            "ac_reservation",
            "ac_address",
        );

    let reservation = builder.address_reservation("ac_reservation");
    let bucket = builder.bucket("bucket");
    let rule_set = ManifestRuleSet::from(RuleSet {
        primary_role: rule!(allow_all),
        recovery_role: rule!(deny_all),
        confirmation_role: rule!(allow_all),
    });
    let timed_recovery_delay_in_minutes = Some(1_440);

    let builder = builder.call_function(
        ACCESS_CONTROLLER_PACKAGE,
        ACCESS_CONTROLLER_BLUEPRINT,
        ACCESS_CONTROLLER_CREATE_IDENT,
        AccessControllerCreateManifestInput {
            controlled_asset: bucket,
            rule_set: rule_set.clone(),
            timed_recovery_delay_in_minutes,
            address_reservation: Some(reservation),
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
        .contains(&ManifestClassification::EntitySecurify));

    let Some(DetailedManifestClassification::EntitySecurify(
        EntitySecurifyOutput {
            securify_operations,
        },
    )) = detailed_manifest_classification.last()
    else {
        panic!("Not an entity securify transaction")
    };

    assert_eq!(
        securify_operations,
        &vec![EntitySecurifyOperation {
            entity_address: ManifestGlobalAddress::Static(account.into()),
            rule_set,
            timed_recovery_delay_in_minutes,
        }]
    );
}

#[test]
fn securify_account_with_initial_access_controller_xrd_top_up_classifies() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = ledger.new_account(true);
    ledger.load_account_from_faucet(account);

    let builder = ManifestBuilder::new()
        .lock_fee(account, 10)
        .withdraw_from_account(account, XRD, 100)
        .call_method(
            account,
            ACCOUNT_SECURIFY_IDENT,
            AccountSecurifyManifestInput {},
        )
        .take_from_worktop(ACCOUNT_OWNER_BADGE, 1, "owner_badge")
        .create_proof_from_bucket_of_all("owner_badge", "owner_badge_proof")
        .push_to_auth_zone("owner_badge_proof")
        .set_metadata(account, "owner_keys", "not important")
        .drop_auth_zone_proofs()
        .allocate_global_address(
            ACCESS_CONTROLLER_PACKAGE,
            ACCESS_CONTROLLER_BLUEPRINT,
            "access_controller_reservation",
            "access_controller_address",
        );

    let owner_badge = builder.bucket("owner_badge");
    let access_controller_reservation =
        builder.address_reservation("access_controller_reservation");
    let access_controller_address = ManifestGlobalAddress::Named(
        ManifestNamedAddress(access_controller_reservation.0),
    );
    let rule_set = ManifestRuleSet::from(RuleSet {
        primary_role: rule!(allow_all),
        recovery_role: rule!(deny_all),
        confirmation_role: rule!(allow_all),
    });
    let timed_recovery_delay_in_minutes = Some(20_160);

    let manifest = builder
        .call_function(
            ACCESS_CONTROLLER_PACKAGE,
            ACCESS_CONTROLLER_BLUEPRINT,
            ACCESS_CONTROLLER_CREATE_IDENT,
            AccessControllerCreateManifestInput {
                controlled_asset: owner_badge,
                rule_set: rule_set.clone(),
                timed_recovery_delay_in_minutes,
                address_reservation: Some(access_controller_reservation),
            },
        )
        .take_from_worktop(XRD, 100, "xrd_top_up")
        .then(|builder| {
            let xrd_top_up = builder.bucket("xrd_top_up");
            builder.call_method(
                access_controller_address,
                ACCESS_CONTROLLER_CONTRIBUTE_RECOVERY_FEE_IDENT,
                AccessControllerContributeRecoveryFeeManifestInput {
                    bucket: xrd_top_up,
                },
            )
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
    assert!(manifest_classification
        .contains(&ManifestClassification::EntitySecurify));
    let Some(DetailedManifestClassification::EntitySecurify(
        EntitySecurifyOutput {
            securify_operations,
        },
    )) = detailed_manifest_classification.last()
    else {
        panic!("Not an entity securify transaction")
    };
    assert_eq!(
        securify_operations,
        &vec![EntitySecurifyOperation {
            entity_address: ManifestGlobalAddress::Static(account.into()),
            rule_set,
            timed_recovery_delay_in_minutes,
        }]
    );
}

#[test]
fn securify_account_with_xrd_contribution_to_existing_access_controller_does_not_classify(
) {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, existing_access_controller) =
        ledger.new_allow_all_access_controller();
    let (_, _, account) = ledger.new_account(true);
    ledger.load_account_from_faucet(account);

    let builder = ManifestBuilder::new()
        .withdraw_from_account(account, XRD, 100)
        .call_method(
            account,
            ACCOUNT_SECURIFY_IDENT,
            AccountSecurifyManifestInput {},
        )
        .take_from_worktop(ACCOUNT_OWNER_BADGE, 1, "owner_badge")
        .allocate_global_address(
            ACCESS_CONTROLLER_PACKAGE,
            ACCESS_CONTROLLER_BLUEPRINT,
            "access_controller_reservation",
            "access_controller_address",
        );

    let owner_badge = builder.bucket("owner_badge");
    let access_controller_reservation =
        builder.address_reservation("access_controller_reservation");
    let manifest = builder
        .call_function(
            ACCESS_CONTROLLER_PACKAGE,
            ACCESS_CONTROLLER_BLUEPRINT,
            ACCESS_CONTROLLER_CREATE_IDENT,
            AccessControllerCreateManifestInput {
                controlled_asset: owner_badge,
                rule_set: ManifestRuleSet::from(RuleSet {
                    primary_role: rule!(allow_all),
                    recovery_role: rule!(deny_all),
                    confirmation_role: rule!(allow_all),
                }),
                timed_recovery_delay_in_minutes: Some(20_160),
                address_reservation: Some(access_controller_reservation),
            },
        )
        .take_from_worktop(XRD, 100, "xrd_top_up")
        .then(|builder| {
            let xrd_top_up = builder.bucket("xrd_top_up");
            builder.call_method(
                existing_access_controller,
                ACCESS_CONTROLLER_CONTRIBUTE_RECOVERY_FEE_IDENT,
                AccessControllerContributeRecoveryFeeManifestInput {
                    bucket: xrd_top_up,
                },
            )
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
    assert!(!manifest_classification
        .contains(&ManifestClassification::EntitySecurify));
    assert!(!detailed_manifest_classification
        .iter()
        .any(|classification| {
            matches!(
                classification,
                DetailedManifestClassification::EntitySecurify(..)
            )
        }));
}

#[test]
fn securify_account_with_xrd_withdrawal_without_contribution_does_not_classify()
{
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = ledger.new_account(true);

    // A zero amount keeps the manifest executable while still exercising the
    // analyzer's withdrawal/contribution pairing requirement.
    let builder = ManifestBuilder::new()
        .withdraw_from_account(account, XRD, 0)
        .call_method(
            account,
            ACCOUNT_SECURIFY_IDENT,
            AccountSecurifyManifestInput {},
        )
        .take_from_worktop(ACCOUNT_OWNER_BADGE, 1, "owner_badge")
        .allocate_global_address(
            ACCESS_CONTROLLER_PACKAGE,
            ACCESS_CONTROLLER_BLUEPRINT,
            "access_controller_reservation",
            "access_controller_address",
        );

    let owner_badge = builder.bucket("owner_badge");
    let access_controller_reservation =
        builder.address_reservation("access_controller_reservation");
    let manifest = builder
        .call_function(
            ACCESS_CONTROLLER_PACKAGE,
            ACCESS_CONTROLLER_BLUEPRINT,
            ACCESS_CONTROLLER_CREATE_IDENT,
            AccessControllerCreateManifestInput {
                controlled_asset: owner_badge,
                rule_set: ManifestRuleSet::from(RuleSet {
                    primary_role: rule!(allow_all),
                    recovery_role: rule!(deny_all),
                    confirmation_role: rule!(allow_all),
                }),
                timed_recovery_delay_in_minutes: Some(20_160),
                address_reservation: Some(access_controller_reservation),
            },
        )
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
        .contains(&ManifestClassification::EntitySecurify));
    assert!(!detailed_manifest_classification
        .iter()
        .any(|classification| {
            matches!(
                classification,
                DetailedManifestClassification::EntitySecurify(..)
            )
        }));
}

#[test]
fn securify_account_with_non_xrd_withdrawal_does_not_classify() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = ledger.new_account(true);
    let resource_address =
        ledger.create_fungible_resource(10_000.into(), 18, account);

    // A zero amount keeps the manifest executable so the resource-address
    // permission is the only reason this classification is rejected.
    let builder = ManifestBuilder::new()
        .withdraw_from_account(account, resource_address, 0)
        .call_method(
            account,
            ACCOUNT_SECURIFY_IDENT,
            AccountSecurifyManifestInput {},
        )
        .take_from_worktop(ACCOUNT_OWNER_BADGE, 1, "owner_badge")
        .allocate_global_address(
            ACCESS_CONTROLLER_PACKAGE,
            ACCESS_CONTROLLER_BLUEPRINT,
            "access_controller_reservation",
            "access_controller_address",
        );

    let owner_badge = builder.bucket("owner_badge");
    let access_controller_reservation =
        builder.address_reservation("access_controller_reservation");
    let manifest = builder
        .call_function(
            ACCESS_CONTROLLER_PACKAGE,
            ACCESS_CONTROLLER_BLUEPRINT,
            ACCESS_CONTROLLER_CREATE_IDENT,
            AccessControllerCreateManifestInput {
                controlled_asset: owner_badge,
                rule_set: ManifestRuleSet::from(RuleSet {
                    primary_role: rule!(allow_all),
                    recovery_role: rule!(deny_all),
                    confirmation_role: rule!(allow_all),
                }),
                timed_recovery_delay_in_minutes: Some(20_160),
                address_reservation: Some(access_controller_reservation),
            },
        )
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
        .contains(&ManifestClassification::EntitySecurify));
    assert!(!detailed_manifest_classification
        .iter()
        .any(|classification| {
            matches!(
                classification,
                DetailedManifestClassification::EntitySecurify(..)
            )
        }));
}

#[test]
fn securify_account_with_xrd_transfer_to_another_account_does_not_classify() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = ledger.new_account(true);
    let (_, _, recipient) = ledger.new_account(true);
    ledger.load_account_from_faucet(account);

    let builder = ManifestBuilder::new()
        .withdraw_from_account(account, XRD, 100)
        .call_method(
            account,
            ACCOUNT_SECURIFY_IDENT,
            AccountSecurifyManifestInput {},
        )
        .take_from_worktop(ACCOUNT_OWNER_BADGE, 1, "owner_badge")
        .allocate_global_address(
            ACCESS_CONTROLLER_PACKAGE,
            ACCESS_CONTROLLER_BLUEPRINT,
            "access_controller_reservation",
            "access_controller_address",
        );

    let owner_badge = builder.bucket("owner_badge");
    let access_controller_reservation =
        builder.address_reservation("access_controller_reservation");
    let manifest = builder
        .call_function(
            ACCESS_CONTROLLER_PACKAGE,
            ACCESS_CONTROLLER_BLUEPRINT,
            ACCESS_CONTROLLER_CREATE_IDENT,
            AccessControllerCreateManifestInput {
                controlled_asset: owner_badge,
                rule_set: ManifestRuleSet::from(RuleSet {
                    primary_role: rule!(allow_all),
                    recovery_role: rule!(deny_all),
                    confirmation_role: rule!(allow_all),
                }),
                timed_recovery_delay_in_minutes: Some(20_160),
                address_reservation: Some(access_controller_reservation),
            },
        )
        .take_from_worktop(XRD, 100, "xrd_transfer")
        .try_deposit_or_abort(recipient, None, "xrd_transfer")
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
        .contains(&ManifestClassification::EntitySecurify));
    assert!(!detailed_manifest_classification
        .iter()
        .any(|classification| {
            matches!(
                classification,
                DetailedManifestClassification::EntitySecurify(..)
            )
        }));
}

#[test]
fn securify_identity_classifies_as_entity_securify() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (pk, _, _) = ledger.new_account(true);
    let identity = ledger.new_identity(pk, true);

    let builder = ManifestBuilder::new()
        .call_method(identity, IDENTITY_SECURIFY_IDENT, ())
        .take_from_worktop(IDENTITY_OWNER_BADGE, 1, "bucket")
        .create_proof_from_bucket_of_all("bucket", "proof")
        .push_to_auth_zone("proof")
        .set_metadata(identity, "owner_keys", "not important")
        .drop_auth_zone_proofs()
        .allocate_global_address(
            ACCESS_CONTROLLER_PACKAGE,
            ACCESS_CONTROLLER_BLUEPRINT,
            "ac_reservation",
            "ac_address",
        );

    let reservation = builder.address_reservation("ac_reservation");
    let bucket = builder.bucket("bucket");
    let rule_set = ManifestRuleSet::from(RuleSet {
        primary_role: rule!(deny_all),
        recovery_role: rule!(allow_all),
        confirmation_role: rule!(deny_all),
    });
    let timed_recovery_delay_in_minutes = None;

    let builder = builder.call_function(
        ACCESS_CONTROLLER_PACKAGE,
        ACCESS_CONTROLLER_BLUEPRINT,
        ACCESS_CONTROLLER_CREATE_IDENT,
        AccessControllerCreateManifestInput {
            controlled_asset: bucket,
            rule_set: rule_set.clone(),
            timed_recovery_delay_in_minutes,
            address_reservation: Some(reservation),
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
        .contains(&ManifestClassification::EntitySecurify));

    let Some(DetailedManifestClassification::EntitySecurify(
        EntitySecurifyOutput {
            securify_operations,
        },
    )) = detailed_manifest_classification.last()
    else {
        panic!("Not an entity securify transaction")
    };

    assert_eq!(
        securify_operations,
        &vec![EntitySecurifyOperation {
            entity_address: ManifestGlobalAddress::Static(identity.into()),
            rule_set,
            timed_recovery_delay_in_minutes,
        }]
    );
}

#[test]
fn multiple_entities_are_paired_with_structures_by_owner_badge() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, first_account) = ledger.new_account(true);
    let (_, _, second_account) = ledger.new_account(true);

    let builder = ManifestBuilder::new()
        .call_method(
            first_account,
            ACCOUNT_SECURIFY_IDENT,
            AccountSecurifyManifestInput {},
        )
        .take_from_worktop(ACCOUNT_OWNER_BADGE, 1, "first_bucket")
        .call_method(
            second_account,
            ACCOUNT_SECURIFY_IDENT,
            AccountSecurifyManifestInput {},
        )
        .take_from_worktop(ACCOUNT_OWNER_BADGE, 1, "second_bucket")
        .allocate_global_address(
            ACCESS_CONTROLLER_PACKAGE,
            ACCESS_CONTROLLER_BLUEPRINT,
            "first_reservation",
            "first_access_controller",
        )
        .allocate_global_address(
            ACCESS_CONTROLLER_PACKAGE,
            ACCESS_CONTROLLER_BLUEPRINT,
            "second_reservation",
            "second_access_controller",
        );

    let first_bucket = builder.bucket("first_bucket");
    let second_bucket = builder.bucket("second_bucket");
    let first_reservation = builder.address_reservation("first_reservation");
    let second_reservation = builder.address_reservation("second_reservation");
    let first_rule_set = ManifestRuleSet::from(RuleSet {
        primary_role: rule!(allow_all),
        recovery_role: rule!(deny_all),
        confirmation_role: rule!(allow_all),
    });
    let second_rule_set = ManifestRuleSet::from(RuleSet {
        primary_role: rule!(deny_all),
        recovery_role: rule!(allow_all),
        confirmation_role: rule!(deny_all),
    });

    let manifest = builder
        .call_function(
            ACCESS_CONTROLLER_PACKAGE,
            ACCESS_CONTROLLER_BLUEPRINT,
            ACCESS_CONTROLLER_CREATE_IDENT,
            AccessControllerCreateManifestInput {
                controlled_asset: second_bucket,
                rule_set: second_rule_set.clone(),
                timed_recovery_delay_in_minutes: Some(20),
                address_reservation: Some(second_reservation),
            },
        )
        .call_function(
            ACCESS_CONTROLLER_PACKAGE,
            ACCESS_CONTROLLER_BLUEPRINT,
            ACCESS_CONTROLLER_CREATE_IDENT,
            AccessControllerCreateManifestInput {
                controlled_asset: first_bucket,
                rule_set: first_rule_set.clone(),
                timed_recovery_delay_in_minutes: Some(10),
                address_reservation: Some(first_reservation),
            },
        )
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
        .contains(&ManifestClassification::EntitySecurify));
    let Some(DetailedManifestClassification::EntitySecurify(
        EntitySecurifyOutput {
            securify_operations,
        },
    )) = detailed_manifest_classification.last()
    else {
        panic!("Not an entity securify transaction")
    };
    assert_eq!(
        securify_operations,
        &vec![
            EntitySecurifyOperation {
                entity_address: ManifestGlobalAddress::Static(
                    second_account.into(),
                ),
                rule_set: second_rule_set,
                timed_recovery_delay_in_minutes: Some(20),
            },
            EntitySecurifyOperation {
                entity_address: ManifestGlobalAddress::Static(
                    first_account.into(),
                ),
                rule_set: first_rule_set,
                timed_recovery_delay_in_minutes: Some(10),
            },
        ]
    );
}
