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
        .assert_worktop_is_empty()
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
        .assert_worktop_is_empty()
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
        .assert_worktop_is_empty()
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
        .assert_worktop_is_empty()
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

type Subintent = ManifestBuilder<SubintentManifestV2>;

/// A subintent of each class that may be a pre-authorization, by the calls it
/// makes on an access controller and the account it may deposit into.
#[allow(clippy::type_complexity)]
fn subintent_classes() -> [(
    ManifestClassification,
    fn(Subintent, ComponentAddress, ComponentAddress) -> Subintent,
); 6] {
    [
        (
            ManifestClassification::AccessControllerRecovery,
            |builder, access_controller, _| {
                builder
                    .initiate_recovery(
                        access_controller,
                        Proposer::Recovery,
                        proposal(allow_all(), None),
                    )
                    .quick_confirm_recovery(
                        access_controller,
                        Proposer::Recovery,
                        proposal(allow_all(), None),
                    )
            },
        ),
        (
            ManifestClassification::AccessControllerTimedRecovery,
            |builder, access_controller, _| {
                builder.initiate_recovery(
                    access_controller,
                    Proposer::Recovery,
                    proposal(allow_all(), None),
                )
            },
        ),
        (
            ManifestClassification::AccessControllerConfirmRecovery,
            |builder, access_controller, _| {
                builder.quick_confirm_recovery(
                    access_controller,
                    Proposer::Primary,
                    proposal(allow_all(), None),
                )
            },
        ),
        (
            ManifestClassification::AccessControllerCancelRecovery,
            |builder, access_controller, _| {
                builder.cancel_recovery(access_controller, Proposer::Primary)
            },
        ),
        (
            ManifestClassification::AccessControllerPrimaryRoleLock,
            |builder, access_controller, _| {
                builder.lock_primary_role(access_controller)
            },
        ),
        (
            ManifestClassification::AccessControllerRecoveryFee,
            |builder, access_controller, account| {
                builder
                    .withdraw_recovery_fee(access_controller, 10)
                    .try_deposit_entire_worktop_or_abort(account, None)
            },
        ),
    ]
}

fn access_controller_classes_of(
    manifest: &SubintentManifestV2,
) -> Vec<ManifestClassification> {
    classify(manifest)
        .into_iter()
        .filter(is_access_controller_class)
        .collect()
}

/// The access controller classes of the transaction manifest that
/// `as_enclosed` turns an enclosed subintent into, which is what gets
/// previewed and dynamically analyzed.
fn enclosed_access_controller_classes_of(
    manifest: &SubintentManifestV2,
) -> Vec<ManifestClassification> {
    let enclosed = radix_engine_toolkit::functions::transaction_v2::subintent_manifest::as_enclosed(manifest)
        .expect("Must be an enclosed subintent");
    let StaticAnalysis {
        manifest_classification,
        ..
    } = statically_analyze(&enclosed).unwrap();
    manifest_classification
        .into_iter()
        .filter(is_access_controller_class)
        .collect()
}

/// The fee of a transaction is paid by whoever submits it. In a subintent a fee
/// lock either fails or is a contingent one, a payment the class's output
/// doesn't show, so a subintent may not pay the fee, even though a transaction
/// manifest of the same class may, see `access_controller_other_effects`.
///
/// The same holds for the transaction manifest an enclosed subintent is turned
/// into to preview it, which no longer yields to its parent.
#[test]
fn paying_the_fee_in_a_subintent_is_not_classified() {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let access_controller = allocator.new_access_controller_address();
    let other_access_controller = allocator.new_access_controller_address();
    let fee_payments: [fn(
        Subintent,
        ComponentAddress,
        ComponentAddress,
    ) -> Subintent; 5] = [
        |builder, account, _| builder.lock_fee(account, 10),
        |builder, account, _| builder.lock_contingent_fee(account, 10),
        |builder, account, _| {
            builder.create_proof_from_account_of_amount(account, XRD, 1)
        },
        // The proof of a securified account's badge, so that it can pay.
        |builder, _, access_controller| {
            builder.call_method(
                access_controller,
                ACCESS_CONTROLLER_CREATE_PROOF_IDENT,
                (),
            )
        },
        |builder, _, access_controller| {
            builder.call_method(
                access_controller,
                ACCESS_CONTROLLER_LOCK_RECOVERY_FEE_IDENT,
                AccessControllerLockRecoveryFeeManifestInput {
                    amount: Decimal::one(),
                },
            )
        },
    ];

    for (class, calls) in subintent_classes() {
        let without_fee = calls(
            ManifestBuilder::new_subintent_v2().assert_worktop_is_empty(),
            access_controller,
            account,
        )
        .yield_to_parent(())
        .build();
        assert_eq!(access_controller_classes_of(&without_fee), vec![class]);
        assert_eq!(
            enclosed_access_controller_classes_of(&without_fee),
            vec![class]
        );

        for pay_fee in fee_payments {
            let fee_first = calls(
                pay_fee(
                    ManifestBuilder::new_subintent_v2()
                        .assert_worktop_is_empty(),
                    account,
                    other_access_controller,
                ),
                access_controller,
                account,
            )
            .yield_to_parent(())
            .build();
            let fee_last = pay_fee(
                calls(
                    ManifestBuilder::new_subintent_v2()
                        .assert_worktop_is_empty(),
                    access_controller,
                    account,
                ),
                account,
                other_access_controller,
            )
            .yield_to_parent(())
            .build();
            for manifest in [fee_first, fee_last] {
                assert_eq!(
                    access_controller_classes_of(&manifest),
                    vec![],
                    "{class:?}"
                );
                assert_eq!(
                    enclosed_access_controller_classes_of(&manifest),
                    vec![],
                    "{class:?} as an enclosed manifest"
                );
            }
        }
    }
}

/// Yielding to the parent before the end lets the parent run its own
/// instructions between the subintent's.
#[test]
fn yielding_to_the_parent_before_the_end_is_not_classified() {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let access_controller = allocator.new_access_controller_address();

    for (class, calls) in subintent_classes() {
        let start =
            || ManifestBuilder::new_subintent_v2().assert_worktop_is_empty();
        let yields_once = calls(start(), access_controller, account)
            .yield_to_parent(())
            .build();
        assert_eq!(access_controller_classes_of(&yields_once), vec![class]);

        // Yielding, verifying the parent, or asserting, after the class's
        // calls and a first yield. Only the last instruction may yield, even
        // when the subintent does nothing in between.
        let yields_twice = calls(start(), access_controller, account)
            .yield_to_parent(())
            .yield_to_parent(())
            .build();
        let yields_first =
            calls(start().yield_to_parent(()), access_controller, account)
                .yield_to_parent(())
                .build();
        let verifies_after_yielding =
            calls(start(), access_controller, account)
                .yield_to_parent(())
                .verify_parent(rule!(allow_all))
                .yield_to_parent(())
                .build();
        let asserts_after_yielding = calls(start(), access_controller, account)
            .yield_to_parent(())
            .assert_worktop_is_empty()
            .yield_to_parent(())
            .build();
        for manifest in [
            yields_twice,
            yields_first,
            verifies_after_yielding,
            asserts_after_yielding,
        ] {
            assert_eq!(
                access_controller_classes_of(&manifest),
                vec![],
                "{class:?}"
            );
        }
    }

    // Between the calls of a class: the parent could hand XRD of its own to
    // the subintent before it deposits what it withdrew.
    let manifests = [
        ManifestBuilder::new_subintent_v2()
            .assert_worktop_is_empty()
            .initiate_recovery(
                access_controller,
                Proposer::Recovery,
                proposal(allow_all(), None),
            )
            .yield_to_parent(())
            .quick_confirm_recovery(
                access_controller,
                Proposer::Recovery,
                proposal(allow_all(), None),
            )
            .yield_to_parent(())
            .build(),
        ManifestBuilder::new_subintent_v2()
            .assert_worktop_is_empty()
            .withdraw_recovery_fee(access_controller, 10)
            .yield_to_parent(())
            .take_all_from_worktop(XRD, "xrd")
            .try_deposit_or_abort(account, None, "xrd")
            .yield_to_parent(())
            .build(),
    ];
    for manifest in manifests {
        assert_eq!(access_controller_classes_of(&manifest), vec![]);
    }
}

/// An open subintent, one that doesn't start with `ASSERT_WORKTOP_IS_EMPTY`,
/// can't be turned into a transaction manifest by `as_enclosed` to preview it,
/// so it would only ever get a static class, which names no rules.
#[test]
fn open_subintent_is_not_classified() {
    let mut allocator = TestAddressAllocator::new();
    let account = allocator.new_account_address();
    let access_controller = allocator.new_access_controller_address();

    for (class, calls) in subintent_classes() {
        let enclosed = calls(
            ManifestBuilder::new_subintent_v2().assert_worktop_is_empty(),
            access_controller,
            account,
        )
        .yield_to_parent(())
        .build();
        assert_eq!(access_controller_classes_of(&enclosed), vec![class]);
        let enclosed_verifying_parent = calls(
            ManifestBuilder::new_subintent_v2()
                .assert_worktop_is_empty()
                .verify_parent(rule!(allow_all)),
            access_controller,
            account,
        )
        .yield_to_parent(())
        .build();
        assert_eq!(
            access_controller_classes_of(&enclosed_verifying_parent),
            vec![class]
        );

        let open = calls(
            ManifestBuilder::new_subintent_v2(),
            access_controller,
            account,
        )
        .yield_to_parent(())
        .build();
        let open_verifying_parent = calls(
            ManifestBuilder::new_subintent_v2().verify_parent(rule!(allow_all)),
            access_controller,
            account,
        )
        .yield_to_parent(())
        .build();
        // The assertion isn't the first instruction.
        let asserting_after_verifying_parent = calls(
            ManifestBuilder::new_subintent_v2()
                .verify_parent(rule!(allow_all))
                .assert_worktop_is_empty(),
            access_controller,
            account,
        )
        .yield_to_parent(())
        .build();
        let asserting_after_the_calls = calls(
            ManifestBuilder::new_subintent_v2(),
            access_controller,
            account,
        )
        .assert_worktop_is_empty()
        .yield_to_parent(())
        .build();
        for manifest in [
            open,
            open_verifying_parent,
            asserting_after_verifying_parent,
            asserting_after_the_calls,
        ] {
            assert_eq!(
                access_controller_classes_of(&manifest),
                vec![],
                "{class:?}"
            );
        }
    }
}
