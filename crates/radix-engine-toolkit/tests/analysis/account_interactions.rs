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
use radix_transactions::manifest::static_resource_movements::{
    AccountWithdraw, ChangeSource, SimpleFungibleResourceBounds,
    SimpleResourceBounds, UnspecifiedResources,
};

#[test]
fn account_securify_is_added_to_securify_interactions_set() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (manifest, account) = account_securify_manifest();
    ledger.load_account_from_faucet(account);
    let account = ManifestGlobalAddress::Static(account.into());

    // Act
    let (static_analysis, dynamic_analysis, ..) = ledger.analyze(manifest);

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
    let (static_analysis, dynamic_analysis, ..) = ledger.analyze(manifest);

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
    let (static_analysis, dynamic_analysis, ..) = ledger.analyze(manifest);

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
    let (static_analysis, dynamic_analysis, ..) = ledger.analyze(manifest);

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
    let (static_analysis, dynamic_analysis, ..) = ledger.analyze(manifest);

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
    let (static_analysis, dynamic_analysis, ..) = ledger.analyze(manifest);

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
    let (static_analysis, dynamic_analysis, ..) = ledger.analyze(manifest);

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
    let (static_analysis, dynamic_analysis, ..) = ledger.analyze(manifest);

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
    let (static_analysis, dynamic_analysis, ..) = ledger.analyze(manifest);

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
    let (static_analysis, dynamic_analysis, ..) = ledger.analyze(manifest);

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
    let (static_analysis, dynamic_analysis, ..) = ledger.analyze(manifest);

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
    let (static_analysis, dynamic_analysis, ..) = ledger.analyze(manifest);

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
    let (static_analysis, dynamic_analysis, ..) = ledger.analyze(manifest);

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
    let (static_analysis, dynamic_analysis, ..) = ledger.analyze(manifest);

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
    let (static_analysis, dynamic_analysis, ..) = ledger.analyze(manifest);

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
    let (static_analysis, dynamic_analysis, ..) = ledger.analyze(manifest);

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
    let (static_analysis, dynamic_analysis, ..) = ledger.analyze(manifest);

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
    let (static_analysis, dynamic_analysis, ..) = ledger.analyze(manifest);

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
    let (static_analysis, dynamic_analysis, ..) = ledger.analyze(manifest);

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
    let (static_analysis, dynamic_analysis, ..) = ledger.analyze(manifest);

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
    let (static_analysis, dynamic_analysis, ..) = ledger.analyze(manifest);

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
    let (static_analysis, dynamic_analysis, ..) = ledger.analyze(manifest);

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

#[test]
fn analysis_works_even_when_dynamic_resources_are_used() {
    // Arrange
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let (_, _, account) = ledger.new_account(true);

    let builder = ManifestBuilder::new();
    let namer = builder.name_lookup();
    let manifest = builder
        .lock_fee_from_faucet()
        .allocate_global_address(
            RESOURCE_PACKAGE,
            FUNGIBLE_RESOURCE_MANAGER_BLUEPRINT,
            "reservation",
            "address",
        )
        .call_function(
            RESOURCE_PACKAGE,
            FUNGIBLE_RESOURCE_MANAGER_BLUEPRINT,
            FUNGIBLE_RESOURCE_MANAGER_CREATE_WITH_INITIAL_SUPPLY_IDENT,
            FungibleResourceManagerCreateWithInitialSupplyManifestInput {
                owner_role: OwnerRole::default().into(),
                track_total_supply: true,
                divisibility: 18,
                resource_roles: Default::default(),
                metadata: Default::default(),
                address_reservation: namer
                    .address_reservation("reservation")
                    .into(),
                initial_supply: dec!(100),
            },
        )
        .deposit_entire_worktop(account)
        .call_method(
            account,
            ACCOUNT_WITHDRAW_IDENT,
            AccountWithdrawManifestInput {
                resource_address: namer.named_address("address").into(),
                amount: dec!(100),
            },
        )
        .deposit_entire_worktop(account)
        .build();

    // Act
    let (static_analysis, dynamic_analysis, receipt) = ledger.analyze(manifest);

    // Assert
    let newly_created_resource = *receipt
        .expect_commit_success()
        .new_resource_addresses()
        .first()
        .unwrap();

    let dynamic_resource = AnalyzerResourceAddress::Dynamic {
        blueprint_id: BlueprintId::new(
            &RESOURCE_PACKAGE,
            FUNGIBLE_RESOURCE_MANAGER_BLUEPRINT,
        ),
        named_address: 0,
    };

    // Static analysis: account withdraws
    {
        let withdraws = &static_analysis
            .account_static_resource_movements_summary
            .account_withdraws;
        assert_eq!(withdraws.len(), 1);
        assert_eq!(
            withdraws.get(&account).unwrap(),
            &vec![AccountWithdraw::Amount(dynamic_resource.clone(), dec!(100))]
        );
    }

    // Static analysis: account deposits
    {
        let deposits = &static_analysis
            .account_static_resource_movements_summary
            .account_deposits;
        assert_eq!(deposits.len(), 1);
        let deposits = deposits.get(&account).unwrap();
        assert_eq!(deposits.len(), 2);

        assert_eq!(
            deposits[0].specified_resources(),
            &indexmap! {
                dynamic_resource.clone() => SimpleResourceBounds::Fungible(
                    SimpleFungibleResourceBounds::AtLeast(dec!(100))
                )
            }
        );
        assert_eq!(
            deposits[0].unspecified_resources(),
            UnspecifiedResources::MayBePresent(indexset! {
                ChangeSource::Invocation { instruction_index: 0 }
            })
        );

        assert_eq!(
            deposits[1].specified_resources(),
            &indexmap! {
                dynamic_resource.clone() => SimpleResourceBounds::Fungible(
                    SimpleFungibleResourceBounds::Exact(dec!(100))
                )
            }
        );
        assert_eq!(
            deposits[1].unspecified_resources(),
            UnspecifiedResources::NonePresent
        );
    }

    assert!(dynamic_analysis
        .account_dynamic_resource_movements_summary
        .account_withdraws
        .is_empty());

    {
        let deposits = &dynamic_analysis
            .account_dynamic_resource_movements_summary
            .account_deposits;
        assert_eq!(deposits.len(), 1);
        assert_eq!(
            deposits.get(&GlobalAddress::from(account)).unwrap(),
            &vec![
                InvocationIoItem::Fungible(
                    newly_created_resource,
                    EitherGuaranteedOrPredicted::Predicted(Tracked {
                        value: dec!(100),
                        created_at: 3.into()
                    })
                ),
                InvocationIoItem::Fungible(
                    newly_created_resource,
                    EitherGuaranteedOrPredicted::Predicted(Tracked {
                        value: dec!(100),
                        created_at: 5.into()
                    })
                ),
            ]
        );
    }
}
