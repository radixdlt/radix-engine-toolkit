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

/// This test ensures that a transaction intent manifest is classified as
/// uncategorized in the Gateway. The wallet currently uses the classification
/// of the transaction intent manifest as the classification of the transaction.
/// For an MVP, we want the wallet to show transactions with subintents as
/// Complex/Uncategorized, so this test checks that. In future, we may wish to
/// revisit this and change this restriction, but we should make sure that the
/// transactions still display reasonably in the wallet.
#[test]
fn subintent_with_yield_to_child_doesnt_classify_as_any_type() {
    // Arrange
    let manifest = ManifestBuilder::new_subintent_v2()
        .use_child("example", SubintentHash(Hash([0; 32])))
        .yield_to_child("example", ())
        .yield_to_parent(())
        .build();

    // Act
    let StaticAnalysis {
        manifest_classification,
        ..
    } = statically_analyze(&manifest).unwrap();

    // Assert
    assert!(manifest_classification.is_empty());
}

#[test]
fn transfer_subintent_classifies_as_general_subintent() {
    // Arrange
    let mut allocator = TestAddressAllocator::new();
    let account1 = allocator.new_account_address();
    let account2 = allocator.new_account_address();
    let manifest = ManifestBuilder::new_subintent_v2()
        .withdraw_from_account(account1, XRD, 10)
        .take_all_from_worktop(XRD, "xrd")
        .try_deposit_or_abort(account2, None, "xrd")
        .yield_to_parent(())
        .build();

    // Act
    let StaticAnalysis {
        manifest_classification,
        ..
    } = statically_analyze(&manifest).unwrap();

    // Assert
    assert_eq!(manifest_classification.len(), 1);
    assert_eq!(
        manifest_classification.first(),
        Some(&ManifestClassification::GeneralSubintent)
    );
}

#[test]
fn subintent_with_verify_parent_classifies_as_general_subintent() {
    // Arrange
    let mut allocator = TestAddressAllocator::new();
    let account1 = allocator.new_account_address();
    let account2 = allocator.new_account_address();
    let manifest = ManifestBuilder::new_subintent_v2()
        .withdraw_from_account(account1, XRD, 10)
        .take_all_from_worktop(XRD, "xrd")
        .try_deposit_or_abort(account2, None, "xrd")
        .verify_parent(rule!(allow_all))
        .yield_to_parent(())
        .build();

    // Act
    let StaticAnalysis {
        manifest_classification,
        ..
    } = statically_analyze(&manifest).unwrap();

    // Assert
    assert_eq!(manifest_classification.len(), 1);
    assert_eq!(
        manifest_classification.first(),
        Some(&ManifestClassification::GeneralSubintent)
    );
}

#[test]
fn subintent_update_metadata_doesnt_classify_as_general_subintent() {
    // Arrange
    let mut allocator = TestAddressAllocator::new();
    let account1 = allocator.new_account_address();
    let account2 = allocator.new_account_address();
    let manifest = ManifestBuilder::new_subintent_v2()
        .withdraw_from_account(account1, XRD, 10)
        .take_all_from_worktop(XRD, "xrd")
        .try_deposit_or_abort(account2, None, "xrd")
        .set_metadata(account1, "key", "value")
        .yield_to_parent(())
        .build();

    // Act
    let StaticAnalysis {
        manifest_classification,
        ..
    } = statically_analyze(&manifest).unwrap();

    // Assert
    assert_eq!(manifest_classification.len(), 0);
}

#[test]
fn faucet_free_xrd_manifest_classifies_as_general() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();

    let (_, _, account) = ledger.new_account(false);
    let manifest = ManifestBuilder::new_subintent_v2()
        .lock_fee_from_faucet()
        .get_free_xrd_from_faucet()
        .take_all_from_worktop(XRD, "xrd")
        .try_deposit_or_abort(account, None, "xrd")
        .yield_to_parent(())
        .build();

    // Act
    let StaticAnalysis {
        manifest_classification,
        ..
    } = statically_analyze(&manifest).unwrap();

    // Assert
    assert_eq!(manifest_classification.len(), 1);
    assert!(manifest_classification
        .iter()
        .any(|classification| matches!(
            classification,
            ManifestClassification::GeneralSubintent
        )));
}

#[test]
fn staking_to_validator_is_permitted_in_general_subintent() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (pk, _, account) = ledger.new_account(true);
    let (validator, _, _) = ledger.new_validator(pk, account);

    let manifest = ManifestBuilder::new_subintent_v2()
        .withdraw_from_account(account, XRD, 10)
        .take_all_from_worktop(XRD, "bucket")
        .stake_validator(validator, "bucket")
        .try_deposit_entire_worktop_or_abort(account, None)
        .yield_to_parent(())
        .build();

    // Act
    let StaticAnalysis {
        manifest_classification,
        ..
    } = statically_analyze(&manifest).unwrap();

    // Assert
    assert!(manifest_classification
        .contains(&ManifestClassification::GeneralSubintent));
}

#[test]
fn unstaking_from_validator_is_permitted_in_general_subintent() {
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

    let manifest = ManifestBuilder::new_subintent_v2()
        .lock_fee_from_faucet()
        .withdraw_from_account(account, lsu, 5)
        .take_all_from_worktop(lsu, "bucket")
        .unstake_validator(validator, "bucket")
        .try_deposit_entire_worktop_or_abort(account, None)
        .yield_to_parent(())
        .build();

    // Act
    let StaticAnalysis {
        manifest_classification,
        ..
    } = statically_analyze(&manifest).unwrap();

    // Assert
    assert!(manifest_classification
        .contains(&ManifestClassification::GeneralSubintent));
}

#[test]
fn claiming_xrd_from_validator_is_permitted_in_general_subintent() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (pk, _, account) = ledger.new_account(true);
    let (validator, lsu, claim_nft) = ledger.new_validator(pk, account);

    let manifest1 = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .withdraw_from_account(account, XRD, 10)
        .take_all_from_worktop(XRD, "bucket")
        .stake_validator(validator, "bucket")
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();
    let manifest2 = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .withdraw_from_account(account, lsu, 5)
        .take_all_from_worktop(lsu, "bucket")
        .unstake_validator(validator, "bucket")
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();

    [manifest1, manifest2].into_iter().for_each(|manifest| {
        ledger
            .execute_manifest(
                manifest,
                vec![NonFungibleGlobalId::from_public_key(&pk)],
            )
            .expect_commit_success();
    });

    ledger.set_current_epoch(Epoch::of(10_000));

    let manifest = ManifestBuilder::new_subintent_v2()
        .lock_fee_from_faucet()
        .withdraw_from_account(account, claim_nft, 1)
        .take_all_from_worktop(claim_nft, "bucket")
        .claim_xrd(validator, "bucket")
        .try_deposit_entire_worktop_or_abort(account, None)
        .yield_to_parent(())
        .build();

    // Act
    let StaticAnalysis {
        manifest_classification,
        ..
    } = statically_analyze(&manifest).unwrap();

    // Assert
    assert!(manifest_classification
        .contains(&ManifestClassification::GeneralSubintent));
}

#[test]
fn pool_contributions_and_redemptions_are_permitted_in_general_subintent() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = ledger.new_account(true);
    let resource_address1 = ledger.create_freely_mintable_fungible_resource(
        Default::default(),
        Some(dec!(1000)),
        18,
        account,
    );
    let resource_address2 = ledger.create_freely_mintable_fungible_resource(
        Default::default(),
        Some(dec!(1000)),
        18,
        account,
    );

    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .call_function(
            POOL_PACKAGE,
            ONE_RESOURCE_POOL_BLUEPRINT,
            ONE_RESOURCE_POOL_INSTANTIATE_IDENT,
            OneResourcePoolInstantiateManifestInput {
                owner_role: Default::default(),
                pool_manager_rule: rule!(allow_all),
                resource_address: resource_address1.into(),
                address_reservation: None,
            },
        )
        .call_function(
            POOL_PACKAGE,
            TWO_RESOURCE_POOL_BLUEPRINT,
            TWO_RESOURCE_POOL_INSTANTIATE_IDENT,
            TwoResourcePoolInstantiateManifestInput {
                owner_role: Default::default(),
                pool_manager_rule: rule!(allow_all),
                resource_addresses: (
                    resource_address1.into(),
                    resource_address2.into(),
                ),
                address_reservation: None,
            },
        )
        .call_function(
            POOL_PACKAGE,
            MULTI_RESOURCE_POOL_BLUEPRINT,
            MULTI_RESOURCE_POOL_INSTANTIATE_IDENT,
            MultiResourcePoolInstantiateManifestInput {
                owner_role: Default::default(),
                pool_manager_rule: rule!(allow_all),
                resource_addresses: indexset![
                    resource_address1.into(),
                    resource_address2.into()
                ],
                address_reservation: None,
            },
        )
        .build();
    let receipt = ledger.execute_manifest(manifest, vec![]);
    let commit_result = receipt.expect_commit_success();
    let [one_pool, two_pool, multi_pool] =
        [0, 1, 2].map(|i| commit_result.new_component_addresses()[i]);
    let [one_pool_unit, two_pool_unit, multi_pool_unit] =
        [0, 1, 2].map(|i| commit_result.new_resource_addresses()[i]);

    let manifest = ManifestBuilder::new_subintent_v2()
        .with_name_lookup(|builder, lookup| {
            builder
                .withdraw_from_account(account, resource_address1, 3)
                .withdraw_from_account(account, resource_address2, 2)
                .take_from_worktop(
                    resource_address1,
                    1,
                    "resource_address1_bucket1",
                )
                .take_from_worktop(
                    resource_address1,
                    1,
                    "resource_address1_bucket2",
                )
                .take_from_worktop(
                    resource_address1,
                    1,
                    "resource_address1_bucket3",
                )
                .take_from_worktop(
                    resource_address2,
                    1,
                    "resource_address2_bucket1",
                )
                .take_from_worktop(
                    resource_address2,
                    1,
                    "resource_address2_bucket2",
                )
                .call_method(
                    one_pool,
                    ONE_RESOURCE_POOL_CONTRIBUTE_IDENT,
                    OneResourcePoolContributeManifestInput {
                        bucket: lookup.bucket("resource_address1_bucket1"),
                    },
                )
                .call_method(
                    two_pool,
                    TWO_RESOURCE_POOL_CONTRIBUTE_IDENT,
                    TwoResourcePoolContributeManifestInput {
                        buckets: (
                            lookup.bucket("resource_address1_bucket2"),
                            lookup.bucket("resource_address2_bucket1"),
                        ),
                    },
                )
                .call_method(
                    multi_pool,
                    MULTI_RESOURCE_POOL_CONTRIBUTE_IDENT,
                    MultiResourcePoolContributeManifestInput {
                        buckets: ManifestBucketBatch::ManifestBuckets(vec![
                            lookup.bucket("resource_address1_bucket3"),
                            lookup.bucket("resource_address2_bucket2"),
                        ]),
                    },
                )
                .take_all_from_worktop(one_pool_unit, "one_pool_unit")
                .take_all_from_worktop(two_pool_unit, "two_pool_unit")
                .take_all_from_worktop(multi_pool_unit, "multi_pool_unit")
                .call_method(
                    one_pool,
                    ONE_RESOURCE_POOL_REDEEM_IDENT,
                    OneResourcePoolRedeemManifestInput {
                        bucket: lookup.bucket("one_pool_unit"),
                    },
                )
                .call_method(
                    two_pool,
                    TWO_RESOURCE_POOL_REDEEM_IDENT,
                    TwoResourcePoolRedeemManifestInput {
                        bucket: lookup.bucket("two_pool_unit"),
                    },
                )
                .call_method(
                    multi_pool,
                    MULTI_RESOURCE_POOL_REDEEM_IDENT,
                    MultiResourcePoolRedeemManifestInput {
                        bucket: lookup.bucket("multi_pool_unit"),
                    },
                )
                .try_deposit_entire_worktop_or_abort(account, None)
        })
        .yield_to_parent(())
        .build();

    // Act
    let StaticAnalysis {
        manifest_classification,
        ..
    } = statically_analyze(&manifest).unwrap();

    // Assert
    assert!(manifest_classification
        .contains(&ManifestClassification::GeneralSubintent));
}

#[test]
fn account_locker_creation_storing_claiming_and_redeeming_are_permitted_in_general_subintent(
) {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = ledger.new_account(true);

    let manifest = ManifestBuilder::new_subintent_v2()
        .with_name_lookup(|builder, lookup| {
            builder
                .allocate_global_address(
                    LOCKER_PACKAGE,
                    ACCOUNT_LOCKER_BLUEPRINT,
                    "locker_reservation",
                    "locker_address",
                )
                .call_function(
                    LOCKER_PACKAGE,
                    ACCOUNT_LOCKER_BLUEPRINT,
                    ACCOUNT_LOCKER_INSTANTIATE_IDENT,
                    AccountLockerInstantiateManifestInput {
                        owner_role: Default::default(),
                        storer_role: rule!(allow_all),
                        storer_updater_role: rule!(allow_all),
                        recoverer_role: rule!(allow_all),
                        recoverer_updater_role: rule!(allow_all),
                        address_reservation: Some(
                            lookup.address_reservation("locker_reservation"),
                        ),
                    },
                )
                .withdraw_from_account(account, XRD, 10)
                .take_all_from_worktop(XRD, "bucket")
                .call_method(
                    "locker_address",
                    ACCOUNT_LOCKER_STORE_IDENT,
                    AccountLockerStoreManifestInput {
                        claimant: account.into(),
                        bucket: lookup.bucket("bucket"),
                        try_direct_send: false,
                    },
                )
                .call_method(
                    "locker_address",
                    ACCOUNT_LOCKER_CLAIM_IDENT,
                    AccountLockerClaimManifestInput {
                        claimant: account.into(),
                        resource_address: XRD.into(),
                        amount: 10.into(),
                    },
                )
                .take_all_from_worktop(XRD, "bucket1")
                .call_method(
                    "locker_address",
                    ACCOUNT_LOCKER_STORE_IDENT,
                    AccountLockerStoreManifestInput {
                        claimant: account.into(),
                        bucket: lookup.bucket("bucket1"),
                        try_direct_send: false,
                    },
                )
                .call_method(
                    "locker_address",
                    ACCOUNT_LOCKER_RECOVER_IDENT,
                    AccountLockerRecoverManifestInput {
                        claimant: account.into(),
                        resource_address: XRD.into(),
                        amount: 10.into(),
                    },
                )
                .try_deposit_entire_worktop_or_abort(account, None)
        })
        .yield_to_parent(())
        .build();

    // Act
    let StaticAnalysis {
        manifest_classification,
        ..
    } = statically_analyze(&manifest).unwrap();

    // Assert
    assert!(manifest_classification
        .contains(&ManifestClassification::GeneralSubintent));
}

#[test]
fn creating_access_controller_create_proof_is_permitted_in_general_subintent() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, access_controller) = ledger.new_allow_all_access_controller();

    let manifest = ManifestBuilder::new_subintent_v2()
        .call_method(access_controller, "create_proof", ())
        .yield_to_parent(())
        .build();

    // Act
    let StaticAnalysis {
        manifest_classification,
        ..
    } = statically_analyze(&manifest).unwrap();

    // Assert
    assert!(manifest_classification
        .contains(&ManifestClassification::GeneralSubintent));
}

#[test]
fn creating_access_controller_initiate_recovery_as_primary_is_not_permitted_in_general_subintent(
) {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, access_controller) = ledger.new_allow_all_access_controller();

    let manifest = ManifestBuilder::new_subintent_v2()
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
            },
        )
        .yield_to_parent(())
        .build();

    // Act
    let StaticAnalysis {
        manifest_classification,
        ..
    } = statically_analyze(&manifest).unwrap();

    // Assert
    assert!(!manifest_classification
        .contains(&ManifestClassification::GeneralSubintent));
}

#[test]
fn creating_access_controller_initiate_recovery_as_recovery_is_not_permitted_in_general_subintent(
) {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, access_controller) = ledger.new_allow_all_access_controller();

    let manifest = ManifestBuilder::new_subintent_v2()
        .call_method(
            access_controller,
            ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_RECOVERY_IDENT,
            AccessControllerInitiateRecoveryAsRecoveryManifestInput {
                rule_set: RuleSet {
                    primary_role: rule!(allow_all),
                    recovery_role: rule!(allow_all),
                    confirmation_role: rule!(allow_all),
                },
                timed_recovery_delay_in_minutes: None,
            },
        )
        .yield_to_parent(())
        .build();

    // Act
    let StaticAnalysis {
        manifest_classification,
        ..
    } = statically_analyze(&manifest).unwrap();

    // Assert
    assert!(!manifest_classification
        .contains(&ManifestClassification::GeneralSubintent));
}

#[test]
fn creating_access_controller_confirm_primary_role_recovery_is_not_permitted_in_general_subintent(
) {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, access_controller) = ledger.new_allow_all_access_controller();

    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
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
            },
        )
        .build();
    ledger
        .execute_manifest(manifest, vec![])
        .expect_commit_success();

    let manifest = ManifestBuilder::new_subintent_v2()
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
            },
        )
        .yield_to_parent(())
        .build();

    // Act
    let StaticAnalysis {
        manifest_classification,
        ..
    } = statically_analyze(&manifest).unwrap();

    // Assert
    assert!(!manifest_classification
        .contains(&ManifestClassification::GeneralSubintent));
}

#[test]
fn creating_access_controller_confirm_recovery_role_recovery_is_not_permitted_in_general_subintent(
) {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, access_controller) = ledger.new_allow_all_access_controller();

    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .call_method(
            access_controller,
            ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_RECOVERY_IDENT,
            AccessControllerInitiateRecoveryAsRecoveryManifestInput {
                rule_set: RuleSet {
                    primary_role: rule!(allow_all),
                    recovery_role: rule!(allow_all),
                    confirmation_role: rule!(allow_all),
                },
                timed_recovery_delay_in_minutes: None,
            },
        )
        .build();
    ledger
        .execute_manifest(manifest, vec![])
        .expect_commit_success();

    let manifest = ManifestBuilder::new_subintent_v2()
        .call_method(
            access_controller,
            ACCESS_CONTROLLER_QUICK_CONFIRM_RECOVERY_ROLE_RECOVERY_PROPOSAL_IDENT,
            AccessControllerQuickConfirmRecoveryRoleRecoveryProposalManifestInput {
                rule_set: RuleSet {
                    primary_role: rule!(allow_all),
                    recovery_role: rule!(allow_all),
                    confirmation_role: rule!(allow_all),
                },
                timed_recovery_delay_in_minutes: None,
            },
        )
        .yield_to_parent(())
        .build();

    // Act
    let StaticAnalysis {
        manifest_classification,
        ..
    } = statically_analyze(&manifest).unwrap();

    // Assert
    assert!(!manifest_classification
        .contains(&ManifestClassification::GeneralSubintent));
}

#[test]
fn address_allocation_and_function_calls_are_permitted_in_general_subintent() {
    // Arrange
    let manifest = ManifestBuilder::new_subintent_v2()
        .allocate_global_address(
            ACCOUNT_PACKAGE,
            ACCOUNT_BLUEPRINT,
            "account_reservation",
            "account_address",
        )
        .new_account_advanced(Default::default(), "account_reservation")
        .yield_to_parent(())
        .build();

    // Act
    let StaticAnalysis {
        manifest_classification,
        ..
    } = statically_analyze(&manifest).unwrap();

    // Assert
    assert!(manifest_classification
        .contains(&ManifestClassification::GeneralSubintent));
}

#[test]
fn locking_fee_is_permitted_in_general_subintent() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = ledger.new_account(true);

    let manifest = ManifestBuilder::new_subintent_v2()
        .lock_fee(account, 1)
        .yield_to_parent(())
        .build();

    // Act
    let StaticAnalysis {
        manifest_classification,
        ..
    } = statically_analyze(&manifest).unwrap();

    // Assert
    assert!(manifest_classification
        .contains(&ManifestClassification::GeneralSubintent));
}

#[test]
fn a_metadata_method_is_not_permitted_in_general_subintent() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = ledger.new_account(true);

    let manifest = ManifestBuilder::new_subintent_v2()
        .call_metadata_method(
            account,
            METADATA_GET_IDENT,
            MetadataGetManifestInput {
                key: "owner_keys".into(),
            },
        )
        .yield_to_parent(())
        .build();

    // Act
    let StaticAnalysis {
        manifest_classification,
        ..
    } = statically_analyze(&manifest).unwrap();

    // Assert
    assert!(!manifest_classification
        .contains(&ManifestClassification::GeneralSubintent));
}

#[test]
fn a_role_assignment_method_is_not_permitted_in_general_subintent() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = ledger.new_account(true);

    let manifest = ManifestBuilder::new_subintent_v2()
        .call_role_assignment_method(
            account,
            ROLE_ASSIGNMENT_GET_IDENT,
            RoleAssignmentGetManifestInput {
                module: ModuleId::Main,
                role_key: ":owner:".into(),
            },
        )
        .yield_to_parent(())
        .build();

    // Act
    let StaticAnalysis {
        manifest_classification,
        ..
    } = statically_analyze(&manifest).unwrap();

    // Assert
    assert!(!manifest_classification
        .contains(&ManifestClassification::GeneralSubintent));
}

#[test]
fn a_royalty_method_is_not_permitted_in_general_subintent() {
    // Arrange
    let mut ledger = LedgerSimulatorBuilder::new()
        .with_custom_protocol(|executor| {
            executor.from_bootstrap_to(ProtocolVersion::Anemone)
        })
        .without_kernel_trace()
        .build();
    let (pk, _) = ledger.new_key_pair();
    let identity = ledger.new_identity(pk, true);

    let manifest = ManifestBuilder::new_subintent_v2()
        .call_royalty_method(
            identity,
            COMPONENT_ROYALTY_SET_ROYALTY_IDENT,
            ComponentRoyaltySetManifestInput {
                method: "something".into(),
                amount: RoyaltyAmount::Free,
            },
        )
        .yield_to_parent(())
        .build();

    // Act
    let StaticAnalysis {
        manifest_classification,
        ..
    } = statically_analyze(&manifest).unwrap();

    // Assert
    assert!(!manifest_classification
        .contains(&ManifestClassification::GeneralSubintent));
}
