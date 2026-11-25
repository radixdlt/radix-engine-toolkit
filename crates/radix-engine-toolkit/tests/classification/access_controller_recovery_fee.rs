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

const RECOVERY_FEE: Option<ManifestClassification> =
    Some(ManifestClassification::AccessControllerRecoveryFee);

struct Addresses {
    account: ComponentAddress,
    other_account: ComponentAddress,
    access_controller: ComponentAddress,
    other_access_controller: ComponentAddress,
    resource: ResourceAddress,
}

fn addresses() -> Addresses {
    let mut allocator = TestAddressAllocator::new();
    Addresses {
        account: allocator.new_account_address(),
        other_account: allocator.new_account_address(),
        access_controller: allocator.new_access_controller_address(),
        other_access_controller: allocator.new_access_controller_address(),
        resource: ResourceAddress::new_or_panic(
            allocator
                .new_node_id(EntityType::GlobalFungibleResourceManager)
                .0,
        ),
    }
}

#[test]
fn topping_up_is_a_recovery_fee_manifest() {
    let Addresses {
        account,
        access_controller,
        other_access_controller,
        ..
    } = addresses();

    let one =
        top_up_recovery_fee(ManifestBuilder::new(), account, access_controller);
    // Twice into one access controller and once into another.
    let several = [
        access_controller,
        access_controller,
        other_access_controller,
    ]
    .into_iter()
    .fold(ManifestBuilder::new(), |builder, access_controller| {
        top_up_recovery_fee(builder, account, access_controller)
    });
    // As the wallet builds it: the fee payer's badge proves it may withdraw.
    let with_proof = top_up_recovery_fee(
        ManifestBuilder::new().lock_fee(account, 10).call_method(
            access_controller,
            ACCESS_CONTROLLER_CREATE_PROOF_IDENT,
            (),
        ),
        account,
        other_access_controller,
    );
    let all_at_once = ManifestBuilder::new()
        .withdraw_from_account(account, XRD, 100)
        .take_all_from_worktop(XRD, "xrd")
        .contribute_recovery_fee(access_controller, "xrd");

    for builder in [one, several, with_proof, all_at_once] {
        assert_access_controller_class(builder.build(), RECOVERY_FEE);
    }
}

#[test]
fn withdrawing_is_a_recovery_fee_manifest() {
    let Addresses {
        account,
        other_account,
        access_controller,
        other_access_controller,
        ..
    } = addresses();

    let deposits: [fn(ManifestBuilder, ComponentAddress) -> ManifestBuilder;
        4] = [
        |builder, account| {
            builder.try_deposit_entire_worktop_or_abort(account, None)
        },
        |builder, account| builder.deposit_entire_worktop(account),
        |builder, account| {
            builder
                .take_all_from_worktop(XRD, "xrd")
                .try_deposit_or_abort(account, None, "xrd")
        },
        |builder, account| {
            builder.take_from_worktop(XRD, 1, "xrd").then(|builder| {
                let bucket = builder.bucket("xrd");
                builder
                    .call_method(account, ACCOUNT_DEPOSIT_IDENT, (bucket,))
                    .deposit_entire_worktop(account)
            })
        },
    ];
    for deposit in deposits {
        let one = deposit(
            ManifestBuilder::new().withdraw_recovery_fee(access_controller, 10),
            account,
        );
        let several = deposit(
            ManifestBuilder::new()
                .withdraw_recovery_fee(access_controller, 10)
                .withdraw_recovery_fee(other_access_controller, 10),
            other_account,
        );
        for builder in [one, several] {
            assert_access_controller_class(builder.build(), RECOVERY_FEE);
        }
    }
}

#[test]
fn moving_xrd_both_ways_is_not_a_recovery_fee_manifest() {
    let Addresses {
        account,
        other_account,
        access_controller,
        other_access_controller,
        ..
    } = addresses();

    let manifests = [
        // Out of one recovery fee vault and into another.
        ManifestBuilder::new()
            .withdraw_recovery_fee(access_controller, 10)
            .take_all_from_worktop(XRD, "xrd")
            .contribute_recovery_fee(other_access_controller, "xrd"),
        // Topping up with XRD withdrawn from an access controller.
        top_up_recovery_fee(
            ManifestBuilder::new().withdraw_recovery_fee(access_controller, 10),
            account,
            other_access_controller,
        )
        .deposit_entire_worktop(account),
        // Topping up, and transferring the rest to another account.
        ManifestBuilder::new()
            .withdraw_from_account(account, XRD, 100)
            .take_from_worktop(XRD, 10, "xrd")
            .contribute_recovery_fee(access_controller, "xrd")
            .try_deposit_entire_worktop_or_abort(other_account, None),
        // Withdrawing, and transferring from an account too.
        ManifestBuilder::new()
            .withdraw_recovery_fee(access_controller, 10)
            .withdraw_from_account(account, XRD, 100)
            .try_deposit_entire_worktop_or_abort(other_account, None),
        ManifestBuilder::new()
            .withdraw_from_account(account, XRD, 100)
            .withdraw_recovery_fee(access_controller, 10)
            .try_deposit_entire_worktop_or_abort(other_account, None),
    ];

    for builder in manifests {
        assert_access_controller_class(builder.build(), None);
    }
}

#[test]
fn moving_only_between_accounts_is_not_a_recovery_fee_manifest() {
    let Addresses {
        account,
        other_account,
        ..
    } = addresses();

    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account, XRD, 100)
        .try_deposit_entire_worktop_or_abort(other_account, None)
        .build();

    assert_access_controller_class(manifest, None);
}

#[test]
fn moving_other_resources_is_not_a_recovery_fee_manifest() {
    let Addresses {
        account,
        access_controller,
        resource,
        ..
    } = addresses();

    let manifests = [
        ManifestBuilder::new()
            .withdraw_from_account(account, resource, 100)
            .take_all_from_worktop(resource, "bucket")
            .contribute_recovery_fee(access_controller, "bucket"),
        top_up_recovery_fee(
            ManifestBuilder::new().withdraw_from_account(account, resource, 1),
            account,
            access_controller,
        )
        .deposit_entire_worktop(account),
        ManifestBuilder::new()
            .withdraw_recovery_fee(access_controller, 10)
            .withdraw_from_account(account, resource, 1)
            .deposit_entire_worktop(account),
    ];

    for builder in manifests {
        assert_access_controller_class(builder.build(), None);
    }
}

#[test]
fn anything_else_is_not_a_recovery_fee_manifest() {
    let Addresses {
        account,
        access_controller,
        ..
    } = addresses();

    type Effect = fn(ManifestBuilder) -> ManifestBuilder;
    let top_up: Effect = |builder| {
        let Addresses {
            account,
            access_controller,
            ..
        } = addresses();
        top_up_recovery_fee(builder, account, access_controller)
    };
    let withdraw: Effect = |builder| {
        let Addresses {
            account,
            access_controller,
            ..
        } = addresses();
        builder
            .withdraw_recovery_fee(access_controller, 10)
            .deposit_entire_worktop(account)
    };
    let effects: [fn(
        ManifestBuilder,
        ComponentAddress,
        ComponentAddress,
    ) -> ManifestBuilder; 5] = [
        |builder, account, _| {
            builder.set_metadata(account, "owner_keys", "not important")
        },
        |builder, _, _| builder.set_metadata(XRD, "name", "not important"),
        |builder, _, access_controller| {
            builder.lock_primary_role(access_controller)
        },
        |builder, _, access_controller| {
            builder.cancel_recovery(access_controller, Proposer::Recovery)
        },
        |builder, _, access_controller| {
            builder.call_method(
                access_controller,
                ACCESS_CONTROLLER_INITIATE_BADGE_WITHDRAW_ATTEMPT_AS_PRIMARY_IDENT,
                (),
            )
        },
    ];

    for movement in [top_up, withdraw] {
        assert_access_controller_class(
            movement(ManifestBuilder::new()).build(),
            RECOVERY_FEE,
        );
        for effect in effects {
            let effect_first = movement(effect(
                ManifestBuilder::new(),
                account,
                access_controller,
            ));
            let effect_last = effect(
                movement(ManifestBuilder::new()),
                account,
                access_controller,
            );
            for builder in [effect_first, effect_last] {
                assert_access_controller_class(builder.build(), None);
            }
        }
    }
}

#[test]
fn paying_the_fee_is_permitted() {
    let Addresses {
        account,
        access_controller,
        other_access_controller,
        ..
    } = addresses();

    let fee_payments: [fn(
        ManifestBuilder,
        ComponentAddress,
        ComponentAddress,
    ) -> ManifestBuilder; 4] = [
        |builder, account, _| builder.lock_fee(account, 10),
        |builder, account, _| builder.lock_contingent_fee(account, 10),
        |builder, account, _| {
            builder.create_proof_from_account_of_amount(account, XRD, 1)
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

    for pay_fee in fee_payments {
        let builder =
            || pay_fee(ManifestBuilder::new(), account, access_controller);
        let top_up =
            top_up_recovery_fee(builder(), account, other_access_controller);
        let withdraw = builder()
            .withdraw_recovery_fee(other_access_controller, 10)
            .deposit_entire_worktop(account);
        for builder in [top_up, withdraw] {
            assert_access_controller_class(builder.build(), RECOVERY_FEE);
        }
    }
}

/// Securifying an entity and topping up the access controller it creates are
/// separate transactions: together they're of neither class.
#[test]
fn securifying_and_topping_up_is_not_classified() {
    let Addresses { account, .. } = addresses();
    let manifest = ManifestBuilder::new()
        .call_method(
            account,
            ACCOUNT_SECURIFY_IDENT,
            AccountSecurifyManifestInput {},
        )
        .take_all_from_worktop(ACCOUNT_OWNER_BADGE, "badge")
        .withdraw_from_account(account, XRD, 100)
        .take_all_from_worktop(XRD, "xrd")
        .allocate_global_address(
            ACCESS_CONTROLLER_PACKAGE,
            ACCESS_CONTROLLER_BLUEPRINT,
            "reservation",
            "access_controller",
        )
        .then(|builder| {
            let badge = builder.bucket("badge");
            let xrd = builder.bucket("xrd");
            let reservation = builder.address_reservation("reservation");
            let access_controller = builder.named_address("access_controller");
            builder
                .call_function(
                    ACCESS_CONTROLLER_PACKAGE,
                    ACCESS_CONTROLLER_BLUEPRINT,
                    ACCESS_CONTROLLER_CREATE_IDENT,
                    AccessControllerCreateManifestInput {
                        controlled_asset: badge,
                        rule_set: allow_all().into(),
                        timed_recovery_delay_in_minutes: None,
                        address_reservation: Some(reservation),
                    },
                )
                .call_method(
                    access_controller,
                    ACCESS_CONTROLLER_CONTRIBUTE_RECOVERY_FEE_IDENT,
                    AccessControllerContributeRecoveryFeeManifestInput {
                        bucket: xrd,
                    },
                )
        })
        .build();

    let StaticAnalysis {
        manifest_classification,
        ..
    } = statically_analyze(&manifest).unwrap();
    assert!(!manifest_classification
        .contains(&ManifestClassification::EntitySecurify));
    assert_access_controller_class(manifest, None);
}

#[test]
fn moving_xrd_into_or_out_of_an_access_controller_of_the_manifest_is_not_classified(
) {
    let calls: [fn(ManifestBuilder, ManifestNamedAddress) -> ManifestBuilder;
        2] = [
        |builder, access_controller| {
            builder
                .withdraw_from_account(addresses().account, XRD, 100)
                .take_all_from_worktop(XRD, "xrd")
                .then(|builder| {
                    let bucket = builder.bucket("xrd");
                    builder.call_method(
                        access_controller,
                        ACCESS_CONTROLLER_CONTRIBUTE_RECOVERY_FEE_IDENT,
                        AccessControllerContributeRecoveryFeeManifestInput {
                            bucket,
                        },
                    )
                })
        },
        |builder, access_controller| {
            builder
                .call_method(
                    access_controller,
                    ACCESS_CONTROLLER_WITHDRAW_RECOVERY_FEE_IDENT,
                    AccessControllerWithdrawRecoveryFeeManifestInput {
                        amount: Decimal::one(),
                    },
                )
                .deposit_entire_worktop(addresses().account)
        },
    ];
    for call in calls {
        let manifest = ManifestBuilder::new()
            .allocate_global_address(
                ACCESS_CONTROLLER_PACKAGE,
                ACCESS_CONTROLLER_BLUEPRINT,
                "reservation",
                "access_controller",
            )
            .then(|builder| {
                let access_controller =
                    builder.named_address("access_controller");
                call(builder, access_controller)
            })
            // The reservation is never used: the access controller isn't
            // created.
            .build_no_validate();
        assert_access_controller_class(manifest, None);
    }
}

fn subintent_classes(
    manifest: &SubintentManifestV2,
) -> Vec<ManifestClassification> {
    let StaticAnalysis {
        manifest_classification,
        ..
    } = statically_analyze(manifest).unwrap();
    ACCESS_CONTROLLER_CLASSES
        .into_iter()
        .filter(|class| manifest_classification.contains(class))
        .collect()
}

#[test]
fn enclosed_subintent_is_a_recovery_fee_manifest() {
    let Addresses {
        account,
        access_controller,
        ..
    } = addresses();

    let top_up = ManifestBuilder::new_subintent_v2()
        .assert_worktop_is_empty()
        .withdraw_from_account(account, XRD, 100)
        .take_all_from_worktop(XRD, "xrd")
        .contribute_recovery_fee(access_controller, "xrd")
        .yield_to_parent(())
        .build();
    let withdraw = ManifestBuilder::new_subintent_v2()
        .assert_worktop_is_empty()
        .withdraw_recovery_fee(access_controller, 10)
        .deposit_entire_worktop(account)
        .yield_to_parent(())
        .build();

    for manifest in [top_up, withdraw] {
        assert_eq!(
            subintent_classes(&manifest),
            vec![ManifestClassification::AccessControllerRecoveryFee]
        );
    }
}

/// What a subintent that isn't enclosed starts with is unknown, so what it
/// contributes or deposits from its worktop may be more than XRD.
#[test]
fn subintent_moving_unknown_resources_is_not_classified() {
    let Addresses {
        account,
        access_controller,
        ..
    } = addresses();

    let top_up = ManifestBuilder::new_subintent_v2()
        .withdraw_from_account(account, XRD, 100)
        .take_all_from_worktop(XRD, "xrd")
        .contribute_recovery_fee(access_controller, "xrd")
        // Whatever the parent passed in.
        .deposit_entire_worktop(account)
        .yield_to_parent(())
        .build();
    let withdraw = ManifestBuilder::new_subintent_v2()
        .withdraw_recovery_fee(access_controller, 10)
        .deposit_entire_worktop(account)
        .yield_to_parent(())
        .build();
    let yielding_xrd = ManifestBuilder::new_subintent_v2()
        .assert_worktop_is_empty()
        .withdraw_recovery_fee(access_controller, 10)
        .yield_to_parent(manifest_args!(ManifestExpression::EntireWorktop))
        .build();

    for manifest in [top_up, withdraw, yielding_xrd] {
        assert_eq!(subintent_classes(&manifest), vec![]);
    }
}

/// A subintent that isn't enclosed can't be previewed, so it isn't classified
/// even when it only moves XRD it names.
#[test]
fn open_subintent_moving_only_xrd_is_not_classified() {
    let Addresses {
        account,
        access_controller,
        ..
    } = addresses();

    let top_up = ManifestBuilder::new_subintent_v2()
        .withdraw_from_account(account, XRD, 100)
        .take_all_from_worktop(XRD, "xrd")
        .contribute_recovery_fee(access_controller, "xrd")
        .yield_to_parent(())
        .build();
    let withdraw = ManifestBuilder::new_subintent_v2()
        .withdraw_recovery_fee(access_controller, 10)
        .take_all_from_worktop(XRD, "xrd")
        .try_deposit_or_abort(account, None, "xrd")
        .yield_to_parent(())
        .build();

    for manifest in [top_up, withdraw] {
        assert_eq!(subintent_classes(&manifest), vec![]);
    }
}
