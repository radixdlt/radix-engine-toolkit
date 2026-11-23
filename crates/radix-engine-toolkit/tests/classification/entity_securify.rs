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
        .allocate_global_address(
            ACCESS_CONTROLLER_PACKAGE,
            ACCESS_CONTROLLER_BLUEPRINT,
            "ac_reservation",
            "ac_address",
        );

    let reservation = builder.address_reservation("ac_reservation");
    let bucket = builder.bucket("bucket");
    let rule_set = RuleSet {
        primary_role: rule!(allow_all),
        recovery_role: rule!(deny_all),
        confirmation_role: rule!(allow_all),
    };
    let timed_recovery_delay_in_minutes = Some(1_440);

    let builder = builder.call_function(
        ACCESS_CONTROLLER_PACKAGE,
        ACCESS_CONTROLLER_BLUEPRINT,
        ACCESS_CONTROLLER_CREATE_IDENT,
        AccessControllerCreateManifestInput {
            controlled_asset: bucket,
            rule_set: rule_set.clone().into(),
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
            entity_address: account.into(),
            rule_set,
            timed_recovery_delay_in_minutes,
        }]
    );
}

/// The recovery fee vault of the access controller created is topped up in a
/// transaction of its own, of the recovery fee class.
#[test]
fn securify_account_with_initial_access_controller_xrd_top_up_does_not_classify(
) {
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
    let rule_set = RuleSet {
        primary_role: rule!(allow_all),
        recovery_role: rule!(deny_all),
        confirmation_role: rule!(allow_all),
    };
    let timed_recovery_delay_in_minutes = Some(20_160);

    let manifest = builder
        .call_function(
            ACCESS_CONTROLLER_PACKAGE,
            ACCESS_CONTROLLER_BLUEPRINT,
            ACCESS_CONTROLLER_CREATE_IDENT,
            AccessControllerCreateManifestInput {
                controlled_asset: owner_badge,
                rule_set: rule_set.clone().into(),
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
    for class in [
        ManifestClassification::EntitySecurify,
        ManifestClassification::AccessControllerRecoveryFee,
    ] {
        assert!(!manifest_classification.contains(&class));
    }
    assert!(!detailed_manifest_classification
        .iter()
        .any(|classification| {
            matches!(
                classification,
                DetailedManifestClassification::EntitySecurify(..)
                    | DetailedManifestClassification::AccessControllerRecoveryFee(
                        ..
                    )
            )
        }));
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

    // A zero amount keeps the manifest executable, so withdrawing is the only
    // reason this classification is rejected.
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
        .allocate_global_address(
            ACCESS_CONTROLLER_PACKAGE,
            ACCESS_CONTROLLER_BLUEPRINT,
            "ac_reservation",
            "ac_address",
        );

    let reservation = builder.address_reservation("ac_reservation");
    let bucket = builder.bucket("bucket");
    let rule_set = RuleSet {
        primary_role: rule!(deny_all),
        recovery_role: rule!(allow_all),
        confirmation_role: rule!(deny_all),
    };
    let timed_recovery_delay_in_minutes = None;

    let builder = builder.call_function(
        ACCESS_CONTROLLER_PACKAGE,
        ACCESS_CONTROLLER_BLUEPRINT,
        ACCESS_CONTROLLER_CREATE_IDENT,
        AccessControllerCreateManifestInput {
            controlled_asset: bucket,
            rule_set: rule_set.clone().into(),
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
            entity_address: identity.into(),
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
    let first_rule_set = RuleSet {
        primary_role: rule!(allow_all),
        recovery_role: rule!(deny_all),
        confirmation_role: rule!(allow_all),
    };
    let second_rule_set = RuleSet {
        primary_role: rule!(deny_all),
        recovery_role: rule!(allow_all),
        confirmation_role: rule!(deny_all),
    };

    let manifest = builder
        .call_function(
            ACCESS_CONTROLLER_PACKAGE,
            ACCESS_CONTROLLER_BLUEPRINT,
            ACCESS_CONTROLLER_CREATE_IDENT,
            AccessControllerCreateManifestInput {
                controlled_asset: second_bucket,
                rule_set: second_rule_set.clone().into(),
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
                rule_set: first_rule_set.clone().into(),
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
                entity_address: second_account.into(),
                rule_set: second_rule_set,
                timed_recovery_delay_in_minutes: Some(20),
            },
            EntitySecurifyOperation {
                entity_address: first_account.into(),
                rule_set: first_rule_set,
                timed_recovery_delay_in_minutes: Some(10),
            },
        ]
    );
}

/// A securify of an account, with extra instructions added by `extra` after
/// the account is securified. The address of the new access controller is
/// allocated with the given package and blueprint.
fn securify_manifest_with(
    allocated_blueprint: (PackageAddress, &str),
    extra: impl FnOnce(ManifestBuilder, ComponentAddress) -> ManifestBuilder,
) -> TransactionManifestV1 {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let builder = ManifestBuilder::new()
        .call_method(
            account,
            ACCOUNT_SECURIFY_IDENT,
            AccountSecurifyManifestInput {},
        )
        .take_from_worktop(ACCOUNT_OWNER_BADGE, 1, "owner_badge")
        .allocate_global_address(
            allocated_blueprint.0,
            allocated_blueprint.1,
            "reservation",
            "access_controller",
        );
    let builder = extra(builder, account);
    let owner_badge = builder.bucket("owner_badge");
    let reservation = builder.address_reservation("reservation");
    builder
        .call_function(
            ACCESS_CONTROLLER_PACKAGE,
            ACCESS_CONTROLLER_BLUEPRINT,
            ACCESS_CONTROLLER_CREATE_IDENT,
            AccessControllerCreateManifestInput {
                controlled_asset: owner_badge,
                rule_set: RuleSet {
                    primary_role: rule!(allow_all),
                    recovery_role: rule!(allow_all),
                    confirmation_role: rule!(allow_all),
                }
                .into(),
                timed_recovery_delay_in_minutes: None,
                address_reservation: Some(reservation),
            },
        )
        .build()
}

fn is_classified_as_securify(manifest: &TransactionManifestV1) -> bool {
    let StaticAnalysis {
        manifest_classification,
        ..
    } = statically_analyze(manifest).unwrap();
    manifest_classification.contains(&ManifestClassification::EntitySecurify)
}

#[test]
fn securify_base_manifest_classifies_as_entity_securify() {
    assert!(is_classified_as_securify(&securify_manifest_with(
        (ACCESS_CONTROLLER_PACKAGE, ACCESS_CONTROLLER_BLUEPRINT),
        |builder, _| builder
    )));
}

#[test]
fn securify_with_non_access_controller_address_allocation_does_not_classify() {
    let manifest = securify_manifest_with(
        (ACCOUNT_PACKAGE, ACCOUNT_BLUEPRINT),
        |builder, _| builder,
    );
    assert!(!is_classified_as_securify(&manifest));
}

/// Setting metadata, even the securified entity's owner keys, isn't in the
/// output, so it's done in a transaction of its own.
#[test]
fn securify_with_metadata_set_does_not_classify() {
    let sets: [fn(ManifestBuilder, ComponentAddress) -> ManifestBuilder; 3] = [
        |builder, account| {
            builder.set_metadata(account, "owner_keys", "not important")
        },
        |builder, account| {
            builder.set_metadata(account, "name", "not important")
        },
        |builder, _| {
            let other_account =
                TestAddressAllocator::new().new_account_address();
            builder.set_metadata(other_account, "owner_keys", "not important")
        },
    ];
    for set in sets {
        let manifest = securify_manifest_with(
            (ACCESS_CONTROLLER_PACKAGE, ACCESS_CONTROLLER_BLUEPRINT),
            set,
        );
        assert!(!is_classified_as_securify(&manifest));
    }
}

/// Proofs of the owner badge were only needed to set metadata.
#[test]
fn securify_with_owner_badge_proof_does_not_classify() {
    let manifest = securify_manifest_with(
        (ACCESS_CONTROLLER_PACKAGE, ACCESS_CONTROLLER_BLUEPRINT),
        |builder, _| {
            builder
                .create_proof_from_bucket_of_all("owner_badge", "proof")
                .push_to_auth_zone("proof")
                .drop_auth_zone_proofs()
        },
    );
    assert!(!is_classified_as_securify(&manifest));
}

#[test]
fn securify_with_metadata_removal_does_not_classify() {
    let manifest = securify_manifest_with(
        (ACCESS_CONTROLLER_PACKAGE, ACCESS_CONTROLLER_BLUEPRINT),
        |builder, account| {
            builder.call_metadata_method(
                account,
                METADATA_REMOVE_IDENT,
                MetadataRemoveInput {
                    key: "owner_keys".into(),
                },
            )
        },
    );
    assert!(!is_classified_as_securify(&manifest));
}

fn create_access_controller(
    builder: ManifestBuilder,
    controlled_asset: &str,
    rule_set: RuleSet,
) -> ManifestBuilder {
    let controlled_asset = builder.bucket(controlled_asset);
    builder.call_function(
        ACCESS_CONTROLLER_PACKAGE,
        ACCESS_CONTROLLER_BLUEPRINT,
        ACCESS_CONTROLLER_CREATE_IDENT,
        AccessControllerCreateManifestInput {
            controlled_asset,
            rule_set: rule_set.into(),
            timed_recovery_delay_in_minutes: None,
            address_reservation: None,
        },
    )
}

fn securify(
    builder: ManifestBuilder,
    account: ComponentAddress,
) -> ManifestBuilder {
    builder.call_method(
        account,
        ACCOUNT_SECURIFY_IDENT,
        AccountSecurifyManifestInput {},
    )
}

fn shown_rule_set() -> RuleSet {
    RuleSet {
        primary_role: rule!(allow_all),
        recovery_role: rule!(deny_all),
        confirmation_role: rule!(deny_all),
    }
}

fn other_rule_set() -> RuleSet {
    RuleSet {
        primary_role: rule!(deny_all),
        recovery_role: rule!(allow_all),
        confirmation_role: rule!(allow_all),
    }
}

/// An access controller created with the owner badges of several entities
/// controls them all, but would be paired with one of them at most. The
/// manifest below commits, with as many access controllers created as entities
/// securified since one is created with no owner badge, and two of the
/// accounts end up under rules that an output pairing one entity with one
/// access controller couldn't show.
#[test]
fn access_controller_created_with_several_owner_badges_does_not_classify() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, first_account) = ledger.new_account(true);
    let (_, _, second_account) = ledger.new_account(true);
    let (_, _, third_account) = ledger.new_account(true);

    let builder = securify(ManifestBuilder::new(), first_account)
        .take_from_worktop(ACCOUNT_OWNER_BADGE, 1, "one_badge");
    let builder = securify(builder, second_account);
    let builder = securify(builder, third_account)
        .take_all_from_worktop(ACCOUNT_OWNER_BADGE, "two_badges")
        .take_from_worktop(ACCOUNT_OWNER_BADGE, 0, "no_badge");
    let builder =
        create_access_controller(builder, "one_badge", shown_rule_set());
    let builder =
        create_access_controller(builder, "two_badges", other_rule_set());
    let manifest =
        create_access_controller(builder, "no_badge", shown_rule_set()).build();

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
        .any(|class| matches!(
            class,
            DetailedManifestClassification::EntitySecurify(..)
        )));
}

/// Taking all the owner badges on the worktop is taking exactly one when only
/// one entity has been securified since the last was taken.
#[test]
fn taking_all_owner_badges_of_one_entity_classifies_as_entity_securify() {
    let mut allocator = TestAddressAllocator::new();
    let first_account = allocator.new_account_address();
    let second_account = allocator.new_account_address();

    let builder = securify(ManifestBuilder::new(), first_account)
        .take_all_from_worktop(ACCOUNT_OWNER_BADGE, "first_badge");
    let builder = securify(builder, second_account)
        .take_all_from_worktop(ACCOUNT_OWNER_BADGE, "second_badge");
    let builder =
        create_access_controller(builder, "first_badge", shown_rule_set());
    let manifest =
        create_access_controller(builder, "second_badge", other_rule_set())
            .build();

    assert!(is_classified_as_securify(&manifest));
}
