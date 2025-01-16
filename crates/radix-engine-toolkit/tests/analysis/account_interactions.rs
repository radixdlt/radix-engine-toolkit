use crate::prelude::*;

#[test]
fn account_securify_is_added_to_securify_interactions_set() {
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
        static_analysis.account_interactions_summary,
        dynamic_analysis.account_interactions_summary
    );
    assert!(static_analysis
        .account_interactions_summary
        .accounts_securified
        .contains(&account));
}

#[test]
fn account_lock_fee_is_added_to_lock_fee_interaction_set() {
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
        static_analysis.account_interactions_summary,
        dynamic_analysis.account_interactions_summary
    );
    assert!(static_analysis
        .account_interactions_summary
        .accounts_locked_fees_from
        .contains(&account));
}

#[test]
fn account_lock_contingent_fee_is_added_to_lock_fee_interaction_set() {
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
        static_analysis.account_interactions_summary,
        dynamic_analysis.account_interactions_summary
    );
    assert!(static_analysis
        .account_interactions_summary
        .accounts_locked_fees_from
        .contains(&account));
}

#[test]
fn account_deposit_is_added_to_deposit_interaction_set() {
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
        static_analysis.account_interactions_summary,
        dynamic_analysis.account_interactions_summary
    );
    assert!(static_analysis
        .account_interactions_summary
        .accounts_deposited_into
        .contains(&account));
}

#[test]
fn account_deposit_batch_is_added_to_deposit_interaction_set() {
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
        static_analysis.account_interactions_summary,
        dynamic_analysis.account_interactions_summary
    );
    assert!(static_analysis
        .account_interactions_summary
        .accounts_deposited_into
        .contains(&account));
}

#[test]
fn account_try_deposit_or_abort_is_added_to_deposit_interaction_set() {
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
        static_analysis.account_interactions_summary,
        dynamic_analysis.account_interactions_summary
    );
    assert!(static_analysis
        .account_interactions_summary
        .accounts_deposited_into
        .contains(&account));
}

#[test]
fn account_try_deposit_batch_or_abort_is_added_to_deposit_interaction_set() {
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
        static_analysis.account_interactions_summary,
        dynamic_analysis.account_interactions_summary
    );
    assert!(static_analysis
        .account_interactions_summary
        .accounts_deposited_into
        .contains(&account));
}

#[test]
fn account_try_deposit_or_refund_is_added_to_deposit_interaction_set() {
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
        static_analysis.account_interactions_summary,
        dynamic_analysis.account_interactions_summary
    );
    assert!(static_analysis
        .account_interactions_summary
        .accounts_deposited_into
        .contains(&account));
}

#[test]
fn account_try_deposit_batch_or_refund_is_added_to_deposit_interaction_set() {
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
        static_analysis.account_interactions_summary,
        dynamic_analysis.account_interactions_summary
    );
    assert!(static_analysis
        .account_interactions_summary
        .accounts_deposited_into
        .contains(&account));
}

#[test]
fn account_withdraw_is_added_to_withdraw_interaction_set() {
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
        static_analysis.account_interactions_summary,
        dynamic_analysis.account_interactions_summary
    );
    assert!(static_analysis
        .account_interactions_summary
        .accounts_withdrawn_from
        .contains(&account));
}

#[test]
fn account_withdraw_non_fungibles_is_added_to_withdraw_interaction_set() {
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
        static_analysis.account_interactions_summary,
        dynamic_analysis.account_interactions_summary
    );
    assert!(static_analysis
        .account_interactions_summary
        .accounts_withdrawn_from
        .contains(&account));
}

#[test]
fn account_lock_fee_and_withdraw_is_added_to_lock_fee_and_withdraw_interaction_sets(
) {
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
        static_analysis.account_interactions_summary,
        dynamic_analysis.account_interactions_summary
    );
    assert!(static_analysis
        .account_interactions_summary
        .accounts_withdrawn_from
        .contains(&account));
    assert!(static_analysis
        .account_interactions_summary
        .accounts_locked_fees_from
        .contains(&account));
}

#[test]
fn account_lock_fee_and_withdraw_non_fungibles_is_added_to_lock_fee_and_withdraw_interaction_sets(
) {
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
        static_analysis.account_interactions_summary,
        dynamic_analysis.account_interactions_summary
    );
    assert!(static_analysis
        .account_interactions_summary
        .accounts_withdrawn_from
        .contains(&account));
    assert!(static_analysis
        .account_interactions_summary
        .accounts_locked_fees_from
        .contains(&account));
}

#[test]
fn account_burn_is_added_to_burn_interaction_set() {
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
        static_analysis.account_interactions_summary,
        dynamic_analysis.account_interactions_summary
    );
    assert!(static_analysis
        .account_interactions_summary
        .accounts_burned_from
        .contains(&account));
}

#[test]
fn account_burn_non_fungible_non_fungibles_is_added_to_burn_interaction_set() {
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
        static_analysis.account_interactions_summary,
        dynamic_analysis.account_interactions_summary
    );
    assert!(static_analysis
        .account_interactions_summary
        .accounts_burned_from
        .contains(&account));
}

#[test]
fn account_create_proof_of_is_added_to_create_proof_interaction_set() {
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
        static_analysis.account_interactions_summary,
        dynamic_analysis.account_interactions_summary
    );
    assert!(static_analysis
        .account_interactions_summary
        .accounts_created_proofs_from
        .contains(&account));
}

#[test]
fn account_create_proof_of_non_fungibles_is_added_to_create_proof_interaction_set(
) {
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
        static_analysis.account_interactions_summary,
        dynamic_analysis.account_interactions_summary
    );
    assert!(static_analysis
        .account_interactions_summary
        .accounts_created_proofs_from
        .contains(&account));
}

#[test]
fn account_set_default_deposit_rule_is_added_to_set_default_deposit_rule_interactions_set(
) {
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
        static_analysis.account_interactions_summary,
        dynamic_analysis.account_interactions_summary
    );
    assert!(static_analysis
        .account_interactions_summary
        .accounts_set_default_deposit_rule_of
        .contains(&account));
}

#[test]
fn account_set_resource_preference_is_added_to_set_resource_preference_interactions_set(
) {
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
        static_analysis.account_interactions_summary,
        dynamic_analysis.account_interactions_summary
    );
    assert!(static_analysis
        .account_interactions_summary
        .accounts_set_resource_preference_into
        .contains(&account));
}

#[test]
fn account_remove_resource_preference_is_added_to_remove_resource_preference_interactions_set(
) {
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
        static_analysis.account_interactions_summary,
        dynamic_analysis.account_interactions_summary
    );
    assert!(static_analysis
        .account_interactions_summary
        .accounts_remove_resource_preference_from
        .contains(&account));
}

#[test]
fn account_add_authorized_depositor_preference_is_added_to_add_authorized_depositor_preference_interactions_set(
) {
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
        static_analysis.account_interactions_summary,
        dynamic_analysis.account_interactions_summary
    );
    assert!(static_analysis
        .account_interactions_summary
        .accounts_add_authorized_depositor_into
        .contains(&account));
}

#[test]
fn account_remove_authorized_depositor_preference_is_added_to_remove_authorized_depositor_preference_interactions_set(
) {
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
        static_analysis.account_interactions_summary,
        dynamic_analysis.account_interactions_summary
    );
    assert!(static_analysis
        .account_interactions_summary
        .accounts_remove_authorized_depositor_from
        .contains(&account));
}

fn account_securify_manifest() -> (TransactionManifestV1, ComponentAddress) {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .call_method(
            account,
            ACCOUNT_SECURIFY_IDENT,
            AccountSecurifyManifestInput {},
        )
        .try_deposit_entire_worktop_or_abort(account, None)
        .build();
    (manifest, account)
}

fn account_lock_fee_manifest() -> (TransactionManifestV1, ComponentAddress) {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new().lock_fee(account, 1).build();
    (manifest, account)
}

fn account_lock_contingent_fee_manifest(
) -> (TransactionManifestV1, ComponentAddress) {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .lock_contingent_fee(account, 1)
        .build();
    (manifest, account)
}

fn account_deposit_manifest() -> (TransactionManifestV1, ComponentAddress) {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .mint_fungible(XRD, 1)
        .take_all_from_worktop(XRD, "bucket")
        .deposit(account, "bucket")
        .build();
    (manifest, account)
}

fn account_deposit_batch_manifest() -> (TransactionManifestV1, ComponentAddress)
{
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .mint_fungible(XRD, 1)
        .deposit_batch(account, ManifestExpression::EntireWorktop)
        .build();
    (manifest, account)
}

fn account_try_deposit_or_abort_manifest(
) -> (TransactionManifestV1, ComponentAddress) {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .mint_fungible(XRD, 1)
        .take_all_from_worktop(XRD, "bucket")
        .try_deposit_or_abort(account, None, "bucket")
        .build();
    (manifest, account)
}

fn account_try_deposit_batch_or_abort_manifest(
) -> (TransactionManifestV1, ComponentAddress) {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .mint_fungible(XRD, 1)
        .try_deposit_batch_or_abort(
            account,
            ManifestExpression::EntireWorktop,
            None,
        )
        .build();
    (manifest, account)
}

fn account_try_deposit_or_refund_manifest(
) -> (TransactionManifestV1, ComponentAddress) {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .mint_fungible(XRD, 1)
        .take_all_from_worktop(XRD, "bucket")
        .try_deposit_or_refund(account, None, "bucket")
        .build();
    (manifest, account)
}

fn account_try_deposit_batch_or_refund_manifest(
) -> (TransactionManifestV1, ComponentAddress) {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .mint_fungible(XRD, 1)
        .try_deposit_batch_or_refund(
            account,
            ManifestExpression::EntireWorktop,
            None,
        )
        .build();
    (manifest, account)
}

fn account_withdraw_manifest() -> (TransactionManifestV1, ComponentAddress) {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account, XRD, 1)
        .deposit_batch(account, ManifestExpression::EntireWorktop)
        .build();
    (manifest, account)
}

fn account_withdraw_non_fungibles_manifest(
    resource_address: ResourceAddress,
) -> (TransactionManifestV1, ComponentAddress) {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .mint_non_fungible(
            resource_address,
            [(NonFungibleLocalId::integer(1), ())],
        )
        .deposit_batch(account, ManifestExpression::EntireWorktop)
        .withdraw_non_fungible_from_account(
            account,
            NonFungibleGlobalId::new(
                resource_address,
                NonFungibleLocalId::integer(1),
            ),
        )
        .deposit_batch(account, ManifestExpression::EntireWorktop)
        .build();
    (manifest, account)
}

fn account_lock_fee_and_withdraw_manifest(
) -> (TransactionManifestV1, ComponentAddress) {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .lock_fee_and_withdraw(account, 1, XRD, 1)
        .deposit_batch(account, ManifestExpression::EntireWorktop)
        .build();
    (manifest, account)
}

fn account_lock_fee_and_withdraw_non_fungibles_manifest(
    resource_address: ResourceAddress,
) -> (TransactionManifestV1, ComponentAddress) {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .mint_non_fungible(
            resource_address,
            [(NonFungibleLocalId::integer(1), ())],
        )
        .deposit_batch(account, ManifestExpression::EntireWorktop)
        .lock_fee_and_withdraw_non_fungibles(
            account,
            1,
            resource_address,
            [NonFungibleLocalId::integer(1)],
        )
        .deposit_batch(account, ManifestExpression::EntireWorktop)
        .build();
    (manifest, account)
}

fn account_burn_manifest() -> (TransactionManifestV1, ComponentAddress) {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .burn_in_account(account, XRD, 1)
        .build();
    (manifest, account)
}

fn account_burn_non_fungible_non_fungibles_manifest(
    resource_address: ResourceAddress,
) -> (TransactionManifestV1, ComponentAddress) {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .mint_non_fungible(
            resource_address,
            [(NonFungibleLocalId::integer(1), ())],
        )
        .deposit_batch(account, ManifestExpression::EntireWorktop)
        .burn_non_fungible_in_account(
            account,
            NonFungibleGlobalId::new(
                resource_address,
                NonFungibleLocalId::integer(1),
            ),
        )
        .build();
    (manifest, account)
}

fn account_create_proof_of_manifest(
) -> (TransactionManifestV1, ComponentAddress) {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_amount(account, XRD, 1)
        .deposit_batch(account, ManifestExpression::EntireWorktop)
        .build();
    (manifest, account)
}

fn account_create_proof_of_non_fungibles_manifest(
    resource_address: ResourceAddress,
) -> (TransactionManifestV1, ComponentAddress) {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .mint_non_fungible(
            resource_address,
            [(NonFungibleLocalId::integer(1), ())],
        )
        .deposit_batch(account, ManifestExpression::EntireWorktop)
        .create_proof_from_account_of_non_fungible(
            account,
            NonFungibleGlobalId::new(
                resource_address,
                NonFungibleLocalId::integer(1),
            ),
        )
        .deposit_batch(account, ManifestExpression::EntireWorktop)
        .build();
    (manifest, account)
}

fn account_set_default_deposit_rule_manifest(
) -> (TransactionManifestV1, ComponentAddress) {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .call_method(
            account,
            ACCOUNT_SET_DEFAULT_DEPOSIT_RULE_IDENT,
            AccountSetDefaultDepositRuleManifestInput {
                default: DefaultDepositRule::Accept,
            },
        )
        .build();
    (manifest, account)
}

fn account_set_resource_preference_manifest(
) -> (TransactionManifestV1, ComponentAddress) {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .call_method(
            account,
            ACCOUNT_SET_RESOURCE_PREFERENCE_IDENT,
            AccountSetResourcePreferenceManifestInput {
                resource_address: XRD.into(),
                resource_preference: ResourcePreference::Allowed,
            },
        )
        .build();
    (manifest, account)
}

fn account_remove_resource_preference_manifest(
) -> (TransactionManifestV1, ComponentAddress) {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .call_method(
            account,
            ACCOUNT_REMOVE_RESOURCE_PREFERENCE_IDENT,
            AccountRemoveResourcePreferenceManifestInput {
                resource_address: XRD.into(),
            },
        )
        .build();
    (manifest, account)
}

fn account_add_authorized_depositor_preference_manifest(
) -> (TransactionManifestV1, ComponentAddress) {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .call_method(
            account,
            ACCOUNT_ADD_AUTHORIZED_DEPOSITOR_IDENT,
            AccountAddAuthorizedDepositorManifestInput {
                badge: ManifestResourceOrNonFungible::Resource(XRD.into()),
            },
        )
        .build();
    (manifest, account)
}

fn account_remove_authorized_depositor_preference_manifest(
) -> (TransactionManifestV1, ComponentAddress) {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let manifest = ManifestBuilder::new()
        .call_method(
            account,
            ACCOUNT_REMOVE_AUTHORIZED_DEPOSITOR_IDENT,
            AccountRemoveAuthorizedDepositorManifestInput {
                badge: ManifestResourceOrNonFungible::Resource(XRD.into()),
            },
        )
        .build();
    (manifest, account)
}
