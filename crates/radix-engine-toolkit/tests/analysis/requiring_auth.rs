use crate::prelude::*;

#[test]
fn account_securify_requires_auth() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (manifest, account) = account_securify_manifest();
    ledger.load_account_from_faucet(account);
    let account = ManifestGlobalAddress::Static(account.into());

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(
        static_analysis.entities_requiring_auth_summary,
        dynamic_analysis.entities_requiring_auth_summary
    );
    assert!(static_analysis
        .entities_requiring_auth_summary
        .accounts
        .contains(&account));
}

#[test]
fn account_lock_fee_requires_auth() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (manifest, account) = account_lock_fee_manifest();
    ledger.load_account_from_faucet(account);
    let account = ManifestGlobalAddress::Static(account.into());

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(
        static_analysis.entities_requiring_auth_summary,
        dynamic_analysis.entities_requiring_auth_summary
    );
    assert!(static_analysis
        .entities_requiring_auth_summary
        .accounts
        .contains(&account));
}

#[test]
fn account_lock_contingent_fee_requires_auth() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (manifest, account) = account_lock_contingent_fee_manifest();
    ledger.load_account_from_faucet(account);
    let account = ManifestGlobalAddress::Static(account.into());

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(
        static_analysis.entities_requiring_auth_summary,
        dynamic_analysis.entities_requiring_auth_summary
    );
    assert!(static_analysis
        .entities_requiring_auth_summary
        .accounts
        .contains(&account));
}

#[test]
fn account_deposit_requires_auth() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (manifest, account) = account_deposit_manifest();
    ledger.load_account_from_faucet(account);
    let account = ManifestGlobalAddress::Static(account.into());

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(
        static_analysis.entities_requiring_auth_summary,
        dynamic_analysis.entities_requiring_auth_summary
    );
    assert!(static_analysis
        .entities_requiring_auth_summary
        .accounts
        .contains(&account));
}

#[test]
fn account_deposit_batch_requires_auth() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (manifest, account) = account_deposit_batch_manifest();
    ledger.load_account_from_faucet(account);
    let account = ManifestGlobalAddress::Static(account.into());

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(
        static_analysis.entities_requiring_auth_summary,
        dynamic_analysis.entities_requiring_auth_summary
    );
    assert!(static_analysis
        .entities_requiring_auth_summary
        .accounts
        .contains(&account));
}

#[test]
fn account_try_deposit_or_abort_doesnt_require_auth() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (manifest, account) = account_try_deposit_or_abort_manifest();
    ledger.load_account_from_faucet(account);
    let account = ManifestGlobalAddress::Static(account.into());

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(
        static_analysis.entities_requiring_auth_summary,
        dynamic_analysis.entities_requiring_auth_summary
    );
    assert!(!static_analysis
        .entities_requiring_auth_summary
        .accounts
        .contains(&account));
}

#[test]
fn account_try_deposit_batch_or_abort_doesnt_require_auth() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (manifest, account) = account_try_deposit_batch_or_abort_manifest();
    ledger.load_account_from_faucet(account);
    let account = ManifestGlobalAddress::Static(account.into());

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(
        static_analysis.entities_requiring_auth_summary,
        dynamic_analysis.entities_requiring_auth_summary
    );
    assert!(!static_analysis
        .entities_requiring_auth_summary
        .accounts
        .contains(&account));
}

#[test]
fn account_try_deposit_or_refund_doesnt_require_auth() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (manifest, account) = account_try_deposit_or_refund_manifest();
    ledger.load_account_from_faucet(account);
    let account = ManifestGlobalAddress::Static(account.into());

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(
        static_analysis.entities_requiring_auth_summary,
        dynamic_analysis.entities_requiring_auth_summary
    );
    assert!(!static_analysis
        .entities_requiring_auth_summary
        .accounts
        .contains(&account));
}

#[test]
fn account_try_deposit_batch_or_refund_doesnt_require_auth() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (manifest, account) = account_try_deposit_batch_or_refund_manifest();
    ledger.load_account_from_faucet(account);
    let account = ManifestGlobalAddress::Static(account.into());

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(
        static_analysis.entities_requiring_auth_summary,
        dynamic_analysis.entities_requiring_auth_summary
    );
    assert!(!static_analysis
        .entities_requiring_auth_summary
        .accounts
        .contains(&account));
}

#[test]
fn account_withdraw_requires_auth() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (manifest, account) = account_withdraw_manifest();
    ledger.load_account_from_faucet(account);
    let account = ManifestGlobalAddress::Static(account.into());

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(
        static_analysis.entities_requiring_auth_summary,
        dynamic_analysis.entities_requiring_auth_summary
    );
    assert!(static_analysis
        .entities_requiring_auth_summary
        .accounts
        .contains(&account));
}

#[test]
fn account_withdraw_non_fungibles_requires_auth() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let resource_address = ledger
        .create_everything_allowed_non_fungible_resource(OwnerRole::Fixed(
            rule!(allow_all),
        ));
    let (manifest, account) =
        account_withdraw_non_fungibles_manifest(resource_address);
    ledger.load_account_from_faucet(account);
    let account = ManifestGlobalAddress::Static(account.into());

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(
        static_analysis.entities_requiring_auth_summary,
        dynamic_analysis.entities_requiring_auth_summary
    );
    assert!(static_analysis
        .entities_requiring_auth_summary
        .accounts
        .contains(&account));
}

#[test]
fn account_lock_fee_and_withdraw_requires_auth() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (manifest, account) = account_lock_fee_and_withdraw_manifest();
    ledger.load_account_from_faucet(account);
    let account = ManifestGlobalAddress::Static(account.into());

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(
        static_analysis.entities_requiring_auth_summary,
        dynamic_analysis.entities_requiring_auth_summary
    );
    assert!(static_analysis
        .entities_requiring_auth_summary
        .accounts
        .contains(&account));
    assert!(static_analysis
        .entities_requiring_auth_summary
        .accounts
        .contains(&account));
}

#[test]
fn account_lock_fee_and_withdraw_non_fungibles_requires_auth() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let resource_address = ledger
        .create_everything_allowed_non_fungible_resource(OwnerRole::Fixed(
            rule!(allow_all),
        ));
    let (manifest, account) =
        account_lock_fee_and_withdraw_non_fungibles_manifest(resource_address);
    ledger.load_account_from_faucet(account);
    let account = ManifestGlobalAddress::Static(account.into());

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(
        static_analysis.entities_requiring_auth_summary,
        dynamic_analysis.entities_requiring_auth_summary
    );
    assert!(static_analysis
        .entities_requiring_auth_summary
        .accounts
        .contains(&account));
    assert!(static_analysis
        .entities_requiring_auth_summary
        .accounts
        .contains(&account));
}

#[test]
fn account_burn_requires_auth() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (manifest, account) = account_burn_manifest();
    ledger.load_account_from_faucet(account);
    let account = ManifestGlobalAddress::Static(account.into());

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(
        static_analysis.entities_requiring_auth_summary,
        dynamic_analysis.entities_requiring_auth_summary
    );
    assert!(static_analysis
        .entities_requiring_auth_summary
        .accounts
        .contains(&account));
}

#[test]
fn account_burn_non_fungible_non_fungibles_requires_auth() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let resource_address = ledger
        .create_everything_allowed_non_fungible_resource(OwnerRole::Fixed(
            rule!(allow_all),
        ));
    let (manifest, account) =
        account_burn_non_fungible_non_fungibles_manifest(resource_address);
    ledger.load_account_from_faucet(account);
    let account = ManifestGlobalAddress::Static(account.into());

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(
        static_analysis.entities_requiring_auth_summary,
        dynamic_analysis.entities_requiring_auth_summary
    );
    assert!(static_analysis
        .entities_requiring_auth_summary
        .accounts
        .contains(&account));
}

#[test]
fn account_create_proof_of_requires_auth() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (manifest, account) = account_create_proof_of_manifest();
    ledger.load_account_from_faucet(account);
    let account = ManifestGlobalAddress::Static(account.into());

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(
        static_analysis.entities_requiring_auth_summary,
        dynamic_analysis.entities_requiring_auth_summary
    );
    assert!(static_analysis
        .entities_requiring_auth_summary
        .accounts
        .contains(&account));
}

#[test]
fn account_create_proof_of_non_fungibles_requires_auth() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let resource_address = ledger
        .create_everything_allowed_non_fungible_resource(OwnerRole::Fixed(
            rule!(allow_all),
        ));
    let (manifest, account) =
        account_create_proof_of_non_fungibles_manifest(resource_address);
    ledger.load_account_from_faucet(account);
    let account = ManifestGlobalAddress::Static(account.into());

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(
        static_analysis.entities_requiring_auth_summary,
        dynamic_analysis.entities_requiring_auth_summary
    );
    assert!(static_analysis
        .entities_requiring_auth_summary
        .accounts
        .contains(&account));
}

#[test]
fn account_set_default_deposit_rule_requires_auth() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (manifest, account) = account_set_default_deposit_rule_manifest();
    ledger.load_account_from_faucet(account);
    let account = ManifestGlobalAddress::Static(account.into());

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(
        static_analysis.entities_requiring_auth_summary,
        dynamic_analysis.entities_requiring_auth_summary
    );
    assert!(static_analysis
        .entities_requiring_auth_summary
        .accounts
        .contains(&account));
}

#[test]
fn account_set_resource_preference_requires_auth() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (manifest, account) = account_set_resource_preference_manifest();
    ledger.load_account_from_faucet(account);
    let account = ManifestGlobalAddress::Static(account.into());

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(
        static_analysis.entities_requiring_auth_summary,
        dynamic_analysis.entities_requiring_auth_summary
    );
    assert!(static_analysis
        .entities_requiring_auth_summary
        .accounts
        .contains(&account));
}

#[test]
fn account_remove_resource_preference_requires_auth() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (manifest, account) = account_remove_resource_preference_manifest();
    ledger.load_account_from_faucet(account);
    let account = ManifestGlobalAddress::Static(account.into());

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(
        static_analysis.entities_requiring_auth_summary,
        dynamic_analysis.entities_requiring_auth_summary
    );
    assert!(static_analysis
        .entities_requiring_auth_summary
        .accounts
        .contains(&account));
}

#[test]
fn account_add_authorized_depositor_preference_requires_auth() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (manifest, account) =
        account_add_authorized_depositor_preference_manifest();
    ledger.load_account_from_faucet(account);
    let account = ManifestGlobalAddress::Static(account.into());

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(
        static_analysis.entities_requiring_auth_summary,
        dynamic_analysis.entities_requiring_auth_summary
    );
    assert!(static_analysis
        .entities_requiring_auth_summary
        .accounts
        .contains(&account));
}

#[test]
fn account_remove_authorized_depositor_preference_requires_auth() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (manifest, account) =
        account_remove_authorized_depositor_preference_manifest();
    ledger.load_account_from_faucet(account);
    let account = ManifestGlobalAddress::Static(account.into());

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(
        static_analysis.entities_requiring_auth_summary,
        dynamic_analysis.entities_requiring_auth_summary
    );
    assert!(static_analysis
        .entities_requiring_auth_summary
        .accounts
        .contains(&account));
}

#[test]
fn account_locker_claim_requires_auth() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = ledger.new_account(true);
    let locker = ledger.new_account_locker(OwnerRole::Fixed(rule!(allow_all)));
    let resource_address = ledger
        .create_everything_allowed_non_fungible_resource(Default::default());

    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .mint_non_fungible(
            resource_address,
            [(NonFungibleLocalId::integer(1), ())],
        )
        .take_all_from_worktop(resource_address, "bucket")
        .with_name_lookup(|builder, namer| {
            builder.call_method(
                locker,
                ACCOUNT_LOCKER_STORE_IDENT,
                AccountLockerStoreManifestInput {
                    claimant: account.into(),
                    bucket: namer.bucket("bucket"),
                    try_direct_send: false,
                },
            )
        })
        .build();

    ledger
        .execute_manifest(manifest, [])
        .expect_commit_success();

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .call_method(
            locker,
            ACCOUNT_LOCKER_CLAIM_NON_FUNGIBLES_IDENT,
            AccountLockerClaimNonFungiblesManifestInput {
                claimant: account.into(),
                resource_address: resource_address.into(),
                ids: indexset![NonFungibleLocalId::integer(1)],
            },
        )
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(
        static_analysis.entities_requiring_auth_summary,
        dynamic_analysis.entities_requiring_auth_summary
    );
    assert!(static_analysis
        .entities_requiring_auth_summary
        .accounts
        .contains(&ManifestGlobalAddress::Static(account.into())));
}

#[test]
fn identity_securify_requires_auth() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();

    let mut allocator = TestAddressAllocator::new();
    let identity = allocator.new_identity_address();
    let identity = ManifestGlobalAddress::Static(identity.into());
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .call_method(
            identity,
            IDENTITY_SECURIFY_IDENT,
            IdentitySecurifyToSingleBadgeManifestInput {},
        )
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();

    // Act
    let (static_analysis, dynamic_analysis) = ledger.analyze(manifest);

    // Assert
    assert_eq!(
        static_analysis.entities_requiring_auth_summary,
        dynamic_analysis.entities_requiring_auth_summary
    );
    assert!(static_analysis
        .entities_requiring_auth_summary
        .identities
        .contains(&identity));
}

#[test]
fn role_assignment_setting_owner_role_of_account_requires_auth() {
    // Arrange
    let mut allocator = TestAddressAllocator::new();
    let account =
        ManifestGlobalAddress::Static(allocator.new_account_address().into());
    let manifest = ManifestBuilder::new()
        .set_owner_role(account, rule!(allow_all))
        .build();

    // Act
    let Ok(StaticAnalysis {
        entities_requiring_auth_summary,
        ..
    }) = statically_analyze(&manifest)
    else {
        unreachable!()
    };

    // Assert
    assert!(entities_requiring_auth_summary.accounts.contains(&account));
}

#[test]
fn role_assignment_locking_owner_role_of_account_requires_auth() {
    // Arrange
    let mut allocator = TestAddressAllocator::new();
    let account =
        ManifestGlobalAddress::Static(allocator.new_account_address().into());
    let manifest = ManifestBuilder::new().lock_owner_role(account).build();

    // Act
    let Ok(StaticAnalysis {
        entities_requiring_auth_summary,
        ..
    }) = statically_analyze(&manifest)
    else {
        unreachable!()
    };

    // Assert
    assert!(entities_requiring_auth_summary.accounts.contains(&account));
}

#[test]
fn role_assignment_setting_role_of_account_requires_auth() {
    // Arrange
    let mut allocator = TestAddressAllocator::new();
    let account =
        ManifestGlobalAddress::Static(allocator.new_account_address().into());
    let manifest = ManifestBuilder::new()
        .set_role(account, ModuleId::Main, "some_role", rule!(allow_all))
        .build();

    // Act
    let Ok(StaticAnalysis {
        entities_requiring_auth_summary,
        ..
    }) = statically_analyze(&manifest)
    else {
        unreachable!()
    };

    // Assert
    assert!(entities_requiring_auth_summary.accounts.contains(&account));
}

#[test]
fn role_assignment_getting_role_of_account_doesnt_require_auth() {
    // Arrange
    let mut allocator = TestAddressAllocator::new();
    let account =
        ManifestGlobalAddress::Static(allocator.new_account_address().into());
    let manifest = ManifestBuilder::new()
        .get_role(account, ModuleId::Main, "some_role".into())
        .build();

    // Act
    let Ok(StaticAnalysis {
        entities_requiring_auth_summary,
        ..
    }) = statically_analyze(&manifest)
    else {
        unreachable!()
    };

    // Assert
    assert!(!entities_requiring_auth_summary.accounts.contains(&account));
}

#[test]
fn role_assignment_setting_owner_role_of_identity_requires_auth() {
    // Arrange
    let mut allocator = TestAddressAllocator::new();
    let identity =
        ManifestGlobalAddress::Static(allocator.new_identity_address().into());
    let manifest = ManifestBuilder::new()
        .set_owner_role(identity, rule!(allow_all))
        .build();

    // Act
    let Ok(StaticAnalysis {
        entities_requiring_auth_summary,
        ..
    }) = statically_analyze(&manifest)
    else {
        unreachable!()
    };

    // Assert
    assert!(entities_requiring_auth_summary
        .identities
        .contains(&identity));
}

#[test]
fn role_assignment_locking_owner_role_of_identity_requires_auth() {
    // Arrange
    let mut allocator = TestAddressAllocator::new();
    let identity =
        ManifestGlobalAddress::Static(allocator.new_identity_address().into());
    let manifest = ManifestBuilder::new().lock_owner_role(identity).build();

    // Act
    let Ok(StaticAnalysis {
        entities_requiring_auth_summary,
        ..
    }) = statically_analyze(&manifest)
    else {
        unreachable!()
    };

    // Assert
    assert!(entities_requiring_auth_summary
        .identities
        .contains(&identity));
}

#[test]
fn role_assignment_setting_role_of_identity_requires_auth() {
    // Arrange
    let mut allocator = TestAddressAllocator::new();
    let identity =
        ManifestGlobalAddress::Static(allocator.new_identity_address().into());
    let manifest = ManifestBuilder::new()
        .set_role(identity, ModuleId::Main, "some_role", rule!(allow_all))
        .build();

    // Act
    let Ok(StaticAnalysis {
        entities_requiring_auth_summary,
        ..
    }) = statically_analyze(&manifest)
    else {
        unreachable!()
    };

    // Assert
    assert!(entities_requiring_auth_summary
        .identities
        .contains(&identity));
}

#[test]
fn role_assignment_getting_role_of_identity_doesnt_require_auth() {
    // Arrange
    let mut allocator = TestAddressAllocator::new();
    let identity =
        ManifestGlobalAddress::Static(allocator.new_identity_address().into());
    let manifest = ManifestBuilder::new()
        .get_role(identity, ModuleId::Main, "some_role".into())
        .build();

    // Act
    let Ok(StaticAnalysis {
        entities_requiring_auth_summary,
        ..
    }) = statically_analyze(&manifest)
    else {
        unreachable!()
    };

    // Assert
    assert!(!entities_requiring_auth_summary
        .identities
        .contains(&identity));
}

#[test]
fn metadata_setting_of_account_requires_auth() {
    // Arrange
    let mut allocator = TestAddressAllocator::new();
    let account =
        ManifestGlobalAddress::Static(allocator.new_account_address().into());
    let manifest = ManifestBuilder::new()
        .set_metadata(account, "foo", "bar")
        .build();

    // Act
    let Ok(StaticAnalysis {
        entities_requiring_auth_summary,
        ..
    }) = statically_analyze(&manifest)
    else {
        unreachable!()
    };

    // Assert
    assert!(entities_requiring_auth_summary.accounts.contains(&account));
}

#[test]
fn metadata_removing_of_account_requires_auth() {
    // Arrange
    let mut allocator = TestAddressAllocator::new();
    let account =
        ManifestGlobalAddress::Static(allocator.new_account_address().into());
    let manifest = ManifestBuilder::new()
        .set_metadata(account, "foo", None)
        .build();

    // Act
    let Ok(StaticAnalysis {
        entities_requiring_auth_summary,
        ..
    }) = statically_analyze(&manifest)
    else {
        unreachable!()
    };

    // Assert
    assert!(entities_requiring_auth_summary.accounts.contains(&account));
}

#[test]
fn metadata_locking_of_account_requires_auth() {
    // Arrange
    let mut allocator = TestAddressAllocator::new();
    let account =
        ManifestGlobalAddress::Static(allocator.new_account_address().into());
    let manifest = ManifestBuilder::new().lock_metadata(account, "foo").build();

    // Act
    let Ok(StaticAnalysis {
        entities_requiring_auth_summary,
        ..
    }) = statically_analyze(&manifest)
    else {
        unreachable!()
    };

    // Assert
    assert!(entities_requiring_auth_summary.accounts.contains(&account));
}

#[test]
fn metadata_getting_of_account_doesnt_require_auth() {
    // Arrange
    let mut allocator = TestAddressAllocator::new();
    let account =
        ManifestGlobalAddress::Static(allocator.new_account_address().into());
    let manifest = ManifestBuilder::new()
        .call_metadata_method(
            account,
            METADATA_GET_IDENT,
            MetadataGetManifestInput { key: "foo".into() },
        )
        .build();

    // Act
    let Ok(StaticAnalysis {
        entities_requiring_auth_summary,
        ..
    }) = statically_analyze(&manifest)
    else {
        unreachable!()
    };

    // Assert
    assert!(!entities_requiring_auth_summary.accounts.contains(&account));
}

#[test]
fn metadata_setting_of_identity_requires_auth() {
    // Arrange
    let mut allocator = TestAddressAllocator::new();
    let identity =
        ManifestGlobalAddress::Static(allocator.new_identity_address().into());
    let manifest = ManifestBuilder::new()
        .set_metadata(identity, "foo", "bar")
        .build();

    // Act
    let Ok(StaticAnalysis {
        entities_requiring_auth_summary,
        ..
    }) = statically_analyze(&manifest)
    else {
        unreachable!()
    };

    // Assert
    assert!(entities_requiring_auth_summary
        .identities
        .contains(&identity));
}

#[test]
fn metadata_removing_of_identity_requires_auth() {
    // Arrange
    let mut allocator = TestAddressAllocator::new();
    let identity =
        ManifestGlobalAddress::Static(allocator.new_identity_address().into());
    let manifest = ManifestBuilder::new()
        .set_metadata(identity, "foo", None)
        .build();

    // Act
    let Ok(StaticAnalysis {
        entities_requiring_auth_summary,
        ..
    }) = statically_analyze(&manifest)
    else {
        unreachable!()
    };

    // Assert
    assert!(entities_requiring_auth_summary
        .identities
        .contains(&identity));
}

#[test]
fn metadata_locking_of_identity_requires_auth() {
    // Arrange
    let mut allocator = TestAddressAllocator::new();
    let identity =
        ManifestGlobalAddress::Static(allocator.new_identity_address().into());
    let manifest = ManifestBuilder::new()
        .lock_metadata(identity, "foo")
        .build();

    // Act
    let Ok(StaticAnalysis {
        entities_requiring_auth_summary,
        ..
    }) = statically_analyze(&manifest)
    else {
        unreachable!()
    };

    // Assert
    assert!(entities_requiring_auth_summary
        .identities
        .contains(&identity));
}

#[test]
fn metadata_getting_of_identity_doesnt_require_auth() {
    // Arrange
    let mut allocator = TestAddressAllocator::new();
    let identity =
        ManifestGlobalAddress::Static(allocator.new_identity_address().into());
    let manifest = ManifestBuilder::new()
        .call_metadata_method(
            identity,
            METADATA_GET_IDENT,
            MetadataGetManifestInput { key: "foo".into() },
        )
        .build();

    // Act
    let Ok(StaticAnalysis {
        entities_requiring_auth_summary,
        ..
    }) = statically_analyze(&manifest)
    else {
        unreachable!()
    };

    // Assert
    assert!(!entities_requiring_auth_summary
        .identities
        .contains(&identity));
}

#[test]
fn royalty_setting_of_account_requires_auth() {
    // Arrange
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .set_component_royalty(account, "foo", RoyaltyAmount::Xrd(10.into()))
        .build();

    // Act
    let Ok(StaticAnalysis {
        entities_requiring_auth_summary,
        ..
    }) = statically_analyze(&manifest)
    else {
        unreachable!()
    };

    // Assert
    assert!(entities_requiring_auth_summary
        .accounts
        .contains(&ManifestGlobalAddress::Static(account.into())));
}

#[test]
fn royalty_locking_of_account_requires_auth() {
    // Arrange
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .lock_component_royalty(account, "foo")
        .build();

    // Act
    let Ok(StaticAnalysis {
        entities_requiring_auth_summary,
        ..
    }) = statically_analyze(&manifest)
    else {
        unreachable!()
    };

    // Assert
    assert!(entities_requiring_auth_summary
        .accounts
        .contains(&ManifestGlobalAddress::Static(account.into())));
}

#[test]
fn royalty_claiming_of_account_requires_auth() {
    // Arrange
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .claim_component_royalties(account)
        .build();

    // Act
    let Ok(StaticAnalysis {
        entities_requiring_auth_summary,
        ..
    }) = statically_analyze(&manifest)
    else {
        unreachable!()
    };

    // Assert
    assert!(entities_requiring_auth_summary
        .accounts
        .contains(&ManifestGlobalAddress::Static(account.into())));
}

#[test]
fn royalty_setting_of_identity_requires_auth() {
    // Arrange
    let mut allocator = TestAddressAllocator::new();
    let identity = allocator.new_identity_address();
    let manifest = ManifestBuilder::new()
        .set_component_royalty(identity, "foo", RoyaltyAmount::Xrd(10.into()))
        .build();

    // Act
    let Ok(StaticAnalysis {
        entities_requiring_auth_summary,
        ..
    }) = statically_analyze(&manifest)
    else {
        unreachable!()
    };

    // Assert
    assert!(entities_requiring_auth_summary
        .identities
        .contains(&ManifestGlobalAddress::Static(identity.into())));
}

#[test]
fn royalty_locking_of_identity_requires_auth() {
    // Arrange
    let mut allocator = TestAddressAllocator::new();
    let identity = allocator.new_identity_address();
    let manifest = ManifestBuilder::new()
        .lock_component_royalty(identity, "foo")
        .build();

    // Act
    let Ok(StaticAnalysis {
        entities_requiring_auth_summary,
        ..
    }) = statically_analyze(&manifest)
    else {
        unreachable!()
    };

    // Assert
    assert!(entities_requiring_auth_summary
        .identities
        .contains(&ManifestGlobalAddress::Static(identity.into())));
}

#[test]
fn royalty_claiming_of_identity_requires_auth() {
    // Arrange
    let mut allocator = TestAddressAllocator::new();
    let identity = allocator.new_identity_address();
    let manifest = ManifestBuilder::new()
        .claim_component_royalties(identity)
        .build();

    // Act
    let Ok(StaticAnalysis {
        entities_requiring_auth_summary,
        ..
    }) = statically_analyze(&manifest)
    else {
        unreachable!()
    };

    // Assert
    assert!(entities_requiring_auth_summary
        .identities
        .contains(&ManifestGlobalAddress::Static(identity.into())));
}
